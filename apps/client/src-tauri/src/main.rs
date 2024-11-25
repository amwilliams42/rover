use reqwest::{Method, Request};
// src-tauri/src/main.rs
use tauri::{http::{self, request, Uri}, Emitter, Url};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream, MaybeTlsStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use reqwest::header::{HeaderMap, HeaderValue};



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

async fn get_warp_token(server_url: &str) -> Option<String> {
    let client = reqwest::Client::new();
    
    println!("Checking WARP status for: {}", server_url);
    
    // Try to make a request through WARP
    match client.head(server_url)
        .send()
        .await {
            Ok(response) => {
                println!("WARP check response status: {}", response.status());
                println!("WARP check headers: {:?}", response.headers());
                
                // Check for WARP token in headers
                if let Some(token) = response.headers().get("CF-Access-Jwt-Assertion") {
                    println!("Found WARP token");
                    return Some(token.to_str().unwrap_or_default().to_string());
                }

                // Also check Cookies for CF_Authorization
                if let Some(cookies) = response.headers().get("set-cookie") {
                    if let Ok(cookie_str) = cookies.to_str() {
                        if cookie_str.contains("CF_Authorization=") {
                            println!("Found CF_Authorization cookie");
                            if let Some(token) = cookie_str
                                .split(';')
                                .find(|s| s.trim().starts_with("CF_Authorization="))
                                .and_then(|s| s.split('=').nth(1)) {
                                return Some(token.to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to get WARP token: {}", e);
            }
        }
    
    println!("No WARP token found");
    None
}

// Helper function to generate WebSocket key
fn generate_key() -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use rand::Rng;
    
    let mut key = [0u8; 16];
    rand::thread_rng().fill(&mut key);
    STANDARD.encode(key)
}

async fn connect_websocket(
    app_handle: tauri::AppHandle,
    state: Arc<WebSocketState>
) {
    let server_url = "https://rover.furcondispatch.org:3031";
    let ws_url = "wss://rover.furcondispatch.org:3031/ws";
    
    loop {
        println!("Attempting to connect to {}", ws_url);
        app_handle.emit("ws-status", "connecting").unwrap_or_default();

        let token = get_warp_token(server_url).await;

        let ws_url = match Url::parse(ws_url) {
            Ok(url) => url,
            Err(e) => {
                println!("Failed to parse WebSocket URL: {}", e);
                continue;
            }
        };

        let mut request = Request::new(Method::GET, ws_url.clone());

        // Add required WebSocket headers
        let headers = request.headers_mut();
        headers.insert("Host", HeaderValue::from_str(ws_url.host_str().unwrap_or_default()).unwrap());
        headers.insert("Upgrade", HeaderValue::from_static("websocket"));
        headers.insert("Connection", HeaderValue::from_static("Upgrade"));
        headers.insert("Sec-WebSocket-Version", HeaderValue::from_static("13"));
        headers.insert("Sec-WebSocket-Key", HeaderValue::from_str(&generate_key()).unwrap());
        headers.insert("Sec-WebSocket-Protocol", HeaderValue::from_static("rust-websocket"));
        
        // Add authorization if we have a token
        if let Some(token) = &token {
            println!("Adding authorization token to request");
            if let Ok(auth_value) = HeaderValue::from_str(&format!("Bearer {}", token)) {
                headers.insert("Authorization", auth_value);
            }

            // Also add as cookie just in case
            if let Ok(cookie_value) = HeaderValue::from_str(&format!("CF_Authorization={}", token)) {
                headers.insert("Cookie", cookie_value);
            }
        } else {
            println!("No authorization token available");
        }

        println!("Request headers: {:?}", headers);

        match connect_async(ws_url.as_str()).await {
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