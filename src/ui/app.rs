use freya::prelude::*;
use tracing::instrument;

use crate::ui::actions::action_buttons;
use crate::ui::app_state::init_state;
use crate::ui::connections::{llm_config_view, postgres_config_view};
use crate::ui::editors::{ai_input_editor_view, sql_editor_view, ai_chat_view};
use crate::ui::handlers::init_handlers;
use crate::ui::overlay_modal::modal;
use crate::ui::results::results_table;

#[instrument]
pub fn app() -> Element {
  let mut state = init_state();
  let mut tables = use_signal(|| Vec::new());
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

        rect {
          height: "400",
          direction: "horizontal",
          spacing: "15",
          { sql_editor_view(&mut state) }
          { ai_chat_view(&mut state) }
        }

        { action_buttons(handlers.trigger_llm_query, handlers.trigger_sql_query) }
      }
      { results_table(&state.results) }

      { modal(show_modal, tables) }
    }
  )
}
