use freya::prelude::*;

pub fn postgres_config_view(pg_config_string: &Resource<String>) -> Element {
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
        "postgres config"
      }

      match &*pg_config_string.read_unchecked() {
        Some(conf) => rsx! {
          label {
            font_size: "14",
            "🔗 {conf}"
          }
        },
        None => rsx! {
          label { "no config loaded" }
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
