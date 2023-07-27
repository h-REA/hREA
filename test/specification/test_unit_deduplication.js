import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockIdentifier,
  mockAddress,
} from '../init.js'

const exampleEntry = {
  label: 'metre',
  symbol: 'm',
}

test('duplicate writes are deduplicated under normal network conditions', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['specification'])

  try {
    let createResp = await alice.graphQL(
      `
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
    `,
      {
        rs: exampleEntry,
        rs2: exampleEntry,
      },
    )
    await pause(100)

    t.ok(createResp.data.res.unit.id, "Unit 1 created")
    t.ok(createResp.data.res2.unit.id, "Unit 1 re-created")
    t.equal(createResp.data.res.unit.id, createResp.data.res2.unit.id, "Unit IDs are consistent if recreated")
    t.notEqual(createResp.data.res.unit.revisionId, createResp.data.res2.unit.revisionId, "Recreating Units leads to distinct revisions")

    const queryAllUnits = await alice.graphQL(`
      query {
        res: units {
          edges {
            node {
              id
              revisionId
            }
          }
        }
      }
    `)
    console.log(queryAllUnits.data.res.edges.map(e => e.node))
    t.equal(queryAllUnits.data.res.edges.length, 1, 'query for all units OK')

  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
