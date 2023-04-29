use axum::{extract::State, response::IntoResponse, Json, Router};
use hyper::StatusCode;

use crate::_entry::state::AppState;

pub async fn get_all_posts(State(app_state): State<AppState>) -> impl IntoResponse {
    let posts = app_state.post_repository.get_all_posts();
    match posts {
        Ok(posts) => Json(posts).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub fn create_post_router() -> Router<AppState> {
    Router::new().route("/", axum::routing::get(get_all_posts))
}
