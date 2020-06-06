/**
 * Resolvers for ResourceSpecification fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Maybe,
  EconomicResource,
  ResourceSpecification,
  Unit,
} from '@valueflows/vf-graphql'

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const queryResources = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_resource', 'query_resources')
  const readUnit = mapZomeFn(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')

  return {
    conformingResources: async (record: ResourceSpecification): Promise<EconomicResource[]> => {
      return (await queryResources({ params: { conformsTo: record.id } })).map(({ economicResource }) => economicResource )
    },

    defaultUnitOfEffort: async (record: ResourceSpecification): Promise<Maybe<Unit>> => {
      if (!record.defaultUnitOfEffort) {
        return null
      }
      return (await readUnit({ id: record.defaultUnitOfEffort })).unit
    },
  }
}
