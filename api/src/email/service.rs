use std::sync::Arc;

use crate::{_utils::error::EmailError, config::service::ConfigService};

pub struct EmailService {
  config_service: Arc<ConfigService>,
}

impl EmailService {
  pub fn new(config_service: Arc<ConfigService>) -> Self {
    Self { config_service }
  }

  pub async fn send_one_email(
    &self,
    email: &String,
    reply_to: &String,
    subject: &String,
    body: &String,
  ) -> Result<(), EmailError> {
    println!(
      "Sending email to {} with subject: {} replying to {}",
      email, subject, reply_to
    );
    println!("Body: {}", body);
    Ok(())
  }
}
