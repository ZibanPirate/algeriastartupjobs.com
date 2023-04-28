use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i32,
    pub slug: String,
    pub name: String,
}

// @TODO-ZM: write a Partial proc derive marco
pub struct PartialTag {
    pub id: Option<i32>,
    pub slug: Option<String>,
    pub name: Option<String>,
}
