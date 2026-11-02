#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cleanup() {
  if [[ -n "${BACKEND_PID:-}" ]]; then kill "$BACKEND_PID" 2>/dev/null || true; fi
}
trap cleanup EXIT INT TERM

echo "Starting backend on :8080..."
(cd "$ROOT/backend" && cargo run) &
BACKEND_PID=$!

echo "Waiting for backend..."
for i in {1..30}; do
  if curl -s http://127.0.0.1:8080/api/health >/dev/null 2>&1; then break; fi
  sleep 1
done

echo "Starting frontend on :3000..."
cd "$ROOT/frontend" && trunk serve --port 3000
