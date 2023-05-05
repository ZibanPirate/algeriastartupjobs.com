#[derive(Debug)]
pub enum DataAccessError {
  NotFound,
  CreationError,
  InternalError,
}

#[derive(Debug)]
pub enum BootError {
  DBSetupError,
  DBLoginError,
  DBNamespaceError,
}
