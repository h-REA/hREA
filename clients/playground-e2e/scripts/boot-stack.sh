#!/usr/bin/env bash
# Boots the playground stack for E2E: sandboxed conductor (mem transport) + vite.
# Run from inside `nix develop` at the repo root (hc + node required).
# Ports: admin 22994, app 29281, ui 8888 — mirrors the repo dev topology.
set -euo pipefail
cd "$(dirname "$0")/../../.."

RUN_DIR=".e2e-stack"
mkdir -p "$RUN_DIR"

if [ ! -f workdir/hrea.happ ]; then
  echo "workdir/hrea.happ missing — run: yarn build:happ" >&2
  exit 1
fi

for p in 8888 22994 29281; do
  if lsof -ti:"$p" >/dev/null 2>&1; then
    echo "port $p is busy — a stale stack is running; run stack:down first" >&2
    exit 1
  fi
done

hc s clean >/dev/null 2>&1 || true

# setsid gives each service its own process group so stop-stack.sh can kill the
# whole tree (yarn/hc wrappers spawn grandchildren that outlive a plain kill).
echo "starting conductor (admin 22994, app 29281)…"
setsid bash -c 'echo pass | RUST_LOG=warn hc s -f=22994 --piped generate workdir/hrea.happ --run=29281 -a hrea network mem' \
  > "$RUN_DIR/conductor.log" 2>&1 &
echo $! > "$RUN_DIR/conductor.pid"

echo "starting vite on :8888…"
setsid bash -c 'VITE_ADMIN_PORT=22994 VITE_APP_PORT=29281 UI_PORT=8888 yarn workspace ui start' \
  > "$RUN_DIR/vite.log" 2>&1 &
echo $! > "$RUN_DIR/vite.pid"

# "Conductor ready" prints BEFORE the admin port binds; wait for the launch
# line (which carries the admin_port) AND an actual TCP accept on 22994.
for i in $(seq 1 60); do
  if curl -sf -o /dev/null http://localhost:8888/ \
     && grep -q "Conductor launched" "$RUN_DIR/conductor.log" \
     && (exec 3<>/dev/tcp/127.0.0.1/22994) 2>/dev/null; then
    exec 3>&- 2>/dev/null || true
    if ! grep -q "localhost:8888" "$RUN_DIR/vite.log"; then
      echo "vite drifted off :8888 (stale server?) — see $RUN_DIR/vite.log" >&2
      exit 1
    fi
    echo "stack ready (conductor admin port accepting + vite on :8888)"
    exit 0
  fi
  sleep 2
done
echo "stack failed to become ready — see $RUN_DIR/*.log" >&2
exit 1
