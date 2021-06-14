pub mod scripts;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct DeepLink {
    pub link: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrontApp {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
}