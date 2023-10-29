use serde_json::json;
use sqlx::{Pool, Row, Sqlite};
use std::sync::Arc;

use super::model::{DBImportedContent, DBImportedContentTrait, ImportedContent};
use crate::_utils::error::DataAccessError;

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
}
