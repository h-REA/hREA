const {
  getDNA,
  buildOrchestrator,
} = require('../init')

const runner = buildOrchestrator({
  observation: getDNA('observation'),
})

runner.registerScenario('create simplest event', async (s, t, { observation }) => {
  const event = {
    note: 'test event',
  }

  const createEventResponse = await observation.call('economic_event', 'create_event', { event })

  await s.consistent()

  console.log(require('util').inspect(createEventResponse, { depth: null, colors: true }))
})

runner.run()
