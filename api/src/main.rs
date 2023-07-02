mod _entry;
mod _test;
mod _utils;
mod account;
mod category;
mod config;
mod email;
mod post;
mod search;
mod tag;
mod task;
mod security;

use _entry::app::actual_main;

#[tokio::main]
async fn main() {
  // work-around the stupid limitation of Rust modules
  actual_main().await
}
