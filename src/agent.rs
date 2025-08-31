use crate::config::LlmConfig;
use crate::db_client::DbClient;
use crate::llm::send_request;
use anyhow::anyhow;
use once_cell::sync::OnceCell;
use tokio::sync::RwLock;
use tracing::debug;
pub static AGENT: OnceCell<Agent> = OnceCell::new();
use crate::conversation::Conversation;

static SYSTEM_PROMPT: &str = r#"
You are a Postgres SQL assistant.

Your ONLY output must be raw JSON, without markdown fences (```), without explanations, without extra text.

Schema:
{
  "explanation": "string",
  "sql": "string",
  "clarification": "string"
}

Rules:
- Do NOT wrap the JSON in ```json or ``` markers.
- Do NOT add any text outside JSON.
- Do NOT jump to providing SQL without having full input
- You must follow the schema even if the value is blank the key should be present
- ONLY if you are 100% confident of the SQL, put it in "sql" otherwise you MUST ask clarifications
- when you need schema details, put your question in "clarification".
- options available for clarification are 'list all available tables', 'What are the columns in ' you MUST follow same syntax
- Never guess schema or table names or assume column names, ask clarification first
- You must NOT return BOTH "sql" and "clarification" empty, either one must be present
- Your response MUST be directly parsable JSON.

Example (the ONLY allowed format):
{ "explanation": "To formulate versions SQL, I need to know columns for table 'versions'", "sql": "", "clarification": "What are the columns in 'versions'?" }
{ "explanation": "To suggest SQL you asked, since you have already given column info of verions", "sql": "select * from versions;", "clarification": "" }
{ "explanation": "Before coming to conclusion on what tables to join for the question you asked, we need to know all other tables available and their schema", "sql": "", "clarification": "list all available tables" }
"#;

#[derive(Debug)]
pub struct Agent {
  pub db_client: DbClient,
  pub llm_client: RwLock<Option<LlmConfig>>,
}

impl Agent {
  pub async fn text_to_sql(&self, query: &str) -> anyhow::Result<String> {
    if self.db_client.config.lock().await.is_none() {
      return Err(anyhow!("PG client is not configured"));
    }

    let client = reqwest::Client::new();

    let mut conv = Conversation::new();
    conv.add_system(SYSTEM_PROMPT);
    conv.add_user(query);

    loop {
      let reply = send_request(&client, &conv).await?;
      debug!("{:?}", reply);
      conv.add_assistant(serde_json::to_string(&reply)?.as_str());

      if reply.clarification.is_empty() && reply.sql.is_empty() {
        conv.add_user("both clarification and sql attribute are empty, this is not allowed! ask questions if you need clarification");
        continue;
      } else if !reply.clarification.is_empty() {
        match self.db_client.fetch_info(&reply.clarification).await {
          Ok(data) => {
            debug!("DB client response for '{}': {}", reply.clarification, data);
            conv.add_user(&data);
          }
          Err(e) => {
            debug!("Unable to clarify LLM's question {:?}", e);
            conv.add_user("currently the only options available for clarification are 'list all available tables', 'What are the columns in '");
          }
        }
        continue;
      }

      if !reply.sql.is_empty() {
        debug!("Final SQL: {}", reply.sql);
        return Ok(reply.sql);
      }
    }
  }
}
