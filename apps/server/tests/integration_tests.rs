use futures_util::SinkExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use sqlx::PgPool;
use std::time::Duration;

#[sqlx::test]
async fn test_websocket_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database
    let database_url = dotenvy::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    sqlx::query!("DELETE FROM action_logs").execute(&pool).await?;

    let (mut ws_stream, _) = connect_async("ws://localhost:3000/ws").await?;

    //send test message
    ws_stream.send(Message::Text("test message".to_string())).await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    ws_stream.close(None).await?;

    let logs = sqlx::query!(
        "SELECT action_type::TEXT as action_type, details FROM action_logs ORDER BY timestamp"
    )
    .fetch_all(&pool)
    .await?;

    assert_eq!(logs.len(), 3); // Connect, Message, Disconnect
    assert_eq!(logs[0].action_type, Some("Connect".to_string()));
    assert!(logs[1].details.contains("test message"));
    assert_eq!(logs[2].action_type, Some("Disconnect".to_string()));
    Ok(())
}