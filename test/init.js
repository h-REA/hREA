/**
 * Test environment bootstrap
 *
 * @package: HoloREA
 * @flow
 */

require('source-map-support').install()

const os = require('os')
const fs = require('fs')
const path = require('path')
const tape = require('tape')
const getPort = require('get-port')

const { Orchestrator, Config, combine, tapeExecutor, localOnly } = require('@holochain/tryorama')

const { GraphQLError } = require('graphql')
const GQLTester = require('easygraphql-tester')
const resolverLoggerMiddleware = require('./graphql-logger-middleware')
const schema = require('@valueflows/vf-graphql/ALL_VF_SDL')
const { resolvers, setConnectionURI } = require('@valueflows/vf-graphql-holochain')

process.on('unhandledRejection', error => {
  console.error('unhandled rejection:', error)
  // delay exit so that debug logs have time to pipe through
  setTimeout(() => {
    process.exit(1)
  }, 500)
})

// DNA loader, to be used with `buildTestScenario` when constructing DNAs for testing
const getDNA = ((dnas) => (path) => (Config.dna(dnas[path], path)))({
  'specification': path.resolve(__dirname, '../happs/specification/dist/specification.dna.json'),
  'observation': path.resolve(__dirname, '../happs/observation/dist/observation.dna.json'),
  'planning': path.resolve(__dirname, '../happs/planning/dist/planning.dna.json'),
  'proposal': path.resolve(__dirname, '../happs/proposal/dist/proposal.dna.json'),
})

const conductorZomePorts = {}

/**
 * Construct a test scenario out of the set of input instances & bridge configurations
 *
 * @param  {object} instances mapping of instance IDs to DNAs (@see getDNA)
 * @param  {object} bridges   (optional) mapping of bridge IDs to DNA instance ID pairs
 * @return Try-o-rama config instance for creating 'players'
 */
const buildConfig = (instances, bridges) => {
  const config = Config.gen(instances, {
    bridges: Object.keys(bridges || {}).reduce((b, bridgeId) => {
      b.push(Config.bridge(bridgeId, ...bridges[bridgeId]))
      return b
    }, []),
    network: {
      type: 'sim2h',
      sim2h_url: 'ws://localhost:9000',
    },
    // logger: Config.logger(!!process.env.VERBOSE_DNA_DEBUG),
  })

  return async (args) => {
    const { playerName } = args
    let interfacePort
    if (conductorZomePorts[playerName]) {
      interfacePort = conductorZomePorts[playerName]
    } else {
      conductorZomePorts[playerName] = await getPort()
      interfacePort = conductorZomePorts[playerName]
    }

    return config({
      ...args,
      interfacePort,
    })
  }
}

/**
 * Temporary directory handling for conductor orchestrator
 * (needed due to overriding of genConfigArgs to set websocket port for GraphQL client bindings)
 */
const mkdirIdempotent = async (dir) => {
  try {
    await fs.access(dir)
  } catch (e) {
    try {
      fs.mkdirSync(dir, { recursive: true })
    } catch (e) {
      console.warn(e)
    }
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

const buildRunner = () => new Orchestrator({
  middleware: combine(
    tapeExecutor(tape),
    localOnly,
  ),
})

/**
 * Create per-agent interfaces to the DNA
 */

const tester = new GQLTester(schema, resolverLoggerMiddleware()(resolvers))

const buildGraphQL = (player, t) => async (query, params) => {
  setConnectionURI(`ws://localhost:${conductorZomePorts[player.name]}`)
  const result = await tester.graphql(query, undefined, undefined, params);

  // GraphQL errors don't get caught internally by resolverLoggerMiddleware, need to be printed separately
  (result.errors || [])
    .filter(err => err instanceof GraphQLError)
    .forEach(e => {
      console.warn('\x1b[1m\x1b[31mGraphQL query error\x1b[0m', e)
    })

  return result
}

const buildPlayer = async (scenario, playerName, config) => {
  const players = await scenario.players({ [playerName]: config }, true)
  const player = players[playerName]

  // inject a GraphQL API handler onto the player
  player.graphQL = buildGraphQL(player)

  // cleanup old port mapping for next test in run upon exiting the conductor
  player.__oldCleanup = player.cleanup
  player.cleanup = () => {
    delete conductorZomePorts[playerName]
    return player.__oldCleanup()
  }

  return player
}

module.exports = {
  getDNA,
  buildConfig,
  buildPlayer,
  buildRunner,
}
