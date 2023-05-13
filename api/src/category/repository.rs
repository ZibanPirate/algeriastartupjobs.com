use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::_utils::{
  database::{db_thing_to_id, DBRecord},
  error::DataAccessError,
  string::escape_single_quote,
};

use super::model::{Category, CompactCategory, DBCategory};

pub struct CategoryRepository {
  main_db: Arc<Surreal<Client>>,
}

impl CategoryRepository {
  pub fn new(main_db: Arc<Surreal<Client>>) -> Self {
    Self { main_db }
  }

  pub async fn get_many_compact_categories_by_filter(
    &self,
    filter: &str,
    limit: u32,
    start: u32,
  ) -> Result<Vec<CompactCategory>, DataAccessError> {
    let query = format!(
      r#"
      SELECT slug, name, id.id as id FROM category WHERE {} LIMIT {} START {}
      "#,
      filter, limit, start
    );

    let query_result = self.main_db.query(&query).await;

    match query_result {
      Ok(mut query_result) => {
        let query_result_string = format!("{:?}", query_result);
        let categories: Result<Vec<CompactCategory>, _> = query_result.take(0);
        if categories.as_ref().is_err() {
          tracing::error!(
            "Error while getting many categories by filter, error: {:?} | query: {}",
            categories.as_ref(),
            query_result_string
          );
          return Err(DataAccessError::InternalError);
        }
        if categories.as_ref().unwrap().len() == 0 {
          tracing::info!(
            "No categories found with filter: {} : {:?}",
            filter,
            query_result_string
          );
          return Ok(vec![]);
        }

        let category = categories.unwrap();

        Ok(category)
      }
      Err(_) => Err(DataAccessError::InternalError),
    }
  }

  pub async fn get_many_compact_categories_by_ids(
    &self,
    ids: Vec<u32>,
  ) -> Result<Vec<CompactCategory>, DataAccessError> {
    self
      .get_many_compact_categories_by_filter(
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

  pub async fn get_one_category_by_id(&self, id: u32) -> Result<Category, DataAccessError> {
    let query = format!(
      r#"
      SELECT *, id.id as id FROM category:{{ id: {} }}
      "#,
      id
    );

    let query_result = self.main_db.query(&query).await;

    match query_result {
      Ok(mut query_result) => {
        let category: Result<Option<Category>, _> = query_result.take(0);
        if category.as_ref().is_err() {
          tracing::error!("Error while getting one category by id: {:?}", query_result);
          return Err(DataAccessError::InternalError);
        }
        if category.as_ref().unwrap().is_none() {
          // @TODO-ZM: stringify query_result before calling .take
          tracing::info!("No category found with id: {} : {:?}", id, query_result);
          return Err(DataAccessError::NotFound);
        }

        let category = category.unwrap().unwrap();

        Ok(category)
      }
      Err(_) => Err(DataAccessError::InternalError),
    }
  }

  pub async fn create_one_category(&self, category: DBCategory) -> Result<u32, DataAccessError> {
    let query = format!(
      r#"
      BEGIN TRANSACTION;

      LET $count = (SELECT count() FROM category GROUP BY count)[0].count || 0;

      CREATE category:{{ id: $count }} CONTENT {{
        slug: '{}',
        name: '{}',
        description: '{}',
      }};

      COMMIT TRANSACTION;
      "#,
      escape_single_quote(&category.slug),
      escape_single_quote(&category.name),
      escape_single_quote(&category.description),
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
                  tracing::error!("failed to get created category id {:?}", record);
                  return Err(DataAccessError::InternalError);
                }
              }
            }
            None => {
              tracing::error!("failed to get created category record {:?}", record);
              return Err(DataAccessError::InternalError);
            }
          },
          Err(e) => {
            tracing::error!("failed to get created category record {:?}", e);
            return Err(DataAccessError::InternalError);
          }
        }
      }
      Err(e) => {
        tracing::error!("failed to create category {:?}, query {:?}", e, &query);
        return Err(DataAccessError::CreationError);
      }
    }
  }
}
