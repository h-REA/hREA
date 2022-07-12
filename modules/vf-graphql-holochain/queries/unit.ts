/**
 * Top-level queries relating to Unit
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Unit, UnitConnection, UnitResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, UnitResponse>(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')
  const readAll = mapZomeFn<PagingParams, UnitConnection>(dnaConfig, conductorUri, 'specification', 'unit_index', 'read_all_units')

  return {
    unit: async (root, args): Promise<Unit> => {
      return (await readOne(args)).unit
    },
    units: async (root, args: PagingParams): Promise<UnitConnection> => {
      return await readAll(args)
    },
  }
}
