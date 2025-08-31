use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Setting {
  pub postgres_profiles: std::collections::HashMap<String, PostgresConfig>,
  pub llm_profiles: std::collections::HashMap<String, LlmConfig>,
  pub active_postgres: String,
  pub active_llm: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostgresConfig {
  pub host: String,
  pub port: u16,
  pub user: String,
  pub password: String,
  pub dbname: String,
}

impl PostgresConfig {
  pub fn as_str(&self) -> String {
    let config_str = format!(
      "host={} user={} password={} dbname={} port={}",
      self.host, self.user, self.password, self.dbname, self.port
    );

    return config_str;
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmConfig {
  pub api_url: String,
  //pub api_key: String,
  pub model: String,
}

impl Setting {
  pub fn config_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    path.push("config.toml");
    path
  }

  pub fn try_load() -> Result<Self, config::ConfigError> {
    let builder = config::Config::builder()
      // set built-in defaults
      /*      .set_default("active_postgres", "local").unwrap()*/
      /*.set_default("active_llm", "default").unwrap()*/
      /*.set_default("postgres_profiles.local.host", "localhost").unwrap()*/
      /*.set_default("postgres_profiles.local.port", 5432).unwrap()*/
      /*.set_default("postgres_profiles.local.user", "postgres").unwrap()*/
      /*.set_default("postgres_profiles.local.password", "password").unwrap()*/
      /*.set_default("postgres_profiles.local.dbname", "postgres").unwrap()*/
      /*.set_default("llm_profiles.default.api_url", "http://localhost:11434/v1").unwrap()*/
      /*.set_default("llm_profiles.default.api_key", "changeme").unwrap()*/
      /*.set_default("llm_profiles.default.model", "llama3").unwrap()*/
      // try to load config.toml (if present)
      .add_source(config::File::with_name("config").required(false))
      // allow env vars like PG_ADMIN__ACTIVE_POSTGRES=remote
      .add_source(config::Environment::with_prefix("PG_ADMIN").separator("__"));

    let cfg = builder.build().unwrap();
    cfg.try_deserialize()
  }

  pub fn active_postgres(&self) -> Option<&PostgresConfig> {
    self.postgres_profiles.get(&self.active_postgres)
  }

  pub fn active_llm(&self) -> Option<&LlmConfig> {
    self.llm_profiles.get(&self.active_llm)
  }
}
