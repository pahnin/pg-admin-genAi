use freya::launch::launch_cfg;
use freya::prelude::{LaunchConfig, WindowConfig};
use pg_admin::agent::{AGENT, Agent};
use pg_admin::db_client::DbClient;
use pg_admin::ui::app;

fn main() {
  tracing_subscriber::fmt().with_env_filter("pg_admin=debug").init();

  // Starting a custom tokio runtime, otherwise freya starts a runtime
  // within it's context which makes it difficult to call async functions
  let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
  let _guard = rt.enter();
  let agent = Agent { db_client: DbClient::new() };
  AGENT.set(agent).unwrap();

  tokio::spawn(async move {
    let agent = AGENT.get().unwrap();
    agent.db_client.setup_db_client().await;
  });

  launch_cfg(
    app,
    LaunchConfig::<()> {
      window_config: WindowConfig {
        size: (900.0, 650.0),
        decorations: true,
        transparent: false,
        title: "PG admin GenAI",
        ..Default::default()
      },
      ..Default::default()
    },
  )
}
