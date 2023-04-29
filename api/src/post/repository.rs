use super::{mocks::generate_posts_seed, model::Post};

pub fn get_post_by_id(post_id: i32) -> Result<Post, ()> {
    Ok(generate_posts_seed().get(post_id as usize).unwrap().clone())
}
