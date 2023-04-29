use std::sync::Arc;

use crate::post::repository::PostRepository;

#[derive(Clone)]
pub struct AppState {
    pub post_repository: Arc<PostRepository>,
}

pub fn create_app_state() -> AppState {
    AppState {
        post_repository: Arc::new(PostRepository {}),
    }
}
