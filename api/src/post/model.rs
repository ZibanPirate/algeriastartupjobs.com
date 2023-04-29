use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub poster_id: i32,
    pub short_description: String,
    pub description: String,
    pub category_id: i32,
    pub tag_ids: Vec<i32>,
}

// @TODO-ZM: write a Partial proc derive marco
pub struct PartialPost {
    pub id: Option<i32>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub poster_id: Option<i32>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub tag_ids: Option<Vec<i32>>,
}
