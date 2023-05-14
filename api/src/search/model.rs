use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRecord {
  pub id: u32,
  pub score: u32,
}
