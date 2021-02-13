/**
 * Top-level Intent queries
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Intent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
export default (dnaConfig: DNAIdMappings, conductorUri?: string, traceAppSignals?: AppSignalCb) => {
  const readRecord = mapZomeFn(dnaConfig, conductorUri, 'planning', 'intent', 'get_intent')

  return {
    intent: async (root, args): Promise<Intent> => {
      return (await (await readRecord)({ address: args.id })).intent
    },
  }
}
