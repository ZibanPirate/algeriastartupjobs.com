use super::database::{create_kv_db, create_sql_db};
use crate::{
  _utils::error::BootError, account::repository::AccountRepository, ai::service::AIService,
  auth::service::AuthService, config::service::ConfigService, email::service::EmailService,
  post::repository::PostRepository, search::service::SearchService,
  security::service::SecurityService, tag::repository::TagRepository,
  task::repository::TaskRepository,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
  // @TODO-ZM: remove this from app state
  pub main_kv_db: Arc<sled::Db>,
  pub post_repository: Arc<PostRepository>,
  pub tag_repository: Arc<TagRepository>,
  pub account_repository: Arc<AccountRepository>,
  pub config_service: Arc<ConfigService>,
  pub task_repository: Arc<TaskRepository>,
  pub search_service: Arc<SearchService>,
  pub email_service: Arc<EmailService>,
  pub security_service: Arc<SecurityService>,
  pub ai_service: Arc<AIService>,
  pub auth_service: Arc<AuthService>,
}

pub async fn create_app_state() -> Result<AppState, BootError> {
  let config_service = Arc::new(ConfigService::new());

  let main_sql_db = Arc::new(
    create_sql_db(
      super::database::SQLDBName::Main,
      "sqlite:sqlite_db_data".to_string(),
    )
    .await?,
  );
  let search_sql_db = Arc::new(
    create_sql_db(
      super::database::SQLDBName::Search,
      "sqlite:sqlite_db_data".to_string(),
    )
    .await?,
  );

  let main_kv_db =
    Arc::new(create_kv_db(format!("{}/main", config_service.get_config().kv_db_dir)).await?);
  let rate_limit_kv_db = Arc::new(
    create_kv_db(format!(
      "{}/rate_limit",
      config_service.get_config().kv_db_dir
    ))
    .await?,
  );

  let search_service = Arc::new(SearchService::new(Arc::clone(&search_sql_db)));
  let post_repository = Arc::new(PostRepository::new(Arc::clone(&main_sql_db)));
  let tag_repository = Arc::new(TagRepository::new(Arc::clone(&main_sql_db)));
  let account_repository = Arc::new(AccountRepository::new(Arc::clone(&main_sql_db)));
  let task_repository = Arc::new(TaskRepository::new(Arc::clone(&main_sql_db)));
  let email_service = Arc::new(EmailService::new(Arc::clone(&config_service)));
  let auth_service = Arc::new(AuthService::new(
    Arc::clone(&config_service),
    Arc::clone(&main_kv_db),
  ));
  let security_service = Arc::new(SecurityService::new(Arc::clone(&rate_limit_kv_db)));
  let ai_service = Arc::new(AIService::new(Arc::clone(&config_service)));

  Ok(AppState {
    main_kv_db: Arc::clone(&main_kv_db),
    post_repository: Arc::clone(&post_repository),
    tag_repository: Arc::clone(&tag_repository),
    account_repository: Arc::clone(&account_repository),
    config_service: Arc::clone(&config_service),
    task_repository: Arc::clone(&task_repository),
    search_service: Arc::clone(&search_service),
    email_service: Arc::clone(&email_service),
    security_service: Arc::clone(&security_service),
    ai_service: Arc::clone(&ai_service),
    auth_service: Arc::clone(&auth_service),
  })
}
