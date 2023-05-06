pub enum Stage {
  Development,
  Staging,
  Production,
}

impl Stage {
  fn as_str(&self) -> &'static str {
    match self {
      Self::Development => "development",
      Self::Staging => "staging",
      Self::Production => "production",
    }
  }
}

pub struct Config {
  pub stage: Stage,
  pub admin_auth_code: String,
}

pub struct ConfigService {}

impl ConfigService {
  // @TODO-ZM: memoize get_config
  pub fn get_config(&self) -> Config {
    // Load the .env file
    dotenv::dotenv().ok();

    Config {
      stage: match std::env::var("STAGE")
        .expect("STAGE env variable is missing!")
        .as_str()
      {
        "development" => Stage::Development,
        "staging" => Stage::Staging,
        "production" => Stage::Production,
        _ => Stage::Development,
      },
      admin_auth_code: std::env::var("ADMIN_AUTH_CODE")
        .expect("ADMIN_AUTH_CODE env variable is missing!"),
    }
  }
}
