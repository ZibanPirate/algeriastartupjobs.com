use crate::utils::error::DataAccessError;

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
}
