use crate::_utils::error::DataAccessError;

use super::{mocks::generate_posts_seed, model::Post};

pub struct PostRepository {}

impl PostRepository {
  pub fn get_all_posts(&self) -> Result<Vec<Post>, DataAccessError> {
    Ok(generate_posts_seed())
  }

  pub fn get_post_by_id(&self, id: i32) -> Result<Post, DataAccessError> {
    let posts = generate_posts_seed();
    for post in posts {
      if post.id == id {
        return Ok(post);
      }
    }
    Err(DataAccessError::NotFound)
  }

  pub fn get_many_similar_posts_by_id(&self, id: i32) -> Result<Vec<Post>, DataAccessError> {
    let posts = generate_posts_seed();
    let current_post = self.get_post_by_id(id).unwrap();
    let mut similar_posts = Vec::new();
    for post in posts {
      if post.id != id
        && post
          .tag_ids
          .iter()
          .any(|tag_id| current_post.tag_ids.contains(tag_id))
      {
        similar_posts.push(post);
      }
    }

    Ok(similar_posts)
  }
}
