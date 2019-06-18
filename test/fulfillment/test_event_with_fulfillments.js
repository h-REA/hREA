const {
  getDNA,
  buildOrchestrator,
} = require('../init')

const runner = buildOrchestrator({
  observation: getDNA('observation'),
  planning: getDNA('planning'),
}, {
  vf_planning: ['observation', 'planning'],
})

runner.registerScenario('create event with linked fulfillments', async (s, t, { observation, planning }) => {
  const event = {
    note: 'test event for which a fulfillment is created at the same time',
    fulfills: ['TODO_COMMITMENT'],
  }

  const createEventResponse = await observation.call('main', 'create_event', { event })

  console.log(require('util').inspect(createEventResponse, { depth: null, colors: true }))
})

runner.run()
