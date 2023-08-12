use axum::{
  extract::{Query, State},
  response::IntoResponse,
  Json, Router,
};
use hyper::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::{_entry::state::AppState, _utils::error::DataAccessError};

#[derive(Deserialize)]
pub struct EmailQuery {
  pub email: String,
}

pub async fn get_one_account_by_email(
  url_query: Query<EmailQuery>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let account = app_state
    .account_repository
    .get_one_account_by_email(&url_query.email)
    .await;

  match account {
    Ok(account) => Json(json!({
      "account": account,
    }))
    .into_response(),
    Err(DataAccessError::NotFound) => StatusCode::NOT_FOUND.into_response(),
    _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}

pub async fn get_me_account(State(app_state): State<AppState>) -> impl IntoResponse {
  let account = app_state.account_repository.get_one_account_by_id(0).await;

  match account {
    Ok(account) => Json(json!({
      "account": account,
    }))
    .into_response(),
    Err(DataAccessError::NotFound) => StatusCode::NOT_FOUND.into_response(),
    _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}

pub fn create_account_router() -> Router<AppState> {
  Router::new()
    .route("/by_email", axum::routing::get(get_one_account_by_email))
    .route("/me", axum::routing::get(get_me_account))
}
