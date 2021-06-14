use std::error::Error;

use deeplinker::scripts;
fn main() -> Result<(), Box<dyn Error>> {
    let app_name: String = scripts::front_app()?;
    println!("{}",app_name);

    let output: scripts::DeepLink = match &app_name[..] {
        "Safari" | "Webkit" => scripts::safari()?,
        "Google Chrome" | "Google Chrome Canary" | "Chromium" => scripts::chrome()?,
        _ => scripts::DeepLink {
            link: String::from("unknown"),
            title: String::from("unavailable"),
        },
    };

    println!("{:?}", output);

    Ok(())
}
