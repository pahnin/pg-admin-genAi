use crate::agent::AGENT;
use crate::config::LlmConfig;
use crate::conversation::Conversation;
use crate::ui::results::TableData;
use freya::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
  pub focus_sql: UseFocus,
  pub focus_text: UseFocus,
  pub editable_sql: UseEditable,
  pub editable_nl: UseEditable,
  pub results: Signal<TableData>,
  pub pg_config: Resource<PostgresStatus>,
  pub llm_config: Resource<LlmStatus>,
  pub conversation: Arc<RwLock<Conversation>>,
}

#[derive(Debug, Clone)]
pub enum PostgresStatus {
  MissingConfig,
  ConnectionFailed(String),
  Connected { config: String, tables: Vec<String> },
}

#[derive(Debug, Clone)]
pub enum LlmStatus {
  MissingConfig,
  Connected { config: LlmConfig },
}

pub fn init_state() -> AppState {
  let focus_sql = use_focus();
  let focus_text = use_focus();

  let editable_sql = use_editable(
    || EditableConfig::new("".into()).with_allow_tabs(true),
    EditableMode::MultipleLinesSingleEditor,
  );
  let editable_nl = use_editable(
    || EditableConfig::new("".into()).with_allow_tabs(true),
    EditableMode::MultipleLinesSingleEditor,
  );

  let results = use_signal(|| TableData { headers: vec![], rows: vec![] });

  let pg_config = use_resource(move || async move {
    if let Some(agent) = AGENT.get() {
      let guard = agent.db_client.config.lock().await;

      if let Some(conf) = guard.as_ref() {
        // Try a connection test
        match agent.db_client.try_connect().await {
          Ok(_) => {
            let tables = agent.db_client.list_tables().await.unwrap_or_default();
            PostgresStatus::Connected {
              config: format!(
                "postgresql://{}:{}@{}/{}",
                conf.user, conf.password, conf.host, conf.dbname
              ),
              tables,
            }
          }
          Err(e) => PostgresStatus::ConnectionFailed(e.to_string()),
        }
      } else {
        PostgresStatus::MissingConfig
      }
    } else {
      PostgresStatus::MissingConfig
    }
  });

  let llm_config = use_resource(move || async move {
    if let Some(agent) = AGENT.get() {
      let guard = agent.llm_client.read().await;
      match guard.as_ref() {
        Some(conf) => LlmStatus::Connected { config: conf.clone() },
        None => LlmStatus::MissingConfig,
      }
    } else {
      LlmStatus::MissingConfig
    }
  });

  let conversation = Arc::new(RwLock::new(Conversation::new()));

  AppState {
    focus_sql,
    focus_text,
    editable_sql,
    editable_nl,
    results,
    pg_config,
    llm_config,
    conversation,
  }
}
