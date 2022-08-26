/**
 * Test to ensure revision retrieval API functions as intended for all
 * supported record types.
 *
 * All this really ensures is that there are no schema compilation issues
 * or typos in the GraphQL resolver layer.
 */

import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
  mockIdentifier,
} from '../init.js'

// helper to check updates for each record type (all flows are the same)
async function assertRevisions(t, alice, recordName, initPayload, updatePayload, overrideUName = null, overrideParamName = null) {
  let upperRecordName = overrideUName || (recordName.substr(0, 1).toUpperCase() + recordName.substr(1))
  let paramName = overrideParamName || recordName
  let resp, recordId, firstRevisionId

  resp = await alice.graphQL(
    `mutation($r: ${upperRecordName}CreateParams!) {
      res: create${upperRecordName}(${paramName}: $r) {
        ${recordName} {
          id
          revisionId
        }
      }
    }`,
    { r: initPayload }
  )
  await pause(100)
  recordId = resp.data.res[recordName].id
  firstRevisionId = resp.data.res[recordName].revisionId

  resp = await alice.graphQL(
    `mutation($r: ${upperRecordName}UpdateParams!) {
      res: update${upperRecordName}(${paramName}: $r) {
        ${recordName} {
          id
        }
      }
    }`,
    { r: {
      ...updatePayload,
      revisionId: firstRevisionId,
    } }
  )
  await pause(100)

  resp = await alice.graphQL(
    `query($r: ID!, $rev: ID!) {
      res: ${recordName}(id: $r) {
        revision(revisionId: $rev) {
          ${Object.keys(updatePayload).join("\n")}
        }
      }
    }`,
    { r: recordId, rev: firstRevisionId },
  )
  let compareTo = filterObject(initPayload, Object.keys(updatePayload))
  t.deepLooseEqual(resp.data.res.revision, compareTo, `archived record retrievable for ${recordName}`)
}

// utility for comparison of revision results so we don't have to deal with complex nested fields in results
function filterObject(obj, fields) {
  let newObj = { ...obj }
  for (var k in newObj) {
    if (!fields.includes(k)) {
        delete newObj[k]
    }
  }
  return newObj
}

test('record revisions API resolvers', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer([
    'agent', 'agreement',
    'planning', 'observation',
    'plan', 'proposal', 'specification',
  ])
  try {
    await assertRevisions(t, alice, 'agent',
      {
        name: 'test person',
        image: 'https://image.png',
        note: 'test person note',
      },
      {
        name: "updated person",
      },
      'Organization', 'organization',
    )

    await assertRevisions(t, alice, 'agreement',
      {
        name: 'test agreement',
        created: new Date(),
        note: 'just testing, nothing was rly agreed',
      },
      {
        name: 'updated agreement',
      },
    )

    await assertRevisions(t, alice, 'commitment',
      {
        'action': 'raise',
        'provider': mockAddress(),
        'receiver': mockAddress(),
        'due': '2019-11-19T04:29:55.056Z',
        'resourceQuantity': { hasNumericalValue: 1, hasUnit: mockIdentifier() },
        'resourceClassifiedAs': ['some-resource-type'],
        'note': 'some input will be provided',
      },
      {
        note: 'updated commitment',
      },
    )

    await assertRevisions(t, alice, 'economicEvent',
      {
        action: 'raise',
        provider: mockAddress(),
        receiver: mockAddress(),
        resourceQuantity: { hasNumericalValue: 1.0, hasUnit: mockIdentifier() },
        resourceClassifiedAs: ['some-type-of-resource'],
        hasPointInTime: new Date(),
        note: 'test event',
      },
      {
        note: 'updated event note',
      },
      undefined, 'event',
    )

    await assertRevisions(t, alice, 'fulfillment',
      {
        fulfills: mockAddress(),
        fulfilledBy: mockAddress(),
        note: 'test fulfillment',
      },
      {
        note: 'updated fulfillment',
      },
    )

    await assertRevisions(t, alice, 'intent',
      {
        action: 'lower',
        note: 'test intent',
        receiver: mockAddress(),
      },
      {
        'note': 'updated intent',
      },
    )

    await assertRevisions(t, alice, 'plan',
      {
        name: 'test plan',
        created: new Date(),
        due: new Date(),
        note: 'just testing, nothing was rly planned',
      },
      {
        name: 'updated plan',
        note: 'plan was updated'
      },
    )

    await assertRevisions(t, alice, 'process',
      {
        name: 'test process2',
      },
      {
        name: 'test process',
      },
    )

    await assertRevisions(t, alice, 'processSpecification',
      {
        name: 'test process spec',
        note: 'testing',
      },
      {
        name: 'updated process spec',
      },
    )

    await assertRevisions(t, alice, 'proposal',
      {
        name: 'String',
        hasBeginning: new Date('2019-11-19T00:00:00.056Z'),
        hasEnd: new Date('2019-11-19T00:00:00.056Z'),
        unitBased: true,
        created: new Date('2019-11-19T00:00:00.056Z'),
        note: 'note',
      },
      {
        note: 'updated proposal'
      },
    )

    await assertRevisions(t, alice, 'resourceSpecification',
      {
        name: 'TRE',
        image: 'https://holochain.org/something',
        note: 'test resource specification',
      },
      {
        note: 'updated resource spec',
      },
    )

    await assertRevisions(t, alice, 'satisfaction',
      {
        satisfies: mockAddress(),
        satisfiedBy: mockAddress(),
        note: 'satisfied by an event',
      },
      {
        note: 'updated satisfaction',
      },
    )

    await assertRevisions(t, alice, 'unit',
      {
        label: 'kilgrams',
        symbol: 'kig',
      },
      {
        label: 'kilograms',
      },
    )
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
