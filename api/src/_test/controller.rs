use axum::{extract::State, http::header::HeaderMap, response::IntoResponse, Json, Router};
use fake::Fake;
use hyper::StatusCode;
use serde_json::json;

use crate::{
  _entry::state::AppState,
  _utils::{is_admin::is_admin, string::slugify},
  account::model::{AccountType, DBAccount},
  category::model::DBCategory,
  post::model::DBPost,
  tag::model::DBTag,
  task::model::{DBTask, TaskName, TaskStatus, TaskType},
};

pub async fn seed_the_database_with_mocks(
  State(app_state): State<AppState>,
  headers: HeaderMap,
) -> impl IntoResponse {
  if is_admin(&app_state, headers).is_none() {
    return StatusCode::UNAUTHORIZED.into_response();
  }

  let mut account_ids: Vec<u32> = [].to_vec();
  for index in 0..9 {
    let company_name = fake::faker::company::en::CompanyName().fake::<String>();
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
    let first_name = fake::faker::name::en::FirstName().fake::<String>();
    let last_name = fake::faker::name::en::LastName().fake::<String>();
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

  let mut category_ids: Vec<u32> = [].to_vec();
  for _ in 0..10 {
    let name = fake::faker::lorem::en::Sentence(1..3).fake::<String>();
    let slug = slugify(&name);
    let description = fake::faker::lorem::en::Paragraph(2..10).fake::<String>();
    let category_id = app_state
      .category_repository
      .create_one_category(DBCategory {
        slug,
        name,
        description,
      })
      .await;
    match category_id {
      Ok(category_id) => {
        category_ids.push(category_id);
      }
      Err(e) => {
        tracing::error!("error {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }

  let mut tag_ids: Vec<u32> = [].to_vec();
  for _ in 0..50 {
    let name = fake::faker::lorem::en::Sentence(3..5).fake::<String>();
    let slug = slugify(&name);
    let tag_id = app_state
      .tag_repository
      .create_one_tag(DBTag { slug, name })
      .await;
    match tag_id {
      Ok(tag_id) => {
        tag_ids.push(tag_id);
      }
      Err(e) => {
        tracing::error!("error {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }

  let mut post_ids: Vec<u32> = [].to_vec();
  let mut task_ids: Vec<u32> = [].to_vec();
  for index in 0..200 {
    let title = fake::faker::lorem::en::Sentence(3..5).fake::<String>();
    let slug = slugify(&title);
    let post_id = app_state
      .post_repository
      .create_one_post(DBPost {
        slug,
        title,
        category_id: category_ids[index % category_ids.len()],
        poster_id: account_ids[index % account_ids.len()],
        description: fake::faker::lorem::en::Paragraph(20..30).fake::<String>(),
        short_description: fake::faker::lorem::en::Sentence(5..10).fake::<String>(),
        tag_ids: tag_ids
          .iter()
          .skip(index % tag_ids.len())
          .take(3)
          .map(|tag_id| *tag_id)
          .collect::<Vec<u32>>(),
      })
      .await;
    match post_id {
      Ok(post_id) => {
        let task_id = app_state
          .task_repository
          .create_one_task(DBTask {
            name: TaskName::Indexing {
              model_name: "post".to_string(),
              model_id: post_id,
            },
            status: TaskStatus::Pending,
            r#type: TaskType::Automated,
          })
          .await;
        match task_id {
          Ok(task_id) => {
            post_ids.push(post_id);
            task_ids.push(task_id);
          }
          Err(e) => {
            tracing::error!("error {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
          }
        }
      }
      Err(e) => {
        tracing::error!("error {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }

  Json(json!({
    "account_ids": account_ids,
    "category_ids": category_ids,
    "tag_ids": tag_ids,
    "post_ids": post_ids,
    "task_ids": task_ids,
  }))
  .into_response()
}

pub async fn clean_the_database_from_mocks(
  State(app_state): State<AppState>,
  headers: HeaderMap,
) -> impl IntoResponse {
  // @TODO-ZM: move this to a middleware with access to the app_state (cloned)
  if is_admin(&app_state, headers).is_none() {
    return StatusCode::UNAUTHORIZED.into_response();
  }

  let query = format!(
    r#"
    DELETE account;
    DELETE post;
    DELETE tag;
    DELETE category;
    DELETE task;
    "#,
  );

  let query_result = app_state.db.query(query.as_str()).await;

  match query_result {
    Ok(_) => return StatusCode::NO_CONTENT.into_response(),
    Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}

pub fn create_test_router() -> Router<AppState> {
  Router::new()
    .route("/seed", axum::routing::post(seed_the_database_with_mocks))
    .route("/clean", axum::routing::post(clean_the_database_from_mocks))
}
