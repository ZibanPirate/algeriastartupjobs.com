use serde::{Deserialize, Serialize};
use utility_types::{omit, partial, pick};

#[omit(DBPost, [id], [Debug, Serialize, Deserialize, Clone])]
#[pick(CompactPost, [id, slug, title, poster_id, short_description, tag_ids, published_at], [Debug, Serialize, Deserialize, Clone])]
#[partial(PartialPost)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
  pub id: u32,
  pub slug: String,
  pub title: String,
  pub poster_id: u32,
  pub short_description: String,
  pub description: String,
  pub tag_ids: Vec<u32>,
  pub is_confirmed: bool,
  pub published_at: String,
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
      tag_ids: self.tag_ids.clone(),
      published_at: self.published_at.clone(),
    }
  }
}

pub trait PartialPostTrait {
  fn to_post(&self, fallback_post: Post) -> Post;
}

impl PartialPostTrait for PartialPost {
  fn to_post(&self, fallback_post: Post) -> Post {
    Post {
      id: self.id.unwrap_or(fallback_post.id),
      slug: self.slug.clone().unwrap_or(fallback_post.slug),
      title: self.title.clone().unwrap_or(fallback_post.title),
      poster_id: self.poster_id.unwrap_or(fallback_post.poster_id),
      short_description: self
        .short_description
        .clone()
        .unwrap_or(fallback_post.short_description),
      description: self
        .description
        .clone()
        .unwrap_or(fallback_post.description),
      tag_ids: self.tag_ids.clone().unwrap_or(fallback_post.tag_ids),
      is_confirmed: self.is_confirmed.unwrap_or(fallback_post.is_confirmed),
      published_at: self
        .published_at
        .clone()
        .unwrap_or(fallback_post.published_at),
    }
  }
}
