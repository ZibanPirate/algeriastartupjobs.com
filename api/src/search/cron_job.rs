use crate::{
  _entry::state::AppState,
  _utils::error::BootError,
  task::model::{DBTask, PartialTask, TaskName, TaskStatus, TaskType},
};
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

async fn run_indexing_cron_job(app_state: AppState) {
  tracing::info!("🚀 Indexing");

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

  let mut task_ids: Vec<u32> = [].to_vec();
  let mut post_ids = vec![];
  for task in tasks {
    task_ids.push(task.id);
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

  let task_status_update_result = app_state
    .task_repository
    .update_many_tasks_by_ids(
      task_ids,
      PartialTask {
        id: None,
        name: None,
        r#type: None,
        status: Some(TaskStatus::Completed),
      },
    )
    .await;

  if task_status_update_result.is_err() {
    tracing::error!("Error while updating indexing tasks");
    return;
  }

  let task_id = app_state
    .task_repository
    .create_one_task(DBTask {
      name: TaskName::RefreshingBKTree,
      status: TaskStatus::Pending,
      r#type: TaskType::Automated,
    })
    .await;

  if task_id.is_err() {
    tracing::error!("Error while creating bk-tree refreshing task");
  }

  tracing::info!("✅ Indexing done");
}

async fn run_bk_tree_refresher_cron_job(app_state: AppState, has_job_ran_once: bool) {
  tracing::info!("🚀 Refreshing bk-tree");

  let tasks = app_state
    .task_repository
    .get_many_compact_tasks_by_filter("name='RefreshingBKTree' AND status='Pending'", 100, 0)
    .await;

  if tasks.is_err() {
    tracing::error!("Error while getting bk-tree refreshing tasks");
    return;
  }
  let tasks = tasks.unwrap();

  if tasks.is_empty() && has_job_ran_once {
    tracing::info!("⏭️  No bk-tree refreshing tasks found, skipping");
    return;
  }

  tracing::info!("Found {} bk-tree refreshing tasks", tasks.len());

  let mut task_ids: Vec<u32> = [].to_vec();
  for task in tasks {
    task_ids.push(task.id);
  }

  tracing::info!("Refreshing bk-tree");

  let bk_tree_refreshing_result = app_state.search_service.refresh_bk_tree().await;
  if bk_tree_refreshing_result.is_err() {
    tracing::error!("Error while indexing posts");
    return;
  }

  let task_status_update_result = app_state
    .task_repository
    .update_many_tasks_by_ids(
      task_ids,
      PartialTask {
        id: None,
        name: None,
        r#type: None,
        status: Some(TaskStatus::Completed),
      },
    )
    .await;

  if task_status_update_result.is_err() {
    tracing::error!("Error while updating indexing tasks");
    return;
  }

  tracing::info!("✅ Refreshing bk-tree done");
}

impl SearchCronJob {
  pub fn create_indexing_cron_job(&self) -> Result<Job, BootError> {
    let app_state = self.app_state.clone();
    let is_job_running = Arc::new(AtomicBool::new(false));

    let job = Job::new_repeated_async(Duration::from_secs(5), move |_, __| {
      let app_state = app_state.clone();
      let is_job_running = is_job_running.clone();

      return Box::pin(async move {
        let compare_and_swap_result =
          is_job_running.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed);
        if compare_and_swap_result.is_ok() && compare_and_swap_result.unwrap() == false {
          run_indexing_cron_job(app_state.clone()).await;
          is_job_running.store(false, Ordering::Relaxed);
        } else {
          tracing::info!("⏳ Still indexing... ");
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

  pub fn create_bk_tree_refresher_cron_job(&self) -> Result<Job, BootError> {
    let app_state = self.app_state.clone();
    let is_job_running = Arc::new(AtomicBool::new(false));
    let has_job_ran_once = Arc::new(AtomicBool::new(false));

    let job = Job::new_repeated_async(Duration::from_secs(5), move |_, __| {
      let app_state = app_state.clone();
      let is_job_running = is_job_running.clone();
      let has_job_ran_once = has_job_ran_once.clone();

      return Box::pin(async move {
        let compare_and_swap_result =
          is_job_running.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed);
        if compare_and_swap_result.is_ok() && compare_and_swap_result.unwrap() == false {
          run_bk_tree_refresher_cron_job(
            app_state.clone(),
            has_job_ran_once.load(Ordering::Relaxed),
          )
          .await;
          has_job_ran_once.store(true, Ordering::Relaxed);
          is_job_running.store(false, Ordering::Relaxed);
        } else {
          tracing::info!("⏳ Still refreshing bk-tree ... ");
        }
      });
    });

    if job.is_err() {
      tracing::error!("Error while creating bk-tree refresher cron job");
      return Err(BootError::CronJobSetupError);
    }
    let job = job.unwrap();

    Ok(job)
  }
}
