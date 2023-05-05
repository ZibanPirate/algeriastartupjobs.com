use axum::{extract::State, response::IntoResponse, Json, Router};
use fake::Fake;
use hyper::StatusCode;
use serde_json::json;

use crate::{
  _entry::state::AppState,
  _utils::string::{escape_single_quote, slugify},
  account::model::{AccountType, DBAccount},
};

pub async fn seed_the_database_with_mocks(State(app_state): State<AppState>) -> impl IntoResponse {
  let mut account_ids: Vec<u32> = [].to_vec();
  for index in 0..9 {
    let company_name =
      escape_single_quote(&fake::faker::company::en::CompanyName().fake::<String>());
    let slug = slugify(&company_name);
    let account_id = app_state
      .account_repository
      .create_one_account(DBAccount {
        email: format!("test+{}.{}@algeriastartupjobs.com", slug, index),
        slug,
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

  for index in 10..19 {
    let first_name = escape_single_quote(&fake::faker::name::en::FirstName().fake::<String>());
    let last_name = escape_single_quote(&fake::faker::name::en::LastName().fake::<String>());
    let slug = slugify(&format!("{}_{}", first_name, last_name));
    let account_id = app_state
      .account_repository
      .create_one_account(DBAccount {
        email: format!("test+{}.{}@algeriastartupjobs.com", slug, index),
        slug,
        r#type: AccountType::Individual {
          first_name,
          last_name,
        },
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
