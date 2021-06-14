use std::error::Error;

use deeplinker::{FrontApp, DeepLink};
use deeplinker::scripts;
fn main() -> Result<(), Box<dyn Error>> {
    let app: FrontApp = scripts::front_app()?;
    println!("{:?}", app);

    let output: DeepLink = match &app.name[..] {
        "Safari" | "Webkit" => scripts::com_apple_Safari()?,
        "Google Chrome" | "Google Chrome Canary" | "Chromium" => scripts::com_google_Chrome()?,
        _ => DeepLink {
            link: String::from("unknown"),
            title: String::from("unavailable"),
        },
    };

    println!("{:?}", output);

    Ok(())
}
