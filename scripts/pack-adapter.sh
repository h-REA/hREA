#!/usr/bin/env bash
#
# Build the GraphQL adapter, check its schema surface, and pack it as the npm
# tarball a release attaches, with a sha256 file beside it.
#
# Usage: scripts/pack-adapter.sh [out-dir]          (default: dist/adapter)
#        HREA_ROOT=<checkout> scripts/pack-adapter.sh   pack another checkout,
#        for example an older tag, with this copy of the script.
#
# Expects the workspace dependencies to be installed already (yarn install).

set -euo pipefail

ROOT="${HREA_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
OUT="${1:-$ROOT/dist/adapter}"
MOD="$ROOT/modules/vf-graphql-holochain"

mkdir -p "$OUT"
OUT="$(cd "$OUT" && pwd)"

# The package's own build script runs `tsc; node ./finish-build`, so a type
# error still emits JavaScript and exits 0. Type-check first so it cannot.
(cd "$MOD" && npx --no-install tsc -p ./tsconfig.dev.json --noEmit)
(cd "$MOD" && npm run build)

test -f "$MOD/build/index.js" || { echo "pack-adapter: build/index.js missing"; exit 1; }
test -f "$MOD/build/index.d.ts" || { echo "pack-adapter: build/index.d.ts missing"; exit 1; }

if [ -f "$ROOT/scripts/verify-purpose-schema.mjs" ]; then
  node "$ROOT/scripts/verify-purpose-schema.mjs"
else
  echo "pack-adapter: no scripts/verify-purpose-schema.mjs in this checkout, schema check skipped"
fi

TARBALL="$(cd "$MOD/build" && npm pack --silent --pack-destination "$OUT")"
(cd "$OUT" && sha256sum "$TARBALL" > "$TARBALL.sha256")

echo "pack-adapter: $OUT/$TARBALL"
cat "$OUT/$TARBALL.sha256"
