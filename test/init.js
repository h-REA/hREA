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
const getDNA = ((dnas) => (name) => (dnas[name]))({
  'agent': path.resolve(__dirname, '../happs/agent/dist/agent.dna.gz'),
  'agreement': path.resolve(__dirname, '../happs/agreement/dist/agreement.dna.gz'),
  'observation': path.resolve(__dirname, '../happs/observation/dist/observation.dna.gz'),
  'planning': path.resolve(__dirname, '../happs/planning/dist/planning.dna.gz'),
  'proposal': path.resolve(__dirname, '../happs/proposal/dist/proposal.dna.gz'),
  'specification': path.resolve(__dirname, '../happs/specification/dist/specification.dna.gz'),
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
const buildGraphQL = (player, apiOptions) => {
  const tester = new GQLTester(schema, resolverLoggerMiddleware()(generateResolvers({
    ...apiOptions,
    conductorUri: player.appWs().client.socket._url,
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
  const [[first_happ]] = await player.installAgentsHapps([[agentDNAs.map(getDNA)]])

  return {
    // :TODO: is it possible to derive GraphQL DNA binding config from underlying Tryorama `config`?
    graphQL: buildGraphQL(player, graphQLAPIOptions),
    cells: first_happ.cells,
    player,
  }
}

module.exports = {
  getDNA,
  buildPlayer,
  buildGraphQL,
  buildRunner,
  bridge: Config.bridge,
  buildConfig: Config.gen,
}
