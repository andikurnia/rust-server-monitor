# rust-server-monitor frontend

Leptos CSR dashboard.

## Run

Start backend first:

```sh
cd backend
cargo run
```

Then frontend:

```sh
cd frontend
trunk serve
```

Open http://127.0.0.1:8080 (trunk default port 8080; if backend also uses 8080, set `PORT=...` for backend or trunk `--port`).
