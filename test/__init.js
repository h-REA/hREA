/**
 * Test environment bootstrap
 *
 * @package: HoloREA
 * @author:  pospi <pospi@spadgos.com>
 * @since:   2019-02-04
 * @flow
 */

const path = require('path')

const { Config, Container } = require("@holochain/holochain-nodejs")

const dnaPath = path.resolve(__dirname, "../dist/bundle.json")

// closure to keep config-only stuff out of test scope
const container = (() => {
  const agentAlice = Config.agent("alice")

  const dna = Config.dna(dnaPath)

  const instanceAlice = Config.instance(agentAlice, dna)

  const containerConfig = Config.container([instanceAlice])
  return new Container(containerConfig)
})()

// Initialize the Container
container.start()

module.exports = {
  container
}
