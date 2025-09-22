use crate::config::PostgresConfig;
use anyhow::{Context, anyhow};
use tokio::sync::{Mutex,RwLock};
use std::sync::Arc;
use tokio_postgres::NoTls;
use tracing::{debug, error};

#[derive(Debug)]
pub struct DbClient {
  pub client: Arc<RwLock<Option<tokio_postgres::Client>>>,
  pub config: Arc<RwLock<Option<PostgresConfig>>>,
  pub schema_cache: Arc<RwLock<Option<String>>>
}

impl Default for DbClient {
  fn default() -> Self {
    Self::new()
  }
}

impl DbClient {
  pub fn new() -> DbClient {
    DbClient {
      client: Arc::new(RwLock::new(None)),
      config: Arc::new(RwLock::new(None)),
      schema_cache: Arc::new(RwLock::new(None)),
    }
  }

  pub async fn setup_db_client(&self, config: Option<PostgresConfig>) -> anyhow::Result<()> {
    let Some(conf) = config.clone() else {
      return Err(anyhow!("Missing postgres config"));
    };

    let config_str = conf.as_str();
    debug!("Trying to connect to db server at: {}", config_str);

    match tokio_postgres::connect(&config_str, NoTls).await {
      Ok((client, connection)) => {
        debug!("Postgres client initialized..");

        tokio::spawn(async move {
          if let Err(e) = connection.await {
              error!("error connecting to postgres: {e}");
          } else {
              debug!("Postgres connection is successful..");
          }
        });

        {
          let mut guard = self.client.write().await;
          *guard = Some(client);
        }
        {
          let mut cfg = self.config.write().await;
          *cfg = Some(conf);
        }

        let schema = self.get_tables_and_columns_for_system_prompt().await?;
        let mut schema_guard = self.schema_cache.write().await;
        *schema_guard = Some(schema);

        Ok(())
      }
    Err(e) => {
      Err(anyhow!("Postgres connect error: {e}"))
      }
    }
  }

  pub async fn query(&self, query_string: &str) -> anyhow::Result<Vec<tokio_postgres::Row>> {
    if self.config.read().await.is_none() {
      return Err(anyhow!("Pg Client is not configured"));
    }
    let client = match self.get_db_client().await {
      Some(client) => client,
      None => {
        self.setup_db_client(None).await;
        self.get_db_client().await.or_else(|| {
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
  pub async fn get_db_client(&self) -> anyhow::Result<tokio_postgres::Client> {
    let guard = self.client.read().await;
    guard.ok_or_else(|| anyhow!("No Postgres client available"))
  }

  pub async fn try_connect(&self) -> anyhow::Result<()> {
    if self.get_db_client().await.is_ok() {
      return Ok(());
    }

    let cfg = self.config.read().await.clone();
    if let Some(conf) = cfg {
      self.setup_db_client(Some(conf)).await
    } else {
      Err(anyhow!("Missing Postgres config"))
    }
  }
  pub async fn list_tables(&self) -> anyhow::Result<Vec<String>> {
    self.try_connect().await?;
    let client =
      self.get_db_client().await.ok_or_else(|| anyhow!("No Postgres client available"))?;

    let rows = client
      .query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public';", &[])
      .await?;

    Ok(rows.into_iter().map(|r| r.get::<_, String>(0)).collect())
  }

  pub async fn get_cached_schema(&self) -> Option<String> {
    self.schema_cache.read().await.clone()
  }


  pub async fn get_tables_and_columns_for_system_prompt(&self) -> anyhow::Result<String> {
    self.try_connect().await?;
    let client = self
      .get_db_client()
      .await
      .or_else(|| anyhow!("No Postgres client available"))?;

    // Fetch all tables + columns in `public` schema
    let rows = client
      .query(
          "
          SELECT table_name, column_name, data_type
          FROM information_schema.columns
          WHERE table_schema = 'public'
          ORDER BY table_name, ordinal_position;
          ",
          &[],
      )
      .await?;

    // Group by table
    let mut schema_map: std::collections::BTreeMap<String, Vec<(String, String)>> =
      std::collections::BTreeMap::new();

    for row in rows {
      let table: String = row.get("table_name");
      let col: String = row.get("column_name");
      let dtype: String = row.get("data_type");

      schema_map
        .entry(table)
        .or_default()
        .push((col, dtype));
    }

    // Convert into a compact string for LLM prompt
    let mut schema_str = String::new();
    for (table, cols) in schema_map {
      let cols_str = cols
        .into_iter()
        .map(|(c, d)| format!("{} {}", c, d))
        .collect::<Vec<_>>()
        .join(", ");
      schema_str.push_str(&format!("{}({})\n", table, cols_str));
    }

    Ok(schema_str)
  }

  pub async fn fetch_info(&self, query_string: &str) -> anyhow::Result<String> {
    // list tables (unchanged)
    if query_string.eq_ignore_ascii_case("list all available tables") {
      let rows = self
        .query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'")
        .await
        .context("failed to query information_schema.tables")?;
      // normalize to strings
      let tables: Vec<String> = rows.iter().map(|r| r.get::<_, String>(0)).collect();
      debug!("Fetched list of tables {:?}", tables);
      return Ok(format!("Available tables: {tables:?}"));
    }

    // extract all single-quoted names: " 'a' and 'b' " -> ["a","b"]
    let mut quoted: Vec<String> = Vec::new();
    let parts: Vec<&str> = query_string.split('\'').collect();
    for i in (1..parts.len()).step_by(2) {
      let candidate = parts[i].trim();
      if !candidate.is_empty() {
        quoted.push(candidate.to_string());
      }
    }

    if !quoted.is_empty() {
      // For each table, query column_name and data_type and return aggregated result.
      let mut outputs: Vec<String> = Vec::with_capacity(quoted.len());
      for table in quoted {
        // Query with simple formatting (ok for internal admin tool). Add context so errors are clear.
        let q = format!(
          "SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '{table}'"
        );
        let rows = self.query(&q).await.with_context(|| {
          format!("error querying information_schema.columns for table '{table}'")
        })?;

        let cols: Vec<String> = rows
          .iter()
          .map(|r| format!("{}:{}", r.get::<_, String>(0), r.get::<_, String>(1)))
          .collect();

        debug!("Fetched table info of {}", table);
        outputs.push(format!("Table {table} has columns: {cols:?}"));
      }

      return Ok(outputs.join("\n"));
    }

    // If nothing matched we give a clear error including the original clarification.
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
