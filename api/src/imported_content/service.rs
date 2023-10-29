use std::sync::Arc;

use reqwest::Url;

use crate::_utils::error::{DataAccessError, ImportError};

use super::{
  model::{DBImportedContent, ImportedContentStatus, ImportedContentType},
  repository::ImportedContentRepository,
};

pub struct ImportedContentService {
  imported_content_repository: Arc<ImportedContentRepository>,
}

impl ImportedContentService {
  pub fn new(imported_content_repository: Arc<ImportedContentRepository>) -> Self {
    Self {
      imported_content_repository,
    }
  }
  pub async fn import_job_post_and_get_status(
    &self,
    url: &str,
    last_known_status: Option<ImportedContentStatus>,
    stop_on_statuses: &Vec<ImportedContentStatus>,
  ) -> Result<(bool, ImportedContentStatus, u32), ImportError> {
    // @TODO-ZM: clean the url
    // @TODO-ZM: allow list
    let url = Url::parse(url);
    if url.is_err() {
      return Err(ImportError::InvalidUrl);
    }
    let url = url.unwrap();

    let mut last_known_status = last_known_status.clone();

    loop {
      let imported_content_id_and_status = match self
        .imported_content_repository
        .get_one_imported_content_by_source_url(&url.to_string())
        .await
      {
        Ok(imported_content) => Ok((imported_content.id, imported_content.status)),
        Err(DataAccessError::NotFound) => {
          let imported_content_id = self
            .imported_content_repository
            .create_one_imported_content(DBImportedContent {
              source_url: url.to_string(),
              r#type: ImportedContentType::JobPost,
              json_data: "".to_string(),
              status: ImportedContentStatus::Pending,
            })
            .await;
          if imported_content_id.is_err() {
            Err(imported_content_id.err().unwrap())
          } else {
            let imported_content_id = imported_content_id.unwrap();
            Ok((imported_content_id, ImportedContentStatus::Pending))
          }
        }
        Err(err) => Err(err),
      };
      if imported_content_id_and_status.is_err() {
        return Err(ImportError::InternalError);
      }
      let (imported_content_id, current_status) = imported_content_id_and_status.unwrap();

      let is_final_status = stop_on_statuses
        .iter()
        .any(|stop_on_status| stop_on_status.to_string() == current_status.to_string());
      let status_changed =
        last_known_status.is_none() || current_status != last_known_status.clone().unwrap();

      if status_changed || is_final_status {
        return Ok((is_final_status, current_status, imported_content_id));
      }

      last_known_status = Some(current_status.clone());

      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
  }
}
