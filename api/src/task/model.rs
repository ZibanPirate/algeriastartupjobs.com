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

#[pick(CompactTask, [id, name, status], [Debug, Serialize, Deserialize, Clone])]
#[partial(PartialTask)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[omit(DBTask, [id])] // @TODO-ZM: move this down for the other models too
pub struct Task {
  pub id: u32,
  #[serde(flatten)]
  pub name: TaskName,
  #[serde(flatten)]
  pub r#type: TaskType,
  #[serde(flatten)]
  pub status: TaskStatus,
}

pub trait TaskTrait {
  fn to_compact_task(&self) -> CompactTask;
}

impl TaskTrait for Task {
  fn to_compact_task(&self) -> CompactTask {
    CompactTask {
      id: self.id,
      name: self.name.clone(),
      status: self.status.clone(),
    }
  }
}

pub trait PartialTaskTrait {
  fn to_task(&self, fallback_task: Task) -> Task;
}

impl PartialTaskTrait for PartialTask {
  fn to_task(&self, fallback_task: Task) -> Task {
    Task {
      id: self.id.unwrap_or(fallback_task.id),
      name: self.name.clone().unwrap_or(fallback_task.name),
      r#type: self.r#type.clone().unwrap_or(fallback_task.r#type),
      status: self.status.clone().unwrap_or(fallback_task.status),
    }
  }
}
