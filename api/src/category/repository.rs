use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::_utils::{
  database::{db_thing_to_id, DBRecord},
  error::DataAccessError,
  string::escape_single_quote,
};

use super::{
  mocks::generate_categories_seed,
  model::{Category, DBCategory},
};

pub struct CategoryRepository {
  pub db: Arc<Surreal<Client>>,
}

impl CategoryRepository {
  pub fn get_many_categories_by_ids(
    &self,
    ids: Vec<u32>,
  ) -> Result<Vec<Category>, DataAccessError> {
    let categories = generate_categories_seed();
    let mut result: Vec<Category> = Vec::new();
    for id in ids.iter() {
      let category = categories
        .iter()
        .find(|category| category.id == *id)
        .ok_or(DataAccessError::NotFound)?;
      result.push(category.clone());
    }
    Ok(result)
  }

  pub fn get_one_category_by_id(&self, id: u32) -> Result<Category, DataAccessError> {
    let categories = generate_categories_seed();
    let category = categories
      .iter()
      .find(|category| category.id == id)
      .ok_or(DataAccessError::NotFound)?;
    Ok(category.clone())
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
