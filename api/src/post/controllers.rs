use axum::{extract::State, response::IntoResponse, Json, Router};
use hyper::StatusCode;
use serde_json::json;

use crate::{
    _entry::state::AppState,
    account::model::{AccountTrait, CompactAccount},
    category::model::{CategoryTrait, CompactCategory},
    tag::model::{CompactTag, TagTrait},
    utils::vec::sort_and_dedup_vec,
};

use super::model::{CompactPost, PostTrait};

pub async fn get_all_posts_for_feed(State(app_state): State<AppState>) -> impl IntoResponse {
    let posts = app_state.post_repository.get_all_posts();
    if !posts.is_ok() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let posts = posts.unwrap();

    let mut unique_category_ids: Vec<i32> = Vec::new();
    let mut unique_tag_ids: Vec<i32> = Vec::new();
    let mut unique_poster_ids: Vec<i32> = Vec::new();

    for post in posts.iter() {
        unique_category_ids.push(post.category_id);
        unique_tag_ids.append(&mut post.tag_ids.clone());
        unique_poster_ids.push(post.poster_id);
    }

    let categories = app_state
        .category_repository
        .get_many_categories_by_ids(unique_category_ids.clone());
    if !categories.is_ok() {
        // @TODO-ZM: log error reason
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let categories = categories.unwrap();

    let tags = app_state
        .tag_repository
        .get_many_tags_by_ids(unique_tag_ids.clone());
    if !tags.is_ok() {
        // @TODO-ZM: log error reason
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let tags = tags.unwrap();

    let posters = app_state
        .account_repository
        .get_many_accounts_by_ids(unique_poster_ids.clone());
    if !posters.is_ok() {
        // @TODO-ZM: log error reason
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let posters = posters.unwrap();

    sort_and_dedup_vec(&mut unique_category_ids);
    sort_and_dedup_vec(&mut unique_tag_ids);
    sort_and_dedup_vec(&mut unique_poster_ids);

    let compact_posts = posts
        .iter()
        .map(|post| post.to_compact_post())
        .collect::<Vec<CompactPost>>();

    let compact_categories = categories
        .iter()
        .map(|category| category.to_compact_category())
        .collect::<Vec<CompactCategory>>();

    let compact_tags = tags
        .iter()
        .map(|tag| tag.to_compact_tag())
        .collect::<Vec<CompactTag>>();

    let compact_posters = posters
        .iter()
        .map(|poster| poster.to_compact_account())
        .collect::<Vec<CompactAccount>>();

    Json(json!({
        "posts": compact_posts,
        "categories": compact_categories,
        "tags": compact_tags,
        "posters": compact_posters,
    }))
    .into_response()
}

pub fn create_post_router() -> Router<AppState> {
    Router::new().route("/feed", axum::routing::get(get_all_posts_for_feed))
}
