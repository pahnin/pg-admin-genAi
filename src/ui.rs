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
  let mut focus_sql = use_focus();
  let mut focus_text = use_focus();
  let mut editable_sql = use_editable(
    || EditableConfig::new("".trim().to_string()).with_allow_tabs(true),
    EditableMode::MultipleLinesSingleEditor,
  );
  let mut editable_natural_language = use_editable(
    || EditableConfig::new("".trim().to_string()).with_allow_tabs(true),
    EditableMode::MultipleLinesSingleEditor,
  );
  let mut results = use_signal(|| TableData { headers: vec![], rows: vec![] });
 
  let mut pg_config_string = use_resource(move || async move {
      let agent = AGENT.get().unwrap();
      let guard = agent.db_client.config.lock().await;
      let s: String = match guard.as_ref() {
        Some(s) => format!("{:?}", s),
        None => "Not configured".to_string()
      };
      
      s
  });
  let mut llm_config_string = use_resource(move || async move {
      let agent = AGENT.get().unwrap();
      let guard = agent.llm_client.read().await;
      let s: String = match guard.as_ref() {
        Some(s) => format!("{:?}", s),
        None => "Not configured".to_string()
      };
      
      s
  });
 let trigger_sql_query = move |_: ()| {
    let sql_query = editable_sql.editor().read().to_string();
    spawn(async move {
      let agent = AGENT.get().unwrap();
      match agent.db_client.query(&sql_query).await {
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

  let trigger_llm_query = move |_: ()| {
    let text_query = editable_natural_language.editor().read().to_string();
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
      editable_sql.editor_mut().write().set(&sql_from_llm);


    });
  };

  let sql_cursor_reference = editable_sql.cursor_attr();
  let sql_highlights = editable_sql.highlights_attr(0);
  let sql_editor = editable_sql.editor().read();
  let sql_cursor_char = sql_editor.cursor_pos();
  let sql_onmousedown = move |e: MouseEvent| {
    editable_sql.process_event(&EditableEvent::MouseDown(e.data, 0));
  };
  let sql_onmousemove = move |e: MouseEvent| {
    editable_sql.process_event(&EditableEvent::MouseMove(e.data, 0));
  };
  let sql_onclick = move |_: MouseEvent| {
    focus_sql.request_focus();
    editable_sql.process_event(&EditableEvent::Click);
  };
  let sql_onglobalkeydown = move |e: KeyboardEvent| {
    editable_sql.process_event(&EditableEvent::KeyDown(e.data));
  };
  let sql_onglobalkeyup = move |e: KeyboardEvent| {
    editable_sql.process_event(&EditableEvent::KeyUp(e.data));
  };

  let text_cursor_reference = editable_natural_language.cursor_attr();
  let text_highlights = editable_natural_language.highlights_attr(0);
  let text_editor = editable_natural_language.editor().read();
  let text_cursor_char = text_editor.cursor_pos();
  let text_onmousedown = move |e: MouseEvent| {
    editable_natural_language.process_event(&EditableEvent::MouseDown(e.data, 0));
  };
  let text_onmousemove = move |e: MouseEvent| {
    editable_natural_language.process_event(&EditableEvent::MouseMove(e.data, 0));
  };
  let text_onclick = move |_: MouseEvent| {
    focus_text.request_focus();
    editable_natural_language.process_event(&EditableEvent::Click);
  };
  let text_onglobalkeydown = move |e: KeyboardEvent| {
    editable_natural_language.process_event(&EditableEvent::KeyDown(e.data));
  };
  let text_onglobalkeyup = move |e: KeyboardEvent| {
    editable_natural_language.process_event(&EditableEvent::KeyUp(e.data));
  };
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
/*          rect {*/
            /*width: "50%",*/
            /*match &*pg_config_string.read_unchecked() {*/
              /*Some(conf) => rsx! { label { "{conf}" } },*/
              /*None =>  rsx! { label { "..." } }*/
            /*}*/
          /*}*/
          /*rect {*/
            /*width: "50%",*/
            /*match &*llm_config_string.read_unchecked() {*/
              /*Some(conf) => rsx! { label { "{conf}" } },*/
              /*None =>  rsx! { label { "..." } }*/
            /*}*/
          /*}*/
        }
        rect {
          height: "calc(80% - 200)",
          padding: "10",
          spacing: "10",
          corner_radius: "6",
          border: "0.3 inner black",
          label {
            "SQL:"
          }
          ScrollView {
            paragraph {
              width: "100%",
              height: "100%",
              main_align: "center",
              cursor_id: "0",
              cursor_index: "{sql_cursor_char}",
              cursor_mode: "editable",
              cursor_color: "black",
              highlights: sql_highlights,
              cursor_reference: sql_cursor_reference,
              a11y_id: focus_sql.attribute(),
              onclick: sql_onclick,
              onmousemove: sql_onmousemove,
              onmousedown: sql_onmousedown,
              onkeydown: sql_onglobalkeydown,
              onglobalkeyup: sql_onglobalkeyup,
              text {
                "{editable_sql.editor()}"
              }
            }
          }
        }
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
              cursor_index: "{text_cursor_char}",
              cursor_mode: "editable",
              cursor_color: "black",
              highlights: text_highlights,
              cursor_reference: text_cursor_reference,
              a11y_id: focus_text.attribute(),
              onclick: text_onclick,
              onmousemove: text_onmousemove,
              onmousedown: text_onmousedown,
              onkeydown: text_onglobalkeydown,
              onglobalkeyup: text_onglobalkeyup,
              text {
                "{editable_natural_language.editor()}"
              }
            }
          }
        }
        rect {
          width: "100%",
          height: "50",
          direction: "horizontal",
          spacing: "10",
          main_align: "end",
          Button { onclick: trigger_llm_query, label { "AI" } }
          Button { onclick: trigger_sql_query, label { "Execute SQL" } }
        }
      }
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
    }
  )
}
