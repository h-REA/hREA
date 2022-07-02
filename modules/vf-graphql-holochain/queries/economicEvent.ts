/**
 * Top-level queries relating to Economic Events
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { DNAIdMappings, injectTypename, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  EconomicEvent,
  EconomicEventConnection,
  EconomicEventResponse,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, EconomicEventResponse>(dnaConfig, conductorUri, 'observation', 'economic_event', 'get_economic_event')
  const readAll = mapZomeFn<null, EconomicEventConnection>(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'read_all_economic_events')

  return {
    economicEvent: injectTypename('EconomicEvent', async (root, args): Promise<EconomicEvent> => {
      return (await readOne({ address: args.id })).economicEvent
    }),

    economicEvents: async (root, args): Promise<EconomicEventConnection> => {
      return await readAll(args)
    },
  }
}
