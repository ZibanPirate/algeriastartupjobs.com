use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::_utils::{
  database::{db_thing_to_id, DBRecord},
  error::DataAccessError,
  string::escape_single_quote,
};

use super::{
  mocks::generate_tags_seed,
  model::{DBTag, Tag},
};

pub struct TagRepository {
  pub db: Arc<Surreal<Client>>,
}

impl TagRepository {
  pub fn get_many_tags_by_ids(&self, ids: Vec<i32>) -> Result<Vec<Tag>, DataAccessError> {
    let tags = generate_tags_seed();
    let mut result: Vec<Tag> = Vec::new();
    for id in ids.iter() {
      let tag = tags
        .iter()
        .find(|tag| tag.id == *id)
        .ok_or(DataAccessError::NotFound)?;
      result.push(tag.clone());
    }
    Ok(result)
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
