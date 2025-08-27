use crate::config::PostgresConfig;
use anyhow::anyhow;
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;
use tokio_postgres::NoTls;
use tracing::{debug, error};

#[derive(Debug)]
pub struct DbClient {
  pub PG_CLIENT: OnceCell<tokio_postgres::Client>,
  pub config: Mutex<Option<PostgresConfig>>,
}

impl Default for DbClient {
  fn default() -> Self {
    Self::new()
  }
}

impl DbClient {
  pub fn new() -> DbClient {
    Self { PG_CLIENT: OnceCell::new(), config: Mutex::new(None) }
  }

  pub async fn query(&self, query_string: &str) -> anyhow::Result<Vec<tokio_postgres::Row>> {
    if self.config.lock().await.is_none() {
      return Err(anyhow!("Pg Client is not configured"));
    }
    let client = match self.get_db_client().await {
      Some(client) => client,
      None => {
        self.setup_db_client(None).await;
        self.get_db_client().await.ok_or_else(|| {
          error!("Unable to setup PG client");
          anyhow!("Unable to setup pg client")
        })?
      }
    };

    debug!(?query_string);
    let rows = client.query(query_string, &[]).await?;
    //debug!(?rows);
    Ok(rows)
  }

  pub async fn get_db_client(&self) -> Option<&tokio_postgres::Client> {
    self.PG_CLIENT.get()
  }

  pub async fn setup_db_client(&self, config: Option<PostgresConfig>) {
    if config.is_none() {
      error!("Missing postgres config")
    } else {
      let config_copy = config.clone();
      let config_str = config_copy.unwrap().as_str();
      debug!("Trying to connect to db server at: {}", config_str);
      match tokio_postgres::connect(
        //"host=localhost user=postgres password=postgres dbname=postgres",
        //"host=localhost user=postgres password=yourpassword dbname=yourdb port=5432",
        &config_str,
        NoTls,
      )
      .await
      {
        Ok((client, connection)) => {
          debug!("Postgres client initialized..");
          tokio::spawn(async move {
            if let Err(e) = connection.await {
              error!("error connecting to postgres: {e}");
            } else {
              debug!("Postgres connection is successful..");
            }
          });
          self.PG_CLIENT.set(client).unwrap();
        }
        Err(e) => {
          error!("postgres connect err {:?}", e);
        }
      };
      let mut cfg = self.config.lock().await;
      *cfg = config;
    }
  }

  pub async fn fetch_info(&self, query_string: &str) -> anyhow::Result<String> {
    if query_string.contains("list all available tables") {
      let rows = self
        .query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'")
        .await?;
      let tables: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
      debug!("Fetched list of tables {:?}", tables);
      return Ok(format!("Available tables: {tables:?}"));
    }

    if let Some(table) =
      query_string.strip_prefix("What are the columns in '").and_then(|s| s.strip_suffix("'?"))
    {
      let rows = self
        .query(
          format!(
            "SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '{table}'"
          )
          .as_str(),
        )
        .await?;
      let cols: Vec<String> = rows
        .iter()
        .map(|r| format!("{}:{}", r.get::<_, String>(0), r.get::<_, String>(1)))
        .collect();
      debug!("Fetched table info of {}", table);
      return Ok(format!("Table {table} has columns: {cols:?}"));
    }
    Err(anyhow!("I cannot resolve clarification: {}", query_string))
  }
}

#[tokio::test]
async fn test_query_select_users() {
  let _ = tracing_subscriber::fmt().with_env_filter("debug").try_init();
  let client = DbClient::new();
  let config = PostgresConfig {
    host: "localhost".to_string(),
    port: 5432,
    user: "postgres".to_string(),
    password: "password".to_string(),
    dbname: "postgres".to_string(),
  };
  client.setup_db_client(Some(config)).await;
  let rows = client.query("SELECT id, email FROM users LIMIT 5").await.unwrap();
  assert!(!rows.is_empty(), "Expected some users from seed data");
  let email: String = rows[0].get("email");
  debug!("First user email = {}", email);
  assert!(email.contains("@"), "User email should look like an email");
}

#[tokio::test]
async fn test_fetch_info_tables() {
  let _ = tracing_subscriber::fmt().with_env_filter("debug").try_init();
  let client = DbClient::new();
  let config = PostgresConfig {
    host: "localhost".to_string(),
    port: 5432,
    user: "postgres".to_string(),
    password: "password".to_string(),
    dbname: "postgres".to_string(),
  };
  client.setup_db_client(Some(config)).await;
  /*  client.setup_db_client().await;*/
  let result = client.fetch_info("list all available tables").await.unwrap();
  assert!(result.contains("users"), "Should list 'users' table");
  assert!(result.contains("orders"), "Should list 'orders' table");
}

#[tokio::test]
async fn test_fetch_info_columns_books() {
  let _ = tracing_subscriber::fmt().with_env_filter("debug").try_init();
  let client = DbClient::new();
  let config = PostgresConfig {
    host: "localhost".to_string(),
    port: 5432,
    user: "postgres".to_string(),
    password: "password".to_string(),
    dbname: "postgres".to_string(),
  };
  client.setup_db_client(Some(config)).await;
  //client.setup_db_client().await;
  let result = client.fetch_info("What are the columns in 'books'?").await.unwrap();
  assert!(result.contains("title"), "Books table should have 'title'");
  assert!(result.contains("price"), "Books table should have 'price'");
}

#[tokio::test]
async fn test_insert_and_select_review() {
  let _ = tracing_subscriber::fmt().with_env_filter("debug").try_init();
  let client = DbClient::new();
  let config = PostgresConfig {
    host: "localhost".to_string(),
    port: 5432,
    user: "postgres".to_string(),
    password: "password".to_string(),
    dbname: "postgres".to_string(),
  };
  client.setup_db_client(Some(config)).await;
  //client.setup_db_client().await;
  // Insert new review
  let insert_sql = "INSERT INTO reviews (user_id, book_id, rating, comment) VALUES (1, 2, 5, 'Amazing read!') RETURNING id";
  let rows = client.query(insert_sql).await.unwrap();
  assert_eq!(rows.len(), 1, "Should return one row with new id");

  let new_id: i32 = rows[0].get("id");

  // Fetch it back
  let fetch_sql = format!("SELECT comment FROM reviews WHERE id = {new_id}");
  let fetched = client.query(&fetch_sql).await.unwrap();
  let comment: String = fetched[0].get("comment");
  assert_eq!(comment, "Amazing read!");
}

#[tokio::test]
async fn test_error_on_invalid_query() {
  let _ = tracing_subscriber::fmt().with_env_filter("debug").try_init();
  let client = DbClient::new();
  let config = PostgresConfig {
    host: "localhost".to_string(),
    port: 5432,
    user: "postgres".to_string(),
    password: "password".to_string(),
    dbname: "postgres".to_string(),
  };
  client.setup_db_client(Some(config)).await;
  //client.setup_db_client().await;
  let result = client.query("SELECT * FROM non_existing_table").await;
  assert!(result.is_err(), "Querying non-existent table should fail");
}
