use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub status: AgentStatus,
    pub last_heartbeat: Option<NaiveDateTime>,
    pub workspace_volume: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "enum('Idle','Busy','Offline')")]
#[sqlx(rename_all = "PascalCase")]
pub enum AgentStatus {
    Idle,
    Busy,
    Offline,
}
