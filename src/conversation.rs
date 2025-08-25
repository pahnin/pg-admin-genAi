use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
  pub role: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmResponse {
  pub clarification: String,
  pub sql: String,
  pub explanation: String,
}

#[derive(Debug)]
pub struct Conversation {
  pub messages: Vec<ChatMessage>,
}

impl Default for Conversation {
  fn default() -> Self {
    Self::new()
  }
}

impl Conversation {
  pub fn new() -> Self {
    Self { messages: vec![] }
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
}
