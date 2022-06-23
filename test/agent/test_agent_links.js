import test from 'tape'
import { pause } from '@connoropolous/tryorama'
import {
  buildPlayer,
  mockIdentifier,
  mockAddress,
  sortById,
} from '../init.js'

const testEventProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
}
const examplePerson = {
  name: 'Alice',
  image: 'https://image.png',
  note: 'Alice is the first person',
}
const examplePerson2 = {
  name: 'Bob',
  image: 'https://bob.png',
  note: 'Bob is the second person',
}

test('Agent links & queries', async (t) => {
  const alice = await buildPlayer(['planning', 'agent'])

  let resp = await alice.graphQL(`
    mutation($rs: AgentCreateParams!, $rs2: AgentCreateParams!) {
      res: createPerson(person: $rs) {
        agent {
          id
        }
      }
      res2: createPerson(person: $rs2) {
        agent {
          id
        }
      }
    }
  `, {
    rs: examplePerson,
    rs2: examplePerson2,
  })
  await pause(100)
  t.ok(resp.data.res.agent.id, 'Alice created')
  t.ok(resp.data.res2.agent.id, 'Bob created')
  const aliceId = resp.data.res.agent.id
  const bobId = resp.data.res2.agent.id

  console.log('alice id: ', aliceId)

  resp = await alice.graphQL(`
    mutation($c: CommitmentCreateParams!) {
      commitment: createCommitment(commitment: $c) {
        commitment {
          id
        }
      }
    }
  `, {
    c: {
      provider: aliceId,
      receiver: bobId,
      note: 'linked commitment 1',
      due: new Date(Date.now() + 86400000),
      ...testEventProps,
    },
  })
  await pause(100)
  console.log('created commitment: ', resp.data)
  t.ok(resp.data.commitment.commitment.id, 'commitment created')
  const cId = resp.data.commitment.commitment.id

  resp = await alice.graphQL(`
    query {
      commitment: commitment(id: "${cId}") {
        provider {
          id
        }
        receiver {
          id
        }
      }
      aliceQuery: person(id: "${aliceId}") {
        id
        name
        commitmentsAsProvider(first: 1, after: "string", last: 2, before: "string") {
          edges {
            node {
              id
            }
          }
        }
      }
      bobQuery: person(id: "${bobId}") {
        commitmentsAsReceiver {
          edges {
            node {
              id
            }
          }
        }
      }
    }
  `)
  console.log('query response: ', resp)
  t.equal(resp.data.commitment.provider.id, aliceId, 'commitment -> agent provider ref OK')
  t.equal(resp.data.commitment.receiver.id, bobId, 'commitment -> agent receiver ref OK')
  t.equal(resp.data.aliceQuery.commitmentsAsProvider.length, 1, 'commitment ref for provider added')
  t.equal(resp.data.bobQuery.commitmentsAsReceiver.length, 1, 'commitment ref for receiver added')
  t.equal(resp.data.aliceQuery.commitmentsAsProvider[0].id, cId, 'commitment ref for provider OK')
  t.equal(resp.data.bobQuery.commitmentsAsReceiver[0].id, cId, 'commitment ref for receiver OK')

  // resp = await alice.graphQL(`
  //   mutation($e: EconomicEventCreateParams!, $c: CommitmentCreateParams!) {
  //     event: createEconomicEvent(event: $e) {
  //       economicEvent {
  //         id
  //       }
  //     }
  //     commitment: createCommitment(commitment: $c) {
  //       commitment {
  //         id
  //       }
  //     }
  //   }
  // `, {
  //   e: {
  //     realizationOf: aId,
  //     note: 'linked event 2',
  //     hasPointInTime: new Date(),
  //     ...testEventProps,
  //   },
  //   c: {
  //     clauseOf: aId,
  //     note: 'linked commitment 2',
  //     due: new Date(Date.now() + 86400000),
  //     ...testEventProps,
  //   },
  // })
  // await pause(100)
  // t.ok(resp.data.event.economicEvent.id, 'event 2 created')
  // t.ok(resp.data.commitment.commitment.id, 'commitment 2 created')
  // const e2Id = resp.data.event.economicEvent.id
  // const c2Id = resp.data.commitment.commitment.id

  // resp = await alice.graphQL(`
  //   query {
  //     agent(id: "${aId}") {
  //       commitments {
  //         id
  //       }
  //       economicEvents {
  //         id
  //       }
  //     }
  //   }
  // `)

  // // :TODO: remove client-side sorting when deterministic time-ordered indexing is implemented
  // const sortedCIds = [{ id: cId }, { id: c2Id }].sort(sortById)
  // resp.data.agent.commitments.sort(sortById)
  // const sortedEIds = [{ id: eId }, { id: e2Id }].sort(sortById)
  // resp.data.agent.economicEvents.sort(sortById)

  // t.equal(resp.data.agent.commitments.length, 2, '2nd commitment ref added')
  // t.equal(resp.data.agent.commitments[0].id, sortedCIds[0].id, 'commitment ref 1 OK')
  // t.equal(resp.data.agent.commitments[1].id, sortedCIds[1].id, 'commitment ref 2 OK')
  // t.equal(resp.data.agent.economicEvents.length, 2, '2nd event ref added')
  // t.equal(resp.data.agent.economicEvents[0].id, sortedEIds[0].id, 'event ref 1 OK')
  // t.equal(resp.data.agent.economicEvents[1].id, sortedEIds[1].id, 'event ref 2 OK')

  await alice.scenario.cleanUp()
})
