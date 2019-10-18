const {
  getDNA,
  buildConfig,
  runner,
} = require('../init')

const config = buildConfig({
  observation: getDNA('observation'),
})

runner.registerScenario('create simplest event', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)

  const event = {
    note: 'test event',
  }

  const createEventResponse = await alice.call('observation', 'economic_event', 'create_event', { event })

  await s.consistency()

  console.log(require('util').inspect(createEventResponse, { depth: null, colors: true }))
})

runner.run()
