
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub size: usize,
    pub name: Option<String>,
    pub mime: String,
    pub delete_at: DateTime<Local>,
    sha512: String
}

impl File {
    pub fn expired(self: &Self) -> bool {
        self.delete_at > chrono::Local::now()
    }
}