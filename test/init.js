/**
 * Test environment bootstrap
 *
 * @package: HoloREA
 * @flow
 */

require('source-map-support').install()

const randombytes = require('randombytes')
const path = require('path')
const tape = require('tape')

const { Orchestrator, Config, combine, tapeExecutor, localOnly } = require('@holochain/tryorama')

const { GraphQLError } = require('graphql')
const GQLTester = require('easygraphql-tester')
const resolverLoggerMiddleware = require('./graphql-logger-middleware')
const schema = require('@valueflows/vf-graphql/ALL_VF_SDL')
const { generateResolvers } = require('@valueflows/vf-graphql-holochain')

process.on('unhandledRejection', error => {
  console.error('unhandled rejection:', error)
  // delay exit so that debug logs have time to pipe through
  setTimeout(() => {
    process.exit(1)
  }, 500)
})

// DNA loader, to be used with `buildTestScenario` when constructing DNAs for testing
const getDNA = ((dnas) => (name) => (dnas[name]))({
  'agent': path.resolve(__dirname, '../happs/agent/hrea_agent.dna'),
  'agreement': path.resolve(__dirname, '../happs/agreement/hrea_agreement.dna'),
  'observation': path.resolve(__dirname, '../happs/observation/hrea_observation.dna'),
  'planning': path.resolve(__dirname, '../happs/planning/hrea_planning.dna'),
  'proposal': path.resolve(__dirname, '../happs/proposal/hrea_proposal.dna'),
  'specification': path.resolve(__dirname, '../happs/specification/hrea_specification.dna'),
})

/**
 * Create a test scenario orchestrator instance
 */
const buildRunner = () => new Orchestrator({
  middleware: combine(
    tapeExecutor(tape),
    localOnly,
  ),
})

/**
 * Create per-agent interfaces to the DNA
 */
const buildGraphQL = async (player, apiOptions, appCellIds) => {
  const appCells = await player.adminWs().listCellIds()
  const appCellMapping = appCells.reduce((r, cell, idx) => {
    r[appCellIds[idx]] = cell
    return r
  }, {})

  const tester = new GQLTester(schema, resolverLoggerMiddleware()(await generateResolvers({
    ...apiOptions,
    conductorUri: player.appWs().client.socket._url,
    dnaConfig: appCellMapping,
    traceAppSignals: (signal) => {
      console.info('App signal received:', signal)
    },
  })))

  return async (query, params) => {
    const result = await tester.graphql(query, undefined, undefined, params);

    // GraphQL errors don't get caught internally by resolverLoggerMiddleware, need to be printed separately
    (result.errors || [])
      .filter(err => err instanceof GraphQLError)
      .forEach(e => {
        console.warn('\x1b[1m\x1b[31mGraphQL query error\x1b[0m', e)
      })

    return result
  }
}

/**
 * Creates bindings for a player against a single hApp, returning a GraphQL client
 * as well as the underlying Holochain DNA `cells`.
 */
const buildPlayer = async (scenario, config, agentDNAs, graphQLAPIOptions) => {
  const [player] = await scenario.players([config])
  const [[firstHapp]] = await player.installAgentsHapps([[agentDNAs.map(getDNA)]])
  const appCellIds = firstHapp.cells.map(c => c.cellNick.match(/hrea_(\w+)\.dna/)[1])

  shimConsistency(scenario)

  return {
    // :TODO: is it possible to derive GraphQL DNA binding config from underlying Tryorama `config`?
    graphQL: await buildGraphQL(player, graphQLAPIOptions, appCellIds),
    cells: firstHapp.cells,
    player,
  }
}

// temporary method for RSM until conductor can interpret consistency
function shimConsistency (s) {
  s.consistency = () => new Promise((resolve, reject) => {
    setTimeout(resolve, 100)
  })
}

// @see https://crates.io/crates/holo_hash
const HOLOCHAIN_RAW_IDENTIFIER_LEN = 36
// @see holo_hash::hash_type::primitive
const HOLOHASH_PREFIX_DNA = [0x84, 0x2d, 0x24] // uhC0k
const HOLOHASH_PREFIX_ENTRY = [0x84, 0x21, 0x24] // uhCEk
// const HOLOHASH_PREFIX_HEADER = [0x84, 0x29, 0x24] // uhCkk
const HOLOHASH_PREFIX_AGENT = [0x84, 0x20, 0x24] // uhCAk

module.exports = {
  getDNA,
  buildPlayer,
  buildGraphQL,
  buildRunner,
  bridge: Config.bridge,
  buildConfig: Config.gen,

  // :TODO: :SHONK: temporary code for mocking, eventually tests will need to populate mock data with referential integrity to pass
  mockAgentId: () => {
    return [
      Buffer.from(HOLOHASH_PREFIX_DNA.concat(randombytes(HOLOCHAIN_RAW_IDENTIFIER_LEN))),
      Buffer.from(HOLOHASH_PREFIX_AGENT.concat(randombytes(HOLOCHAIN_RAW_IDENTIFIER_LEN))),
    ]
  },
  mockAddress: () => {
    return [
      Buffer.from(HOLOHASH_PREFIX_DNA.concat(randombytes(HOLOCHAIN_RAW_IDENTIFIER_LEN))),
      Buffer.from(HOLOHASH_PREFIX_ENTRY.concat(randombytes(HOLOCHAIN_RAW_IDENTIFIER_LEN))),
    ]
  },
}
