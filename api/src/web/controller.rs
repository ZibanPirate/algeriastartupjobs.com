use crate::{_entry::state::AppState, _utils::post_url::get_post_url};
use axum::{
  extract::{Path, State},
  headers::ContentType,
  response::{Html, IntoResponse},
  Router, TypedHeader,
};
use hyper::StatusCode;
use serde::Deserialize;
use sitewriter::{UrlEntry, UrlEntryBuilder};
use std::fs;

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

pub async fn sitemap(State(app_state): State<AppState>) -> impl IntoResponse {
  // @TODO-ZM: fetch post count
  let count = 1_000_000;
  let mut url_string = vec![];

  url_string.extend(vec!["/".to_string(), "/post_a_job_ad_for_free".to_string()]);

  let all_posts = app_state
    .post_repository
    .get_many_compact_posts_by_filter("true", "", count, 0)
    .await;
  if all_posts.is_err() {
    // @TODO-ZM: log error
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let all_posts = all_posts.unwrap();

  // post_ids dedup
  let poster_ids = all_posts
    .iter()
    .map(|post| post.poster_id)
    .collect::<Vec<u32>>();
  let posters = app_state
    .account_repository
    .get_many_compact_accounts_by_ids(poster_ids)
    .await;
  if posters.is_err() {
    // @TODO-ZM: log error
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }
  let posters = posters.unwrap();

  for post in all_posts {
    url_string.push(get_post_url(
      &post,
      &posters
        .iter()
        .find(|poster| poster.id == post.poster_id)
        .unwrap(),
    ));
  }

  // @TODO-ZM: add other info to sitemap urls, like frequency, priority, etc.
  let urls = url_string
    .iter()
    .map(|url| {
      UrlEntryBuilder::default()
        .loc(
          format!("https://www.algeriastartupjobs.com{}", url)
            .parse()
            .unwrap(),
        )
        .build()
        .unwrap()
    })
    .collect::<Vec<UrlEntry>>();

  let xml_content = sitewriter::generate_str(&urls);

  (TypedHeader(ContentType::xml()), xml_content).into_response()
}

pub fn create_web_router() -> Router<AppState> {
  Router::new()
    .route("/", axum::routing::get(index))
    .route("/jobs/:job_slug", axum::routing::get(jobs))
    .route("/sitemap.xml", axum::routing::get(sitemap))
    .route("/*path", axum::routing::get(fallback))
}
