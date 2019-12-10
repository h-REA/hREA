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

const GQLTester = require('easygraphql-tester')
const resolverLoggerMiddleware = require('./graphql-logger-middleware')
const { all_vf: schema } = require('@valueflows/vf-graphql/typeDefs')
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
  'observation': path.resolve(__dirname, '../happs/observation/dist/observation.dna.json'),
  'planning': path.resolve(__dirname, '../happs/planning/dist/planning.dna.json'),
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

    console.error(`BGERK ${playerName} is on port ${interfacePort}`, {
      ...args,
      interfacePort,
    })

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
  const result = await tester.graphql(query, undefined, undefined, params)

  // print errors to stderr
  if (result.errors && result.errors.length) {
    if (t) { // use tape assertion API if it's been injected
      result.errors.forEach(err => {
        t.error(err, "\x1b[1m\x1b[31mGraphQL query error\x1b[0m" + (err.path ? (" at \x1b[1m" + err.path.join('.')) : ":") + "\x1b[0m")
      })
    } else {
      console.error("\x1b[1m\x1b[31mGraphQL query errors:\x1b[0m", require('util').inspect(result.errors, { depth: null, colors: true }))
    }
  }

  return result
}

module.exports = {
  getDNA,
  buildConfig,
  buildRunner,
  buildGraphQL,
}
