use crate::{
  account::model::{Account, AccountNameTrait, AccountType},
  post::model::Post,
};

pub fn get_post_long_title(post: &Post, poster: &Account) -> String {
  let poster_extension = format!(
    " needed {} {}",
    match poster.r#type {
      AccountType::Company { .. } => "at",
      _ => "by",
    },
    poster.get_display_name()
  );

  format!("{}{}", post.title, poster_extension)
}
