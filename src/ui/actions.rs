use freya::prelude::*;

pub fn action_buttons(trigger_llm_query: Callback<()>, trigger_sql_query: Callback<()>) -> Element {
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
