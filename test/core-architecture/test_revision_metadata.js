import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
} from '../init.js'

const exampleEntry = {
  name: 'test agreement 1',
  created: new Date(),
  note: 'just testing, nothing was rly agreed',
}
const update = {
  name: 'test agreement 2',
}
const update2 = {
  name: 'test agreement 3',
}

test('record previous revision metadata', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['agreement'])
  try {
    const revision1 = await alice.graphQL(`
      mutation($record: AgreementCreateParams!) {
        res: createAgreement(agreement: $record) {
          agreement {
            id
            revisionId
          }
        }
      }
    `, { record: exampleEntry })
    await pause(100)
    t.ok(revision1.data.res.agreement.id, 'record created')
    const r1Id = revision1.data.res.agreement.revisionId

    const revision2 = await alice.graphQL(`
      mutation($record: AgreementUpdateParams!) {
        res: updateAgreement(agreement: $record) {
          agreement {
            id
            revisionId
            meta {
              previousRevision {
                id
              }
            }
          }
        }
      }
    `, { record: { revisionId: r1Id, ...update } })
    await pause(100)
    t.notEqual(revision2.data.res.agreement.revisionId, r1Id, 'record updated')
    t.equal(revision2.data.res.agreement.meta.previousRevision.id, r1Id, 'previous revision linked')
    const r2Id = revision2.data.res.agreement.revisionId

    const revision3 = await alice.graphQL(`
      mutation($record: AgreementUpdateParams!) {
        res: updateAgreement(agreement: $record) {
          agreement {
            id
            revisionId
            meta {
              previousRevision {
                id
              }
            }
          }
        }
      }
    `, { record: { revisionId: r2Id, ...update2 } })
    await pause(100)
    t.notEqual(revision3.data.res.agreement.revisionId, r2Id, 'record subsequently updated')
    t.equal(revision3.data.res.agreement.meta.previousRevision.id, r2Id, 'previous revision subsequently linked')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
