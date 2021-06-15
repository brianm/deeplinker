pub mod scripts;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeepLink {
    pub link: Option<String>,
    pub title: Option<String>,
}

pub const UNKNOWN: DeepLink = DeepLink {
    link: None,
    title: None,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    // App bundle id, ie: "com.googlecode.iterm2"
    pub id: String,
    // App name, ie: "iTerm2"
    pub name: String,
    // Frontmost window title, ie "osascript -s s ./src/scripts/front_app.applescript"
    pub title: Option<String>,
}

impl App {
    pub fn deep_link(&self) -> Result<DeepLink, Box<dyn Error>> {
        let rs: DeepLink = match &self.id[..] {
            //"com.googlecode.iterm2" => scripts::com_googlecode_iterm2()?,
            "com.apple.Safari" => scripts::com_apple_Safari()?,
            "com.apple.mail" => scripts::com_apple_mail()?,
            "com.google.Chrome" | "org.chromium.Chromium" => scripts::com_google_Chrome()?,
            _ => UNKNOWN,
        };
        Ok(rs)
    }
}
