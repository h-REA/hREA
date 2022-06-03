/**
 * Top-level Intent queries
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, ReadParams } from '../types'
import { mapZomeFn } from '../connection'

import {
  Intent, IntentResponse,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readRecord = mapZomeFn<ReadParams, IntentResponse>(dnaConfig, conductorUri, 'planning', 'intent', 'get_intent')

  return {
    intent: async (root, args): Promise<Intent> => {
      return (await (await readRecord)(args)).intent
    },
  }
}
