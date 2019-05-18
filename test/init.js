/**
 * Test environment bootstrap
 *
 * @package: HoloREA
 * @flow
 */

const path = require('path')

const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

// IDs for DNAs, to be used with `buildAgentAppSuiteInstances` when constructing DNAs for testing
const getDnaPath = ((dnas) => (path) => (dnas[path]))({
  "observation": "../happs/observation/dist/observation.dna.json",
  "planning": "../happs/planning/dist/planning.dna.json",
})

// Builds a set of 'connected' app instances, all shared by the same agent
const buildAgentAppSuiteInstances = (agentName, dnaIds) => {
  const agent = Config.agent(agentName)
  return dnaIds.map(dna =>
    Config.instance(agent, Config.dna(getDnaPath(dna)), `${agentName}_${dna}`)
  )
}

// Construct a test scenario out of the set of input instances (for one or more agents, as needed)
const buildTestScenario = (...instances) => new Scenario(instances)

module.exports = {
  buildAgentAppSuiteInstances,
  buildTestScenario,
}
