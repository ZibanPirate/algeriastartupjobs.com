use serde::{Deserialize, Serialize};
use utility_types::{omit, partial, pick};

#[omit(DBTag, [id], [Debug, Serialize, Deserialize, Clone])]
#[pick(CompactTag, [id, slug, name], [Debug, Serialize, Deserialize, Clone])]
#[partial(PartialTag)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
  pub id: u32,
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

pub trait PartialTagTrait {
  fn to_tag(&self, fallback_tag: Tag) -> Tag;
}

impl PartialTagTrait for PartialTag {
  fn to_tag(&self, fallback_tag: Tag) -> Tag {
    Tag {
      id: self.id.unwrap_or(fallback_tag.id),
      slug: self.slug.clone().unwrap_or(fallback_tag.slug),
      name: self.name.clone().unwrap_or(fallback_tag.name),
    }
  }
}
