use axum::{extract::State, response::IntoResponse, Json, Router};
use fake::Fake;
use hyper::StatusCode;
use serde_json::json;

use crate::{
  _entry::state::AppState,
  account::model::{AccountType, DBAccount},
};

pub async fn seed_the_database_with_mocks(State(app_state): State<AppState>) -> impl IntoResponse {
  let mut account_ids: Vec<u32> = [].to_vec();
  for i in 0..10 {
    let company_name = fake::faker::company::en::CompanyName().fake::<String>();
    let account_id = app_state
      .account_repository
      .create_one_account(DBAccount {
        email: format!(
          "test+{}.{}@algeriastartupjobs.com",
          company_name.replace(" ", "_"),
          i
        ),
        slug: format!("{}_{}", company_name, i),
        r#type: AccountType::Company { company_name },
      })
      .await;
    match account_id {
      Ok(account_id) => {
        account_ids.push(account_id);
      }
      Err(e) => {
        tracing::error!("error {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }

  Json(json!({ "account_ids": account_ids })).into_response()
}

pub fn create_test_router() -> Router<AppState> {
  Router::new().route("/seed", axum::routing::post(seed_the_database_with_mocks))
}
