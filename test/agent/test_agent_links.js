const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  agent: getDNA('agent'),
  observation: getDNA('observation'),
  planning: getDNA('planning'),
  proposal: getDNA('proposal'),
}, {
  vf_observation: ['planning', 'observation'],
  vf_planning: ['proposal', 'planning'],
})

// required attributes, not involved with test logic
const testEventProps = {
  resourceClassifiedAs: ['thing'],
  hasPointInTime: '2019-11-19T04:29:55.056Z',
}

runner.registerScenario('Agent relationship traversal', async (s, t) => {
  const alice = await buildPlayer(s, 'alice', config)
  const aliceAddr = alice.instance('agent').agentAddress
  const bob = await buildPlayer(s, 'bob', config)
  const bobAddr = bob.instance('agent').agentAddress

  // event which shares provider & receiver

  let res = await alice.graphQL(`
    mutation($e: EconomicEventCreateParams!) {
      e: createEconomicEvent(event: $e) {
        economicEvent {
          id
        }
      }
    }
  `, {
    e: {
      action: 'raise',
      provider: aliceAddr,
      receiver: aliceAddr,
      resourceQuantity: { hasNumericalValue: 3 },
      ...testEventProps,
    },
  })
  await s.consistency()
  t.ok(res.data.e.economicEvent.id, 'inventory logged')
  const event1Id = res.data.e.economicEvent.id

  res = await alice.graphQL(`
    query {
      economicEvent(id: "${event1Id}") {
        provider {
          id
          name
        }
        receiver {
          id
          name
        }
      }
    }
  `)

  t.equal(res.data.economicEvent.provider.id, aliceAddr, 'provider record retrieved')
  t.equal(res.data.economicEvent.receiver.id, aliceAddr, 'receiver record retrieved')

  // :TODO: reverse query edges: myAgent.[economicEvents / inventoriedEconomicResources / processes]

  // transfer event

  res = await alice.graphQL(`
    mutation($e: EconomicEventCreateParams!) {
      e: createEconomicEvent(event: $e) {
        economicEvent {
          id
        }
      }
    }
  `, {
    e: {
      action: 'transfer',
      provider: aliceAddr,
      receiver: bobAddr,
      resourceQuantity: { hasNumericalValue: 1 },
      ...testEventProps,
    },
  })
  await s.consistency()
  t.ok(res.data.e.economicEvent.id, 'transfer logged')
  const event2Id = res.data.e.economicEvent.id

  res = await alice.graphQL(`
    query {
      economicEvent(id: "${event2Id}") {
        provider {
          id
          name
        }
        receiver {
          id
          name
        }
      }
    }
  `)

  t.equal(res.data.economicEvent.provider.id, aliceAddr, 'provider record retrieved')
  t.equal(res.data.economicEvent.receiver.id, bobAddr, 'receiver record retrieved')

  // commitment & intent

  res = await alice.graphQL(`
    mutation($c: CommitmentCreateParams!, $i: IntentCreateParams!) {
      c: createCommitment(commitment: $c) {
        commitment {
          id
        }
      }
      i: createIntent(intent: $i) {
        intent {
          id
        }
      }
    }
  `, {
    c: {
      action: 'transfer',
      provider: aliceAddr,
      receiver: bobAddr,
      resourceQuantity: { hasNumericalValue: 1 },
      ...testEventProps,
    },
    i: {
      action: 'transfer',
      provider: bobAddr,
      receiver: aliceAddr,
      resourceQuantity: { hasNumericalValue: 1 },
      ...testEventProps,
    },
  })
  await s.consistency()
  t.ok(res.data.c.commitment.id, 'commitment logged')
  t.ok(res.data.i.intent.id, 'intent logged')
  const commitmentId = res.data.c.commitment.id
  const intentId = res.data.i.intent.id

  res = await alice.graphQL(`
    query {
      commitment(id: "${commitmentId}") {
        provider {
          id
          name
        }
        receiver {
          id
          name
        }
      }
      intent(id: "${intentId}") {
        provider {
          id
          name
        }
        receiver {
          id
          name
        }
      }
    }
  `)

  t.equal(res.data.commitment.provider.id, aliceAddr, 'commitment provider record retrieved')
  t.equal(res.data.commitment.receiver.id, bobAddr, 'commitment receiver record retrieved')
  t.equal(res.data.intent.provider.id, bobAddr, 'intent provider record retrieved')
  t.equal(res.data.intent.receiver.id, aliceAddr, 'intent receiver record retrieved')

  // :TODO: reverse query edges: myAgent.[commitments / intents]
})

runner.run()
