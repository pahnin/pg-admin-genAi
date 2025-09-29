use crate::config::LlmConfig;
use crate::db_client::DbClient;
use crate::llm::send_request;
use anyhow::anyhow;
use freya::prelude::{Readable, Signal, Writable};
use once_cell::sync::OnceCell;
use tokio::sync::RwLock;
use tracing::{debug, error};
pub static AGENT: OnceCell<Agent> = OnceCell::new();
use crate::conversation::{Conversation, LlmResponse};
use std::collections::HashSet;

static SYSTEM_PROMPT: &str = r#"
You are a Postgres SQL assistant.
The user asks questions in natural language; you must respond with valid JSON only that matches this schema:
{
  "explanation": "string",
  "sql": "string",
  "clarification": "string"
}
Rules

* Output only JSON (no markdown fences, no text outside).
* Always include all three keys.
* Either sql or clarification must be non-empty (never both empty).
* If you are 100% confident, fill sql; otherwise ask via clarification.
* Valid clarifications are:
  - "list all available tables"
  - "What are the columns in '<table>'?"
  - Or other direct schema questions needed to form SQL.
* Never guess schema/table/column names.
* Ask step by step if multiple tables are involved.
* Your response must always be valid JSON that can be parsed without modification.

#### 1. Simple single-table question
**User:**
> Show me all versions

**Assistant:**
{ "explanation": "I need the columns for 'versions' before writing the SQL", "sql": "", "clarification": "What are the columns in 'versions'?" }

**User:**
> The table 'versions' has columns: id, name, created\_at

**Assistant:**
{ "explanation": "Using the provided columns, I can now query all versions", "sql": "SELECT id, name, created_at FROM versions;", "clarification": "" }
---

#### 2. Query requiring clarification first
**User:**
> Get me the latest 5 branches
**Assistant:**
{ "explanation": "To query branches, I first need to know the columns in 'branches'", "sql": "", "clarification": "What are the columns in 'branches'?" }

**User:**
> The 'branches' table has id, name, created\_at
**Assistant:**
{ "explanation": "Now I can select the latest 5 branches by created_at", "sql": "SELECT id, name FROM branches ORDER BY created_at DESC LIMIT 5;", "clarification": "" }
---

#### 3. Multi-table join (multi-turn)

**User:**
> Show me branch heads with their branch names
**Assistant:**
{ "explanation": "I need to know available tables first to determine which ones are relevant", "sql": "", "clarification": "list all available tables" }
**User:**
> Available tables: branch\_heads, branches, users
**Assistant:**
{ "explanation": "I suspect 'branch_heads' and 'branches' are needed, so I must know their columns", "sql": "", "clarification": "What are the columns in 'branch_heads'?" }
**User:**
> branch\_heads: id, branch\_id, created\_at
> branches: id, name
**Assistant:**
{ "explanation": "Now I can join branch_heads with branches to get branch head id and branch name", "sql": "SELECT bh.id AS branch_head_id, b.name AS branch_name FROM branch_heads bh JOIN branches b ON bh.branch_id = b.id;", "clarification": "" }
---

#### 4. Multi-table with extra clarification
**User:**
> Show me orders with their customer name and product name
**Assistant:**
{ "explanation": "I first need all available tables to understand relationships", "sql": "", "clarification": "list all available tables" }
**User:**
> Available tables: orders, customers, products
**Assistant:**
{ "explanation": "I need to know the columns in 'orders' to see how it links to customers and products", "sql": "", "clarification": "What are the columns in 'orders'?" }
**User:**
> orders: id, customer\_id, product\_id, created\_at
> customers: id, name
> products: id, title
**Assistant:**
{ "explanation": "Orders links to customers and products via foreign keys, so I can join them", "sql": "SELECT o.id AS order_id, c.name AS customer_name, p.title AS product_name FROM orders o JOIN customers c ON o.customer_id = c.id JOIN products p ON o.product_id = p.id;", "clarification": "" }
---

#### 5. Case where SQL cannot yet be generated
**User:**
> Show me all active users
**Assistant:**
{ "explanation": "I need to know the columns of 'users' to see if there is an 'active' flag or equivalent", "sql": "", "clarification": "What are the columns in 'users'?" }
**User:**
> users: id, name, email
**Assistant:**
{ "explanation": "No 'active' column exists, so I cannot form the SQL. I need clarification from the user.", "sql": "", "clarification": "Which column indicates whether a user is active?" }
---

