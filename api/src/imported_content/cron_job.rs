use crate::{
  _entry::state::AppState,
  _utils::{database::DBOrderDirection, error::BootError},
  imported_content::model::{ImportedContentStatus, PartialImportedContent},
};
use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  time::Duration,
};
use tokio::time::sleep;
use tokio_cron_scheduler::Job;

pub struct ImportedContentCronJob {
  pub app_state: AppState,
}

async fn run_importing_content_cron_job(app_state: AppState) {
  tracing::info!("🚀 Importing");

  let imported_contents = app_state
    .imported_content_repository
    .get_many_pending_imported_content("created_at", DBOrderDirection::DESC, 10, 0)
    .await;

  if imported_contents.is_err() {
    tracing::error!("Error while getting imported_contents");
    return;
  }

  let imported_contents = imported_contents.unwrap();

  if imported_contents.is_empty() {
    tracing::info!("No importing_contents found");
    return;
  }

  tracing::info!("Found {} importing_contents", imported_contents.len());

  let imported_content_ids: Vec<u32> = imported_contents
    .iter()
    .map(|imported_content| imported_content.id)
    .collect();

  let imported_content_status_update_result = app_state
    .imported_content_repository
    .update_status_of_many_imported_contents_by_ids(
      &imported_content_ids,
      ImportedContentStatus::InProgress,
    )
    .await;

  if imported_content_status_update_result.is_err() {
    tracing::error!("Error while updating imported_contents status");
    return;
  }

  for imported_content in imported_contents {
    // @TODO-ZM: actually import content
    sleep(Duration::from_secs(1)).await;
    let mock_json_data = format!(
      r#"{{
      "title": "mock title for {}",
      "description": "mock description for {}"
    }}"#,
      imported_content.source_url, imported_content.source_url,
    );

    let imported_content_update_result = app_state
      .imported_content_repository
      .update_one_imported_content_by_id(
        imported_content.id,
        PartialImportedContent {
          json_data: Some(mock_json_data),
          status: Some(ImportedContentStatus::Completed),
          id: None,
          r#type: None,
          source_url: None,
          created_at: None,
          updated_at: None,
        },
      )
      .await;

    if imported_content_update_result.is_err() {
      tracing::error!("Error while updating imported_content");
      return;
    }
  }

  tracing::info!("✅ Importing done");
}

impl ImportedContentCronJob {
  pub fn create_importing_content_cron_job(&self) -> Result<Job, BootError> {
    let app_state = self.app_state.clone();
    let is_job_running = Arc::new(AtomicBool::new(false));

    let job = Job::new_repeated_async(Duration::from_secs(1), move |_, __| {
      let app_state = app_state.clone();
      let is_job_running = is_job_running.clone();

      return Box::pin(async move {
        let compare_and_swap_result =
          is_job_running.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed);
        if compare_and_swap_result.is_ok() && compare_and_swap_result.unwrap() == false {
          run_importing_content_cron_job(app_state.clone()).await;
          is_job_running.store(false, Ordering::Relaxed);
        } else {
          tracing::info!("⏳ Still importing_content... ");
        }
      });
    });

    if job.is_err() {
      tracing::error!("Error while creating imported content cron job");
      return Err(BootError::CronJobSetupError);
    }
    let job = job.unwrap();

    Ok(job)
  }
}
