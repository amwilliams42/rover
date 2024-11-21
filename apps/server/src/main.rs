use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use shared_types::{ActionLog, ActionType};
use hyper::Server;
use std::sync::Arc;
use time::OffsetDateTime;
use tokio::sync::RwLock;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
mod db;
use crate::db::*;

struct AppState {
    action_logs: Vec<ActionLog>,
    db: DbPool,
}

#[tokio::main]
async fn main() {
    // Initialize Logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    let state = Arc::new(RwLock::new(AppState {
        action_logs:Vec::new(), 
        db: create_pool().await }));

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    info!("Server running on port 3000");
    hyper::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn log_action(
    state: &AppState,
    action_type: ActionType,
    user_id: Option<String>,
    ip_address: Option<IpAddr>,
    details: String,
) -> Result<(), sqlx::Error> {
    let log = ActionLog {
        timestamp: OffsetDateTime::now_utc(),
        action_type,
        user_id,
        ip_address,
        details,
    };

    save_action_log(&state.db, &log).await?;
    tracing::info!(?log, "Action logged to database");
    Ok(())
}