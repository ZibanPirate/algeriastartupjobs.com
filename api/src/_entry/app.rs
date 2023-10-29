use crate::{
  _utils::error::BootError,
  account::controller::create_account_router,
  auth::controller::create_auth_router,
  imported_content::{
    controller::create_imported_content_router, cron_job::ImportedContentCronJob,
  },
  post::controller::create_post_router,
  search::{controller::create_search_router, cron_job::SearchCronJob},
  tag::controller::create_tag_router,
  web::controller::create_web_router,
};
use axum::{routing::get, Json, Router};
use local_ip_address::local_ip;
use serde_json::json;
use std::sync::Arc;
use tokio_cron_scheduler::JobScheduler;

use super::{
  layers::{
    cors::create_cors_layer,
    trace::{create_trace_layer, enable_tracing},
  },
  servers::{local::run_local_server, loopback::run_loopback_server},
  state::{create_app_state, AppState},
};

pub async fn actual_main() {
  let _guard = enable_tracing();

  // create a shared-by-reference state
  let app_state = create_app_state().await.unwrap();

  // setup cron jobs
  let cron_jobs = create_cron_jobs(app_state.clone()).await.unwrap();
  cron_jobs.start().await.unwrap();

  // create the app router
  let app = create_app(app_state.clone()).await.unwrap();

  // get the local IP address of the system
  match local_ip() {
    Ok(ip) => {
      // run both loopback and local servers
      let loopback_server = run_loopback_server(app.clone());
      let local_server = run_local_server(app, ip);
      // await both servers concurrently
      let (_, _) = tokio::join!(loopback_server, local_server);
    }
    Err(e) => {
      // log the error as info and run only the loopback server
      tracing::info!("Running only on the loopback address: {}", e);
      let loopback_server = run_loopback_server(app);
      // await the loopback server
      loopback_server.await;
    }
  }
}

// create and configure the app router
async fn create_app(app_state: AppState) -> Result<Router, BootError> {
  let app = Router::new();
  let app = app
    // @TODO-ZM: align on model naming convention
    .nest("/posts", create_post_router())
    .nest("/search", create_search_router())
    .nest("/accounts", create_account_router())
    .nest("/tags", create_tag_router())
    .nest("/auth", create_auth_router())
    .nest("/web/", create_web_router())
    .nest("/imported_content", create_imported_content_router())
    .route(
      "/",
      get(|| async {
        Json(json!({
          "app": { "version": env!("CARGO_PKG_VERSION") },
          "repository": { "url": env!("CARGO_PKG_REPOSITORY") }
        }))
      }),
    )
    .with_state(app_state);

  let app = app.layer(create_cors_layer()).layer(create_trace_layer());

  Ok(app)
}

// @TODO-ZM: use Arc instead of cloning app_state
async fn create_cron_jobs(app_state: AppState) -> Result<JobScheduler, BootError> {
  let schedule = JobScheduler::new().await;
  if schedule.is_err() {
    return Err(BootError::CronJobSetupError);
  }
  let schedule = schedule.unwrap();

  let search_cron_job = Arc::new(SearchCronJob {
    app_state: app_state.clone(),
  });
  let imported_content = Arc::new(ImportedContentCronJob {
    app_state: app_state.clone(),
  });

  // @TODO-ZM: add un-indexing cron job
  let registration_result = schedule
    .add(search_cron_job.create_indexing_cron_job().unwrap())
    .await;
  if registration_result.is_err() {
    tracing::error!(
      "Error while registering search cron job: {:?}",
      registration_result.err()
    );
    return Err(BootError::CronJobSetupError);
  }

  let registration_result = schedule
    .add(search_cron_job.create_bk_tree_refresher_cron_job().unwrap())
    .await;
  if registration_result.is_err() {
    tracing::error!(
      "Error while registering bk-tree refresher cron job: {:?}",
      registration_result.err()
    );
    return Err(BootError::CronJobSetupError);
  }

  let registration_result = schedule
    .add(
      imported_content
        .create_importing_content_cron_job()
        .unwrap(),
    )
    .await;
  if registration_result.is_err() {
    tracing::error!(
      "Error while registering imported content cron job: {:?}",
      registration_result.err()
    );
    return Err(BootError::CronJobSetupError);
  }

  Ok(schedule)
}
