use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::_utils::{
  database::{db_thing_to_id, DBRecord},
  error::DataAccessError,
  string::escape_single_quote,
};

use super::model::{CompactTag, DBTag};

pub struct TagRepository {
  main_db: Arc<Surreal<Client>>,
}

impl TagRepository {
  pub fn new(main_db: Arc<Surreal<Client>>) -> Self {
    Self { main_db }
  }

  pub async fn get_many_compact_tags_by_filter(
    &self,
    filter: &str,
    limit: u32,
    start: u32,
  ) -> Result<Vec<CompactTag>, DataAccessError> {
    let query = format!(
      r#"
      SELECT slug, name, id.id as id FROM tag WHERE {} LIMIT {} START {}
      "#,
      filter,
      if limit > 0 { limit } else { 1 },
      start
    );

    let query_result = self.main_db.query(&query).await;

    match query_result {
      Ok(mut query_result) => {
        let query_result_string = format!("{:?}", query_result);
        let tags: Result<Vec<CompactTag>, _> = query_result.take(0);
        if tags.as_ref().is_err() {
          tracing::error!(
            "Error while getting many tags by filter, error: {:?} | query: {}",
            tags.as_ref(),
            query_result_string
          );
          return Err(DataAccessError::InternalError);
        }
        if tags.as_ref().unwrap().len() == 0 {
          return Ok(vec![]);
        }

        let tag = tags.unwrap();

        Ok(tag)
      }
      Err(_) => {
        tracing::error!(
          "Error while getting many tags by filter, error: {:?} | query: {}",
          query_result,
          query
        );
        return Err(DataAccessError::InternalError);
      }
    }
  }

  pub async fn get_many_compact_tags_by_ids(
    &self,
    ids: &Vec<u32>,
  ) -> Result<Vec<CompactTag>, DataAccessError> {
    self
      .get_many_compact_tags_by_filter(
        &format!(
          "array::any([{}])",
          ids
            .iter()
            .map(|id| format!("id.id={}", id))
            .collect::<Vec<String>>()
            .join(", "),
        ),
        ids.len() as u32,
        0,
      )
      .await
  }

  pub async fn create_one_tag(&self, tag: DBTag) -> Result<u32, DataAccessError> {
    let query = format!(
      r#"
      BEGIN TRANSACTION;

      LET $count = (SELECT count() FROM tag GROUP BY count)[0].count || 0;

      CREATE tag:{{ id: $count }} CONTENT {{
        slug: '{}',
        name: '{}',
      }};

      COMMIT TRANSACTION;
      "#,
      escape_single_quote(&tag.slug),
      escape_single_quote(&tag.name),
    );

    let query_result = self.main_db.query(&query).await;
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
                  tracing::error!("failed to get created tag id {:?}", record);
                  return Err(DataAccessError::InternalError);
                }
              }
            }
            None => {
              tracing::error!("failed to get created tag record {:?}", record);
              return Err(DataAccessError::InternalError);
            }
          },
          Err(e) => {
            tracing::error!("failed to get created tag record {:?}", e);
            return Err(DataAccessError::InternalError);
          }
        }
      }
      Err(e) => {
        tracing::error!("failed to create tag {:?}, query {:?}", e, &query);
        return Err(DataAccessError::CreationError);
      }
    }
  }
}
