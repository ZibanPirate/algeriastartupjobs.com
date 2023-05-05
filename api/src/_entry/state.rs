use std::sync::Arc;

use surrealdb::{
  engine::remote::ws::{Client, Ws},
  opt::auth::Root,
  Surreal,
};

use crate::{
  _utils::error::BootError, account::repository::AccountRepository,
  category::repository::CategoryRepository, post::repository::PostRepository,
  tag::repository::TagRepository,
};

#[derive(Clone)]
pub struct AppState {
  pub db: Arc<Surreal<Client>>,
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

  let db_login = db
    .signin(Root {
      username: "root",
      password: "root",
    })
    .await;

  if db_login.is_err() {
    tracing::error!(
      "Failed to login to the database: {}",
      db_login.err().unwrap()
    );
    return Err(BootError::DBLoginError);
  }

  let db_namespace = db.use_ns("test").use_db("test").await;
  if db_namespace.is_err() {
    tracing::error!(
      "Failed to use the namespace and database: {}",
      db_namespace.err().unwrap()
    );
    return Err(BootError::DBNamespaceError);
  }

  let db = Arc::new(db);

  Ok(AppState {
    db: db.clone(),
    post_repository: Arc::new(PostRepository {}),
    category_repository: Arc::new(CategoryRepository {}),
    tag_repository: Arc::new(TagRepository {}),
    account_repository: Arc::new(AccountRepository { db: db.clone() }),
  })
}
