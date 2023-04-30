use std::sync::Arc;

use crate::{
    account::repository::AccountRepository, category::repository::CategoryRepository,
    post::repository::PostRepository, tag::repository::TagRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub post_repository: Arc<PostRepository>,
    pub category_repository: Arc<CategoryRepository>,
    pub tag_repository: Arc<TagRepository>,
    pub account_repository: Arc<AccountRepository>,
}

pub fn create_app_state() -> AppState {
    AppState {
        post_repository: Arc::new(PostRepository {}),
        category_repository: Arc::new(CategoryRepository {}),
        tag_repository: Arc::new(TagRepository {}),
        account_repository: Arc::new(AccountRepository {}),
    }
}
