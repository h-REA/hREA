import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
  mockAddress,
} from '../init.js'
import { decodeHashFromBase64, encodeHashToBase64 } from '@holochain/client'

function translateId(id) {
  return id.replace(/-/g, "+").replace(/_/g, "/");
}

test('create simplest event', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['combined'])
  try {
    const { cells: [combined] } = alice

    const processSpecification = {
      name: 'some-process',
      note: 'test process',
    }

    const processSpecificationResponse = await combined.call('process_specification', 'create_process_specification', { processSpecification })
    await pause(100)
    t.ok(processSpecificationResponse.processSpecification, 'process created')

    const process = {
      name: 'some-process',
      note: 'test process',
      basedOn: processSpecificationResponse.processSpecification.id,
    }

    const processResponse = await combined.call('process', 'create_process', { process })
    await pause(100)
    t.ok(processResponse.process, 'process created')

    const processId = processResponse.process.id

    const event = {
      note: 'test event',
      action: 'pickup',
      provider: mockAddress(false),
      receiver: mockAddress(false),
      hasPointInTime: '2019-11-19T12:12:42.739+01:00',
      resourceClassifiedAs: ['some-resource-type'],
      resourceQuantity: { hasNumericalValue: 1 },
      inScopeOf: ['some-accounting-scope'],
      inputOf: processId,
      outputOf: processId,
    }

    const newInventoriedResource = {
        note: 'test resource',
        trackingIdentifier: 'some-tracking-identifier',
        currentQuantity: { hasNumericalValue: 1 },
        resourceClassifiedAs: ['some-resource-type'],
        inScopeOf: ['some-accounting-scope'],
    }

    const createEventResponse = await combined.call('economic_event', 'create_economic_event', { event, newInventoriedResource })
    await pause(100)
    t.ok(createEventResponse.economicEvent, 'event created')

    const erid = translateId(`${encodeHashToBase64(createEventResponse.economicEvent.resourceInventoriedAs[1])}:${encodeHashToBase64(createEventResponse.economicEvent.resourceInventoriedAs[0])}`);

    const fetchEconomicResourcesResponse = await combined.call('economic_resource', 'get_economic_resource', { address: createEventResponse.economicEvent.resourceInventoriedAs })
    t.ok(fetchEconomicResourcesResponse.economicResource, 'resource created')

    t.ok(createEventResponse.economicResource.stage, 'resource has a stage')

    t.deepLooseEqual(createEventResponse.economicEvent.inScopeOf, ['some-accounting-scope'], 'event inScopeOf saved')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
