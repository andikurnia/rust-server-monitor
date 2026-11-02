use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
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

impl Server {
    pub fn new(name: String, ip: String, port: u16, tags: Vec<String>) -> Self {
        let now = OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_default();
        Self {
            id: Uuid::new_v4(),
            name,
            ip,
            port,
            tags,
            status: ServerStatus::Unknown,
            latency_ms: None,
            last_checked: None,
            last_error: None,
            created_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    pub ip: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_port() -> u16 {
    22
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent {
    Snapshot { servers: Vec<Server> },
    ServerAdded { server: Server },
    ServerRemoved { id: Uuid },
    ServerUpdated { server: Server },
}
