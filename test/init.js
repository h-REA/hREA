/**
 * Test environment bootstrap
 *
 * @package: HoloREA
 * @flow
 */

const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  console.error('unhandled rejection:', error)
  process.exit(1)
})

// DNA loader, to be used with `buildTestScenario` when constructing DNAs for testing
const getDNA = ((dnas) => (path) => (Diorama.dna(dnas[path], path)))({
  'observation': path.resolve(__dirname, '../happs/observation/dist/observation.dna.json'),
  'planning': path.resolve(__dirname, '../happs/planning/dist/planning.dna.json'),
})

/**
 * Construct a test scenario out of the set of input instances & bridge configurations
 *
 * @param  {object} instances mapping of instance IDs to DNAs (@see getDNA)
 * @param  {object} bridges   (optional) mapping of bridge IDs to DNA instance ID pairs
 * @return Diorama instance for testing
 */
const buildOrchestrator = (instances, bridges) => new Diorama({
  instances,
  bridges: Object.keys(bridges || {}).reduce((b, bridgeId) => {
    b.push(Diorama.bridge(bridgeId, ...bridges[bridgeId]))
    return b
  }, []),
  debugLog: process.env.VERBOSE_DNA_DEBUG || false,
  executor: tapeExecutor(tape),
})

module.exports = {
  getDNA,
  buildOrchestrator,
}
