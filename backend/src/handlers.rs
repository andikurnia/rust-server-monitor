use crate::error::AppError;
use crate::models::{CreateServerRequest, Server};
use crate::state::SharedState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

pub async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

pub async fn list_servers(State(state): State<SharedState>) -> Json<Vec<Server>> {
    Json(state.list_servers())
}

pub async fn create_server(
    State(state): State<SharedState>,
    Json(req): Json<CreateServerRequest>,
) -> Result<(StatusCode, Json<Server>), AppError> {
    let server = state.add_server(req)?;
    Ok((StatusCode::CREATED, Json(server)))
}

pub async fn get_server(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Server>, AppError> {
    state.get_server(id).map(Json).ok_or(AppError::NotFound)
}

pub async fn delete_server(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.remove_server(id)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn check_server_now(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Server>, AppError> {
    if state.get_server(id).is_none() {
        return Err(AppError::NotFound);
    }
    crate::health::check_server(&state, id).await;
    state.get_server(id).map(Json).ok_or(AppError::NotFound)
}
