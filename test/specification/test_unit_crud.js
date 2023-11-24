import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
} from '../init.js'

const exampleEntry = {
  label: 'kilgrams',
  symbol: 'kig',
}
const exampleEntry2 = {
  label: 'metre',
  symbol: 'm',
}
const updatedExampleEntry = {
  label: 'kilograms',
  symbol: 'kg',
}

test('Unit record API', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['specification'])
  try {
    let createResp = await alice.graphQL(`
      mutation($rs: UnitCreateParams!, $rs2: UnitCreateParams!) {
        res: createUnit(unit: $rs) {
          unit {
            id
            revisionId
          }
        }
        res2: createUnit(unit: $rs2) {
          unit {
            id
            revisionId
          }
        }
      }
      `, {
      rs: exampleEntry,
      rs2: exampleEntry2,
    })
    await pause(100)

    t.ok(createResp.data.res.unit.id, 'record created')
    t.equal(createResp.data.res.unit.id.split(':')[0], exampleEntry.symbol, 'record index set')
    let uId = createResp.data.res.unit.id
    let u2Id = createResp.data.res2.unit.id
    let uRevision = createResp.data.res.unit.revisionId
    const getResp = await alice.graphQL(`
      query($id: ID!) {
        res: unit(id: $id) {
          id
          revisionId
          label
          symbol
        }
      }
      `, {
      id: uId,
    })

    t.deepLooseEqual(getResp.data.res, { 'id': uId, revisionId: uRevision, ...exampleEntry }, 'record read OK')

    const ALL_UNITS_QUERY = `
      query {
        res: units {
          edges {
            node {
              id
            }
          }
        }
      }
    `
    const queryAllUnits = await alice.graphQL(ALL_UNITS_QUERY)

    t.equal(queryAllUnits.data.res.edges.length, 2, 'query for all units OK')
    t.deepEqual(queryAllUnits.data.res.edges[1].node.id, uId, 'query for all units, first unit in order OK')
    t.deepEqual(queryAllUnits.data.res.edges[0].node.id, u2Id, 'query for all units, second unit in order OK')
    const updateResp = await alice.graphQL(`
      mutation($rs: UnitUpdateParams!) {
        res: updateUnit(unit: $rs) {
          unit {
            id
            revisionId
          }
        }
      }
      `, {
      rs: { revisionId: uRevision, ...updatedExampleEntry },
    })
    const updatedUnitRevId = updateResp.data.res.unit.revisionId
    await pause(100)

    t.notEqual(updateResp.data.res.unit.id, uId, 'update operation succeeded')
    t.equal(updateResp.data.res.unit.id.split(':')[0], updatedExampleEntry.symbol, 'record index updated')
    uId = updateResp.data.res.unit.id

    // now we fetch the Entry again to check that the update was successful
    const updatedGetResp = await alice.graphQL(`
      query($id: ID!) {
        res: unit(id: $id) {
          id
          revisionId
          label
          symbol
        }
      }
    `, {
      id: uId,
    })

    t.deepLooseEqual(updatedGetResp.data.res, { id: uId, revisionId: updatedUnitRevId, ...updatedExampleEntry }, 'record updated OK')

    const deleteResult = await alice.graphQL(`
      mutation($revisionId: ID!) {
        res: deleteUnit(revisionId: $revisionId)
      }
    `, {
      revisionId: updatedUnitRevId,
    })
    await pause(100)

    t.equal(deleteResult.data.res, true)

    const queryForDeleted = await alice.graphQL(`
      query($id: ID!) {
        res: unit(id: $id) {
          id
          label
          symbol
        }
      }
    `, {
      id: uId,
    })

    t.equal(queryForDeleted.errors.length, 1, 'querying deleted record is an error')
    t.notEqual(-1, queryForDeleted.errors[0].message.indexOf('No entry at this address'), 'correct error reported')

    createResp = await alice.graphQL(`
      mutation($rs: UnitCreateParams!) {
        res: createUnit(unit: $rs) {
          unit {
            id
            revisionId
          }
        }
      }
      `, {
      rs: exampleEntry2,
    })
    await pause(100)
    t.ok(createResp.data.res.unit.id, 'record (re)created; this should be a no-op')

    const queryAllUnits2 = await alice.graphQL(ALL_UNITS_QUERY)

    t.equal(queryAllUnits2.data.res.edges.length, 1, 'duplicate unit records are not created under ideal network conditions')

    // :TODO:
    //  1. Add test to verify that duplicate unit records are *not* de-duplicated on reading if created on both sides of a network partition.
    //  2. Fix this bug (perhaps using novel Holochain features like 'bucketing') such that repeat duplicate writes of `Unit` data do not bloat DHT storage with `Action` headers.
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
