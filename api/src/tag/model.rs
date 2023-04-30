use serde::{Deserialize, Serialize};
use utility_types::{partial, pick};

#[pick(CompactTag, [id, slug, name], [Serialize, Deserialize, Clone])]
#[partial(PartialTag)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i32,
    pub slug: String,
    pub name: String,
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
