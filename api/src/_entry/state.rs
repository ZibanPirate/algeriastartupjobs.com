use std::sync::Arc;

use surrealdb::{
  engine::remote::ws::{Client, Ws},
  opt::auth::Root,
  Surreal,
};

use crate::{
  _utils::error::BootError, account::repository::AccountRepository,
  category::repository::CategoryRepository, config::service::ConfigService,
  post::repository::PostRepository, search::service::SearchService, tag::repository::TagRepository,
  task::repository::TaskRepository,
};

#[derive(Clone)]
pub struct AppState {
  pub db: Arc<Surreal<Client>>,
  pub post_repository: Arc<PostRepository>,
  pub category_repository: Arc<CategoryRepository>,
  pub tag_repository: Arc<TagRepository>,
  pub account_repository: Arc<AccountRepository>,
  pub config_service: Arc<ConfigService>,
  pub task_repository: Arc<TaskRepository>,
  pub search_service: Arc<SearchService>,
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
  let config_service = Arc::new(ConfigService {});
  let search_service = Arc::new(SearchService {
    config_service: Arc::clone(&config_service),
  });
  let post_repository = Arc::new(PostRepository {
    db: Arc::clone(&db),
  });
  let category_repository = Arc::new(CategoryRepository {
    db: Arc::clone(&db),
  });
  let tag_repository = Arc::new(TagRepository {
    db: Arc::clone(&db),
  });
  let account_repository = Arc::new(AccountRepository {
    db: Arc::clone(&db),
  });
  let task_repository = Arc::new(TaskRepository {
    db: Arc::clone(&db),
  });

  Ok(AppState {
    db: Arc::clone(&db),
    post_repository: Arc::clone(&post_repository),
    category_repository: Arc::clone(&category_repository),
    tag_repository: Arc::clone(&tag_repository),
    account_repository: Arc::clone(&account_repository),
    config_service: Arc::clone(&config_service),
    task_repository: Arc::clone(&task_repository),
    search_service: Arc::clone(&search_service),
  })
}
