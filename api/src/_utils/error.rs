#[derive(Debug)]
pub enum DataAccessError {
  NotFound,
  InternalError,
}

#[derive(Debug)]
pub enum BootError {
  DBSetupError,
}
