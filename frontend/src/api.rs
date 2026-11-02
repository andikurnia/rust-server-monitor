use crate::models::{CreateServerRequest, Server};
use uuid::Uuid;

const BASE: &str = "http://127.0.0.1:8080";

pub async fn fetch_servers() -> Result<Vec<Server>, String> {
    reqwest::get(format!("{BASE}/api/servers"))
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Server>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_server(req: CreateServerRequest) -> Result<Server, String> {
    reqwest::Client::new()
        .post(format!("{BASE}/api/servers"))
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Server>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_server(id: Uuid) -> Result<(), String> {
    reqwest::Client::new()
        .delete(format!("{BASE}/api/servers/{id}"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

pub async fn check_server(id: Uuid) -> Result<Server, String> {
    reqwest::Client::new()
        .post(format!("{BASE}/api/servers/{id}/check"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Server>()
        .await
        .map_err(|e| e.to_string())
}
