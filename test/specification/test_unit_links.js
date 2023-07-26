import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
} from '../init.js'

const exampleResourceSpec = {
  name: 'test_resourceSpec',
  note: 'Resource specification to test references with',
}

const exampleUnit = {
  label: 'kilograms',
  symbol: 'kg',
}

test('ResourceSpecification Unit references', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['specification'])
  try {
    let resp = await alice.graphQL(`
      mutation($rs: UnitCreateParams!) {
        res: createUnit(unit: $rs) {
          unit {
            id
            revisionId
          }
        }
      }
    `, {
      rs: exampleUnit,
    })
    await pause(100)

    t.ok(resp.data.res.unit.id, 'Unit created')
    const uId = resp.data.res.unit.id

    resp = await alice.graphQL(`
      mutation(
        $rs: ResourceSpecificationCreateParams!,
      ) {
        res: createResourceSpecification(resourceSpecification: $rs) {
          resourceSpecification {
            id
            defaultUnitOfEffort {
              id
            }
            defaultUnitOfResource {
              id
            }
          }
        }
      }
    `, {
      rs: {
        ...exampleResourceSpec,
        defaultUnitOfResource: uId,
        defaultUnitOfEffort: uId,
      },
    })
    await pause(100)

    t.ok(resp.data.res.resourceSpecification.id, 'resource specification created')

    t.ok(resp.data.res.resourceSpecification.defaultUnitOfEffort.id, 'resource specification default unit of effort ok')
    t.ok(resp.data.res.resourceSpecification.defaultUnitOfResource.id, 'resource specification default unit of resource ok')
    const rsId = resp.data.res.resourceSpecification.id

    const getResp = await alice.graphQL(`
      query($id: ID!) {
        res: resourceSpecification(id: $id) {
          id
          defaultUnitOfResource {
            id
            label
            symbol
          }
          defaultUnitOfEffort {
            id
            label
            symbol
          }
        }
      }
      `, {
      id: rsId,
    })

    t.ok(getResp.data.res.id, 'resource specification retrieved')
    t.ok(getResp.data.res.defaultUnitOfResource.id, 'defaultUnitOfResource assigned')
    t.ok(getResp.data.res.defaultUnitOfEffort.id, 'defaultUnitOfEffort assigned')

    const listResp = await alice.graphQL(`
      query {
        res: resourceSpecifications(last: 100000) {
          edges {
            node {
              id
              defaultUnitOfResource {
                id
                label
                symbol
              }
              defaultUnitOfEffort {
                id
                label
                symbol
              }
            }
          }
        }
      }
    `)

    t.ok(listResp.data.res.edges[0].node.id, 'resource specification retrieved from list API')
    t.ok(listResp.data.res.edges[0].node.defaultUnitOfResource.id, 'defaultUnitOfResource OK via list API')
    t.ok(listResp.data.res.edges[0].node.defaultUnitOfEffort.id, 'defaultUnitOfEffort OK via list API')

  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
