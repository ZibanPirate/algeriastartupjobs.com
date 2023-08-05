mod _entry;
mod _test;
mod _utils;
mod account;
mod ai;
mod category;
mod config;
mod email;
mod post;
mod search;
mod security;
mod tag;
mod task;

use _entry::app::actual_main;

#[tokio::main]
async fn main() {
  // work-around the stupid limitation of Rust modules
  actual_main().await
}
