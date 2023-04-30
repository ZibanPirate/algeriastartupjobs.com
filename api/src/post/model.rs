use serde::{Deserialize, Serialize};
use utility_types::{partial, pick};

#[pick(CompactPost, [id, slug, title, poster_id, short_description, category_id, tag_ids], [Serialize, Deserialize, Clone])]
#[partial(PartialPost)]
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

pub trait PostTrait {
    fn to_compact_post(&self) -> CompactPost;
}

impl PostTrait for Post {
    fn to_compact_post(&self) -> CompactPost {
        CompactPost {
            id: self.id,
            slug: self.slug.clone(),
            title: self.title.clone(),
            poster_id: self.poster_id,
            short_description: self.short_description.clone(),
            category_id: self.category_id,
            tag_ids: self.tag_ids.clone(),
        }
    }
}
