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

runner.registerScenario('links can be written and read between DNAs', async (s, t, { planning, observation }) => {
  const commitment = {
    note: 'a commitment to provide something',
  }

  const commitmentResponse = await planning.call('commitment', 'create_commitment', { commitment })
  t.ok(commitmentResponse.Ok.commitment && commitmentResponse.Ok.commitment.id, 'commitment created successfully')
  await s.consistent()

  const event = {
    note: 'test event for which a fulfillment is created at the same time',
    fulfills: [commitmentResponse.Ok.commitment.id],
  }

  const eventResp = await observation.call('economic_event', 'create_event', { event })

  t.ok(eventResp.Ok.economicEvent && eventResp.Ok.economicEvent.id, 'event created successfully')
  await s.consistent()

  const readResponse = await observation.call('economic_event', 'get_event', { address: eventResp.Ok.economicEvent.id })
  t.equal(readResponse.Ok.economicEvent.fulfills.length, 1, 'fulfillment reference saved')
  t.equal(readResponse.Ok.economicEvent.fulfills[0], commitmentResponse.Ok.commitment.id, 'fulfillment commitment ID stored correctly')

  const readResponse2 = await planning.call('commitment', 'get_commitment', { address: commitmentResponse.Ok.commitment.id })

  t.ok(readResponse2.Ok.commitment.fulfilledBy, 'fulfilledBy reciprocal value present')
  t.equal(readResponse2.Ok.commitment.fulfilledBy.length, 1, 'fulfilledBy reciprocal reference saved')
  t.equal(readResponse2.Ok.commitment.fulfilledBy[0], eventResp.Ok.economicEvent.id, 'fulfilledBy reciprocal event ID stored correctly')
})

runner.run()
