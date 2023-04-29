use super::{mocks::generate_accounts_seed, model::Account};

pub fn get_account_by_id(account_id: i32) -> Result<Account, ()> {
    Ok(generate_accounts_seed()
        .get(account_id as usize)
        .unwrap()
        .clone())
}
