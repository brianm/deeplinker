use log::debug;
use std::error::Error;

use deeplinker::scripts;
use deeplinker::{App, DeepLink};
fn main() -> Result<(), Box<dyn Error>> {
    let env = env_logger::Env::default().filter_or("DL_LOG", "info");
    env_logger::init_from_env(env);

    let app: App = scripts::front_app()?;
    debug!("{:?}", app);

    let output: DeepLink = app.deep_link()?;
    debug!("{:?}", output);

    let txt = output.title.or(app.title).unwrap_or(app.name);
    let link = output.link.unwrap_or(format!("{}://", app.id));
    println!("[{}]({})", txt, link);

    Ok(())
}
