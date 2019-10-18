/**
 * Top-level queries relating to Economic Events
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { zomeFunction } from '../connection'

import {
  EconomicEvent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readEvent = zomeFunction('observation', 'economic_event', 'get_event')

// Read a single event by ID
export const economicEvent = async (root, args): Promise<EconomicEvent> => {
  const { id } = args
  return (await (await readEvent)({ address: id })).economicEvent
}
