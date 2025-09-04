// tests/ui_smoke.rs
use std::sync::{
  Arc,
  atomic::{AtomicBool, Ordering},
};

use freya::prelude::*;
use freya_testing::launch::launch_test;
use freya_testing::prelude::*; // re-exports TestEvent, EventName, MouseButton, TestNode helpers // launch_test helper

use pg_admin::ui::{
  actions::action_buttons,
  // adjust paths if your crate layout differs
  app::app,
  app_state::init_state,
  connections::{llm_config_view, postgres_config_view},
  editors::{ai_input_editor_view, sql_editor_view},
  results::{TableData, results_table},
};

#[tokio::test]
async fn postgres_config_view_shows_value() {
  // component must call hooks inside itself
  fn comp() -> Element {
    let r = use_resource(|| async { "pg-conn-string".to_string() });
    postgres_config_view(&r)
  }

  let mut utils = launch_test(comp);
  utils.wait_for_update().await;

  let root = utils.root();
  // top-level rect is root.get(0); first child is title label
  let rect = root.get(0);
  assert_eq!(rect.get(0).get(0).text(), Some("postgres config"));
  assert_eq!(rect.get(1).get(0).text(), Some("🔗 pg-conn-string"));
}

#[tokio::test]
async fn llm_config_view_shows_none_when_missing() {
  fn comp() -> Element {
    // return None to simulate not configured
    let r = use_resource(|| async { "Not Configured".into() });
    llm_config_view(&r)
  }

  let mut utils = launch_test(comp);
  utils.wait_for_update().await;

  let root = utils.root();
  let rect = root.get(0);
  // title + "no config loaded"
  assert_eq!(rect.get(0).get(0).text(), Some("llm config"));
  assert_eq!(rect.get(1).get(0).text(), Some("🤖 Not Configured"));
}

#[tokio::test]
async fn app_mounts_and_has_key_labels() {
  // mount top-level app (your `app()` function)
  let mut utils = launch_test(app);
  utils.wait_for_update().await;

  let root = utils.root();
  // Use TestNode::get_by_text helper to find nodes anywhere in the subtree
  assert!(root.get_by_text("postgres config").is_some());
  assert!(root.get_by_text("llm config").is_some());
  assert!(root.get_by_text("SQL:").is_some());
  assert!(root.get_by_text("Ai input:").is_some());
}
