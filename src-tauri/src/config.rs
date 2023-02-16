use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use time::OffsetDateTime;

pub struct ConfigState(pub Mutex<CombinedConfig>);

pub struct CombinedConfig(
    pub Config,
    pub DynamicEventList,
    pub LibraryCategoryList,
    pub StudentList,
);

// config.json

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "libraryPath")]
    pub library_path: Option<String>,
}

pub fn create_default_config() -> Config {
    Config { library_path: None }
}

// dynamic-events.json

pub type DynamicEventList = Vec<DynamicEvent>;

#[derive(Serialize, Deserialize, Clone)]
pub struct DynamicEvent {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "startDate", with = "time::serde::timestamp")]
    pub start_date: OffsetDateTime,
    #[serde(rename = "duration")]
    pub duration: i64,
    #[serde(rename = "repeatWeeks")]
    pub repeat_weeks: i64,
    #[serde(rename = "user")]
    pub user: i64,
}

// library-categories.json
pub type LibraryCategoryList = Vec<LibraryCategory>;

#[derive(Serialize, Deserialize, Clone)]
pub struct LibraryCategory {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}

// students.json
pub type StudentList = Vec<Student>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Student {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}
