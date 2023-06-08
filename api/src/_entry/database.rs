use sled::Db;
use surrealdb::{
  engine::remote::ws::{Client, Ws},
  opt::auth::Root,
  Surreal,
};

use crate::_utils::error::BootError;

pub async fn create_db_client(
  namespace: String,
  database: String,
  setup_query: Option<String>,
) -> Result<Surreal<Client>, BootError> {
  tracing::info!(
    "Setting up the database client for the namespace: {} and database: {}",
    namespace,
    database
  );
  let db = Surreal::new::<Ws>("127.0.0.1:7070").await;
  if db.is_err() {
    tracing::error!("Failed to setup the database: {}", db.err().unwrap());
    return Err(BootError::DBSetupError);
  }
  let db = db.unwrap();

  let db_login = db
    .signin(Root {
      username: "root",
      password: "root",
    })
    .await;

  if db_login.is_err() {
    tracing::error!(
      "Failed to login to the database: {}",
      db_login.err().unwrap()
    );
    return Err(BootError::DBLoginError);
  }

  let db_namespace = db.use_ns(&namespace).use_db(&database).await;
  if db_namespace.is_err() {
    tracing::error!(
      "Failed to use the namespace and database: {}",
      db_namespace.err().unwrap()
    );
    return Err(BootError::DBNamespaceError);
  }

  if setup_query.is_some() {
    let setup_query_result = db.query(setup_query.unwrap()).await;
    if setup_query_result.is_err() {
      tracing::error!(
        "Failed to setup the database: {}",
        setup_query_result.err().unwrap()
      );
      return Err(BootError::DBSetupError);
    }
  }

  Ok(db)
}

pub async fn create_kv_db(path: String) -> Result<Db, BootError> {
  tracing::info!(
    "Setting up the key-value database client for the database: {}",
    path
  );

  let db = sled::open(path);

  if db.is_err() {
    tracing::error!(
      "Failed to setup the key-value database: {}",
      db.err().unwrap()
    );
    return Err(BootError::KVDBSetupError);
  }

  let db = db.unwrap();

  Ok(db)
}
