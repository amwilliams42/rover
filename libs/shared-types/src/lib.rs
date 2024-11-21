use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionLog {
    pub timestamp: OffsetDateTime,
    pub action_type: ActionType,
    pub user_id: Option<String>,
    pub ip_address: Option<IpAddr>,
    pub details: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActionType {
    Connect,
    Disconnect,
    CreateCall,
    UpdateCall,
    DeleteCall,
    OpenCall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerMessage{
    pub message_type: MessageType,
    pub payload: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageType{
    StateUpdate,
    Error,
    Acknowledgment
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::str::FromStr;

    #[test]
    fn test_action_log_with_ip() {
        let log = ActionLog {
            timestamp: OffsetDateTime::now_utc(),
            action_type: ActionType::Connect,
            user_id: Some("user123".to_string()),
            ip_address: Some(IpAddr::from_str("127.0.0.1").unwrap()),
            details: "Test connection".to_string(),
        };
        
        assert!(log.ip_address.is_some());
    }
}