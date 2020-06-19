const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
  bridge,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  observation: getDNA('observation'),
  planning: getDNA('planning'),
  agreement: getDNA('agreement'),
}, [
  bridge('vf_agreement', 'planning', 'agreement'),
  bridge('vf_agreement', 'observation', 'agreement'),
])

const testEventProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: 'dangling-unit-todo-tidy-up' },
  provider: 'agentid-1-todo',
  receiver: 'agentid-2-todo',
}

runner.registerScenario('Agreement links & queries', async (s, t) => {
  const alice = await buildPlayer(s, 'alice', config)

  let resp = await alice.graphQL(`
    mutation($rs: AgreementCreateParams!) {
      res: createAgreement(agreement: $rs) {
        agreement {
          id
        }
      }
    }
  `, {
    rs: {
      name: 'test agreement',
      created: new Date(),
      note: 'just testing, nothing was rly agreed',
    },
  })
  await s.consistency()
  t.ok(resp.data.res.agreement.id, 'agreement created')
  const aId = resp.data.res.agreement.id

  resp = await alice.graphQL(`
    mutation($e: EventCreateParams!, $c: CommitmentCreateParams!) {
      event: createEconomicEvent(event: $e) {
        economicEvent {
          id
        }
      }
      commitment: createCommitment(commitment: $c) {
        commitment {
          id
        }
      }
    }
  `, {
    e: {
      realizationOf: aId,
      note: 'linked event 1',
      hasPointInTime: new Date(),
      ...testEventProps,
    },
    c: {
      clauseOf: aId,
      note: 'linked commitment 1',
      due: new Date(Date.now() + 86400000),
      ...testEventProps,
    },
  })
  await s.consistency()
  t.ok(resp.data.event.economicEvent.id, 'event created')
  t.ok(resp.data.commitment.commitment.id, 'commitment created')
  const eId = resp.data.event.economicEvent.id
  const cId = resp.data.commitment.commitment.id

  resp = await alice.graphQL(`
    query {
      economicEvent(id: "${eId}") {
        realizationOf {
          id
        }
      }
      commitment(id: "${cId}") {
        clauseOf {
          id
        }
      }
      agreement(id: "${aId}") {
        commitments {
          id
        }
        economicEvents {
          id
        }
      }
    }
  `)
  t.equal(resp.data.economicEvent.realizationOf.id, aId, 'event -> agreement ref OK')
  t.equal(resp.data.commitment.clauseOf.id, aId, 'commitment -> agreement ref OK')
  t.equal(resp.data.agreement.commitments.length, 1, 'commitment ref added')
  t.equal(resp.data.agreement.commitments[0].id, cId, 'commitment ref OK')
  t.equal(resp.data.agreement.economicEvents.length, 1, 'event ref added')
  t.equal(resp.data.agreement.economicEvents[0].id, eId, 'event ref OK')

  resp = await alice.graphQL(`
    mutation($e: EventCreateParams!, $c: CommitmentCreateParams!) {
      event: createEconomicEvent(event: $e) {
        economicEvent {
          id
        }
      }
      commitment: createCommitment(commitment: $c) {
        commitment {
          id
        }
      }
    }
  `, {
    e: {
      realizationOf: aId,
      note: 'linked event 2',
      hasPointInTime: new Date(),
      ...testEventProps,
    },
    c: {
      clauseOf: aId,
      note: 'linked commitment 2',
      due: new Date(Date.now() + 86400000),
      ...testEventProps,
    },
  })
  await s.consistency()
  t.ok(resp.data.event.economicEvent.id, 'event 2 created')
  t.ok(resp.data.commitment.commitment.id, 'commitment 2 created')
  const e2Id = resp.data.event.economicEvent.id
  const c2Id = resp.data.commitment.commitment.id

  resp = await alice.graphQL(`
    query {
      agreement(id: "${aId}") {
        commitments {
          id
        }
        economicEvents {
          id
        }
      }
    }
  `)
  t.equal(resp.data.agreement.commitments.length, 2, '2nd commitment ref added')
  t.equal(resp.data.agreement.commitments[0].id, c2Id, 'commitment ref 1 OK')
  t.equal(resp.data.agreement.commitments[0].id, cId, 'commitment ref 2 OK')
  t.equal(resp.data.agreement.economicEvents.length, 2, '2nd event ref added')
  t.equal(resp.data.agreement.economicEvents[0].id, e2Id, 'event ref 1 OK')
  t.equal(resp.data.agreement.economicEvents[0].id, eId, 'event ref 2 OK')
})

runner.run()
