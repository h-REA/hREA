/**
 * Top-level Intent queries
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Intent, IntentConnection, IntentResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

// :TODO: how to inject DNA identifier?
export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readRecord = mapZomeFn<ReadParams, IntentResponse>(dnaConfig, conductorUri, 'planning', 'intent', 'get_intent')
  const readAll = mapZomeFn<PagingParams, IntentConnection>(dnaConfig, conductorUri, 'planning', 'intent_index', 'read_all_intents')

  return {
    intent: async (root, args): Promise<Intent> => {
      return (await readRecord({ address: args.id })).intent
    },
    intents: async (root, args: PagingParams): Promise<IntentConnection> => {
      return await readAll(args)
    },
  }
}
