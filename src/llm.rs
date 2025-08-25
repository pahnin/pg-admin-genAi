use crate::conversation::{Conversation, LlmResponse};
use anyhow::Result;
use reqwest::Client;

pub async fn send_request(client: &Client, conv: &Conversation) -> Result<LlmResponse> {
  #[derive(serde::Serialize)]
  struct Request<'a> {
    model: &'a str,
    temperature: f32,
    messages: &'a [crate::conversation::ChatMessage],
  }

  let req =
    Request { model: "deepseek-coder-v2:latest", temperature: 0.0, messages: &conv.messages };

  let resp = client
    .post("http://localhost:1234/v1/chat/completions")
    .json(&req)
    .send()
    .await?
    .json::<serde_json::Value>()
    .await?;

  let content = resp["choices"][0]["message"]["content"].as_str().unwrap_or("");
  let cleaned = clean_json(content);
  let parsed: LlmResponse = serde_json::from_str(cleaned)?;

  Ok(parsed)
}

pub fn clean_json(raw: &str) -> &str {
  raw.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim()
}
