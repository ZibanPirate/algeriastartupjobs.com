#[derive(serde::Deserialize, Debug)]
pub struct SearchRecord {
  pub id: u32,
}

#[derive(serde::Deserialize, Debug)]
pub struct SearchResult<T = SearchRecord> {
  num_hits: u64,
  hits: Vec<T>,
}
