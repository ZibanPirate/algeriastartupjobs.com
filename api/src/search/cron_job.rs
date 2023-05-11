use crate::{_entry::state::AppState, _utils::error::BootError, task::model::TaskName};
use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  time::Duration,
};
use tokio_cron_scheduler::Job;

pub struct SearchCronJob {
  pub app_state: AppState,
}

async fn run(app_state: AppState) {
  tracing::info!("🚀 Indexing");

  app_state.search_service.setup_search().await;

  let tasks = app_state
    .task_repository
    .get_many_compact_tasks_by_filter("name='Indexing' AND status='Pending'", 10, 0)
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

  let mut post_ids = vec![];
  for task in tasks {
    match task.name {
      TaskName::Indexing {
        model_name,
        model_id,
      } => {
        if model_name == "post" {
          post_ids.push(model_id.clone());
        }
      }
      _ => {}
    }
  }

  tracing::info!("indexing {} posts", post_ids.len());
  let posts = app_state
    .post_repository
    .get_many_posts_by_ids(post_ids)
    .await;
  if posts.is_err() {
    tracing::error!("Error while getting posts");
    return;
  }
  let posts = posts.unwrap();

  let indexing_result = app_state.search_service.index_posts(posts).await;
  if indexing_result.is_err() {
    tracing::error!("Error while indexing posts");
    return;
  }

  tracing::info!("✅ Indexing done");
}

impl SearchCronJob {
  pub fn create_cron_job(&self) -> Result<Job, BootError> {
    let app_state = self.app_state.clone();
    let is_job_running = Arc::new(AtomicBool::new(false));

    let job = Job::new_repeated_async(Duration::from_secs(5), move |_, __| {
      let app_state = app_state.clone();
      let is_job_running = is_job_running.clone();

      return Box::pin(async move {
        let compare_and_swap_result =
          is_job_running.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed);
        if compare_and_swap_result.is_ok() && compare_and_swap_result.unwrap() == false {
          run(app_state.clone()).await;
          is_job_running.store(false, Ordering::Relaxed);
        } else {
          tracing::info!("⏭️  Indexing job is already running, skipping");
        }
      });
    });

    if job.is_err() {
      tracing::error!("Error while creating search cron job");
      return Err(BootError::CronJobSetupError);
    }
    let job = job.unwrap();

    Ok(job)
  }
}