"#;

#[derive(Debug)]
pub struct Agent {
  pub db_client: DbClient,
  pub llm_client: RwLock<Option<LlmConfig>>,
}
/// Simple helper: extract table name from clarifications like
/// "What are the columns in 'branch_heads'?"
fn extract_table_from_clarification(clar: &str) -> Option<String> {
  let parts: Vec<&str> = clar.split('\'').collect();
  if parts.len() >= 2 {
    let table = parts[1].trim();
    if !table.is_empty() {
      return Some(table.to_string());
    }
  }
  None
}

/// Parse comma / bracket style column lists from DB text or from LLM explanation.
/// Heuristic: looks for bracketed lists first, then colon-separated tail.
fn parse_columns_from_db_response(resp: &str) -> Vec<String> {
  // bracketed form: [a, b, c]
  if let Some(start) = resp.find('[') {
    if let Some(end_rel) = resp[start + 1..].find(']') {
      let inside = &resp[start + 1..start + 1 + end_rel];
      return inside
        .split(',')
        .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
        .filter(|s| !s.is_empty())
        .collect();
    }
  }

  // after colon: "The columns are: a, b, c"
  if let Some(colon) = resp.find(':') {
    let tail = &resp[colon + 1..];
    let cols: Vec<String> = tail
      .split(',')
      .map(|s| {
        s.split_whitespace().next().unwrap_or("").trim_matches('"').trim_matches('\'').to_string()
      })
      .filter(|s| !s.is_empty())
      .collect();
    if !cols.is_empty() {
      return cols;
    }
  }

  vec![]
}

/// Naive extractor of table names from SQL (handles `FROM` and `JOIN` simple cases).
fn extract_tables_from_sql(sql: &str) -> HashSet<String> {
  let mut tables = HashSet::new();
  let sql_lc = sql.to_lowercase();
  let tokens: Vec<&str> = sql_lc.split_whitespace().collect();

  for (i, &tok) in tokens.iter().enumerate() {
    if tok == "from" || tok == "join" {
      if let Some(name_token) = tokens.get(i + 1) {
        let mut name = name_token.trim_end_matches(&[',', ';', ')'][..]).trim();
        if let Some(pos) = name.rfind('.') {
          name = &name[pos + 1..];
        }
        let name = name.trim_matches('"').trim_matches('\'').to_string();
        if !name.is_empty() {
          tables.insert(name);
        }
      }
    }
  }
  tables
}

