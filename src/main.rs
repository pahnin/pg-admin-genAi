use freya::launch::launch_cfg;
use freya::prelude::{LaunchConfig, WindowConfig};
use pg_admin::agent::{AGENT, Agent};
use pg_admin::config::{LlmConfig, PostgresConfig, Setting};
use pg_admin::db_client::DbClient;
use pg_admin::ui::app::app;
use tokio::sync::RwLock;
use tracing::debug;

fn main() {
  tracing_subscriber::fmt().with_env_filter("pg_admin=debug").init();

  // Starting a custom tokio runtime, otherwise freya starts a runtime
  // within it's context which makes it difficult to call async functions
  let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
  let _guard = rt.enter();

  let agent = Agent { db_client: DbClient::new(), llm_client: RwLock::new(None) };
  AGENT.set(agent).unwrap();

  match Setting::try_load() {
    Ok(cfg) => {
      debug!("Loaded config: {:?}", cfg);
      let pg_config = cfg.active_postgres().cloned().unwrap();
      tokio::spawn(async move {
        let agent = AGENT.get().unwrap();
        agent.db_client.setup_db_client(Some(pg_config)).await;
        let llm_config = cfg.active_llm().cloned().unwrap();
        let mut llm_client_guard = agent.llm_client.write().await;
        *llm_client_guard = Some(llm_config);
      });
    }
    Err(e) => (),
  };

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
