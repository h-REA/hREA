/**
 * Top-level queries relating to Unit
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ById } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Unit, UnitConnection, UnitResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export interface BySymbol {
  symbol: string,
}

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ById, UnitResponse>(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')
  const readOneBySymbol = mapZomeFn<BySymbol, UnitResponse>(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit_by_symbol')
  const readAll = mapZomeFn<PagingParams, UnitConnection>(dnaConfig, conductorUri, 'specification', 'unit_index', 'read_all_units')

  return {
    unit: async (root, args): Promise<Unit> => {
      let res
      try {
        res = await readOne(args)
      } catch (e) {
        try {
          // attempt GUID-based read of Unit based on symbol
          res = await readOneBySymbol(args)
        } catch (_retried) {
          // throw original (UUID-based) read error if both fail
          throw e
        }
      }
      return res.unit
    },
    units: async (root, args: PagingParams): Promise<UnitConnection> => {
      return await readAll(args)
    },
  }
}
