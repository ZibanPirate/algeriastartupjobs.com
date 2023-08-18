use std::fs;

use axum::{
  extract::{Path, State},
  response::{Html, IntoResponse},
  Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::_entry::state::AppState;

#[derive(Deserialize)]
pub struct EmailQuery {
  pub email: String,
}

struct ReadHtmlParam {
  pub file_name: String,
  // @TODO-ZM: add title, description, image
}

fn read_html(ReadHtmlParam { file_name }: ReadHtmlParam) -> String {
  let file_content = fs::read_to_string(file_name);
  if file_content.is_err() {
    return "".to_string();
  }
  let file_content = file_content.unwrap();

  file_content
}

pub async fn index(State(app_state): State<AppState>) -> impl IntoResponse {
  Html(read_html(ReadHtmlParam {
    file_name: format!(
      "{}/index.html",
      app_state.config_service.get_config().html_path
    ),
  }))
  .into_response()
}

pub async fn jobs(
  Path(job_slug): Path<String>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  Html(read_html(ReadHtmlParam {
    file_name: format!(
      "{}/index.html",
      app_state.config_service.get_config().html_path
    ),
  }))
  .into_response()
}

pub async fn fallback(
  Path(path): Path<String>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  Html(read_html(ReadHtmlParam {
    file_name: format!(
      "{}/index.html",
      app_state.config_service.get_config().html_path
    ),
  }))
  .into_response()
}

pub fn create_web_router() -> Router<AppState> {
  Router::new()
    .route("/", axum::routing::get(index))
    .route("/jobs/:job_slug", axum::routing::get(jobs))
    .route("/*path", axum::routing::get(fallback))
}
