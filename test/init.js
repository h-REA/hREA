/**
 * Test environment bootstrap
 *
 * @package: HoloREA
 * @flow
 */

import sourceMapSupport from 'source-map-support'

import { fileURLToPath } from 'url'
import path from 'path'
import { randomBytes } from 'crypto'
import { Base64 } from 'js-base64'
import readline from 'readline'

import { Scenario } from '@connoropolous/tryorama'

import { GraphQLError } from 'graphql'
import GQLTester from 'easygraphql-tester'
import resolverLoggerMiddleware from './graphql-logger-middleware.js'
import schema from '@valueflows/vf-graphql/ALL_VF_SDL.js'
import {
  generateResolvers,
  remapCellId,
} from '@valueflows/vf-graphql-holochain'
sourceMapSupport.install()

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

process.on('unhandledRejection', (error) => {
  console.error('unhandled rejection:', error)
  // delay exit so that debug logs have time to pipe through
  setTimeout(() => {
    process.exit(1)
  }, 500)
})

// DNA loader, to be used with `buildTestScenario` when constructing DNAs for testing
const dnaPaths = {
  agent: path.resolve(__dirname, '../bundles/dna/agent/hrea_agent.dna'),
  agreement: path.resolve(
    __dirname,
    '../bundles/dna/agreement/hrea_agreement.dna',
  ),
  observation: path.resolve(
    __dirname,
    '../bundles/dna/observation/hrea_observation.dna',
  ),
  planning: path.resolve(
    __dirname,
    '../bundles/dna/planning/hrea_planning.dna',
  ),
  proposal: path.resolve(
    __dirname,
    '../bundles/dna/proposal/hrea_proposal.dna',
  ),
  specification: path.resolve(
    __dirname,
    '../bundles/dna/specification/hrea_specification.dna',
  ),
  plan: path.resolve(__dirname, '../bundles/dna/plan/hrea_plan.dna'),
}
const getDNA = (name) => dnaPaths[name]

/**
 * Create per-agent interfaces to the DNA
 */
const buildGraphQL = async (player, apiOptions, appCellMapping) => {
  const tester = new GQLTester(
    schema,
    resolverLoggerMiddleware()(
      await generateResolvers({
        ...apiOptions,
        conductorUri: player.conductor.appWs().client.socket._url,
        dnaConfig: appCellMapping,
        traceAppSignals: (signal) => {
          console.info('App signal received:', signal)
        },
      }),
    ),
  )

  return async (query, params) => {
    const result = await tester.graphql(query, undefined, undefined, params);

    // GraphQL errors don't get caught internally by resolverLoggerMiddleware, need to be printed separately
    (result.errors || [])
      .filter((err) => err instanceof GraphQLError)
      .forEach((e) => {
        console.warn('\x1b[1m\x1b[31mGraphQL query error\x1b[0m', e)
      })

    return result
  }
}

/**
 * Creates bindings for a player against a single hApp, returning a GraphQL client
 * as well as the underlying Holochain DNA `cells`.
 */
const buildPlayer = async (agentDNAs, graphQLAPIOptions) => {
  // Create an empty scenario.
  const scenario = new Scenario({
    timeout: 30000,
  })
  const player = await scenario.addPlayerWithHappBundle({
    bundle: {
      manifest: {
        name: 'installed-app-id',
        manifest_version: '1',
        roles: agentDNAs.map((name) => ({
          // role_id is used again below, when accessing player.namedCells
          // this is why the name from agentDNAs must be equal to the role_id
          id: name,
          dna: {
            path: getDNA(name),
          },
        })),
      },
      resources: {},
    },
  })

  const cellIdsKeyedByRole = {}
  const cellsKeyedByRole = {}
  for (const [name, cell] of player.namedCells.entries()) {
    cellIdsKeyedByRole[name] = cell.cell_id
    cellsKeyedByRole[name] = cell
  }

  const cells = agentDNAs.map(name => cellsKeyedByRole[name]).map((cell) => {
    // patch for old synxtax for calling
    cell.call = (zomeName, fnName, payload) => {
      return cell.callZome({
        zome_name: zomeName,
        fn_name: fnName,
        payload,
      })
    }
    return cell
  })

  return {
    // :TODO: is it possible to derive GraphQL DNA binding config from underlying Tryorama `config`?
    graphQL: await buildGraphQL(player, graphQLAPIOptions, cellIdsKeyedByRole),
    cells,
    player,
    scenario,
  }
}

