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
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting NetCtl daemon");

    // Initialize database
    let db = Database::open("/tmp/netctl.db")?;
    info!("Database initialized");

    // Load or create system state
    let state = if let Some(loaded_state) = db.load_state()? {
        loaded_state
    } else {
        SystemState::new()
    };

    let shared_state: SharedState = Arc::new(RwLock::new(state));

    // Create and bind API server
    let app = create_router(shared_state.clone());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    info!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
