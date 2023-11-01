use serde::{Deserialize, Serialize};
use strum_macros::Display;
use utility_types::{omit, partial};

// @TODO: add DRY [model]Status
#[derive(Debug, Serialize, Deserialize, Display, Clone, PartialEq)]
#[serde(tag = "status")] // to flatten the enum to the parent struct
pub enum ImportedContentStatus {
  Pending,
  InProgress,
  Completed,
  Failed { failure_reason: String },
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[serde(tag = "type")] // to flatten the enum to the parent struct
pub enum ImportedContentType {
  JobPost,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobJsonData {
  pub title: String,
  pub description: String,
  pub poster: String,
}

#[partial(PartialImportedContent)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[omit(DBImportedContent, [id, created_at, updated_at], [Debug, Serialize, Deserialize, Clone])]
pub struct ImportedContent {
  pub id: u32,
  pub source_url: String,
  #[serde(flatten)]
  pub r#type: ImportedContentType,
  #[serde(flatten)]
  pub status: ImportedContentStatus,
  pub json_data: String,
  pub created_at: String,
  pub updated_at: String,
}

pub trait DBImportedContentTrait {
  fn get_failed_imported_content_info(&self) -> Option<String>;
}

impl DBImportedContentTrait for DBImportedContent {
  fn get_failed_imported_content_info(&self) -> Option<String> {
    match &self.status {
      ImportedContentStatus::Failed { failure_reason } => Some(failure_reason.clone()),
      _ => None,
    }
  }
}
