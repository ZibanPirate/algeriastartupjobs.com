use crate::{
  _entry::state::AppState,
  _utils::{post_long_title::get_post_long_title, post_url::get_post_url},
};
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
  pub title: String,
  pub description: String,
  pub image: String,
}

fn read_html(
  ReadHtmlParam {
    file_name,
    title,
    description,
    image,
  }: ReadHtmlParam,
) -> String {
  let file_content = fs::read_to_string(file_name);
  if file_content.is_err() {
    return "".to_string();
  }
  let file_content = file_content.unwrap();

  let file_content = file_content
    .replace(
      "{{HTML_TITLE}}",
      format!("🇩🇿 {} | Algeria Startup Jobs", &title).as_str(),
    )
    .replace("{{HTML_DESCRIPTION}}", &description)
    .replace("{{HTML_IMAGE}}", &image);

  file_content
}

fn return404(app_state: &AppState) -> impl IntoResponse {
  (
    StatusCode::NOT_FOUND,
    Html(read_html(ReadHtmlParam {
      file_name: format!(
        "{}/index.html",
        app_state.config_service.get_config().html_path
      ),
      title: "404 - Page Not Found".to_string(),
      description: "You are in the wrong place".to_string(),
      image: "https://images.unsplash.com/photo-1555861496-0666c8981751?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=1200&h=630&q=80".to_string(),
    })),
  )
}

pub async fn index(State(app_state): State<AppState>) -> impl IntoResponse {
  Html(read_html(ReadHtmlParam {
    file_name: format!(
      "{}/index.html",
      app_state.config_service.get_config().html_path,
    ),
    title: "Join a startup in Algeria".to_string(),
    description: "Algeria Startup Jobs is a job board for startups in Algeria".to_string(),
    image: format!(
      "https://{}.assets.algeriastartupjobs.com/assets/apple-touch-startup-image-1136x640.png",
      app_state.config_service.get_config().stage.as_str()
    ),
  }))
  .into_response()
}

pub async fn jobs(
  Path(job_slug): Path<String>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let post_id = job_slug.split("_").last();
  if post_id.is_none() {
    return return404(&app_state).into_response();
  }
  let post_id = post_id.unwrap().parse::<u32>();
  if post_id.is_err() {
    return return404(&app_state).into_response();
  }
  let post_id = post_id.unwrap();

  let post = app_state.post_repository.get_one_post_by_id(post_id).await;
  if post.is_err() {
    return return404(&app_state).into_response();
  }
  let post = post.unwrap();

  let poster = app_state
    .account_repository
    .get_one_account_by_id(post.poster_id)
    .await;
  if poster.is_err() {
    return return404(&app_state).into_response();
  }
  let poster = poster.unwrap();

  Html(read_html(ReadHtmlParam {
    file_name: format!(
      "{}/index.html",
      app_state.config_service.get_config().html_path
    ),
    title: get_post_long_title(&post, &poster),
    description: post.short_description,
    image: format!(
      "https://{}.assets.algeriastartupjobs.com/assets/apple-touch-startup-image-1136x640.png",
      app_state.config_service.get_config().stage.as_str()
    ),
  }))
  .into_response()
}

pub async fn fallback(State(app_state): State<AppState>) -> impl IntoResponse {
  return404(&app_state).into_response()
}

pub async fn create(State(app_state): State<AppState>) -> impl IntoResponse {
  Html(read_html(ReadHtmlParam {
    file_name: format!(
      "{}/index.html",
      app_state.config_service.get_config().html_path
    ),
    title: "Post a job ad for free".to_string(),
    description: "Free job board for startups in Algeria".to_string(),
    image: format!(
      "https://{}.assets.algeriastartupjobs.com/assets/apple-touch-startup-image-1136x640.png",
      app_state.config_service.get_config().stage.as_str()
    ),
  }))
  .into_response()
}

pub async fn sitemap(State(app_state): State<AppState>) -> impl IntoResponse {
  // @TODO-ZM: fetch post count
  let count = 1_000_000;
  let mut url_string = vec![];

  url_string.extend(vec![
    "/".to_string(),
    "/post_a_job_ad_for_free".to_string(),
    "/import".to_string(),
  ]);

  // @TODO-ZM: fetch only published posts
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

pub async fn import(State(app_state): State<AppState>) -> impl IntoResponse {
  Html(read_html(ReadHtmlParam {
    file_name: format!(
      "{}/index.html",
      app_state.config_service.get_config().html_path
    ),
    title: "Import your job post from other platforms".to_string(),
    description: "Free job board for startups in Algeria".to_string(),
    image: format!(
      "https://{}.assets.algeriastartupjobs.com/assets/apple-touch-startup-image-1136x640.png",
      app_state.config_service.get_config().stage.as_str()
    ),
  }))
  .into_response()
}

pub fn create_web_router() -> Router<AppState> {
  Router::new()
    .route("/", axum::routing::get(index))
    .route("/jobs/*job_slug", axum::routing::get(jobs))
    .route("/post_a_job_ad_for_free", axum::routing::get(create))
    // @TODO-ZM: add robot.txt route
    .route("/import", axum::routing::get(import))
    .route("/sitemap.xml", axum::routing::get(sitemap))
    .route("/*path", axum::routing::get(fallback))
}
