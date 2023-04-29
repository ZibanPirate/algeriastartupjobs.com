use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub description: String,
}

// @TODO-ZM: write a Partial proc derive marco
pub struct PartialCategory {
    pub id: Option<i32>,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}
