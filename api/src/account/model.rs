use serde::{Deserialize, Serialize};
use utility_types::{partial, pick};

#[derive(Clone, Serialize, Deserialize)]
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

#[pick(CompactAccount, [id, slug, r#type], [Serialize, Deserialize, Clone])]
#[partial(PartialAccount)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub slug: String,
    pub email: String,
    #[serde(flatten)]
    pub r#type: AccountType,
}

pub trait AccountTrait {
    fn get_display_name(&self) -> String;
    fn to_compact_account(&self) -> CompactAccount;
}

impl AccountTrait for Account {
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

    fn to_compact_account(&self) -> CompactAccount {
        CompactAccount {
            id: self.id,
            slug: self.slug.clone(),
            r#type: self.r#type.clone(),
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
