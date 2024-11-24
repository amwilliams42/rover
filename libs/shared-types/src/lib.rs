use serde::{Deserialize, Serialize};
mod incident;

/// Protocol messages that can be sent in either direction
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum ProtocolMessage {
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "get_active_calls")]
    GetActiveCalls,
    #[serde(rename = "get_call")]
    GetCall { id: String },
    #[serde(rename = "create_call")]
    CreateCall,
    #[serde(rename = "update_call")]
    UpdateCall { id: String },
    Text(String),
    Json(String),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Call {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_open: bool,
}