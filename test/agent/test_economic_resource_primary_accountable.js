import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockIdentifier,
} from '../init.js'

const testProps = {
  action: 'raise',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
}
const testEventProps = {
  action: 'produce',
  resourceClassifiedAs: ['some-resource-type'],
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
}
const testResource = {
  name: 'test resource',
}
// this resource is different than the first
// to prevent identical identity hashes of two resources
const testResource2 = {
  name: 'test resource2',
}
const testProcess = {
  name: 'test process',
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
  const alice = await buildPlayer(['observation', 'agent'])
  try {
    let resp = await alice.graphQL(`
      mutation($rs: AgentCreateParams!, $rs2: AgentCreateParams!, $p: ProcessCreateParams!) {
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
        process: createProcess(process: $p) {
          process {
            id
          }
        }
      }
    `, {
      rs: examplePerson,
      rs2: examplePerson2,
      p: testProcess,
    })
    await pause(100)
    t.ok(resp.data.res.agent.id, 'Alice created')
    t.ok(resp.data.res2.agent.id, 'Bob created')
    t.ok(resp.data.process.process.id, 'process created')
    const aliceId = resp.data.res.agent.id
    const bobId = resp.data.res2.agent.id
    const pId = resp.data.process.process.id

    resp = await alice.graphQL(`
      mutation($e: EconomicEventCreateParams!, $e2: EconomicEventCreateParams!, $r: EconomicResourceCreateParams!, $r2: EconomicResourceCreateParams!) {
        economicEvent: createEconomicEvent(event: $e, newInventoriedResource: $r) {
          economicEvent {
            id
          }
          economicResource {
            id
            primaryAccountable {
              id
              name
            }
          }
        }
        economicEvent2: createEconomicEvent(event: $e2, newInventoriedResource: $r2) {
          economicEvent {
            id
          }
          economicResource {
            id
            primaryAccountable {
              id
              name
            }
          }
        }
      }
    `, {
      e: {
        provider: aliceId,
        receiver: bobId,
        note: 'linked economic event 1, produce action',
        hasPointInTime: new Date(Date.now() + 86400000),
        outputOf: pId,
        ...testEventProps,
      },
      r: testResource,
      e2: {
        provider: aliceId,
        receiver: bobId,
        note: 'linked economic event 2, raise action',
        hasPointInTime: new Date(Date.now() + 86400000),
        ...testProps,
      },
      r2: testResource2,
    })
    await pause(100)
    t.ok(resp.data.economicEvent.economicEvent.id, 'first economicEvent created')
    t.ok(resp.data.economicEvent.economicResource.id, 'first economicResource created')
    t.ok(resp.data.economicEvent2.economicEvent.id, 'second economicEvent created')
    t.ok(resp.data.economicEvent2.economicResource.id, 'second economicResource created')
    let resourceIds = []
    const eId = resp.data.economicEvent.economicEvent.id
    const rId = resp.data.economicEvent.economicResource.id
    resourceIds.push(rId)
    const e2Id = resp.data.economicEvent2.economicEvent.id
    const r2Id = resp.data.economicEvent2.economicResource.id
    resourceIds.push(r2Id)

    resp = await alice.graphQL(`
      query {
        economicResource: economicResource(id: "${rId}") {
          primaryAccountable {
            id
            name
          }
        }
        economicResource2: economicResource(id: "${r2Id}") {
          primaryAccountable {
            id
            name
          }
        }
        bobQuery: person(id: "${bobId}") {
          inventoriedEconomicResources {
            edges {
              node {
                id
              }
            }
          }
        }
      }
    `)
    t.equal(resp.data.economicResource.primaryAccountable.id, bobId, 'economicResource 1 -> agent ref OK')
    t.equal(resp.data.economicResource2.primaryAccountable.id, bobId, 'economicResource 2 -> agent ref OK')
    t.equal(resp.data.bobQuery.inventoriedEconomicResources.edges.length, 2, 'economicResources ref for bob added')
    t.ok(resourceIds.includes(resp.data.bobQuery.inventoriedEconomicResources.edges[0].node.id), 'first resource ref for bob OK')
    t.ok(resourceIds.includes(resp.data.bobQuery.inventoriedEconomicResources.edges[1].node.id), 'second resource ref for bob OK')

    resp = await alice.graphQL(`
      mutation($e: EconomicEventCreateParams!) {
        economicEvent: createEconomicEvent(event: $e) {
          economicEvent {
            id
          }
        }
      }
    `, {
      e: {
        provider: bobId,
        receiver: aliceId,
        note: 'economic event trying to transfer from bob to alice, but fails',
        hasPointInTime: new Date(Date.now() + 86400000),
        action: 'transfer',
        resourceInventoriedAs: rId,
        toResourceInventoriedAs: rId,
        resourceClassifiedAs: ['some-resource-type'],
        resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
      },
    })

    await pause(100)

    resp = await alice.graphQL(`
      query {
        economicResource: economicResource(id: "${rId}") {
          primaryAccountable {
            id
          }
        }
        economicResource2: economicResource(id: "${r2Id}") {
          primaryAccountable {
            id
          }
        }
        bobQuery: person(id: "${bobId}") {
          inventoriedEconomicResources {
            edges {
              node {
                id
              }
            }
          }
        }
        aliceQuery: person(id: "${aliceId}") {
          inventoriedEconomicResources {
            edges {
              node {
                id
              }
            }
          }
        }
      }
    `)
    t.equal(resp.data.economicResource.primaryAccountable.id, aliceId, 'economicResource primary accountable for alice update OK')
    t.equal(resp.data.economicResource2.primaryAccountable.id, bobId, 'economicResource primary accountable for bob update OK')
    t.equal(resp.data.bobQuery.inventoriedEconomicResources.edges.length, 1, 'economicResources ref for bob added')
    t.equal(resp.data.bobQuery.inventoriedEconomicResources.edges[0].node.id, r2Id, 'economicResource ref for bob OK')
    t.equal(resp.data.aliceQuery.inventoriedEconomicResources.edges.length, 1, 'economicResources ref for alice added')
    t.equal(resp.data.aliceQuery.inventoriedEconomicResources.edges[0].node.id, rId, 'economicResource ref for alice OK')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
