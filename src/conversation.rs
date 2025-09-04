use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
  pub role: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmResponse {
  #[serde(default)]
  pub clarification: String,
  #[serde(default)]
  pub sql: String,
  #[serde(default)]
  pub explanation: String,
}

#[derive(Debug, Clone)]
pub struct Conversation {
  pub messages: Vec<ChatMessage>,
  /// Tracks what tables have been asked about and what columns we know
  pub known_tables: HashMap<String, HashSet<String>>,
}

impl Default for Conversation {
  fn default() -> Self {
    Self::new()
  }
}

impl Conversation {
  pub fn new() -> Self {
    Self { messages: vec![], known_tables: HashMap::new() }
  }

  pub fn add_user(&mut self, content: &str) {
    self.messages.push(ChatMessage { role: "user".into(), content: content.into() });
  }

  pub fn add_system(&mut self, content: &str) {
    self.messages.push(ChatMessage { role: "system".into(), content: content.into() });
  }

  pub fn add_assistant(&mut self, content: &str) {
    self.messages.push(ChatMessage { role: "assistant".into(), content: content.into() });
  }
  /// Update knowledge of a table → columns mapping
  pub fn remember_table_columns(&mut self, table: &str, columns: Vec<String>) {
    self.known_tables.entry(table.to_string()).or_default().extend(columns);
  }

  /// Check if we already know all requested columns for a table
  pub fn has_columns_for(&self, table: &str, requested: &[String]) -> bool {
    self
      .known_tables
      .get(table)
      .map(|known| requested.iter().all(|c| known.contains(c)))
      .unwrap_or(false)
  }
}
