use std::sync::atomic::{AtomicBool, Ordering};

use freya::prelude::*;
use freya_testing::launch::launch_test;
use freya_testing::prelude::*;

use pg_admin::{
  config::PostgresConfig,
  ui::{
    actions::action_buttons,
    app::app,
    app_state::{LlmStatus, PostgresStatus, init_state},
    connections::{llm_config_view, postgres_config_view},
    editors::{ai_chat_view, sql_editor_view},
    results::{TableData, results_table},
  },
};

#[tokio::test]
async fn postgres_config_view_shows_value() {
  // component must call hooks inside itself
  fn comp() -> Element {
    let show_modal = use_signal(|| false);

    let tables = vec![];
    let t_signal = use_signal(|| tables.clone());
    let r = use_resource(move || async move {
      let conf = PostgresConfig {
        dbname: "postgres".to_string(),
        host: "postgres".to_string(),
        password: "postgres".to_string(),
        user: "postgres".to_string(),
        port: 5432,
      };
      let tables = vec![];
      PostgresStatus::Connected {
        config: format!(
          "postgresql://{}:{}@{}/{}",
          conf.user, conf.password, conf.host, conf.dbname
        ),
        tables,
      }
    });
    postgres_config_view(&r, show_modal, t_signal)
  }

  let mut utils = launch_test(comp);
  utils.wait_for_update().await;

  let root = utils.root();
  let rect = root.get(0);
  assert!(rect.get(0).get(0).get(0).get(0).text().unwrap().contains("postgres"));
}

#[tokio::test]
async fn llm_config_view_shows_none_when_missing() {
  fn comp() -> Element {
    let r = use_resource(|| async { LlmStatus::MissingConfig {} });

    llm_config_view(&r)
  }

  let mut utils = launch_test(comp);
  utils.wait_for_update().await;

  let root = utils.root();
  let rect = root.get(0);
  assert!(rect.get(0).get(0).get(0).text().unwrap().contains("LLM config is incorrect"));
}

#[tokio::test]
async fn app_mounts_and_has_key_labels() {
  let mut utils = launch_test(app);
  utils.wait_for_update().await;

  let root = utils.root();
  //dump(&root, 0);
  // Use TestNode::get_by_text helper to find nodes anywhere in the subtree
  assert!(root.get_by_text("Postgres config missing").is_some());
  assert!(root.get_by_text("LLM config is incorrect or not found").is_some());
  assert!(root.get_by_text("SQL:").is_some());
  assert!(root.get_by_text("Execute SQL").is_some());
  assert!(root.get_by_text("Text to SQL").is_some());
  assert!(root.get_by_text("Ask LLM:").is_some());
}

fn dump(node: &TestNode, depth: usize) {
  if let Some(text) = node.text() {
    println!("{:indent$}{}", "", text, indent = depth * 2);
  } else {
    println!("{:indent$}", "", indent = depth * 2);
  }
  for i in 0..node.children_ids().len() {
    dump(&node.get(i), depth + 1);
  }
}
