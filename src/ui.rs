use crate::agent::AGENT;
use freya::prelude::*;
use tracing::{debug, error};

#[derive(Clone)]
pub struct TableData {
  headers: Vec<String>,
  rows: Vec<Vec<String>>,
}

fn postgres_config_view(pg_config_string: &Resource<String>) -> Element {
  rsx!(
    rect {
      width: "50%",
      padding: "8",
      spacing: "5",
      background: "rgb(245,245,245)",
      border: "1 solid rgb(200,200,200)",
      corner_radius: "12",

      label {
        font_size: "18",
        font_weight: "bold",
        "Postgres Config"
      }

      match &*pg_config_string.read_unchecked() {
        Some(conf) => rsx! {
          label {
            font_size: "14",
            "🔗 {conf}"
          }
        },
        None => rsx! {
          label { "No config loaded" }
        }
      }
    }
  )
}

fn llm_config_view(llm_config_string: &Resource<String>) -> Element {
  rsx!(
    rect {
      width: "50%",
      padding: "8",
      spacing: "5",
      background: "rgb(245,245,245)",
      border: "1 solid rgb(200,200,200)",
      corner_radius: "12",

      label {
        font_size: "18",
        font_weight: "bold",
        "LLM Config"
      }

      match &*llm_config_string.read_unchecked() {
        Some(conf) => rsx! {
          label {
            font_size: "14",
            "🤖 {conf}"
          }
        },
        None => rsx! {
          label { "No config loaded" }
        }
      }
    }
  )
}

fn sql_editor_view(state: &mut AppState) -> Element {
  let mut focus_sql = state.focus_sql.clone();
  let mut editable_sql = state.editable_sql.clone();
  rsx!(
    rect {
      height: "calc(80% - 200)",
      padding: "10",
      spacing: "10",
      corner_radius: "6",
      border: "0.3 inner black",
      label { "SQL:" }
      ScrollView {
        paragraph {
          width: "100%",
          height: "100%",
          main_align: "center",
          cursor_id: "0",
          cursor_index: "{editable_sql.editor().read().cursor_pos()}",
          cursor_mode: "editable",
          cursor_color: "black",
          highlights: state.editable_sql.highlights_attr(0),
          cursor_reference: state.editable_sql.cursor_attr(),
          a11y_id: state.focus_sql.attribute(),
          onclick: move |_: Event<MouseData>| {
            focus_sql.request_focus();
            editable_sql.process_event(&EditableEvent::Click);
          },
          onmousemove: move |e: Event<MouseData>| {
            editable_sql.process_event(&EditableEvent::MouseMove(e.data, 0));
          },
          onmousedown: move |e: Event<MouseData>| {
            editable_sql.process_event(&EditableEvent::MouseDown(e.data, 0));
          },
          onkeydown: move |e: Event<KeyboardData>| {
            editable_sql.process_event(&EditableEvent::KeyDown(e.data));
          },
          onglobalkeyup: move |e: Event<KeyboardData>| {
            editable_sql.process_event(&EditableEvent::KeyUp(e.data));
          },
          text { "{editable_sql.editor()}" }
        }
      }
    }
  )
}

fn ai_input_editor_view(state: &mut AppState) -> Element {
  let mut focus_text = state.focus_text.clone();
  let mut editable_nl = state.editable_nl.clone();
  rsx!(
    rect {
      width: "100%",
      height: "120",
      corner_radius: "6",
      border: "0.3 inner black",
      padding: "10",
      spacing: "10",
      background: "rgb(233, 233, 233)",
      label { "Ai input:" }
      ScrollView {
        paragraph {
          width: "100%",
          height: "100%",
          main_align: "center",
          cursor_id: "0",
          cursor_index: "{editable_nl.editor().read().cursor_pos()}",
          cursor_mode: "editable",
          cursor_color: "black",
          highlights: state.editable_nl.highlights_attr(0),
          cursor_reference: state.editable_nl.cursor_attr(),
          a11y_id: state.focus_text.attribute(),
          onclick: move |_: Event<MouseData>| {
            focus_text.request_focus();
            editable_nl.process_event(&EditableEvent::Click);
          },
          onmousemove: move |e: Event<MouseData>| {
            editable_nl.process_event(&EditableEvent::MouseMove(e.data, 0));
          },
          onmousedown: move |e: Event<MouseData>| {
            editable_nl.process_event(&EditableEvent::MouseDown(e.data, 0));
          },
          onkeydown: move |e: Event<KeyboardData>| {
            editable_nl.process_event(&EditableEvent::KeyDown(e.data));
          },
          onglobalkeyup: move |e: Event<KeyboardData>| {
            editable_nl.process_event(&EditableEvent::KeyUp(e.data));
          },
          text { "{editable_nl.editor()}" }
        }
      }
    }
  )
}

fn action_buttons(trigger_llm_query: Callback<()>, trigger_sql_query: Callback<()>) -> Element {
  rsx!(
    rect {
      width: "100%",
      height: "50",
      direction: "horizontal",
      spacing: "10",
      main_align: "end",
      Button { onclick: trigger_llm_query, label { "AI" } }
      Button { onclick: trigger_sql_query, label { "Execute SQL" } }
    }
  )
}

