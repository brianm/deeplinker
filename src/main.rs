use std::error::Error;

use deeplinker::scripts;
fn main() -> Result<(), Box<dyn Error>> {
    let app_name = scripts::front_app()?;
    println!("'{}'", app_name);

    let output = match &app_name[..] {
        "Safari" | "Webkit" => scripts::safari()?,
        "Google Chrome" | "Google Chrome Canary" | "Chromium" => scripts::chrome()?,
        _ => String::from("unknown application"),
    };

    println!("{}", output);

    Ok(())
}
