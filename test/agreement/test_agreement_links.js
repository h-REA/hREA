import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockIdentifier,
  mockAddress,
} from '../init.js'

const testEventProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
  provider: mockAddress(),
  receiver: mockAddress(),
}

test('Agreement links & queries', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['observation', 'planning', 'agreement'])

  try {
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
    await pause(100)
    t.ok(resp.data.res.agreement.id, 'agreement created')
    const aId = resp.data.res.agreement.id

    resp = await alice.graphQL(`
      mutation($e: EconomicEventCreateParams!, $c: CommitmentCreateParams!) {
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
    await pause(100)
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
      mutation($e: EconomicEventCreateParams!, $c: CommitmentCreateParams!) {
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
    await pause(100)
    t.ok(resp.data.event.economicEvent.id, 'event 2 created')
    t.ok(resp.data.commitment.commitment.id, 'commitment 2 created')
    const e2Id = resp.data.event.economicEvent.id
    const c2Id = resp.data.commitment.commitment.id

    resp = await alice.graphQL(`
      mutation($c2: CommitmentCreateParams!, $e2: EconomicEventCreateParams!) {
        dangleCommitment: createCommitment(commitment: $c2) {
          commitment {
            id
          }
        }
        dangleEvent: createEconomicEvent(event: $e2) {
          economicEvent {
            id
          }
        }
      }
    `, {
      c2: {
        note: 'commitment without associated Agreement to test empty refs',
        due: new Date(Date.now() + 86400000),
        ...testEventProps,
      },
      e2: {
        note: 'event without associated Agreement to test empty refs',
        hasPointInTime: new Date(),
        ...testEventProps,
      },
    })
    await pause(100)

    resp = await alice.graphQL(`
      query {
        # test nested inverse query resolvers
        agreement(id: "${aId}") {
          commitments {
            id
          }
          economicEvents {
            id
          }
        }

        # test resolver logic for nullable relationships
        commitments {
          clauseOf {
            id
          }
        }
        economicEvents {
          realizationOf {
            id
          }
        }
      }
    `)

    const sortedCIds = [{ id: c2Id }, { id: cId }]
    const sortedEIds = [{ id: e2Id }, { id: eId }]

    t.equal(resp.data.agreement.commitments.length, 2, '2nd commitment ref added')
    t.equal(resp.data.agreement.commitments[0].id, sortedCIds[0].id, 'commitment ref 1 OK')
    t.equal(resp.data.agreement.commitments[1].id, sortedCIds[1].id, 'commitment ref 2 OK')
    t.equal(resp.data.agreement.economicEvents.length, 2, '2nd event ref added')
    t.equal(resp.data.agreement.economicEvents[0].id, sortedEIds[0].id, 'event ref 1 OK')
    t.equal(resp.data.agreement.economicEvents[1].id, sortedEIds[1].id, 'event ref 2 OK')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
