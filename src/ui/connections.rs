use crate::ui::app_state::{LlmStatus, PostgresStatus};
use freya::prelude::*;

pub fn postgres_config_view(
  pg_status: &Resource<PostgresStatus>,
  mut show_modal: Signal<bool>,
  mut tables_data: Signal<Vec<String>>,
) -> Element {
  let mut pg_background = match &*pg_status.read_unchecked() {
    Some(PostgresStatus::MissingConfig) => "rgb(255,230,230)",
    Some(PostgresStatus::ConnectionFailed(_)) => "rgb(255,230,230)",
    Some(PostgresStatus::Connected { .. }) => "rgb(230,255,230)",
    None => "rgb(245,245,245)",
  };
  let status = pg_status.read_unchecked().clone();

  rsx!(
    rect {
      width: "50%",
      rect {
        width: "400",
        padding: "8",
        spacing: "5",
        border: "1 solid rgb(200,200,200)",
        corner_radius: "12",
        background: "{pg_background}",
        match status {
          Some(PostgresStatus::MissingConfig) => rsx! {
            rect {
              label { "Postgres config missing" }
            }
          },
          Some(PostgresStatus::ConnectionFailed(reason)) => rsx! {
            rect {
              label { "Unable to connect to DB server: {reason}" }
            }
          },
          Some(PostgresStatus::Connected { config, tables }) => rsx! {
            rect {
              direction: "vertical",
              onclick: move |_| {
                tables_data.set(tables.to_vec());
                show_modal.set(true)
              },
              label {
                font_size: "14",
                "{config} ▼"
              }
            }
          },
          None => rsx! {
            label { "loading..." }
          }
        }
      }
    }
  )
}
pub fn llm_config_view(llm_config_string: &Resource<LlmStatus>) -> Element {
  rsx!(
    rect {
      width: "50%",
      main_align: "end",
      direction: "horizontal",
      rect {
        width: "250",
        padding: "8",
        spacing: "5",
        background: "rgb(245,245,245)",
        border: "1 solid rgb(200,200,200)",
        corner_radius: "12",
        direction: "vertical",

        match &*llm_config_string.read_unchecked() {
          Some(LlmStatus::Connected { config } ) => rsx! {
            label {
              font_size: "14",
              font_weight: "bold",
              "{config.api_url}"
            }
            label {
              font_size: "10",
              "🤖 {config.model}"
            }
          },
          Some(LlmStatus::MissingConfig) => rsx! {
            label {
              "LLM config is incorrect or not found"
            }
          },
          None => rsx! {
            label { "no config loaded" }
          }
        }
      }
    }
  )
}
