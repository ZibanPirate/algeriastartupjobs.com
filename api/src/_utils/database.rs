use serde::Deserialize;
use strum_macros::Display;

#[derive(Debug, Deserialize)]
pub struct DBCount {
  pub count: u32,
}

#[derive(Display)]
pub enum DBOrderDirection {
  ASC,
  DESC,
}
