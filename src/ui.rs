use crate::agent::AGENT;
use freya::prelude::*;
use tracing::{debug, error};

#[derive(Clone)]
pub struct TableData {
  headers: Vec<String>,
  rows: Vec<Vec<String>>,
}

#[tracing::instrument]
pub fn app() -> Element {
  let mut editable = use_editable(
    || EditableConfig::new("select * from pacticipants;".trim().to_string()).with_allow_tabs(true),
    EditableMode::MultipleLinesSingleEditor,
  );
  let mut results = use_signal(|| TableData { headers: vec![], rows: vec![] });

  let trigger_query = move |_: ()| {
    let text_query = editable.editor().read().to_string();
    spawn(async move {
      let agent = AGENT.get().unwrap();
      let sql_from_llm = match agent.text_to_sql(&text_query).await {
        Err(e) => {
          error!("Error while trying to fetch SQL from LLM");
          results
            .set(TableData { headers: vec!["Error".into()], rows: vec![vec![format!("{e}")]] });
          return;
        }
        Ok(res) => res,
      };
      editable.editor_mut().write().set(&sql_from_llm);

      match agent.db_client.query(&sql_from_llm).await {
        Ok(rows) => {
          debug!(?rows);
          if rows.is_empty() {
            results.set(TableData { headers: vec!["0 rows returned".into()], rows: vec![] });
            return;
          }

          let headers = rows[0].columns().iter().map(|c| c.name().to_string()).collect::<Vec<_>>();
          let data = rows
            .iter()
            .map(|row| {
              row
                .columns()
                .iter()
                .enumerate()
                .map(|(i, col)| {
                  let t = col.type_();
                  if *t == tokio_postgres::types::Type::INT4 {
                    row.get::<usize, Option<i32>>(i).map(|v| v.to_string()).unwrap_or("NULL".into())
                  } else if *t == tokio_postgres::types::Type::VARCHAR {
                    row.get::<usize, Option<String>>(i).unwrap_or("NULL".into())
                  } else if *t == tokio_postgres::types::Type::TIMESTAMP {
                    row
                      .get::<usize, Option<chrono::NaiveDateTime>>(i)
                      .map(|v| v.to_string())
                      .unwrap_or("NULL".into())
                  } else if *t == tokio_postgres::types::Type::DATE {
                    row
                      .get::<usize, Option<chrono::NaiveDate>>(i)
                      .map(|v| v.to_string())
                      .unwrap_or("NULL".into())
                  } else if *t == tokio_postgres::types::Type::TEXT {
                    row
                      .get::<usize, Option<String>>(i)
                      .map(|v| v.to_string())
                      .unwrap_or("NULL".into())
                  } else {
                    format!("<unhandled {t:?}>")
                  }
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
          results.set(TableData { headers, rows: data });
        }
        Err(e) => {
          error!("Error running query: {:?}", e);
          results
            .set(TableData { headers: vec!["Error".into()], rows: vec![vec![format!("{e}")]] });
        }
      }
    });
  };

  let cursor_reference = editable.cursor_attr();
  let highlights = editable.highlights_attr(0);
  let editor = editable.editor().read();
  let cursor_char = editor.cursor_pos();
  let onmousedown = move |e: MouseEvent| {
    editable.process_event(&EditableEvent::MouseDown(e.data, 0));
  };
  let onmousemove = move |e: MouseEvent| {
    editable.process_event(&EditableEvent::MouseMove(e.data, 0));
  };
  let onclick = move |_: MouseEvent| {
    editable.process_event(&EditableEvent::Click);
  };
  let onglobalkeydown = move |e: KeyboardEvent| {
    editable.process_event(&EditableEvent::KeyDown(e.data));
  };
  let onglobalkeyup = move |e: KeyboardEvent| {
    editable.process_event(&EditableEvent::KeyUp(e.data));
  };

  rsx!(
    Body {
      padding: "10",
      spacing: "10",
      rect {
        width: "100%",
        height: "30%",
        padding: "5",
        spacing: "5",
        rect {
          height: "calc(100% - 30)",
          corner_radius: "6",
          border: "0.5 inner black",
          padding: "10",
          spacing: "10",
          background: "rgb(243, 243, 243)",
          ScrollView {
            paragraph {
              width: "100%",
              main_align: "center",
              cursor_id: "0",
              cursor_index: "{cursor_char}",
              cursor_mode: "editable",
              cursor_color: "black",
              highlights,
              cursor_reference,
              onclick,
              onmousemove,
              onmousedown,
              onglobalkeydown,
              onglobalkeyup,
              text {
                "{editable.editor()}"
              }
            }
          }
        }
        rect {
          width: "100%",
          height: "30",
          Button { onclick: trigger_query, label { "Execute query" } }
        }
      }
      rect {
        width: "100%",
        height: "70%",
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
    }
  )
}
