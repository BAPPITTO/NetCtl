use netctl::{
    state::SystemState,
    api::handlers::{create_router, SharedState},
    db::Database,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::Server;
use tracing::info;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {


    let state = Arc::new(AppState::new());

    let ctx = PluginContext {
        state: state.clone(),
    };

    let mut plugin_manager = PluginManager::new();

    // register plugins
    plugin_manager.register(Box::new(plugins::logger::LoggerPlugin));

    // test event loop
    loop {
        plugin_manager.dispatch(&Event::Tick, &ctx)
        std::thread::sleep(std::time::Duration::from_secs(5));

    }

    // Initialize structured logging with environment filter
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting NetCtl daemon");

    // Initialize database
    let db = Database::open("/tmp/netctl.db")?;
    info!("Database initialized");

    // Load existing system state or create new
    let state = db.load_state()?.unwrap_or_else(SystemState::new);
    let shared_state: SharedState = Arc::new(RwLock::new(state));

    // Build API router
    let app = create_router(shared_state.clone());

    // Bind server to localhost:3001
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}


mod plugins;

use plugins::manager::PluginManager;
use plugins::plugin::Event;
use plugins::context::PluginContext;

use std::sync::Arc;