fn results_table(results: &Signal<TableData>) -> Element {
  rsx!(
    rect {
      width: "100%",
      height: "50%",
      padding: "5",
      spacing: "5",
      Table {
        columns: results.read().headers.len().max(1),
        TableHead {
          TableRow {
            for (i, col) in results.read().headers.iter().enumerate() {
              TableCell {
                key: "{i}",
                label { text_align: "left", font_size: "16", font_weight: "bold", "{col}" }
              }
            }
          }
        }
        TableBody {
          for (idx, row) in results.read().rows.iter().enumerate() {
            TableRow {
              key: "{idx}",
              for (col_idx, cell) in row.iter().enumerate() {
                TableCell {
                  key: "{idx}-{col_idx}",
                  label { text_align: "left", font_size: "12", "{cell}" }
                }
              }
            }
          }
        }
      }
    }
  )
}
struct AppState {
  focus_sql: UseFocus,
  focus_text: UseFocus,
  editable_sql: UseEditable,
  editable_nl: UseEditable,
  results: Signal<TableData>,
  pg_config: Resource<String>,
  llm_config: Resource<String>,
}

fn init_state() -> AppState {
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
    let agent = AGENT.get().unwrap();
    let guard = agent.db_client.config.lock().await;
    guard.as_ref().map(|s| format!("{s:?}")).unwrap_or("Not configured".into())
  });

  let llm_config = use_resource(move || async move {
    let agent = AGENT.get().unwrap();
    let guard = agent.llm_client.read().await;
    guard.as_ref().map(|s| format!("{s:?}")).unwrap_or("Not configured".into())
  });

  AppState { focus_sql, focus_text, editable_sql, editable_nl, results, pg_config, llm_config }
}
struct AppHandlers {
  trigger_sql_query: Callback<()>,
  trigger_llm_query: Callback<()>,
}

fn format_cell(row: &tokio_postgres::Row, i: usize) -> String {
  let col = &row.columns()[i];
  let t = col.type_();

  if *t == tokio_postgres::types::Type::INT4 {
    row.get::<usize, Option<i32>>(i).map(|v| v.to_string()).unwrap_or("NULL".into())
  } else if *t == tokio_postgres::types::Type::VARCHAR || *t == tokio_postgres::types::Type::TEXT {
    row.get::<usize, Option<String>>(i).unwrap_or("NULL".into())
  } else if *t == tokio_postgres::types::Type::TIMESTAMP {
    row
      .get::<usize, Option<chrono::NaiveDateTime>>(i)
      .map(|v| v.to_string())
      .unwrap_or("NULL".into())
  } else if *t == tokio_postgres::types::Type::DATE {
    row.get::<usize, Option<chrono::NaiveDate>>(i).map(|v| v.to_string()).unwrap_or("NULL".into())
  } else {
    format!("<unhandled {t:?}>")
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
) {
  let agent = AGENT.get().unwrap();
  match agent.text_to_sql(text_query).await {
    Ok(sql) => editable_sql.editor_mut().write().set(&sql),
    Err(e) => {
      error!("Error while trying to fetch SQL from LLM");
      results.set(TableData { headers: vec!["Error".into()], rows: vec![vec![format!("{e}")]] });
    }
  }
}

fn init_handlers(state: &AppState) -> AppHandlers {
  let mut editable_sql = state.editable_sql;
  let mut editable_nl = state.editable_nl;
  let mut results = state.results;

  let trigger_sql_query = Callback::new(move |_: ()| {
    let sql_query = editable_sql.editor().read().to_string();
    spawn({
      let mut results = results.clone();
      async move {
        let table = execute_sql_query(&sql_query).await;
        results.set(table);
      }
    });
  });

  let trigger_llm_query = Callback::new(move |_: ()| {
    let text_query = editable_nl.editor().read().to_string();
    spawn({
      let mut editable_sql = editable_sql.clone();
      let mut results = results.clone();
      async move {
        llm_to_sql_and_update(&mut editable_sql, &text_query, &mut results).await;
      }
    });
  });

  AppHandlers { trigger_sql_query, trigger_llm_query }
}

#[tracing::instrument]
pub fn app() -> Element {
  let mut state = init_state();
  let handlers = init_handlers(&state);

  rsx!(
    Body {
      padding: "10",
      spacing: "10",
      rect {
        width: "100%",
        height: "40%",
        padding: "5",
        spacing: "15",

        rect {
          height: "20%",
          spacing: "5",
          padding: "5",
          direction: "horizontal",
          { postgres_config_view(&state.pg_config) }
          { llm_config_view(&state.llm_config) }
        }

        { sql_editor_view(&mut state)}

        { ai_input_editor_view(&mut state)}

        { action_buttons(handlers.trigger_llm_query, handlers.trigger_sql_query) }
      }
      { results_table(&state.results) }
    }
  )
}
