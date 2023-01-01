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

import { Scenario } from '@holochain/tryorama'

import { GraphQLError } from 'graphql'
import GQLTester from 'easygraphql-tester'
import resolverLoggerMiddleware from './graphql-logger-middleware.js'
import { buildSchema, printSchema } from '@valueflows/vf-graphql'
import {
  generateResolvers,
  remapCellId,
  hreaExtensionSchemas,
  DEFAULT_VF_MODULES,
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
const buildGraphQL = async (player, apiOptions = {}, appCellMapping) => {
  const {
    // use full supported set of modules by default
    enabledVFModules = DEFAULT_VF_MODULES,
    extensionSchemas = [],
  } = apiOptions
  const overriddenExtensionSchemas = [...extensionSchemas, hreaExtensionSchemas.associateMyAgentExtension]
  const schema = printSchema(buildSchema(enabledVFModules, overriddenExtensionSchemas))
  const tester = new GQLTester(
    schema,
    resolverLoggerMiddleware()(
      await generateResolvers({
        ...apiOptions,
        enabledVFModules,
        conductorUri: player.conductor.appWs().client.socket._url,
        adminConductorUri: player.conductor.adminWs().client.socket._url,
        appId: player.appId,
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
const buildPlayer = async (dnasToInstall, graphQLAPIOptions) => {
  // Create an empty scenario.
  const scenario = new Scenario({
    timeout: 60000,
  })
  try {
    const player = await scenario.addPlayerWithApp({
      bundle: {
        manifest: {
          name: 'installed-app-id',
          manifest_version: '1',
          roles: dnasToInstall.map((name) => ({
            name: `hrea_${name}_1`,
            dna: {
              path: getDNA(name),
            },
          })),
        },
        resources: {},
      },
    })

    console.info(`Created new player with admin URI ${player.conductor.adminWs().client.socket._url}`)

    const cellIdsKeyedByRole = {}
    const cellsKeyedByRole = {}
    for (const [name, cell] of player.namedCells.entries()) {
      const hreaCellMatch = name.match(/hrea_(\w+)_\d+/)
      if (hreaCellMatch) {
        const hreaRole = hreaCellMatch[1]
        cellIdsKeyedByRole[hreaRole] = cell.cell_id
        cellsKeyedByRole[hreaRole] = cell
      }
    }

    const cells = dnasToInstall.map((name) => cellsKeyedByRole[name]).map((cell) => {
      // patch for old syntax for calling
      cell.call = (zomeName, fnName, payload) => {
        return cell.callZome({
          zome_name: zomeName,
          fn_name: fnName,
          payload,
        }, 60000)
      }
      return cell
    })

    try {
      const graphQL = await buildGraphQL(player, graphQLAPIOptions, cellIdsKeyedByRole)
      return {
        // :TODO: is it possible to derive GraphQL DNA binding config from underlying Tryorama `config`?
        graphQL,
        cells,
        player,
        scenario,
      }
    } catch (e) {
      await scenario.cleanUp()
      console.error('error during buildGraphQL: ', e)
      throw e
    }
  } catch (e) {
    await scenario.cleanUp()
    console.error('error during scenario.addPlayerWithHappBundle: ', e)
    throw e
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

function serializeId (id) {
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
  return asStr ? serializeId(a) : a
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
  return asStr ? serializeId(a) : a
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

export {
  getDNA,
  buildPlayer,
  buildGraphQL,
  serializeId,
  mockAgentId,
  remapCellId,
  mockAddress,
  mockIdentifier,
  waitForInput,
}
