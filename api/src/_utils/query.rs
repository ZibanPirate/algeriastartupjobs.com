use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationQuery {
  page: u32,
  per_page: u32,
}

pub struct DBPaginationQuery {
  pub limit: u32,
  pub start: u32,
}

pub trait PaginationQueryTrait {
  fn to_db_query(&self) -> DBPaginationQuery;
}

impl PaginationQueryTrait for PaginationQuery {
  fn to_db_query(&self) -> DBPaginationQuery {
    let start = self.page * self.per_page;
    let limit = self.per_page;

    DBPaginationQuery { limit, start }
  }
}
