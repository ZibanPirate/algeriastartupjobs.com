#[derive(Debug)]
pub enum DataAccessError {
  NotFound,
  CreationError,
  UpdateError,
  InternalError,
}

#[derive(Debug)]
pub enum BootError {
  DBSetupError,
  KVDBSetupError,
  DBLoginError,
  DBNamespaceError,
  CronJobSetupError,
}

#[derive(Debug)]
pub enum SearchError {
  InternalError,
}

#[derive(Debug)]
pub enum EmailError {
  InternalError,
}

#[derive(Debug)]
pub enum SecurityError {
  RateLimitError,
  InternalError,
}

#[derive(Debug)]
pub enum AIError {
  InternalError,
}

#[derive(Debug)]
pub enum AuthError {
  InvalidToken,
  InternalError,
}
