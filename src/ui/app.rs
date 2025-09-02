use freya::prelude::*;
use tracing::instrument;

use crate::ui::actions::action_buttons;
use crate::ui::app_state::init_state;
use crate::ui::connections::{llm_config_view, postgres_config_view};
use crate::ui::editors::{ai_input_editor_view, sql_editor_view};
use crate::ui::handlers::init_handlers;
use crate::ui::results::results_table;

#[instrument]
pub fn app() -> Element {
  let mut state = init_state();
  let mut tables = use_signal(|| vec![]);
  let handlers = init_handlers(&state);
  let mut show_modal = use_signal(|| false);


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
          height: "80",
          direction: "horizontal",
          { postgres_config_view(&state.pg_config, show_modal, tables ) }
          { llm_config_view(&state.llm_config) }
        }

        { sql_editor_view(&mut state) }

        { ai_input_editor_view(&mut state) }

        { action_buttons(handlers.trigger_llm_query, handlers.trigger_sql_query) }
      }
      { results_table(&state.results) }

      if show_modal() {
        rect {
          width: "100%",
          height: "100%",
          position: "absolute", // overlay modal
          layer: "-100",
          rect {
            background: "rgb(0,0,0)",
            opacity: "0.5",
            width: "100%",
            height: "100%",
            position: "absolute",
            layer: "-101",
            onclick: move |_| show_modal.set(false),
          }
          rect {
            width: "100%",
            height: "100%",
            position: "absolute",
            layer: "-150",
            main_align: "center",
            cross_align: "center",

            ScrollView {
              width: "400",
              height: "300",
              rect {
                width: "100%",
                padding: "12",
                opacity: "1",
                background: "white",
                corner_radius: "8",
                direction: "vertical",
                spacing: "8",
                label { "Tables: {tables.read().len()}" }
                for table in tables.read().iter() {
                  // For future: make this expandable accordion
                  label { "{table}" }
                }

                Button {
                  onclick: move |_| show_modal.set(false),
                  label { "Close" }
                }
              }
            }
          }
        }
      }
    }
  )
}
