use serde::{Deserialize, Serialize};
use strum_macros::Display;
use utility_types::{omit, partial, pick};

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[serde(tag = "type")] // to flatten the enum to the parent struct
pub enum AccountType {
  Admin {
    first_name: String,
    last_name: String,
  },
  Individual {
    first_name: String,
    last_name: String,
  },
  Company {
    company_name: String,
  },
  // JobSeeker,
}

#[omit(DBAccount, [id], [Serialize, Deserialize, Clone])]
#[pick(CompactAccount, [id, slug, r#type], [Debug, Serialize, Deserialize, Clone])]
#[partial(PartialAccount)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
  pub id: u32,
  pub slug: String,
  pub email: String,
  #[serde(flatten)]
  pub r#type: AccountType,
}

pub trait AccountTrait {
  fn to_compact_account(&self) -> CompactAccount;
}

impl AccountTrait for Account {
  fn to_compact_account(&self) -> CompactAccount {
    CompactAccount {
      id: self.id,
      slug: self.slug.clone(),
      r#type: self.r#type.clone(),
    }
  }
}

pub trait AccountNameTrait {
  fn get_display_name(&self) -> String;
  fn get_names(&self) -> (Option<&String>, Option<&String>, Option<&String>);
}

impl AccountNameTrait for Account {
  fn get_display_name(&self) -> String {
    match &self.r#type {
      AccountType::Admin {
        first_name,
        last_name,
      } => format!("{} {}", first_name, last_name),
      AccountType::Individual {
        first_name,
        last_name,
      } => format!("{} {}", first_name, last_name),
      AccountType::Company { company_name } => company_name.to_string(),
    }
  }
  fn get_names(&self) -> (Option<&String>, Option<&String>, Option<&String>) {
    match &self.r#type {
      AccountType::Admin {
        first_name,
        last_name,
      }
      | AccountType::Individual {
        first_name,
        last_name,
      } => (Some(first_name), Some(last_name), None),
      AccountType::Company { company_name } => (None, None, Some(company_name)),
    }
  }
}

impl AccountNameTrait for DBAccount {
  fn get_display_name(&self) -> String {
    match &self.r#type {
      AccountType::Admin {
        first_name,
        last_name,
      } => format!("{} {}", first_name, last_name),
      AccountType::Individual {
        first_name,
        last_name,
      } => format!("{} {}", first_name, last_name),
      AccountType::Company { company_name } => company_name.to_string(),
    }
  }
  fn get_names(&self) -> (Option<&String>, Option<&String>, Option<&String>) {
    match &self.r#type {
      AccountType::Admin {
        first_name,
        last_name,
      }
      | AccountType::Individual {
        first_name,
        last_name,
      } => (Some(first_name), Some(last_name), None),
      AccountType::Company { company_name } => (None, None, Some(company_name)),
    }
  }
}

impl AccountNameTrait for CompactAccount {
  fn get_display_name(&self) -> String {
    match &self.r#type {
      AccountType::Admin {
        first_name,
        last_name,
      } => format!("{} {}", first_name, last_name),
      AccountType::Individual {
        first_name,
        last_name,
      } => format!("{} {}", first_name, last_name),
      AccountType::Company { company_name } => company_name.to_string(),
    }
  }
  fn get_names(&self) -> (Option<&String>, Option<&String>, Option<&String>) {
    match &self.r#type {
      AccountType::Admin {
        first_name,
        last_name,
      }
      | AccountType::Individual {
        first_name,
        last_name,
      } => (Some(first_name), Some(last_name), None),
      AccountType::Company { company_name } => (None, None, Some(company_name)),
    }
  }
}

pub trait PartialAccountTrait {
  fn to_account(&self, fallback_account: Account) -> Account;
}

impl PartialAccountTrait for PartialAccount {
  fn to_account(&self, fallback_account: Account) -> Account {
    Account {
      id: self.id.unwrap_or(fallback_account.id),
      slug: self.slug.clone().unwrap_or(fallback_account.slug),
      email: self.email.clone().unwrap_or(fallback_account.email),
      r#type: self.r#type.clone().unwrap_or(fallback_account.r#type),
    }
  }
}
