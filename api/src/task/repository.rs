use serde_json::json;
use sqlx::{Pool, Row, Sqlite};
use std::sync::Arc;

use super::model::{DBTask, DBTaskTrait};
use crate::{
  _utils::{database::DBOrderDirection, error::DataAccessError},
  task::model::Task,
};

pub struct TaskRepository {
  main_sql_db: Arc<Pool<Sqlite>>,
}

impl TaskRepository {
  pub fn new(main_sql_db: Arc<Pool<Sqlite>>) -> Self {
    Self { main_sql_db }
  }

  pub async fn get_many_pending_indexing_tasks(
    &self,
    task_name: &str,
    order_by: &str,
    order_direction: DBOrderDirection,
    limit: u32,
    start: u32,
  ) -> Result<Vec<Task>, DataAccessError> {
    let conn = self.main_sql_db.acquire().await;
    if conn.is_err() {
      tracing::error!("Error while getting sql connection: {:?}", conn);
      return Err(DataAccessError::InternalError);
    }
    let mut conn = conn.unwrap();

    // @TODO-ZM: figure out how query $ replacement work, there is some unneeded "magic" here
    let db_result = sqlx::query(
      format!(
        r#"
      SELECT *
      FROM task
      WHERE status = 'Pending' AND name = $1
      ORDER BY {} {}
      LIMIT $2
      OFFSET $3
      "#,
        order_by, order_direction,
      )
      .as_str(),
    )
    .bind(task_name)
    .bind(limit)
    .bind(start)
    .fetch_all(&mut *conn)
    .await;

    if db_result.is_err() {
      tracing::error!(
        "Error while getting many published tasks: {:?}",
        db_result.err()
      );
      return Err(DataAccessError::InternalError);
    }
    let db_result = db_result.unwrap();

    let mut tasks = Vec::new();

    for row in db_result {
      let json_task = json!({
        "id": row.get::<u32, _>("id"),
        "name": row.get::<String, _>("name"),
        "model_name": row.get::<String, _>("model_name"),
        "model_id": row.get::<u32, _>("model_id"),
        "type": row.get::<String, _>("type"),
        "manual_task_owner": row.get::<Option<String>, _>("manual_task_owner"),
        "status": row.get::<String, _>("status"),
        "failure_reason": row.get::<Option<String>, _>("failure_reason"),
        "created_at": row.get::<String, _>("created_at"),
        "updated_at": row.get::<String, _>("updated_at"),
      });

      let task = serde_json::from_value::<Task>(json_task);
      if task.is_err() {
        tracing::error!("Error while deserializing task: {:?}", task);
        return Err(DataAccessError::InternalError);
      }
      let task = task.unwrap();

      tasks.push(task);
    }

    Ok(tasks)
  }

  pub async fn create_one_task(&self, task: DBTask) -> Result<u32, DataAccessError> {
    let conn = self.main_sql_db.acquire().await;
    if conn.is_err() {
      tracing::error!("Error while getting sql connection: {:?}", conn);
      return Err(DataAccessError::InternalError);
    }
    let mut conn = conn.unwrap();
    let (model_name, model_id) = task.get_indexing_task_info();
    let manual_task_owner = task.get_manual_task_info();
    let failure_reason = task.get_failed_task_info();

    let db_result = sqlx::query(
      r#"
      INSERT INTO task (name, model_name, model_id, type, manual_task_owner, status, failure_reason, created_at, updated_at)
      VALUES ($1, $2, $3, $4, $5, $6, $7, strftime('%Y-%m-%dT%H:%M:%S.%fZ', 'now'), '')
      "#,
    )
    .bind(task.name.to_string())
    .bind(model_name)
    .bind(model_id)
    .bind(task.r#type.to_string())
    .bind(manual_task_owner)
    .bind(task.status.to_string())
    .bind(failure_reason)
    .execute(&mut *conn)
    .await;

    if db_result.is_err() {
      tracing::error!("Error while creating one task: {:?}", db_result);
      return Err(DataAccessError::InternalError);
    }
    let id = db_result.unwrap().last_insert_rowid() as u32;

    Ok(id)
  }

  pub async fn complete_many_tasks_by_ids(&self, ids: Vec<u32>) -> Result<(), DataAccessError> {
    let conn = self.main_sql_db.acquire().await;
    if conn.is_err() {
      tracing::error!("Error while getting sql connection: {:?}", conn);
      return Err(DataAccessError::InternalError);
    }
    let mut conn = conn.unwrap();

    let db_result = sqlx::query(
      format!(
        r#"
      UPDATE task
      SET status = 'Completed', updated_at = strftime('%Y-%m-%dT%H:%M:%S.%fZ', 'now')
      WHERE id IN ({})
      "#,
        ids
          .iter()
          .map(|id| id.to_string())
          .collect::<Vec<String>>()
          .join(","),
      )
      .as_str(),
    )
    .execute(&mut *conn)
    .await;

    if db_result.is_err() {
      tracing::error!("Error while completing many tasks: {:?}", db_result);
      return Err(DataAccessError::InternalError);
    }

    Ok(())
  }
}
