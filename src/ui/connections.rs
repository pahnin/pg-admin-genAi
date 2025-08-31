use crate::ui::app_state::PostgresStatus;
use freya::prelude::*;

pub fn postgres_config_view(pg_status: &Resource<PostgresStatus>) -> Element {
  let mut expanded = use_signal(|| false);

  rsx!(
    rect {
      width: "50%",
      padding: "8",
      spacing: "5",
      border: "1 solid rgb(200,200,200)",
      corner_radius: "12",

      match &*pg_status.read_unchecked() {
        Some(PostgresStatus::MissingConfig) => rsx! {
          rect {
            background: "rgb(255,230,230)", // pastel red
            label { "❌ Postgres config missing" }
          }
        },
        Some(PostgresStatus::ConnectionFailed(reason)) => rsx! {
          rect {
            background: "rgb(255,230,230)",
            label { "❌ Unable to connect to DB server: {reason}" }
          }
        },
        Some(PostgresStatus::Connected { config, tables }) => rsx! {
          rect {
            background: "rgb(230,255,230)", // pastel green
            direction: "vertical",
            onclick: move |_| expanded.set(!expanded()),
            label {
              font_size: "14",
              "✅ {config} ⬇"
            }
            if expanded() {
              for table in tables {
                label { "- {table}" }
              }
            }
          }
        },
        None => rsx! {
          label { "loading..." }
        }
      }
    }
  )
}
pub fn llm_config_view(llm_config_string: &Resource<String>) -> Element {
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
        "llm config"
      }

      match &*llm_config_string.read_unchecked() {
        Some(conf) => rsx! {
          label {
            font_size: "14",
            "🤖 {conf}"
          }
        },
        None => rsx! {
          label { "no config loaded" }
        }
      }
    }
  )
}