impl Agent {
  pub async fn text_to_sql(
    &self,
    query: &str,
    mut conversation: Signal<Conversation>,
  ) -> anyhow::Result<String> {
    if self.db_client.config.read().await.is_none() {
      return Err(anyhow!("PG client is not configured"));
    }

    let client = reqwest::Client::new();
    //let mut conv = conversation.write();
    conversation.write().add_system(SYSTEM_PROMPT);
    conversation.write().add_user(query);

    let mut attempts = 0usize;
    let max_attempts = 12usize;

    loop {
      attempts += 1;
      if attempts > max_attempts {
        return Err(anyhow!("LLM did not converge after {} attempts", max_attempts));
      }

      let conv_snapshot = {
        let conv = conversation.read();
        conv.clone()
      };

      // call LLM
      let reply: LlmResponse = match send_request(&client, &conv_snapshot).await {
        Ok(r) => {
          debug!(?r, "llm reply");
          r
        }
        Err(e) => {
          error!(?e, "failed request to LLM");
          // gentle backoff / retry
          conversation.write().add_user(&format!("encounted error: {e}"));
          continue;
        }
      };

      // record assistant reply (serialized JSON) so conversation has LLM output
      let serialized = serde_json::to_string(&reply)?;
      conversation.write().add_assistant(&serialized);

      let sql_trim = reply.sql.trim();
      let clar_trim = reply.clarification.trim();
      let expl_trim = reply.explanation.trim();

      // --- Case: both empty -> try parsing explanation for column list (LLM sometimes puts schema there)
      if sql_trim.is_empty() && clar_trim.is_empty() {
        // Nothing usable -> tell LLM what options are valid
        conversation.write().add_user(
          "Both clarification and sql are empty — NOT allowed. \
           If you need schema details, ask 'list all available tables' or \
           \"What are the columns in 'table'\".

           You must generate SQL in sql field in response",
        );
        continue;
      }

      // --- Case: clarification present (LLM asked something) and no sql yet
      if sql_trim.is_empty() && !clar_trim.is_empty() {
        let clar = clar_trim;

        // "list all available tables"
        if clar.eq_ignore_ascii_case("list all available tables") {
          match self.db_client.fetch_info(clar).await {
            Ok(data) => {
              debug!("DB client response for '{}': {}", clar, data);
              // send a clearly formatted reply containing the available tables and instruct LLM what to do next
              conversation.write().add_user(&format!(
                "Available tables: {data}. Based on these, ask for columns of the tables you need with \
                 \"What are the columns in 'table'\" or produce SQL if you have full information."
              ));
            }
            Err(e) => {
              debug!("Unable to fetch tables: {:?}", e);
              conversation.write().add_user(
                "Unable to fetch list of tables from the DB; try again or check DB connection.",
              );
            }
          }
          // LLM will get this info in the next iteration
          continue;
        }

        // "What are the columns in 'X'?" -> fetch and remember columns, then instruct the LLM to retry SQL
        if let Some(table) = extract_table_from_clarification(clar) {
          match self.db_client.fetch_info(clar).await {
            Ok(data) => {
              debug!("DB client response for '{}': {}", clar, data);
              let cols = parse_columns_from_db_response(&data);
              if !cols.is_empty() {
                let table_lc = table.to_lowercase();
                conversation.write().remember_table_columns(&table_lc, cols.clone());
                debug!("Remembered {} cols for table {}", cols.len(), table_lc);
                conversation.write().add_user(&format!(
                  "Table '{}' has columns: [{}]. Now that you have the schema, please retry and produce the SQL.",
                  table, cols.join(", ")
                ));
                conversation.write().add_user(
                  "If you need schema details, ask 'list all available tables' or \
                   \"What are the columns in 'table'\".",
                );
              } else {
                debug!("No columns parsed from DB response for '{}': {}", table, data);
                conversation.write().add_user(&format!(
                  "DB returned: {data}. I couldn't parse columns from that response for '{table}'."
                ));
                conversation.write().add_user(
                  "I couldn't satisfy that clarification. The allowed clarifications are: \
                   'list all available tables' or 'What are the columns in <table>'.",
                );
              }
            }
            Err(e) => {
              debug!("Unable to fetch columns for '{}': {:?}", table, e);
              conversation.write().add_user("Unable to fetch columns from DB for that table. Ensure the table exists and try again.");
              conversation.write().add_user(
                "I couldn't satisfy that clarification. The allowed clarifications are: \
                 'list all available tables' or 'What are the columns in <table>'.",
              );
            }
          }
          continue; // let LLM see the schema message and retry
        }

        // fallback: unknown clarification text — forward to DB client and pass the response back
        match self.db_client.fetch_info(clar).await {
          Ok(data) => {
            debug!("DB client generic response for '{}': {}", clar, data);
            conversation.write().add_user(&format!("DB response: {data}. Now please continue."));
          }
          Err(e) => {
            debug!("Unable to clarify LLM's question {:?}", e);
            conversation.write().add_user(
              "I couldn't satisfy that clarification. The ONLY allowed clarifications are: \
               'list all available tables' or 'What are the columns in <table>'.",
            );
          }
        }
        continue;
      }

      // --- At this point, reply.sql is non-empty (or LLM put a SQL attempt); validate referenced tables
      let referenced_tables = extract_tables_from_sql(&reply.sql);
      let mut missing_tables: Vec<String> = Vec::new();
      for t in referenced_tables.iter() {
        if !conversation.write().known_tables.contains_key(&t.to_lowercase()) {
          missing_tables.push(t.clone());
        }
      }

      if !missing_tables.is_empty() {
        // Ask for columns for the first missing table (sequential approach).
        // IMPORTANT: we explicitly phrase the user message to follow your allowed syntax.
        let ask_table = &missing_tables[0];
        conversation.write().add_user(&format!("What are the columns in '{ask_table}'?"));
        continue;
      }

      // All referenced tables have schema known — accept SQL
      debug!("Final SQL accepted: {}", reply.sql);
      return Ok(reply.sql);
    }
  }
}
