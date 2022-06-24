import test from 'tape'
import { pause } from '@connoropolous/tryorama'
import {
  buildPlayer,
  mockIdentifier,
  mockAddress,
  sortById,
} from '../init.js'

const testCommitmentProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
}
const testIntentProps = {
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

  resp = await alice.graphQL(`
    mutation($c: CommitmentCreateParams!, $i: IntentCreateParams!) {
      commitment: createCommitment(commitment: $c) {
        commitment {
          id
        }
      }
      intent: createIntent(intent: $i) {
        intent {
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
      ...testCommitmentProps,
    },
    i: {
      provider: aliceId,
      receiver: bobId,
      note: 'linked intent 1',
      due: new Date(Date.now() + 86400000),
      ...testIntentProps,
    },
  })
  await pause(100)
  t.ok(resp.data.commitment.commitment.id, 'commitment created')
  const cId = resp.data.commitment.commitment.id
  const iId = resp.data.intent.intent.id

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
      intent: intent(id: "${iId}") {
        provider {
          id
        }
        receiver {
          id
        }
      }
      aliceQuery: person(id: "${aliceId}") {
        commitmentsAsProvider {
          edges {
            node {
              id
            }
          }
        }
        intentsAsProvider {
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
        intentsAsReceiver {
          edges {
            node {
              id
            }
          }
        }
      }
    }
  `)
  t.equal(resp.data.commitment.provider.id, aliceId, 'commitment -> agent provider ref OK')
  t.equal(resp.data.commitment.receiver.id, bobId, 'commitment -> agent receiver ref OK')
  t.equal(resp.data.intent.provider.id, aliceId, 'intent -> agent provider ref OK')
  t.equal(resp.data.intent.receiver.id, bobId, 'intent -> agent receiver ref OK')
  t.equal(resp.data.aliceQuery.commitmentsAsProvider.edges.length, 1, 'commitment ref for provider added')
  t.equal(resp.data.bobQuery.commitmentsAsReceiver.edges.length, 1, 'commitment ref for receiver added')
  t.equal(resp.data.aliceQuery.commitmentsAsProvider.edges[0].node.id, cId, 'commitment ref for provider OK')
  t.equal(resp.data.bobQuery.commitmentsAsReceiver.edges[0].node.id, cId, 'commitment ref for receiver OK')
  t.equal(resp.data.aliceQuery.intentsAsProvider.edges.length, 1, 'intent ref for provider added')
  t.equal(resp.data.bobQuery.intentsAsReceiver.edges.length, 1, 'intent ref for receiver added')
  t.equal(resp.data.aliceQuery.intentsAsProvider.edges[0].node.id, iId, 'intent ref for provider OK')
  t.equal(resp.data.bobQuery.intentsAsReceiver.edges[0].node.id, iId, 'intent ref for receiver OK')

  resp = await alice.graphQL(`
    mutation($c: CommitmentCreateParams!, $i: IntentCreateParams!) {
      commitment: createCommitment(commitment: $c) {
        commitment {
          id
        }
      }
      intent: createIntent(intent: $i) {
        intent {
          id
        }
      }
    }
  `, {
    c: {
      provider: aliceId,
      receiver: bobId,
      note: 'linked commitment 2',
      due: new Date(Date.now() + 86400000),
      ...testCommitmentProps,
    },
    i: {
      provider: aliceId,
      receiver: bobId,
      note: 'linked intent 2',
      due: new Date(Date.now() + 86400000),
      ...testIntentProps,
    },
  })
  await pause(100)
  t.ok(resp.data.commitment.commitment.id, 'commitment created')

  resp = await alice.graphQL(`
    query {
      aliceQuery: person(id: "${aliceId}") {
        id
        name
        commitmentsAsProvider {
          edges {
            node {
              id
            }
          }
        }
        intentsAsProvider {
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
        intentsAsReceiver {
          edges {
            node {
              id
            }
          }
        }
      }
    }
  `)
  t.equal(resp.data.aliceQuery.commitmentsAsProvider.edges.length, 2, 'commitment ref for provider added')
  t.equal(resp.data.bobQuery.commitmentsAsReceiver.edges.length, 2, 'commitment ref for receiver added')
  t.equal(resp.data.aliceQuery.commitmentsAsProvider.edges.length, 2, 'commitment ref for provider added')
  t.equal(resp.data.bobQuery.commitmentsAsReceiver.edges.length, 2, 'commitment ref for receiver added')

  await alice.scenario.cleanUp()
})
