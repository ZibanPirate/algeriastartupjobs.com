use std::sync::Arc;

use rand::{distributions::Alphanumeric, prelude::Distribution, thread_rng};

use crate::{
  _utils::error::{AIError, AuthError},
  config::service::ConfigService,
};

enum TokenScope {
  CreatePost,
  Login,
}

pub struct ConfirmationObject {
  pub id: String,
  pub code: String,
}

pub struct AuthService {
  config_service: Arc<ConfigService>,
  main_kv_db: Arc<sled::Db>,
}

impl AuthService {
  pub fn new(config_service: Arc<ConfigService>, main_kv_db: Arc<sled::Db>) -> Self {
    Self {
      config_service,
      main_kv_db,
    }
  }

  pub async fn generate_confirmation_object(
    &self,
    key: String,
  ) -> Result<ConfirmationObject, AuthError> {
    let random_16: String = Alphanumeric
      .sample_iter(&mut thread_rng())
      .take(16)
      .map(char::from)
      .collect();
    let random_16 = random_16.to_uppercase();
    let id = &random_16[..12];
    let code = &random_16[12..];

    let kv_db_result = self.main_kv_db.insert(key, random_16.as_bytes());
    if !kv_db_result.is_ok() {
      // @TODO-ZM: log error reason
      return Err(AuthError::InternalError);
    }

    Ok(ConfirmationObject {
      id: id.to_string(),
      code: code.to_string(),
    })
  }

  pub async fn verify_confirmation_object(
    &self,
    confirmation_object: ConfirmationObject,
  ) -> Result<(), AuthError> {
    let kv_db_result = self.main_kv_db.compare_and_swap(
      confirmation_object.id,
      Some(confirmation_object.code),
      None as Option<&[u8]>,
    );

    if !kv_db_result.is_ok() || kv_db_result.unwrap().is_err() {
      // @TODO-ZM: log error reason
      return Err(AuthError::InternalError);
    }

    Ok(())
  }

  pub async fn generate_scoped_token(
    &self,
    scopes: Vec<TokenScope>,
    account_id: i32,
  ) -> Result<String, AIError> {
    todo!("generate_scoped_token")
  }
}
