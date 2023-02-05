use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LibraryMeta {
    pub sha256: String,
    pub name: String,
    pub categories: Vec<String>,
}
