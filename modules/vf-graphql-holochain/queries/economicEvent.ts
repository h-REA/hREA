/**
 * Top-level queries relating to Economic Events
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { zomeFunction } from '../connection'
import { injectTypename, addTypename } from '../types'

import {
  EconomicEvent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readOne = zomeFunction('observation', 'economic_event', 'get_event')
const readAll = zomeFunction('observation', 'economic_event', 'get_all_events')

const withTypename = addTypename('EconomicEvent')

// Read a single event by ID
export const economicEvent = injectTypename('EconomicEvent', async (root, args): Promise<EconomicEvent> => {
  return (await readOne({ address: args.id })).economicEvent
})

export const allEconomicEvents = async (root, args): Promise<EconomicEvent[]> => {
  return (await readAll({})).map(e => withTypename(e.economicEvent))
}
