use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct ConfigState(pub Mutex<Config>);

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "libraryPath")]
    pub library_path: Option<String>,
}

pub fn create_default_config() -> Config {
    Config { library_path: None }
}
