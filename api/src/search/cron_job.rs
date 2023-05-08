use crate::{_entry::state::AppState, _utils::error::BootError, task::model::TaskName};
use std::{sync::Arc, time::Duration};
use tokio_cron_scheduler::Job;

pub struct SearchCronJob {
  pub app_state: Arc<AppState>,
}

// @TODO-ZM: set concurrency to 1
async fn run(app_state: Arc<AppState>) {
  tracing::info!("Indexing");
  app_state.search_service.setup_search().await;

  let tasks = app_state
    .task_repository
    .get_many_compact_tasks_by_filter("name='Indexing'", 10, 0)
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

    let job = Job::new_one_shot_async(Duration::from_secs(1), move |_, __| {
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
