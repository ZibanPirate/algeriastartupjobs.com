use super::{
  mocks::generate_accounts_seed,
  model::{Account, AccountType, DBAccount},
};
use crate::_utils::{
  database::{db_thing_to_id, DBRecord},
  error::DataAccessError,
  string::escape_single_quote,
};
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub struct AccountRepository {
  pub db: Arc<Surreal<Client>>,
}

impl AccountRepository {
  pub fn get_many_accounts_by_ids(&self, ids: Vec<u32>) -> Result<Vec<Account>, DataAccessError> {
    let accounts = generate_accounts_seed();
    let mut result: Vec<Account> = Vec::new();
    for id in ids.iter() {
      let account = accounts
        .iter()
        .find(|account| account.id == *id)
        .ok_or(DataAccessError::NotFound)?;
      result.push(account.clone());
    }
    Ok(result)
  }

  pub fn get_one_account_by_id(&self, id: u32) -> Result<Account, DataAccessError> {
    let accounts = generate_accounts_seed();
    for account in accounts {
      if account.id == id {
        return Ok(account);
      }
    }
    Err(DataAccessError::NotFound)
  }

  pub async fn create_one_account(&self, account: DBAccount) -> Result<u32, DataAccessError> {
    let query = format!(
      r#"
      BEGIN TRANSACTION;

      LET $count = (SELECT count() FROM account GROUP BY count)[0].count || 0;

      CREATE account:{{ id: $count }} CONTENT {{
        email: '{}',
        slug: '{}',
        type: '{}',
        {}
      }};

      COMMIT TRANSACTION;
      "#,
      escape_single_quote(&account.email),
      escape_single_quote(&account.slug),
      escape_single_quote(&account.r#type.to_string()),
      match account.r#type {
        AccountType::Company { company_name } => format!("company_name: '{}'", company_name),
        AccountType::Admin {
          first_name,
          last_name,
        }
        | AccountType::Individual {
          first_name,
          last_name,
        } => {
          format!(
            "first_name: '{}', last_name: '{}'",
            escape_single_quote(&first_name),
            escape_single_quote(&last_name)
          )
        }
      },
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
                  tracing::error!("failed to get created account id {:?}", record);
                  return Err(DataAccessError::InternalError);
                }
              }
            }
            None => {
              tracing::error!("failed to get created account record {:?}", record);
              return Err(DataAccessError::InternalError);
            }
          },
          Err(e) => {
            tracing::error!("failed to get created account record {:?}", e);
            return Err(DataAccessError::InternalError);
          }
        }
      }
      Err(e) => {
        tracing::error!("failed to create account {:?}, query {:?}", e, &query);
        return Err(DataAccessError::CreationError);
      }
    }
  }
}
