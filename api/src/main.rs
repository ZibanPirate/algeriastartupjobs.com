mod _entry;
mod _utils;
mod account;
mod ai;
mod auth;
mod config;
mod email;
mod imported_content;
mod post;
mod search;
mod security;
mod tag;
mod task;
mod web;

use _entry::app::actual_main;

#[tokio::main]
async fn main() {
  // work-around the stupid limitation of Rust modules
  actual_main().await
}
