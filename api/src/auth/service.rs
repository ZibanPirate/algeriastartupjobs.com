use std::sync::Arc;

use jsonwebtoken::{EncodingKey, Header};
use rand::{distributions::Alphanumeric, prelude::Distribution, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{_utils::error::AuthError, config::service::ConfigService};

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenScope {
  CreatePost,
  Login,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScopedToken {
  scope: TokenScope,
  id: u32,
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
    scope: TokenScope,
    id: u32,
  ) -> Result<String, AuthError> {
    let header = Header::new(jsonwebtoken::Algorithm::HS512);
    let secret = self.config_service.get_config().jwt_secret;
    let key = EncodingKey::from_secret(secret.as_ref());

    let scoped_token = ScopedToken { scope, id };

    let token = jsonwebtoken::encode(&header, &scoped_token, &key);

    if token.is_err() {
      // @TODO-ZM: log error reason
      return Err(AuthError::InternalError);
    }
    let token = token.unwrap();

    Ok(token)
  }
}
