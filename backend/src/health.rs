use crate::models::ServerStatus;
use crate::state::SharedState;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn check_server(state: &SharedState, id: uuid::Uuid) {
    let server = match state.get_server(id) {
        Some(s) => s,
        None => return,
    };

    let addr = format!("{}:{}", server.ip, server.port);
    let start = Instant::now();
    let result = timeout(state.check_timeout, TcpStream::connect(&addr)).await;

    match result {
        Ok(Ok(_stream)) => {
            let latency = start.elapsed().as_millis() as u64;
            state.update_status(id, ServerStatus::Online, Some(latency), None);
        }
        Ok(Err(err)) => {
            state.update_status(id, ServerStatus::Offline, None, Some(err.to_string()));
        }
        Err(_) => {
            state.update_status(
                id,
                ServerStatus::Offline,
                None,
                Some("connection timed out".into()),
            );
        }
    }
}

pub async fn run(state: SharedState, interval: Duration) {
    let mut ticker = tokio::time::interval(interval);
    ticker.tick().await;
    loop {
        ticker.tick().await;
        let ids: Vec<uuid::Uuid> = state.list_servers().into_iter().map(|s| s.id).collect();
        for id in ids {
            check_server(&state, id).await;
        }
    }
}
