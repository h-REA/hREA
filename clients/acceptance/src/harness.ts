import { AdminWebsocket, AppWebsocket, CellType } from '@holochain/client'
import { ApolloClient, InMemoryCache, type NormalizedCacheObject } from '@apollo/client/core'
import { SchemaLink } from '@apollo/client/link/schema'
import { createHolochainSchema } from '@valueflows/vf-graphql-holochain'
import { spawn, type ChildProcess } from 'node:child_process'
import { existsSync } from 'node:fs'
import { mkdtemp, rm } from 'node:fs/promises'
import { createServer } from 'node:net'
import { tmpdir } from 'node:os'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const HAPP_PATH = resolve(dirname(fileURLToPath(import.meta.url)), '../../../workdir/hrea.happ')
const APP_ID = 'hrea'
const ROLE_NAME = 'hrea'
const PASSPHRASE = 'acceptance'
// `hc sandbox generate` builds a lair keystore before the admin port opens,
// which takes the best part of a minute on a cold machine.
const BOOT_TIMEOUT_MS = 180_000
// Holochain rejects a websocket upgrade that carries no `Origin` header with a
// bare 400, even when the interface allows any origin. So we always send one,
// and the app interface we attach below names it explicitly.
const ORIGIN = 'hrea-acceptance'

export interface Harness {
  client: ApolloClient<NormalizedCacheObject>
  teardown: () => Promise<void>
}

/** Ask the OS for a port nobody is using, then hand it straight back. */
async function freePort(): Promise<number> {
  return new Promise((res, rej) => {
    const probe = createServer()
    probe.on('error', rej)
    probe.listen(0, '127.0.0.1', () => {
      const port = (probe.address() as { port: number }).port
      probe.close(() => res(port))
    })
  })
}

const sleep = (ms: number) => new Promise(res => setTimeout(res, ms))

/** The conductor takes a few seconds to open its admin port. Wait for it. */
async function connectAdmin(port: number): Promise<AdminWebsocket> {
  const deadline = Date.now() + BOOT_TIMEOUT_MS
  let last: unknown
  while (Date.now() < deadline) {
    try {
      return await AdminWebsocket.connect({
        url: new URL(`ws://127.0.0.1:${port}`),
        wsClientOptions: { origin: ORIGIN },
      })
    } catch (e) {
      last = e
      await sleep(250)
    }
  }
  throw new Error(`conductor admin port ${port} never came up: ${last}`)
}

/**
 * Spawns an ephemeral `hc sandbox` conductor, installs the hREA hApp, and wires
 * an Apollo Client over a SchemaLink.
 *
 * The GraphQL schema is built from `{ appWebSocket, roleName }`, which is the
 * access path real consumers use: `createHolochainSchema` wraps it in its own
 * `callZome`. Passing a Tryorama cell through the optional `cell` escape hatch
 * would test a path nobody ships on, and would keep `@holochain/tryorama` on
 * the critical path for the Holochain 0.7 upgrade.
 */
export async function createHarness(): Promise<Harness> {
  if (!existsSync(HAPP_PATH)) {
    throw new Error(`hApp bundle not found at ${HAPP_PATH} — run \`yarn build:happ\` from the repo root first.`)
  }

  const root = await mkdtemp(join(tmpdir(), 'hrea-acceptance-'))
  const adminPort = await freePort()
  // `--run` needs a port to bind, but we attach our own app interface below so
  // we control `allowed_origins` rather than depending on the sandbox default.
  const sandboxAppPort = await freePort()

  const conductor: ChildProcess = spawn('hc', [
    'sandbox', '--piped', `-f=${adminPort}`,
    'generate', HAPP_PATH,
    '-a', APP_ID,
    '--root', root,
    '-d', 'acceptance',
    `--run=${sandboxAppPort}`,
    'network', 'mem',
  ], { stdio: ['pipe', 'ignore', 'pipe'], cwd: root })

  const stderr: string[] = []
  conductor.stderr?.on('data', (d: Buffer) => { stderr.push(d.toString()) })
  conductor.stdin?.write(`${PASSPHRASE}\n`)
  conductor.stdin?.end()

  const cleanup = async () => {
    if (conductor.exitCode === null) {
      conductor.kill('SIGTERM')
      await Promise.race([
        new Promise(res => conductor.once('exit', res)),
        sleep(5_000).then(() => conductor.kill('SIGKILL')),
      ])
    }
    await rm(root, { recursive: true, force: true })
  }

  try {
    const admin = await connectAdmin(adminPort)
    const { port: appPort } = await admin.attachAppInterface({ port: 0, allowed_origins: ORIGIN })
    const { token } = await admin.issueAppAuthenticationToken({ installed_app_id: APP_ID })
    const appWebSocket = await AppWebsocket.connect({
      url: new URL(`ws://127.0.0.1:${appPort}`),
      token,
      wsClientOptions: { origin: ORIGIN },
    })

    // Zome calls are signed, and the signing key needs a capability grant the
    // conductor knows about. Tryorama did this lazily on first call; doing it
    // once here keeps the failure ("no signing credentials have been authorized
    // for cell") at setup rather than inside a test step.
    //
    // The grant is itself a call into the cell, and the cell is still coming up
    // for a moment after `hc sandbox` reports the app installed, so this retries
    // past the `CellDisabled` window.
    const deadline = Date.now() + BOOT_TIMEOUT_MS
    let authorized = false
    let lastAuthError: unknown
    while (!authorized && Date.now() < deadline) {
      try {
        await admin.enableApp({ installed_app_id: APP_ID })
        const info = await appWebSocket.appInfo()
        const provisioned = (info?.cell_info?.[ROLE_NAME] ?? [])
          .find(c => c.type === CellType.Provisioned)
        if (!provisioned) throw new Error(`app "${APP_ID}" has no provisioned cell for role "${ROLE_NAME}"`)
        await admin.authorizeSigningCredentials(provisioned.value.cell_id)
        authorized = true
      } catch (e) {
        lastAuthError = e
        await sleep(500)
      }
    }
    if (!authorized) throw new Error(`could not authorize signing credentials: ${lastAuthError}`)

    const schema = createHolochainSchema({ appWebSocket, roleName: ROLE_NAME })
    const client = new ApolloClient({
      link: new SchemaLink({ schema }),
      cache: new InMemoryCache(),
      defaultOptions: {
        query: { fetchPolicy: 'no-cache' },
        mutate: { fetchPolicy: 'no-cache' },
      },
    })

    return {
      client,
      teardown: async () => {
        // client 0.21 types AppWebsocket.client as AppClientTransport, which
        // declares only `request` and `on`. The websocket path still hands back
        // a WsClient underneath, but the Tauri IPC path has no socket to close,
        // so this asks rather than casts.
        closeIfCloseable(appWebSocket.client)
        admin.client.close()
        await cleanup()
      },
    }
  } catch (e) {
    await cleanup()
    throw new Error(`failed to bring up the acceptance conductor: ${e}\n${stderr.join('')}`)
  }
}


/** Close a transport that has a socket under it; no-op for one that does not. */
function closeIfCloseable(transport: unknown): void {
  const close = (transport as { close?: unknown }).close
  if (typeof close === 'function') (close as () => unknown).call(transport)
}

export async function teardownHarness(harness: Harness): Promise<void> {
  await harness.teardown()
}
