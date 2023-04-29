mod _entry;
mod account;
mod category;
mod post;
mod tag;
mod utils;

use _entry::app::actual_main;

#[tokio::main]
async fn main() {
    // work-around the stupid limitation of Rust modules
    actual_main().await
}
