use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRecord {
  pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult<T = SearchRecord> {
  pub num_hits: u64,
  pub hits: Vec<T>,
}
