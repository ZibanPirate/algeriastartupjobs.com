use serde::{Deserialize, Serialize};
use strum_macros::Display;
use utility_types::{omit, partial, pick};

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[serde(tag = "status")] // to flatten the enum to the parent struct
pub enum TaskStatus {
  Pending,
  InProgress,
  Completed,
  Failed { failure_reason: String },
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[serde(tag = "type")] // to flatten the enum to the parent struct
pub enum TaskType {
  Manual { manual_task_owner: u32 },
  Automated,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[serde(tag = "name")] // to flatten the enum to the parent struct
pub enum TaskName {
  Indexing { model_name: String, model_id: u32 },
  RefreshingBKTree,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[omit(DBTask, [id, created_at, updated_at])]
pub struct Task {
  pub id: u32,
  #[serde(flatten)]
  pub name: TaskName,
  #[serde(flatten)]
  pub r#type: TaskType,
  #[serde(flatten)]
  pub status: TaskStatus,
  pub created_at: String,
  pub updated_at: String,
}

pub trait DBTaskTrait {
  fn get_indexing_task_info(&self) -> (Option<String>, Option<u32>);
  fn get_manual_task_info(&self) -> (Option<u32>);
  fn get_failed_task_info(&self) -> (Option<String>);
}

impl DBTaskTrait for DBTask {
  fn get_indexing_task_info(&self) -> (Option<String>, Option<u32>) {
    match &self.name {
      TaskName::Indexing {
        model_name,
        model_id,
      } => (Some(model_name.clone()), Some(*model_id)),
      _ => (None, None),
    }
  }
  fn get_manual_task_info(&self) -> (Option<u32>) {
    match &self.r#type {
      TaskType::Manual { manual_task_owner } => (Some(*manual_task_owner)),
      _ => (None),
    }
  }
  fn get_failed_task_info(&self) -> (Option<String>) {
    match &self.status {
      TaskStatus::Failed { failure_reason } => (Some(failure_reason.clone())),
      _ => (None),
    }
  }
}
