/**
 * Top-level queries relating to Unit
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Unit,
} from '@valueflows/vf-graphql'

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const readOne = mapZomeFn(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')

  return {
    unit: async (root, args): Promise<Unit> => {
      return (await readOne(args)).unit
    },
  }
}
