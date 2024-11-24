// src-tauri/src/main.rs
use tauri::Emitter;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream, MaybeTlsStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

// Store WebSocket connection in app state
#[derive(Default)]
struct WebSocketState {
    tx: Arc<Mutex<Option<mpsc::Sender<Message>>>>,
}

// Command to send ping
#[tauri::command]
async fn send_ping(state: tauri::State<'_, WebSocketState>) -> Result<(), String> {
    println!("Attempting to send ping message...");
    
    let tx_lock = state.tx.lock().await;
    if let Some(tx) = tx_lock.as_ref() {
        tx.send(Message::Text("PING".to_string()))
            .await
            .map_err(|e| {
                println!("Error sending ping: {}", e);
                e.to_string()
            })?;
        println!("Ping sent successfully");
        Ok(())
    } else {
        println!("No sender available - websocket not connected");
        Err("WebSocket not connected".to_string())
    }
}

async fn handle_ws_messages(
    ws: WsStream,
    mut rx: mpsc::Receiver<Message>,
    app_handle: tauri::AppHandle
) {
    let (ws_sink, mut ws_stream) = ws.split();
    let ws_sink = Arc::new(Mutex::new(ws_sink));
    let ws_sink_clone = ws_sink.clone();


    // Task to send messages to WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let mut sink = ws_sink_clone.lock().await;
            if sink.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Task to handle incoming WebSocket messages
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received text message: {}", text);
                app_handle.emit("ws-message", text).unwrap_or_default();
            }
            Ok(Message::Ping(data)) => {
                println!("Received ping, sending pong!");
                let mut sink = ws_sink.lock().await;
                if let Err(e) = sink.send(Message::Pong(data)).await {
                    println!("Failed to send pong: {}", e);
                    break;
                }
            }
            Ok(Message::Pong(_)) => {
                println!("Received pong from server!");
                app_handle.emit("ws-message", "Received PONG from server").unwrap_or_default();
            }
            Ok(Message::Close(_)) => {
                println!("Server closed connection");
                break;
            }
            Err(e) => {
                println!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Clean up
    send_task.abort();
}

async fn connect_websocket(
    app_handle: tauri::AppHandle,
    state: Arc<WebSocketState>
) {
    let ws_url = "ws://localhost:3000/ws";
    
    loop {
        println!("Attempting to connect to {}", ws_url);
        app_handle.emit("ws-status", "connecting").unwrap_or_default();

        match connect_async(ws_url).await {
            Ok((ws_stream, _)) => {
                println!("WebSocket connected successfully!");
                app_handle.emit("ws-status", "connected").unwrap_or_default();
                
                // Create channel for sending messages to WebSocket
                let (tx, rx) = mpsc::channel(32);
                
                // Store sender in state
                {
                    let mut tx_lock = state.tx.lock().await;
                    *tx_lock = Some(tx);
                }
                
                // Handle messages
                handle_ws_messages(ws_stream, rx, app_handle.clone()).await;
                
                // Clear sender from state
                let mut tx_lock = state.tx.lock().await;
                *tx_lock = None;
                
                println!("WebSocket connection ended");
                app_handle.emit("ws-status", "disconnected").unwrap_or_default();
            }
            Err(e) => {
                println!("Failed to connect to WebSocket: {}", e);
                app_handle.emit("ws-status", "disconnected").unwrap_or_default();
            }
        }

        // Wait before reconnecting
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}


fn main() {
    let state = Arc::new(WebSocketState {
        tx: Arc::new(Mutex::new(None)),
    });
    let state_clone = state.clone();

    tauri::Builder::default()
        .manage(WebSocketState {
            tx: state.tx.clone(),
        })
        .invoke_handler(tauri::generate_handler![send_ping])
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Start WebSocket connection in background
            tauri::async_runtime::spawn(async move {
                println!("Starting WebSocket connection...");
                connect_websocket(app_handle, state_clone).await;
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}