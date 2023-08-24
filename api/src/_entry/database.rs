use sled::Db;
use sqlx::{Pool, Sqlite};
use std::fmt;
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

#[derive(Debug)]
pub enum SQLDBName {
  Main,
  Search,
}

impl fmt::Display for SQLDBName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SQLDBName::Main => write!(f, "main"),
      SQLDBName::Search => write!(f, "search"),
    }
  }
}

pub async fn create_sql_db(name: SQLDBName, base_url: String) -> Result<Pool<Sqlite>, BootError> {
  tracing::info!(
    "Setting up the SQL database client for the database: {}",
    name
  );

  let pool = sqlx::sqlite::SqlitePool::connect(format!("{}/{}.db", &base_url, name).as_str()).await;
  if pool.is_err() {
    tracing::error!(
      "Failed to connect to the SQL database: {}",
      pool.err().unwrap()
    );
    return Err(BootError::DBSetupError);
  }
  let pool = pool.unwrap();

  tracing::info!("Successfully connected to the SQL database: {}", name);
  tracing::info!("Migrating the SQL database: {}", name);

  let migration_result = match name {
    SQLDBName::Main => sqlx::migrate!("./db/main/migrations").run(&pool).await,
    SQLDBName::Search => sqlx::migrate!("./db/search/migrations").run(&pool).await,
  };
  if migration_result.is_err() {
    tracing::error!(
      "Failed to migrate the SQL database: {}",
      migration_result.err().unwrap()
    );
    return Err(BootError::DBSetupError);
  }
  tracing::info!("Successfully migrated the SQL database: {}", name);

  let db_health = sqlx::query("PRAGMA integrity_check").fetch_all(&pool).await;
  if db_health.is_err() {
    tracing::error!(
      "Failed to query the SQL database: {}",
      db_health.err().unwrap()
    );
    return Err(BootError::DBSetupError);
  }
  let db_health = db_health.unwrap();

  if db_health.len() != 1 {
    tracing::error!("Failed to query the SQL database: {}", name);
    return Err(BootError::DBSetupError);
  }

  tracing::info!("Successfully queried the SQL database: {}", name);

  Ok(pool)
}
