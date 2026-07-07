# hREA Playground E2E (Playwright)

Browser tests that drive the real playground UI (GqlForm / GqlTable / GoldenLayout) against a live sandboxed conductor. Complements `clients/acceptance` (schema-level API battery) by proving the human-facing surface.

## What it proves

| Step | Guards |
|---|---|
| Load + connect | AdminWebsocket token flow, AppWebsocket connection, schema construction in-browser |
| Create Organization via form | GqlForm submit path, success feedback states |
| Organization appears in table | svelte-apollo refetch, GqlTable rendering |
| + Window | GoldenLayout `addWindow` crash regression |
| Proposals table renders | invalid `status` field regression (query ↔ schema drift) |
| Create Unit → Units table | multi-field form, second create/read round-trip |
| Console health | zero uncaught errors across the whole session |

## Usage

```bash
nix develop                      # hc + node toolchain
yarn build:happ                  # if workdir/hrea.happ is missing/stale

# one-time: install the browser binary
cd clients/playground-e2e && npx playwright install chromium && cd ../..

yarn workspace hrea-playground-e2e run stack:up     # conductor (mem) + vite :8888
yarn workspace hrea-playground-e2e run e2e          # headless run, exit code 0/1
yarn workspace hrea-playground-e2e run e2e:headed   # watch it drive the UI
yarn workspace hrea-playground-e2e run stack:down   # teardown + hc s clean
```

Screenshots land in `clients/playground-e2e/shots/` at each step. `FATAL` + exit 2 means harness failure (stack not up) rather than an assertion.

## Notes

- Ports mirror the repo dev topology: admin 22994, app 29281, ui 8888. Override the target with `PLAYGROUND_URL`.
- The stack scripts use `network mem` (Holochain 0.6.1 sandbox CLI). The root `start:happ` / `network` scripts still use the pre-0.6.1 `webrtc` syntax and `hc run-local-services`, which no longer exist — use these scripts (or `launch:happ1`) until those are updated.
- Selectors are ARIA-role based (`getByRole`) with `.last()` targeting the newest GoldenLayout panel — keep new steps in that idiom.
