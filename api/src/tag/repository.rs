use super::{mocks::generate_tags_seed, model::Tag};

pub fn get_tag_by_id(tag_id: i32) -> Result<Tag, ()> {
    Ok(generate_tags_seed().get(tag_id as usize).unwrap().clone())
}
