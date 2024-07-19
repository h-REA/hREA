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
} from '@leosprograms/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, UnitResponse>(dnaConfig, conductorUri, 'combined', 'unit', 'get_unit')
  const readAll = mapZomeFn<PagingParams, UnitConnection>(dnaConfig, conductorUri, 'combined', 'indexing', 'read_all_units')

  return {
    unit: async (root, args): Promise<Unit> => {
      return (await readOne(args)).unit
    },
    units: async (root, args: PagingParams): Promise<UnitConnection> => {
      console.log("Unit revision resolver 1")
      console.log("Unit revision resolver 2", args)
      console.log("Unit revision resolver 3", dnaConfig, conductorUri)
      console.log("Unit revision resolver 4", readAll(args))
      console.log("Unit revision resolver 4", await readAll(args))
      return await readAll(args)
    },
  }
}
