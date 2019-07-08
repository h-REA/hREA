const {
  getDNA,
  buildOrchestrator,
} = require('../init')

const runner = buildOrchestrator({
  planning: getDNA('planning'),
})

runner.registerScenario('links can be written and read between zomes', async (s, t, { planning }) => {
  const intent = {
    note: 'an intent to do something',
  }

  const intentResponse = await planning.call('intent', 'create_intent', { intent })
  await s.consistent()

  t.ok(intentResponse.Ok.intent && intentResponse.Ok.intent.id, 'intent created successfully')

  const commitment = {
    satisfies: [intentResponse.Ok.intent.id],
  }

  const commitmentResponse = await planning.call('commitment', 'create_commitment', { commitment })
  await s.consistent()

  t.ok(commitmentResponse.Ok.commitment && commitmentResponse.Ok.commitment.id, 'commitment created successfully')

  const readResponse = await planning.call('commitment', 'get_commitment', { address: commitmentResponse.Ok.commitment.id })

  t.equal(readResponse.Ok.commitment.satisfies.length, 1, 'satisfaction reference saved')
  t.equal(readResponse.Ok.commitment.satisfies[0], intentResponse.Ok.intent.id, 'satisfaction intent ID stored correctly')

  const readResponse2 = await planning.call('intent', 'get_intent', { address: intentResponse.Ok.intent.id })

  t.ok(readResponse2.Ok.intent.satisfiedBy, 'satisfiedBy reciprocal value present')
  t.equal(readResponse2.Ok.intent.satisfiedBy.length, 1, 'satisfiedBy reciprocal reference saved')
  t.equal(readResponse2.Ok.intent.satisfiedBy[0], commitmentResponse.Ok.commitment.id, 'satisfiedBy reciprocal commitment ID stored correctly')
})

// :TODO: can't pass this test yet, errors in the DNA code are uncaptured AFAIK
runner.registerScenario('cross-zome linking errors if target is not present', async (s, t, { planning }) => {
  const commitment = {
    satisfies: ['some_non_hash'],
  }

  try {
    await planning.call('commitment', 'create_commitment', { commitment })
    await s.consistent()
  } catch (e) {
    t.equal(e.message, 'Holochain Instance Error: WASM invocation failed: Trap: Trap { kind: Unreachable }')
  }
})

runner.run()
