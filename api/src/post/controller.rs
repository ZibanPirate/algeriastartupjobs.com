use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
  Json, Router,
};
use hyper::StatusCode;
use rand::{distributions::Alphanumeric, prelude::Distribution, thread_rng};
use serde::Deserialize;
use serde_json::json;

use crate::{
  _entry::state::AppState,
  _utils::{
    error::DataAccessError,
    query::{PaginationQuery, PaginationQueryTrait},
    vec::sort_and_dedup_vec,
  },
  account::model::DBAccount,
  category::model::CategoryTrait,
  task::model::{DBTask, TaskName, TaskStatus, TaskType},
};

use super::model::{DBPost, PartialPost};

pub async fn get_all_posts_for_feed(State(app_state): State<AppState>) -> impl IntoResponse {
  let compact_posts = app_state
    .post_repository
    .get_many_compact_posts_by_filter("true", 20, 0)
    .await;
  if !compact_posts.is_ok() {
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let compact_posts = compact_posts.unwrap();

  let mut unique_category_ids: Vec<u32> = Vec::new();
  let mut unique_tag_ids: Vec<u32> = Vec::new();
  let mut unique_poster_ids: Vec<u32> = Vec::new();

  for post in compact_posts.iter() {
    unique_category_ids.push(post.category_id);
    unique_tag_ids.append(&mut post.tag_ids.clone());
    unique_poster_ids.push(post.poster_id);
  }

  sort_and_dedup_vec(&mut unique_category_ids);
  sort_and_dedup_vec(&mut unique_tag_ids);
  sort_and_dedup_vec(&mut unique_poster_ids);

  let compact_categories = app_state
    .category_repository
    .get_many_compact_categories_by_ids(unique_category_ids.clone())
    .await;
  if !compact_categories.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let compact_categories = compact_categories.unwrap();

  let compact_tags = app_state
    .tag_repository
    .get_many_compact_tags_by_ids(&unique_tag_ids)
    .await;
  if !compact_tags.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let compact_tags = compact_tags.unwrap();

  let compact_posters = app_state
    .account_repository
    .get_many_compact_accounts_by_ids(unique_poster_ids.clone())
    .await;
  if !compact_posters.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let compact_posters = compact_posters.unwrap();

  Json(json!({
      "posts": compact_posts,
      "categories": compact_categories,
      "tags": compact_tags,
      "posters": compact_posters,
  }))
  .into_response()
}

pub async fn get_one_post_by_id(
  State(app_state): State<AppState>,
  Path(id): Path<u32>,
) -> impl IntoResponse {
  let post = app_state.post_repository.get_one_post_by_id(id).await;

  if !post.is_ok() {
    match post {
      Err(DataAccessError::NotFound) => {
        return StatusCode::NOT_FOUND.into_response();
      }
      _ => {
        // @TODO-ZM: log error reason
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }
  let post = post.unwrap();

  let category = app_state
    .category_repository
    .get_one_category_by_id(post.category_id)
    .await;
  if !category.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let category = category.unwrap();

  let compact_tags = app_state
    .tag_repository
    .get_many_compact_tags_by_ids(&post.tag_ids)
    .await;
  if !compact_tags.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let compact_tags = compact_tags.unwrap();

  let poster = app_state
    .account_repository
    .get_one_account_by_id(post.poster_id)
    .await;
  if !poster.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let poster = poster.unwrap();

  let compact_category = category.to_compact_category();

  Json(json!({
      "post": post,
      "category": compact_category,
      "tags": compact_tags,
      "poster": poster,
  }))
  .into_response()
}

pub async fn get_many_similar_posts_by_id(
  State(app_state): State<AppState>,
  Path(id): Path<u32>,
  pagination: Query<PaginationQuery>,
) -> impl IntoResponse {
  let db_pagination = pagination.to_db_query();
  let similar_compact_posts = app_state
    .post_repository
    .get_many_similar_compact_posts_by_id(id, db_pagination.limit, db_pagination.limit)
    .await;

  if !similar_compact_posts.is_ok() {
    match similar_compact_posts {
      Err(DataAccessError::NotFound) => {
        return StatusCode::NOT_FOUND.into_response();
      }
      _ => {
        // @TODO-ZM: log error reason
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }
  let similar_compact_posts = similar_compact_posts.unwrap();

  let mut unique_category_ids: Vec<u32> = Vec::new();
  let mut unique_tag_ids: Vec<u32> = Vec::new();
  let mut unique_poster_ids: Vec<u32> = Vec::new();

  for post in similar_compact_posts.iter() {
    unique_category_ids.push(post.category_id);
    unique_tag_ids.append(&mut post.tag_ids.clone());
    unique_poster_ids.push(post.poster_id);
  }

  sort_and_dedup_vec(&mut unique_category_ids);
  sort_and_dedup_vec(&mut unique_tag_ids);
  sort_and_dedup_vec(&mut unique_poster_ids);

  let compact_categories = app_state
    .category_repository
    .get_many_compact_categories_by_ids(unique_category_ids.clone())
    .await;
  if !compact_categories.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let compact_categories = compact_categories.unwrap();

  let compact_tags = app_state
    .tag_repository
    .get_many_compact_tags_by_ids(&unique_tag_ids)
    .await;
  if !compact_tags.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let compact_tags = compact_tags.unwrap();

  let compact_posters = app_state
    .account_repository
    .get_many_compact_accounts_by_ids(unique_poster_ids.clone())
    .await;
  if !compact_posters.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let compact_posters = compact_posters.unwrap();

  Json(json!({
      "posts": similar_compact_posts,
      "categories": compact_categories,
      "tags": compact_tags,
      "posters": compact_posters,
  }))
  .into_response()
}

pub async fn get_post_count(State(app_state): State<AppState>) -> impl IntoResponse {
  let post_count = app_state.post_repository.get_post_count().await;

  if !post_count.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let post_count = post_count.unwrap();

  Json(json!({
      "count": post_count,
  }))
  .into_response()
}

#[derive(Deserialize)]
pub struct CreateOnePostBody {
  poster: DBAccount,
  post: DBPost,
}

pub async fn create_one_post_with_poster(
  State(app_state): State<AppState>,
  Json(body): Json<CreateOnePostBody>,
) -> impl IntoResponse {
  match body.poster.r#type.to_string().as_str() {
    "Individual" | "Company" => {}
    _ => {
      return StatusCode::BAD_REQUEST.into_response();
    }
  }

  let poster_id;

  let existing_poster = app_state
    .account_repository
    .get_one_account_by_email(&body.poster.email)
    .await;

  if !existing_poster.is_ok() {
    match existing_poster {
      Err(DataAccessError::NotFound) => {
        let poster_id_result = app_state
          .account_repository
          .create_one_account(&body.poster)
          .await;

        if !poster_id_result.is_ok() {
          // @TODO-ZM: log error reason
          return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        poster_id = poster_id_result.unwrap();
      }
      _ => {
        // @TODO-ZM: log error reason
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  } else {
    poster_id = existing_poster.unwrap().id;
  }

  let post_id = app_state
    .post_repository
    .create_one_post(&DBPost {
      poster_id,
      is_confirmed: false,
      ..body.post.clone()
    })
    .await;

  if !post_id.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let post_id = post_id.unwrap();

  let random_16: String = Alphanumeric
    .sample_iter(&mut thread_rng())
    .take(16)
    .map(char::from)
    .collect();
  let random_16 = random_16.to_uppercase();
  let confirmation_id = &random_16[..12];
  let confirmation_code = &random_16[12..];

  let kv_db_result = app_state
    .main_kv_db
    .insert(post_id.to_be_bytes(), random_16.as_bytes());
  if !kv_db_result.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }

  let email_result = app_state
    .email_service
    .send_one_email(
      &body.poster.email,
      &"Confirm your email".to_string(),
      &format!(
        r#"Your email is used to create a FREE job post at algeriastartupjobs.com with title:

{}

Please confirm your email by copying the code below into the confirmation page:

<div style="width: 100%; text-align: center;">
  <span style="font-size: x-large; letter-spacing: .2em; border: 1px solid #9999; border-radius: .2em; padding: .4em; display: inline-block;">{}</span>
</div>

Thank you for using our service!

ASJ Team
contact@algeriastartupjobs.com
https://www.algeriastartupjobs.com
"#,
        &body.post.title, confirmation_code,
      ),
    )
    .await;

  if !email_result.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }

  Json(json!({
      "post_id": post_id,
      "poster_id": poster_id,
      "confirmation_id": confirmation_id,
  }))
  .into_response()
}

#[derive(Deserialize)]
pub struct ConfirmPostBody {
  post_id: u32,
  confirmation_id: String,
  confirmation_code: String,
}

pub async fn confirm_post(
  State(app_state): State<AppState>,
  Json(body): Json<ConfirmPostBody>,
) -> impl IntoResponse {
  let kv_db_result = app_state.main_kv_db.compare_and_swap(
    body.post_id.to_be_bytes(),
    Some(format!("{}{}", body.confirmation_id, body.confirmation_code).as_bytes()),
    None as Option<&[u8]>,
  );

  if !kv_db_result.is_ok() || kv_db_result.unwrap().is_err() {
    // @TODO-ZM: log error reason
    return StatusCode::UNAUTHORIZED.into_response();
  }

  let update_result = app_state
    .post_repository
    .update_many_posts_by_ids(
      [body.post_id].to_vec(),
      PartialPost {
        id: None,
        slug: None,
        title: None,
        poster_id: None,
        short_description: None,
        description: None,
        category_id: None,
        tag_ids: None,
        is_confirmed: Some(true),
      },
    )
    .await;
  if !update_result.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }

  let post = app_state
    .post_repository
    .get_one_post_by_id(body.post_id)
    .await;
  if !post.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let post = post.unwrap();

  let task_id = app_state
    .task_repository
    .create_one_task(DBTask {
      name: TaskName::Indexing {
        model_name: "post".to_string(),
        model_id: post.id,
      },
      status: TaskStatus::Pending,
      r#type: TaskType::Automated,
    })
    .await;
  if !task_id.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }

  let category = app_state
    .category_repository
    .get_one_category_by_id(post.category_id)
    .await;
  if !category.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let category = category.unwrap();
  let compact_category = category.to_compact_category();

  let poster = app_state
    .account_repository
    .get_one_account_by_id(post.poster_id)
    .await;
  if !poster.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let poster = poster.unwrap();

  let compact_tags = app_state
    .tag_repository
    .get_many_compact_tags_by_ids(&post.tag_ids)
    .await;
  if !compact_tags.is_ok() {
    // @TODO-ZM: log error reason
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let compact_tags = compact_tags.unwrap();

  Json(json!({
      "post": post,
      "category": compact_category,
      "poster": poster,
      "tags": compact_tags,
  }))
  .into_response()
}

pub fn create_post_router() -> Router<AppState> {
  Router::new()
    .route("/feed", axum::routing::get(get_all_posts_for_feed))
    .route("/:post_id", axum::routing::get(get_one_post_by_id))
    .route(
      "/:post_id/similar",
      axum::routing::get(get_many_similar_posts_by_id),
    )
    .route("/count", axum::routing::get(get_post_count))
    .route("/confirm", axum::routing::post(confirm_post))
    .route("/", axum::routing::post(create_one_post_with_poster))
}
