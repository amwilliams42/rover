use std::{net::SocketAddr, ops::ControlFlow, sync::Arc, time::Duration};

use axum::{
    extract::{ws::
        {Message, WebSocket}, ConnectInfo, State, WebSocketUpgrade
    }, middleware::from_fn_with_state, response::IntoResponse, routing::get, Router
};
use axum_extra::{headers, TypedHeader};
use futures_util::{SinkExt, StreamExt};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod logging;
mod auth;
mod users;
use auth::{cloudflare_auth_middleware, CloudflareAuth};
use db::{create_pool, DbPool};

use axum::middleware;




#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub cf_auth: Arc<CloudflareAuth>,
}

impl AppState {
    pub async fn new() -> Self {
        let db_pool = create_pool().await;

        let cloudflare_config = auth::CloudflareConfig::new(
            dotenvy::var("CF_TEAM_NAME").unwrap(),
            dotenvy::var("CLOUDFLARE_AUD").unwrap(),
        );
        let cf_auth = Arc::new(CloudflareAuth::new(cloudflare_config).await.unwrap());
        Self { db: db_pool, cf_auth }
    }
}

#[tokio::main]
async fn main() {
    // Setup logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Initialize database
    let db_pool = create_pool().await;
    
    // Create shared state
    let state = Arc::new(AppState::new().await);

    // Setup router
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(from_fn_with_state(state.clone(), cloudflare_auth_middleware))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true))
        )
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3031").await.unwrap();
    tracing::debug!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<SocketAddr>(),
    ).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(_state): State<Arc<AppState>>,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let _user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    

    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(socket: WebSocket, addr: SocketAddr) {
    let (mut sender, mut receiver) = socket.split();
    
    let (ping_tx, mut ping_rx) = tokio::sync::mpsc::channel(32);

    // Send initial ping
    if sender.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        tracing::info!("Sent ping to {}", addr);
    } else {
        tracing::error!("Failed to send ping to {}", addr);
        return;
    }

    let ping_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            tracing::info!("Sending ping to {}", addr);
            if ping_tx.send(Message::Ping(vec![1, 2, 3])).await.is_err() {
                break;
            }
        }
    });

     // Spawn task to forward ping messages to the WebSocket
     let forward_task = tokio::spawn(async move {
        while let Some(msg) = ping_rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Main message loop
    // Main message loop
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(message) => {
                match message {
                    Message::Text(text) => {
                        tracing::debug!("Received text message from {}: {}", addr, text);
                        // Process text message if needed
                    }
                    Message::Ping(_payload) => {
                        tracing::debug!("Received ping from {}", addr);
                        // No need to manually respond - axum handles pings automatically
                    }
                    Message::Pong(_) => {
                        tracing::debug!("Received pong from {}", addr);
                    }
                    Message::Close(_) => {
                        tracing::info!("Client {} requested close", addr);
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                tracing::error!("Error receiving message from {}: {}", addr, e);
                break;
            }
        }
    }

    // Clean up
    ping_task.abort();
    forward_task.abort();
    
    tracing::info!("Client {} disconnected", addr);
}

// Helper function to process messages (if needed)
fn process_message(msg: Message, addr: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(text) => {
            tracing::debug!("Received text message from {}: {}", addr, text);
            ControlFlow::Continue(())
        }
        Message::Binary(data) => {
            tracing::debug!("Received {} bytes from {}", data.len(), addr);
            ControlFlow::Continue(())
        }
        Message::Ping(_) => {
            tracing::debug!("Received ping from {}", addr);
            ControlFlow::Continue(())
        }
        Message::Pong(_) => {
            tracing::debug!("Received pong from {}", addr);
            ControlFlow::Continue(())
        }
        Message::Close(_) => {
            tracing::info!("Client {} requested close", addr);
            ControlFlow::Break(())
        }
    }
}