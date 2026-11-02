use crate::error::AppError;
use crate::models::{CreateServerRequest, Server, ServerEvent, ServerStatus};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct AppState {
    pub servers: RwLock<HashMap<Uuid, Server>>,
    pub events: broadcast::Sender<ServerEvent>,
    pub check_timeout: Duration,
}

impl AppState {
    pub fn new(check_timeout: Duration) -> Self {
        let (tx, _) = broadcast::channel(256);
        Self {
            servers: RwLock::new(HashMap::new()),
            events: tx,
            check_timeout,
        }
    }

    pub fn list_servers(&self) -> Vec<Server> {
        self.servers.read().values().cloned().collect()
    }

    pub fn get_server(&self, id: Uuid) -> Option<Server> {
        self.servers.read().get(&id).cloned()
    }

    pub fn add_server(&self, req: CreateServerRequest) -> Result<Server, AppError> {
        if req.name.trim().is_empty() {
            return Err(AppError::BadRequest("name is required".into()));
        }
        if req.ip.trim().is_empty() {
            return Err(AppError::BadRequest("ip is required".into()));
        }
        let server = Server::new(req.name, req.ip, req.port, req.tags);
        let clone = server.clone();
        self.servers.write().insert(server.id, server);
        let _ = self.events.send(ServerEvent::ServerAdded { server: clone.clone() });
        Ok(clone)
    }

    pub fn remove_server(&self, id: Uuid) -> Result<(), AppError> {
        if self.servers.write().remove(&id).is_none() {
            return Err(AppError::NotFound);
        }
        let _ = self.events.send(ServerEvent::ServerRemoved { id });
        Ok(())
    }

    pub fn update_status(
        &self,
        id: Uuid,
        status: ServerStatus,
        latency_ms: Option<u64>,
        error: Option<String>,
    ) {
        let now = time::OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_default();
        let mut guard = self.servers.write();
        if let Some(server) = guard.get_mut(&id) {
            server.status = status;
            server.latency_ms = latency_ms;
            server.last_checked = Some(now);
            server.last_error = error;
            let clone = server.clone();
            drop(guard);
            let _ = self.events.send(ServerEvent::ServerUpdated { server: clone });
        }
    }
}

pub type SharedState = Arc<AppState>;
