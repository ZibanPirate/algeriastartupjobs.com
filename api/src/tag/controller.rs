use axum::{extract::State, response::IntoResponse, Json, Router};
use hyper::StatusCode;
use rand::{thread_rng, Rng};
use serde_json::json;

use crate::_entry::state::AppState;

pub async fn get_many_tags_for_job_description(
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let ids = (0..5)
    .map(|_| thread_rng().gen_range(0..100))
    .collect::<Vec<u32>>();
  let suggested_compact_tags = app_state
    .tag_repository
    .get_many_compact_tags_by_ids(&ids)
    .await;

  if !suggested_compact_tags.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let suggested_compact_tags = suggested_compact_tags.unwrap();

  Json(json!({
      "tags": suggested_compact_tags,
  }))
  .into_response()
}

pub fn create_tag_router() -> Router<AppState> {
  Router::new().route(
    "/suggestions_for_description",
    axum::routing::post(get_many_tags_for_job_description),
  )
}
