use crate::_utils::error::SecurityError;

#[derive(Debug)]
pub struct RateLimitConstraint {
  pub id: String,
  pub max_requests: u32,
  pub duration: u32,
}

pub struct SecurityService {}

impl SecurityService {
  pub fn new() -> Self {
    Self {}
  }

  pub fn rate_limit(&self, constraints: Vec<RateLimitConstraint>) -> Result<(), SecurityError> {
    tracing::info!("rate_limit: {:?}", constraints);
    todo!("rate_limit");
    // @TODO-ZM: implement rate limiting
    Err(SecurityError::RateLimitError)
  }
}
