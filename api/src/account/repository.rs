use crate::_utils::error::DataAccessError;

use super::{mocks::generate_accounts_seed, model::Account};

pub struct AccountRepository {}

impl AccountRepository {
  pub fn get_many_accounts_by_ids(&self, ids: Vec<i32>) -> Result<Vec<Account>, DataAccessError> {
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

  pub fn get_one_account_by_id(&self, id: i32) -> Result<Account, DataAccessError> {
    let accounts = generate_accounts_seed();
    for account in accounts {
      if account.id == id {
        return Ok(account);
      }
    }
    Err(DataAccessError::NotFound)
  }
}
