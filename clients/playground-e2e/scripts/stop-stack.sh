#!/usr/bin/env bash
# Tears down the E2E playground stack started by boot-stack.sh.
# Services were started with setsid, so killing the negative PID takes the
# whole process group (wrappers AND grandchildren like vite/holochain).
set -uo pipefail
cd "$(dirname "$0")/../../.."
RUN_DIR=".e2e-stack"

for name in conductor vite; do
  if [ -f "$RUN_DIR/$name.pid" ]; then
    pid="$(cat "$RUN_DIR/$name.pid")"
    kill -TERM -- "-$pid" 2>/dev/null || kill -TERM "$pid" 2>/dev/null
    rm -f "$RUN_DIR/$name.pid"
    echo "stopped $name (pgid $pid)"
  fi
done
sleep 1
# belt and braces: anything still holding the stack ports dies
for p in 8888 22994 29281; do
  lsof -ti:"$p" 2>/dev/null | xargs -r kill -9 2>/dev/null
done
hc s clean >/dev/null 2>&1 || true
echo "stack down"
