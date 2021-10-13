/**
 * Top-level queries relating to Economic Events
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { DNAIdMappings, injectTypename, addTypename } from '../types'
import { mapZomeFn } from '../connection'

import {
  EconomicEvent,
} from '@valueflows/vf-graphql'

const withTypename = addTypename('EconomicEvent')

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event', 'get_economic_event')
  const readAll = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event', 'get_all_economic_events')

  return {
    economicEvent: injectTypename('EconomicEvent', async (root, args): Promise<EconomicEvent> => {
      return (await readOne({ address: args.id })).economicEvent
    }),

    economicEvents: async (root, args): Promise<EconomicEvent[]> => {
      return (await readAll(null)).map(e => withTypename(e.economicEvent))
    },
  }
}
