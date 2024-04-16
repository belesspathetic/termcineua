use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Cartoon {
    pub category: Category,
    pub title: String,
    pub seria_title: Option<String>,
    pub season: Option<u8>,
    pub seria: Option<u8>,
    pub description: Option<String>,
    pub stream: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Category {
    Film,
    Cartoon,
    Anime,
}
