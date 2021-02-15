/**
 * Top-level queries related to Satisfactions
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Satisfaction,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn(dnaConfig, conductorUri, 'planning', 'satisfaction', 'get_satisfaction')

  return {
    satisfaction: async (root, args): Promise<Satisfaction> => {
      return (await readOne({ address: args.id })).satisfaction
    },
  }
}
