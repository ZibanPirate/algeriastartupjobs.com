use serde::{Deserialize, Serialize};
use utility_types::pick;

#[pick(CompactTag, [id, slug, name], [Serialize, Deserialize, Clone])]
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

pub trait TagTrait {
    fn to_compact_tag(&self) -> CompactTag;
}

impl TagTrait for Tag {
    fn to_compact_tag(&self) -> CompactTag {
        CompactTag {
            id: self.id,
            slug: self.slug.clone(),
            name: self.name.clone(),
        }
    }
}
