use axum::{
    extract::{ConnectInfo, State, ws::{Message, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
    routing::get,
    Router,
};
use logging::ActionType;
mod logging;
use crate::logging::ActionLog;
use std::{net::{IpAddr, SocketAddr}, sync::Arc};
use time::OffsetDateTime;
use tokio::sync::RwLock;
use tracing::{info, error, Level};
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
use tracing_subscriber::FmtSubscriber;
use dotenvy;
use std::env;
mod db;
use crate::db::*;

pub struct AppState {
    db: DbPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Initialize database
    let db_pool = create_pool().await;
    
    // Create shared state
    let state = Arc::new(AppState { 
        db: db_pool 
    });

    // Setup router
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    // Start server
    info!("Server starting on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
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
    info!(?log, "Action logged to database");
    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, addr))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, addr: SocketAddr) {
    if let Err(e) = log_action(
        &state,
        ActionType::Connect,
        None,
        Some(addr.ip()),
        "New WebSocket connection".to_string(),
    ).await {
        error!("Failed to log connection: {}", e);
    }

    let (mut sender, mut receiver) = socket.split();

    // Handle incoming messages
    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(text) => {
                if let Err(e) = log_action(
                    &state,
                    ActionType::UpdateCall,
                    None,
                    Some(addr.ip()),
                    format!("Received message: {}", text),
                ).await {
                    error!("Failed to log message: {}", e);
                }
                // Handle message processing here
            }
            Message::Close(_) => {
                if let Err(e) = log_action(
                    &state,
                    ActionType::Disconnect,
                    None,
                    Some(addr.ip()),
                    "WebSocket disconnected".to_string(),
                ).await {
                    error!("Failed to log disconnection: {}", e);
                }
                break;
            }
            _ => {}
        }
    }
}