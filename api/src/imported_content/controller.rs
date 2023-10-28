use axum::{
  response::sse::{Event, Sse},
  Router,
};
use futures_util::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _;

use crate::_entry::state::AppState;

pub async fn import_status() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
  // @TODO-ZM: trigger and import task, and sub into its updates and stream them here.
  // return a stream of status with "FETCHING", "PROCESSING" then "DONE" every second. and finishes immediately when status is "DONE"
  let stream = stream::iter(vec![
    Event::default().data(r#"{"status": "FETCHING"}"#),
    Event::default().data(r#"{"status": "PROCESSING"}"#),
    Event::default().data(r#"{"status": "DONE", "draft_id":0}"#),
    // Event::default().data(r#"{"status": "ERROR"}"#),
  ])
  .map(Ok)
  .throttle(Duration::from_secs(1));

  // @TODO-ZM: close the stream after some time.
  Sse::new(stream)
}

pub fn create_import_router() -> Router<AppState> {
  Router::new().route("/status", axum::routing::get(import_status))
}
