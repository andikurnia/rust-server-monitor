# rust-server-monitor backend

Axum-based backend for registering and monitoring remote servers by external IP.

## Run

```sh
cd backend
cargo run
```

Environment variables:

- `HOST` (default `0.0.0.0`)
- `PORT` (default `8080`)
- `CHECK_INTERVAL_SECS` (default `15`)
- `CHECK_TIMEOUT_SECS` (default `3`)

## API

- `GET /api/health`
- `GET /api/servers`
- `POST /api/servers` `{ "name", "ip", "port", "tags" }`
- `GET /api/servers/{id}`
- `DELETE /api/servers/{id}`
- `POST /api/servers/{id}/check`
- `GET /ws` WebSocket live updates