// @see https://crates.io/crates/holo_hash
const HOLOCHAIN_RAW_IDENTIFIER_LEN = 36
// @see holo_hash::hash_type::primitive
const HOLOHASH_PREFIX_DNA = Uint8Array.of(0x84, 0x2d, 0x24) // uhC0k
const HOLOHASH_PREFIX_ENTRY = Uint8Array.of(0x84, 0x21, 0x24) // uhCEk
// const HOLOHASH_PREFIX_HEADER = Uint8Array.of(0x84, 0x29, 0x24) // uhCkk
const HOLOHASH_PREFIX_AGENT = Uint8Array.of(0x84, 0x20, 0x24) // uhCAk

function serializeHash (hash) {
  return `u${Base64.fromUint8Array(hash, true)}`
}

function seralizeId (id) {
  return `${serializeHash(id[1])}:${serializeHash(id[0])}`
}

function concatenate (...arrays) {
  // Calculate byteSize from all arrays
  let size = arrays.reduce((a, b) => a + b.byteLength, 0)
  // Allcolate a new buffer
  let result = new Uint8Array(size)

  // Build the new array
  let offset = 0
  for (let arr of arrays) {
    result.set(arr, offset)
    offset += arr.byteLength
  }

  return result
}

/// Utility helper for use with connecting `holochain-playground` CLI to test runner (see `playground` NPM package script).
/// Simply `await waitForInput()` with optional message to pause the test terminal until user input is given.
function waitForInput (query = 'Press [ENTER] to continue...') {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  })

  return new Promise((resolve) =>
    rl.question(query, (ans) => {
      rl.close()
      resolve(ans)
    }),
  )
}

// :TODO: :SHONK: temporary code for mocking, eventually tests will need to populate mock data with referential integrity to pass
const mockAgentId = (asStr = true) => {
  const a = [
    Buffer.from(
      concatenate(
        HOLOHASH_PREFIX_DNA,
        randomBytes(HOLOCHAIN_RAW_IDENTIFIER_LEN).buffer,
      ),
    ),
    Buffer.from(
      concatenate(
        HOLOHASH_PREFIX_AGENT,
        randomBytes(HOLOCHAIN_RAW_IDENTIFIER_LEN).buffer,
      ),
    ),
  ]
  return asStr ? seralizeId(a) : a
}

const mockAddress = (asStr = true) => {
  const a = [
    Buffer.from(
      concatenate(
        HOLOHASH_PREFIX_DNA,
        randomBytes(HOLOCHAIN_RAW_IDENTIFIER_LEN).buffer,
      ),
    ),
    Buffer.from(
      concatenate(
        HOLOHASH_PREFIX_ENTRY,
        randomBytes(HOLOCHAIN_RAW_IDENTIFIER_LEN).buffer,
      ),
    ),
  ]
  return asStr ? seralizeId(a) : a
}

const mockIdentifier = (asStr = true) => {
  const dna = Buffer.from(
    concatenate(
      HOLOHASH_PREFIX_DNA,
      randomBytes(HOLOCHAIN_RAW_IDENTIFIER_LEN).buffer,
    ),
  )
  const id = 'mock'

  return asStr ? `${id}:${serializeHash(dna)}` : [dna, id]
}

// :TODO: temporary code until date indexing order is implemented
const sortById = (a, b) => {
  if (a.id === b.id) return 0
  return a.id < b.id ? -1 : 1
}

const sortByIdBuffer = (a, b) => {
  // :NOTE: this sorts on EntryHash, ignores DnaHash
  if (a.id[1] === b.id[1]) return 0
  return a.id[1] < b.id[1] ? -1 : 1
}

const sortBuffers = (a, b) => {
  // :NOTE: this sorts on EntryHash, ignores DnaHash
  if (a[1] === b[1]) return 0
  return a[1] < b[1] ? -1 : 1
}

export {
  getDNA,
  buildPlayer,
  buildGraphQL,
  seralizeId,
  mockAgentId,
  remapCellId,
  mockAddress,
  mockIdentifier,
  sortById,
  sortByIdBuffer,
  sortBuffers,
  waitForInput,
}
