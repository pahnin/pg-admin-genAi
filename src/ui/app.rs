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

        { sql_editor_view(&mut state) }

        { ai_input_editor_view(&mut state) }

        { action_buttons(handlers.trigger_llm_query, handlers.trigger_sql_query) }
      }
      { results_table(&state.results) }
    }
  )
}
