/**
 * Top-level queries related to Satisfactions
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Satisfaction, SatisfactionResponse,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, SatisfactionResponse>(dnaConfig, conductorUri, 'planning', 'satisfaction', 'get_satisfaction')

  return {
    satisfaction: async (root, args): Promise<Satisfaction> => {
      return (await readOne({ address: args.id })).satisfaction
    },
  }
}
