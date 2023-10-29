use super::model::{
  DBImportedContent, DBImportedContentTrait, ImportedContent, ImportedContentStatus,
  PartialImportedContent,
};
use crate::_utils::{database::DBOrderDirection, error::DataAccessError};
use serde_json::json;
use sqlx::{Pool, Row, Sqlite};
use std::sync::Arc;

pub struct ImportedContentRepository {
  main_sql_db: Arc<Pool<Sqlite>>,
}

impl ImportedContentRepository {
  pub fn new(main_sql_db: Arc<Pool<Sqlite>>) -> Self {
    Self { main_sql_db }
  }

  pub async fn create_one_imported_content(
    &self,
    imported_content: DBImportedContent,
  ) -> Result<u32, DataAccessError> {
    let conn = self.main_sql_db.acquire().await;
    if conn.is_err() {
      tracing::error!("Error while getting sql connection: {:?}", conn);
      return Err(DataAccessError::InternalError);
    }
    let mut conn = conn.unwrap();
    let failure_reason = imported_content.get_failed_imported_content_info();

    let db_result = sqlx::query(
      r#"
      INSERT INTO imported_content (source_url, type, json_data, status, failure_reason, created_at, updated_at)
      VALUES ($1, $2, $3, $4, $5, strftime('%Y-%m-%dT%H:%M:%S.%fZ', 'now'), '')
      "#,
    )
    .bind(imported_content.source_url.to_string())
    .bind(imported_content.r#type.to_string())
    .bind(imported_content.json_data.to_string())
    .bind(imported_content.status.to_string())
    .bind(failure_reason)
    .execute(&mut *conn)
    .await;

    if db_result.is_err() {
      tracing::error!("Error while creating one imported_content: {:?}", db_result);
      return Err(DataAccessError::InternalError);
    }
    let id = db_result.unwrap().last_insert_rowid() as u32;

    Ok(id)
  }

  pub async fn get_one_imported_content_by_source_url(
    &self,
    source_url: &str,
  ) -> Result<ImportedContent, DataAccessError> {
    let conn = self.main_sql_db.acquire().await;
    if conn.is_err() {
      tracing::error!("Error while getting sql connection: {:?}", conn);
      return Err(DataAccessError::InternalError);
    }
    let mut conn = conn.unwrap();

    let db_result = sqlx::query(
      r#"
      SELECT * FROM imported_content WHERE source_url = $1
      "#,
    )
    .bind(source_url)
    .fetch_one(&mut *conn)
    .await;

    if db_result.is_err() {
      match db_result.err().unwrap() {
        sqlx::Error::RowNotFound => {
          return Err(DataAccessError::NotFound);
        }
        err => {
          tracing::error!("Error while getting one imported_content by id: {:?}", err);
          return Err(DataAccessError::InternalError);
        }
      }
    }

    let db_result = db_result.unwrap();

    let json_imported_content = json!({
      "id": db_result.get::<u32, _>("id"),
      "source_url": db_result.get::<String, _>("source_url"),
      "type": db_result.get::<String, _>("type"),
      "json_data": db_result.get::<String, _>("json_data"),
      "status": db_result.get::<String, _>("status"),
      "failure_reason": db_result.get::<Option<String>, _>("failure_reason"),
      "created_at": db_result.get::<String, _>("created_at"),
      "updated_at": db_result.get::<String, _>("updated_at"),
    });

    let imported_content = serde_json::from_value::<ImportedContent>(json_imported_content);

    if imported_content.is_err() {
      tracing::error!(
        "Error while deserializing imported_content: {:?}",
        imported_content
      );
      return Err(DataAccessError::InternalError);
    }
    let imported_content = imported_content.unwrap();

    Ok(imported_content)
  }

  pub async fn get_many_pending_imported_content(
    &self,
    order_by: &str,
    order_direction: DBOrderDirection,
    limit: u32,
    start: u32,
  ) -> Result<Vec<ImportedContent>, DataAccessError> {
    let conn = self.main_sql_db.acquire().await;
    if conn.is_err() {
      tracing::error!("Error while getting sql connection: {:?}", conn);
      return Err(DataAccessError::InternalError);
    }
    let mut conn = conn.unwrap();

    let db_result = sqlx::query(
      format!(
        r#"
      SELECT *
      FROM imported_content
      WHERE status != 'Completed'
      ORDER BY {} {}
      LIMIT $1
      OFFSET $2
      "#,
        order_by, order_direction,
      )
      .as_str(),
    )
    .bind(limit)
    .bind(start)
    .fetch_all(&mut *conn)
    .await;

    if db_result.is_err() {
      tracing::error!(
        "Error while getting many pending imported_content: {:?}",
        db_result.err()
      );
      return Err(DataAccessError::InternalError);
    }
    let db_result = db_result.unwrap();

    let mut imported_contents = Vec::new();

    for row in db_result {
      let json_imported_content = json!({
        "id": row.get::<u32, _>("id"),
        "source_url": row.get::<String, _>("source_url"),
        "type": row.get::<String, _>("type"),
        "json_data": row.get::<String, _>("json_data"),
        "status": row.get::<String, _>("status"),
        "failure_reason": row.get::<Option<String>, _>("failure_reason"),
        "created_at": row.get::<String, _>("created_at"),
        "updated_at": row.get::<String, _>("updated_at"),
      });

      let imported_content = serde_json::from_value::<ImportedContent>(json_imported_content);

      if imported_content.is_err() {
        tracing::error!(
          "Error while deserializing imported_content: {:?}",
          imported_content
        );
        return Err(DataAccessError::InternalError);
      }
      let imported_content = imported_content.unwrap();

      imported_contents.push(imported_content);
    }

    Ok(imported_contents)
  }

  pub async fn update_status_of_many_imported_contents_by_ids(
    &self,
    ids: &Vec<u32>,
    status: ImportedContentStatus,
  ) -> Result<(), DataAccessError> {
    let conn = self.main_sql_db.acquire().await;
    if conn.is_err() {
      tracing::error!("Error while getting sql connection: {:?}", conn);
      return Err(DataAccessError::InternalError);
    }
    let mut conn = conn.unwrap();

    let db_result = sqlx::query(
      format!(
        r#"
      UPDATE imported_content
      SET status = '{}', updated_at = strftime('%Y-%m-%dT%H:%M:%S.%fZ', 'now')
      WHERE id IN ({})
      "#,
        status,
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
      tracing::error!(
        "Error while completing many imported_content by ids: {:?}",
        db_result.err()
      );
      return Err(DataAccessError::InternalError);
    }

    Ok(())
  }

  pub async fn update_one_imported_content_by_id(
    &self,
    id: u32,
    imported_content: PartialImportedContent,
  ) -> Result<(), DataAccessError> {
    let conn = self.main_sql_db.acquire().await;
    if conn.is_err() {
      tracing::error!("Error while getting sql connection: {:?}", conn);
      return Err(DataAccessError::InternalError);
    }
    let mut conn = conn.unwrap();

    let mut update_fields: Vec<(String, String)> = vec![];
    if imported_content.status.is_some() {
      update_fields.push((
        "status".to_string(),
        imported_content.status.unwrap().to_string(),
      ));
    }
    if imported_content.json_data.is_some() {
      update_fields.push((
        "json_data".to_string(),
        imported_content.json_data.unwrap().to_string(),
      ));
    }

    let db_result = sqlx::query(
      format!(
        r#"
      UPDATE imported_content
      SET {}
      WHERE id = {}
      "#,
        update_fields
          .iter()
          .map(|(field, value)| format!("{} = '{}'", field, value))
          .collect::<Vec<String>>()
          .join(","),
        id,
      )
      .as_str(),
    )
    .execute(&mut *conn)
    .await;

    if db_result.is_err() {
      tracing::error!(
        "Error while completing many imported_content by ids: {:?}",
        db_result.err()
      );
      return Err(DataAccessError::InternalError);
    }

    Ok(())
  }
}
