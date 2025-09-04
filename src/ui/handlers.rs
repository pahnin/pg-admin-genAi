use crate::agent::AGENT;
use crate::conversation::Conversation;
use crate::ui::app_state::AppState;
use crate::ui::results::TableData;
use freya::prelude::*;
use tracing::error;

pub struct AppHandlers {
  pub trigger_sql_query: Callback<()>,
  pub trigger_llm_query: Callback<()>,
}

fn format_cell(row: &tokio_postgres::Row, i: usize) -> String {
  let col = &row.columns()[i];
  let t = col.type_();

  match *t {
    tokio_postgres::types::Type::INT4 => {
      row.get::<usize, Option<i32>>(i).map(|v| v.to_string()).unwrap_or("NULL".into())
    }
    tokio_postgres::types::Type::VARCHAR | tokio_postgres::types::Type::TEXT => {
      row.get::<usize, Option<String>>(i).unwrap_or("NULL".into())
    }
    tokio_postgres::types::Type::TIMESTAMP => row
      .get::<usize, Option<chrono::NaiveDateTime>>(i)
      .map(|v| v.to_string())
      .unwrap_or("NULL".into()),
    tokio_postgres::types::Type::DATE => {
      row.get::<usize, Option<chrono::NaiveDate>>(i).map(|v| v.to_string()).unwrap_or("NULL".into())
    }
    _ => {
      format!("<unhandled {t:?}>")
    }
  }
}

fn rows_to_table(rows: Vec<tokio_postgres::Row>) -> TableData {
  if rows.is_empty() {
    return TableData { headers: vec!["0 rows returned".into()], rows: vec![] };
  }

  let headers = rows[0].columns().iter().map(|c| c.name().to_string()).collect::<Vec<_>>();

  let data = rows
    .iter()
    .map(|row| {
      row.columns().iter().enumerate().map(|(i, _)| format_cell(row, i)).collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  TableData { headers, rows: data }
}

async fn execute_sql_query(sql: &str) -> TableData {
  let agent = AGENT.get().unwrap();
  match agent.db_client.query(sql).await {
    Ok(rows) => rows_to_table(rows),
    Err(e) => TableData { headers: vec!["Error".into()], rows: vec![vec![format!("{e}")]] },
  }
}

async fn llm_to_sql_and_update(
  editable_sql: &mut UseEditable,
  text_query: &str,
  results: &mut Signal<TableData>,
  conversation: Signal<Conversation>,
) {
  let Some(agent) = AGENT.get() else {
    error!("Agent not initialized");
    results.set(TableData {
      headers: vec!["Error".into()],
      rows: vec![vec!["Agent not initialized".to_string()]],
    });
    return;
  };
  match agent.text_to_sql(text_query, conversation).await {
    Ok(sql) => editable_sql.editor_mut().write().set(&sql),
    Err(e) => {
      error!("Error while trying to fetch SQL from LLM");
      results.set(TableData { headers: vec!["Error".into()], rows: vec![vec![format!("{e}")]] });
    }
  }
}

pub fn init_handlers(state: &AppState) -> AppHandlers {
  let editable_sql = state.editable_sql;
  let editable_nl = state.editable_nl;
  let results = state.results;
  let conversation = state.conversation;

  let trigger_sql_query = Callback::new(move |_: ()| {
    let sql_query = editable_sql.editor().read().to_string();
    spawn({
      let mut results = results;
      async move {
        let table = execute_sql_query(&sql_query).await;
        results.set(table);
      }
    });
  });

  let trigger_llm_query = Callback::new(move |_: ()| {
    let text_query = editable_nl.editor().read().to_string();
    let conversation = conversation;
    spawn({
      let mut editable_sql = editable_sql;
      let mut results = results;
      async move {
        llm_to_sql_and_update(&mut editable_sql, &text_query, &mut results, conversation).await;
      }
    });
  });

  AppHandlers { trigger_sql_query, trigger_llm_query }
}
