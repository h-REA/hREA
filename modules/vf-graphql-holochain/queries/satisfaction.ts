/**
 * Top-level queries related to Satisfactions
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Satisfaction, SatisfactionConnection, SatisfactionResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, SatisfactionResponse>(dnaConfig, conductorUri, 'planning', 'satisfaction', 'get_satisfaction')
  const readAll = mapZomeFn<PagingParams, SatisfactionConnection>(dnaConfig, conductorUri, 'planning', 'satisfaction_index', 'read_all_satisfactions')

  return {
    satisfaction: async (root, args): Promise<Satisfaction> => {
      return (await readOne({ address: args.id })).satisfaction
    },
    satisfactions: async (root, args: PagingParams): Promise<SatisfactionConnection> => {
      return await readAll(args)
    },
  }
}
