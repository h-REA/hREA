import { Scenario } from '@holochain/tryorama'
import { ApolloClient, InMemoryCache, type NormalizedCacheObject } from '@apollo/client/core'
import { SchemaLink } from '@apollo/client/link/schema'
import { createHolochainSchema } from '@valueflows/vf-graphql-holochain'
import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'
import { existsSync } from 'node:fs'

const HAPP_PATH = resolve(dirname(fileURLToPath(import.meta.url)), '../../../workdir/hrea.happ')

export interface Harness {
  client: ApolloClient<NormalizedCacheObject>
  scenario: Scenario
}

/**
 * Spawns an ephemeral tryorama conductor, installs the hREA hApp, and wires an
 * Apollo Client over a SchemaLink — the same access path real consumers
 * (e.g. Requests-and-Offers) use, minus the WebSocket transport.
 */
export async function createHarness(): Promise<Harness> {
  if (!existsSync(HAPP_PATH)) {
    throw new Error(`hApp bundle not found at ${HAPP_PATH} — run \`yarn build:happ\` from the repo root first.`)
  }
  const scenario = new Scenario()
  const [player] = await scenario.addPlayersWithApps([
    { appBundleSource: { type: 'path' as const, value: HAPP_PATH } },
  ])
  const schema = createHolochainSchema({ cell: player.cells[0] })
  const client = new ApolloClient({
    link: new SchemaLink({ schema }),
    cache: new InMemoryCache(),
    defaultOptions: {
      query: { fetchPolicy: 'no-cache' },
      mutate: { fetchPolicy: 'no-cache' },
    },
  })
  return { client, scenario }
}

export async function teardownHarness(harness: Harness): Promise<void> {
  await harness.scenario.cleanUp()
}
