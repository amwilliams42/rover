use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use std::net::IpAddr;


#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ActionLog {
    pub timestamp: OffsetDateTime,
    pub action_type: ActionType,
    pub user_id: Option<String>,
    pub ip_address: Option<IpAddr>,
    pub details: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "action_type", rename_all = "snake_case")]
pub enum ActionType {
    Connect,
    Disconnect,
    CreateCall,
    UpdateCall,
    DeleteCall,
    OpenCall,
}