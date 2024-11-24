use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncidentCall {
    pub incident_number: String,
    pub event: String,
    pub date_of_service: OffsetDateTime,
    pub name: String,
    pub location: String,
    pub dob: Option<OffsetDateTime>,
    pub badge_number: Option<String>,
    pub phone_number: String,
    pub caller_name: String,
    pub incident_type: IncidentType,
    pub call_nature: CallNature,
    pub notes: Vec<Note>,
    pub disposition: Disposition,
    pub units_assigned: Vec<Unit>,
    pub times: IncidentTimes,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IncidentType {
    Security,
    Medical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CallNature {
    ChiefComplaint,
    SecurityComplaint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub author: String,
    pub content: String,
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Disposition {
    Resolved,
    Unresolved,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub unit_type: UnitType,
    pub status: UnitStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UnitType {
    Security,
    FirstAid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UnitStatus {
    Available,
    Dispatched,
    OnScene,
    Unavailable,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncidentTimes {
    pub received: OffsetDateTime,
    pub assigned: Option<OffsetDateTime>,
    pub responding: Option<OffsetDateTime>,
    pub on_scene: Option<OffsetDateTime>,
    pub transporting: Option<OffsetDateTime>,
    pub at_destination: Option<OffsetDateTime>,
    pub cleared: Option<OffsetDateTime>,
}