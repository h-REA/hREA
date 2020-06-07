/**
 * Test environment bootstrap
 *
 * @package: HoloREA
 * @flow
 */

require('source-map-support').install()

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
const getDNA = ((dnas) => (path, name) => (Config.dna(dnas[path], name || path)))({
  'agent': path.resolve(__dirname, '../happs/agent/dist/agent.dna.json'),
  'specification': path.resolve(__dirname, '../happs/specification/dist/specification.dna.json'),
  'observation': path.resolve(__dirname, '../happs/observation/dist/observation.dna.json'),
  'planning': path.resolve(__dirname, '../happs/planning/dist/planning.dna.json'),
  'proposal': path.resolve(__dirname, '../happs/proposal/dist/proposal.dna.json'),
})

/**
 * Construct a test scenario out of the set of input instances & bridge configurations
 *
 * @param  {object} instances mapping of instance IDs to DNAs (@see getDNA)
 * @param  {object} bridges   (optional) mapping of bridge IDs to DNA instance ID pairs
 * @return Try-o-rama config instance for creating 'players'
 */
const buildConfig = (instances, bridges) => {
  return Config.gen(instances, {
    bridges: Array.isArray(bridges)
      ? bridges // accept pre-generated array
      : Object.keys(bridges || {}).reduce((b, bridgeId) => {
        b.push(Config.bridge(bridgeId, ...bridges[bridgeId]))
        return b
      }, []),
    network: {
      type: 'sim2h',
      sim2h_url: 'ws://localhost:9000',
    },
    // logger: Config.logger(!!process.env.VERBOSE_DNA_DEBUG),
  })
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

const buildGraphQL = (player, t, dnaConfig) => {
  const tester = new GQLTester(schema, resolverLoggerMiddleware()(generateResolvers({
    dnaConfig,
    conductorUri: `ws://localhost:${player._interfacePort}`,
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

const buildPlayer = async (scenario, playerName, config) => {
  const players = await scenario.players({ [playerName]: config }, true)
  const player = players[playerName]

  // inject a GraphQL API handler onto the player
  player.graphQL = buildGraphQL(player)

  return player
}

module.exports = {
  getDNA,
  buildConfig,
  buildPlayer,
  buildGraphQL,
  buildRunner,
  bridge: Config.bridge,
}
