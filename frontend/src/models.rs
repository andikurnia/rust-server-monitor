use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerStatus {
    Unknown,
    Online,
    Offline,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub ip: String,
    pub port: u16,
    #[serde(default)]
    pub tags: Vec<String>,
    pub status: ServerStatus,
    pub latency_ms: Option<u64>,
    pub last_checked: Option<String>,
    pub last_error: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    pub ip: String,
    pub port: u16,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent {
    Snapshot { servers: Vec<Server> },
    ServerAdded { server: Server },
    ServerRemoved { id: Uuid },
    ServerUpdated { server: Server },
}
