/**
 * Test environment bootstrap
 *
 * @package: HoloREA
 * @flow
 */

// :TODO: need to implement per-agent GraphQL query interface via dynamic websocket port bindings
const WEBSOCKET_PORT = 4001
process.env.REACT_APP_HC_CONN_URL = `ws://localhost:${WEBSOCKET_PORT}`

const os = require('os')
const fs = require('fs')
const path = require('path')
const tape = require('tape')
const getPort = require('get-port')

const { Orchestrator, Config, tapeExecutor, combine } = require('@holochain/try-o-rama')

const GQLTester = require('easygraphql-tester')
const { all_vf: schema } = require('@valueflows/vf-graphql/typeDefs')
const resolvers = require('@valueflows/vf-graphql-holochain/build/resolvers')

// :SHONK: workaround TS compiler props
delete resolvers['__esModule']

process.on('unhandledRejection', error => {
  console.error('unhandled rejection:', error)
  // delay exit so that debug logs have time to pipe through
  setTimeout(() => {
    process.exit(1)
  }, 500)
})

// DNA loader, to be used with `buildTestScenario` when constructing DNAs for testing
const getDNA = ((dnas) => (path) => (Config.dna(dnas[path], path)))({
  'observation': path.resolve(__dirname, '../happs/observation/dist/observation.dna.json'),
  'planning': path.resolve(__dirname, '../happs/planning/dist/planning.dna.json'),
})

/**
 * Construct a test scenario out of the set of input instances & bridge configurations
 *
 * @param  {object} instances mapping of instance IDs to DNAs (@see getDNA)
 * @param  {object} bridges   (optional) mapping of bridge IDs to DNA instance ID pairs
 * @return Try-o-rama config instance for creating 'players'
 */
const buildConfig = (instances, bridges) => Config.genConfig({
  instances,
  bridges: Object.keys(bridges || {}).reduce((b, bridgeId) => {
    b.push(Config.bridge(bridgeId, ...bridges[bridgeId]))
    return b
  }, []),
  network: "memory",
}, {
  logger: process.env.VERBOSE_DNA_DEBUG ? true : false,
})

/**
 * Temporary directory handling for conductor orchestrator
 * (needed due to overriding of genConfigArgs to set websocket port for GraphQL client bindings)
 */
const mkdirIdempotent = async (dir) => {
  try {
    await fs.access(dir)
  } catch (e) {
    fs.mkdirSync(dir, { recursive: true })
  }
}
const tempDirBase = path.join(os.tmpdir(), 'try-o-rama/')
const tempDir = async () => {
    await mkdirIdempotent(tempDirBase)
    return fs.mkdtempSync(tempDirBase)
}

/**
 * Create a test scenario orchestrator instance
 */
const runner = new Orchestrator({
  genConfigArgs: async (conductorName, uuid) => ({
    conductorName,
    uuid,
    configDir: await tempDir(),
    adminPort: await getPort(),
    zomePort: WEBSOCKET_PORT,  // :TODO: allow multiple agents to be interacted with via multiple websocket ports
  }),
  middleware: combine(tapeExecutor(tape)),
})

/**
 * GraphQL client interface connected to scenario websocket connection
 */
const tester = new GQLTester(schema, resolvers)
const graphQL = (query, params) => tester.graphql(query, undefined, undefined, params)

module.exports = {
  getDNA,
  buildConfig,
  runner,
  graphQL,
}
