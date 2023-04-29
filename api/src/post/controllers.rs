use axum::{extract::State, response::IntoResponse, Json, Router};
use hyper::StatusCode;

use super::repository::PostRepositoryState;

pub async fn get_all_posts(
    State(post_repository): State<PostRepositoryState>,
) -> impl IntoResponse {
    let posts = post_repository.get_all_posts();
    match posts {
        Ok(posts) => Json(posts).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub fn create_post_router() -> Router<PostRepositoryState> {
    Router::new().route("/", axum::routing::get(get_all_posts))
}
