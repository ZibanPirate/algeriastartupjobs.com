use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::_utils::{
  database::{db_thing_to_id, DBRecord},
  error::DataAccessError,
  string::escape_single_quote,
};

use super::model::{CompactTask, DBTask, TaskName, TaskStatus, TaskType};

pub struct TaskRepository {
  pub db: Arc<Surreal<Client>>,
}

impl TaskRepository {
  pub async fn get_many_compact_tasks_by_filter(
    &self,
    filter: &str,
    limit: u32,
    start: u32,
  ) -> Result<Vec<CompactTask>, DataAccessError> {
    let query = format!(
      r#"
      SELECT status, name, id.id as id FROM task WHERE {} LIMIT {} START {}
      "#,
      filter, limit, start
    );

    let query_result = self.db.query(&query).await;

    match query_result {
      Ok(mut query_result) => {
        let query_result_string = format!("{:?}", query_result);
        let tasks: Result<Vec<CompactTask>, _> = query_result.take(0);
        if tasks.as_ref().is_err() {
          tracing::error!(
            "Error while getting many tasks by filter, error: {:?} | query: {}",
            tasks.as_ref(),
            query_result_string
          );
          return Err(DataAccessError::InternalError);
        }
        if tasks.as_ref().unwrap().len() == 0 {
          tracing::info!(
            "No tasks found with filter: {} : {:?}",
            filter,
            query_result_string
          );
          return Err(DataAccessError::NotFound);
        }

        let task = tasks.unwrap();

        Ok(task)
      }
      Err(_) => Err(DataAccessError::InternalError),
    }
  }

  pub async fn get_many_compact_tasks_by_ids(
    &self,
    ids: Vec<u32>,
  ) -> Result<Vec<CompactTask>, DataAccessError> {
    self
      .get_many_compact_tasks_by_filter(
        &format!(
          "array::any([{}])",
          ids
            .iter()
            .map(|id| format!("id.id={}", id))
            .collect::<Vec<String>>()
            .join(", "),
        ),
        100,
        0,
      )
      .await
  }

  pub async fn create_one_task(&self, task: DBTask) -> Result<u32, DataAccessError> {
    let query = format!(
      r#"
      BEGIN TRANSACTION;

      LET $count = (SELECT count() FROM task GROUP BY count)[0].count || 0;

      CREATE task:{{ id: $count }} CONTENT {{
        name: '{}',
        {}
        type: '{}',
        {}
        status: '{}',
        {}
      }};

      COMMIT TRANSACTION;
      "#,
      escape_single_quote(&task.name.to_string()),
      match &task.name {
        TaskName::Indexing {
          model_name,
          model_id,
        } => format!(
          r#"
          model_name: '{}',
          model_id: {},
          "#,
          model_name.to_string(),
          model_id
        ),
      },
      escape_single_quote(&task.r#type.to_string()),
      match &task.r#type {
        TaskType::Manual { manual_task_owner } => format!(
          r#"
          manual_task_owner: '{}',
          "#,
          manual_task_owner,
        ),
        TaskType::Automated {} => "".to_string(),
      },
      escape_single_quote(&task.status.to_string()),
      match &task.status {
        TaskStatus::Failed { failure_reason } => format!(
          r#"
          failure_reason: '{}',
          "#,
          escape_single_quote(failure_reason),
        ),
        _ => "".to_string(),
      }
    );

    let query_result = self.db.query(&query).await;
    match query_result {
      Ok(mut query_result) => {
        let record: Result<Option<DBRecord>, _> = query_result.take(1);

        match record {
          Ok(record) => match record {
            Some(record) => {
              let id = db_thing_to_id(&record.id);
              match id {
                Some(id) => return Ok(id),
                None => {
                  tracing::error!("failed to get created task id {:?}", record);
                  return Err(DataAccessError::InternalError);
                }
              }
            }
            None => {
              tracing::error!("failed to get created task record {:?}", record);
              return Err(DataAccessError::InternalError);
            }
          },
          Err(e) => {
            tracing::error!("failed to get created task record {:?}", e);
            return Err(DataAccessError::InternalError);
          }
        }
      }
      Err(e) => {
        tracing::error!("failed to create task {:?}, query {:?}", e, &query);
        return Err(DataAccessError::CreationError);
      }
    }
  }
}
