use crate::_utils::error::DataAccessError;

use super::{mocks::generate_tags_seed, model::Tag};

pub struct TagRepository {}

impl TagRepository {
  pub fn get_many_tags_by_ids(&self, ids: Vec<i32>) -> Result<Vec<Tag>, DataAccessError> {
    let tags = generate_tags_seed();
    let mut result: Vec<Tag> = Vec::new();
    for id in ids.iter() {
      let tag = tags
        .iter()
        .find(|tag| tag.id == *id)
        .ok_or(DataAccessError::NotFound)?;
      result.push(tag.clone());
    }
    Ok(result)
  }
}
