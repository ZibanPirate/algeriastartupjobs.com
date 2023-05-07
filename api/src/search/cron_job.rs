use crate::{_entry::state::AppState, _utils::error::BootError};
use std::{sync::Arc, time::Duration};
use tokio_cron_scheduler::Job;

pub struct SearchCronJob {
  pub app_state: Arc<AppState>,
}

// @TODO-ZM: set concurrency to 1
async fn run(app_state: Arc<AppState>) {
  tracing::info!("Indexing");

  let tasks = app_state
    .task_repository
    .get_many_compact_tasks_by_filter("false", 10, 0)
    .await;

  if tasks.is_err() {
    tracing::error!("Error while getting indexing tasks");
    return;
  }
  let tasks = tasks.unwrap();

  if tasks.is_empty() {
    tracing::info!("⏭️  No indexing tasks found, skipping");
    return;
  }

  tracing::info!("Found {} indexing tasks", tasks.len());

  // @TODO-ZM: index content

  tracing::info!("✅ Index done");
}

impl SearchCronJob {
  pub fn create_cron_job(&self) -> Result<Job, BootError> {
    let app_state = self.app_state.clone();

    let job = Job::new_repeated_async(Duration::from_secs(5), move |_, __| {
      let app_state = app_state.clone();

      Box::pin(async move {
        run(app_state).await;
      })
    });

    if job.is_err() {
      tracing::error!("Error while creating search cron job");
      return Err(BootError::CronJobSetupError);
    }
    let job = job.unwrap();

    Ok(job)
  }
}
