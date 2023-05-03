use std::sync::Arc;

use surrealdb::{
  engine::remote::ws::{Client, Ws},
  Surreal,
};

use crate::{
  _utils::error::BootError, account::repository::AccountRepository,
  category::repository::CategoryRepository, post::repository::PostRepository,
  tag::repository::TagRepository,
};

#[derive(Clone)]
pub struct AppState {
  pub db: Surreal<Client>,
  pub post_repository: Arc<PostRepository>,
  pub category_repository: Arc<CategoryRepository>,
  pub tag_repository: Arc<TagRepository>,
  pub account_repository: Arc<AccountRepository>,
}

pub async fn create_app_state() -> Result<AppState, BootError> {
  let db = Surreal::new::<Ws>("127.0.0.1:7070").await;
  if db.is_err() {
    tracing::error!("Failed to setup the database: {}", db.err().unwrap());
    return Err(BootError::DBSetupError);
  }
  let db = db.unwrap();

  Ok(AppState {
    db,
    post_repository: Arc::new(PostRepository {}),
    category_repository: Arc::new(CategoryRepository {}),
    tag_repository: Arc::new(TagRepository {}),
    account_repository: Arc::new(AccountRepository {}),
  })
}
