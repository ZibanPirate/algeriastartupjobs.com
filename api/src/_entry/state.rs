use super::database::create_db_client;
use crate::{
  _utils::error::BootError, account::repository::AccountRepository,
  category::repository::CategoryRepository, config::service::ConfigService,
  post::repository::PostRepository, search::service::SearchService, tag::repository::TagRepository,
  task::repository::TaskRepository,
};
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Clone)]
pub struct AppState {
  pub main_db: Arc<Surreal<Client>>,
  pub search_db: Arc<Surreal<Client>>,
  pub post_repository: Arc<PostRepository>,
  pub category_repository: Arc<CategoryRepository>,
  pub tag_repository: Arc<TagRepository>,
  pub account_repository: Arc<AccountRepository>,
  pub config_service: Arc<ConfigService>,
  pub task_repository: Arc<TaskRepository>,
  pub search_service: Arc<SearchService>,
}

pub async fn create_app_state() -> Result<AppState, BootError> {
  let main_db = Arc::new(create_db_client("asj".to_string(), "main".to_string(), None).await?);
  let search_db = Arc::new(
    create_db_client(
      "asj".to_string(),
      "search".to_string(),
      Some(
        r#"
        DEFINE INDEX word ON TABLE word FIELDS word;
        DEFINE INDEX model_id ON TABLE word FIELDS model_id;
        DEFINE INDEX appear_in ON TABLE word FIELDS appear_in;
      "#
        .to_string(),
      ),
    )
    .await?,
  );

  let config_service = Arc::new(ConfigService::new());
  let search_service = Arc::new(SearchService::new(Arc::clone(&search_db)));
  let post_repository = Arc::new(PostRepository::new(Arc::clone(&main_db)));
  let category_repository = Arc::new(CategoryRepository::new(Arc::clone(&main_db)));
  let tag_repository = Arc::new(TagRepository::new(Arc::clone(&main_db)));
  let account_repository = Arc::new(AccountRepository::new(Arc::clone(&main_db)));
  let task_repository = Arc::new(TaskRepository::new(Arc::clone(&main_db)));

  Ok(AppState {
    main_db: Arc::clone(&main_db),
    search_db: Arc::clone(&search_db),
    post_repository: Arc::clone(&post_repository),
    category_repository: Arc::clone(&category_repository),
    tag_repository: Arc::clone(&tag_repository),
    account_repository: Arc::clone(&account_repository),
    config_service: Arc::clone(&config_service),
    task_repository: Arc::clone(&task_repository),
    search_service: Arc::clone(&search_service),
  })
}
