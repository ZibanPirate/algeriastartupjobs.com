use crate::{
  account::model::{AccountType, CompactAccount},
  post::model::CompactPost,
};

pub fn get_post_url(post: &CompactPost, poster: &CompactAccount) -> String {
  let where_is = match poster.r#type {
    AccountType::Company { .. } => "at",
    _ => "by",
  };
  format!(
    "/jobs/{}_{}_{}_{}",
    post.slug, where_is, poster.slug, post.id
  )
}
