use serde::{Deserialize, Serialize};

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

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub slug: String,
    pub email: String,
    #[serde(flatten)]
    pub r#type: AccountType,
}

// @TODO-ZM: write a Partial proc derive marco
pub struct PartialAccount {
    pub id: Option<i32>,
    pub slug: Option<String>,
    pub email: Option<String>,
    pub r#type: Option<AccountType>,
}
