use crate::utils::error::DataAccessError;

use super::{mocks::generate_posts_seed, model::Post};

pub struct PostRepository {}

impl PostRepository {
    pub fn get_all_posts(&self) -> Result<Vec<Post>, DataAccessError> {
        Ok(generate_posts_seed())
    }
}
