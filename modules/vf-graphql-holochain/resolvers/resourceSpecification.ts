/**
 * Resolvers for ResourceSpecification fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES } from '../types'
import { mapZomeFn } from '../connection'

import {
  Maybe,
  EconomicResourceConnection,
  ResourceSpecification,
  Unit,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: string[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasMeasurement = -1 !== enabledVFModules.indexOf("measurement")
  const hasObservation = -1 !== enabledVFModules.indexOf("observation")

  const queryResources = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_resource_index', 'query_economic_resources')
  const readUnit = mapZomeFn(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')

  return Object.assign(
    (hasObservation ? {
      conformingResources: async (record: ResourceSpecification): Promise<EconomicResourceConnection> => {
        return await queryResources({ params: { conformsTo: record.id } })
      },
    } : {}),
    (hasMeasurement ? {
      defaultUnitOfEffort: async (record: ResourceSpecification): Promise<Maybe<Unit>> => {
        if (!record.defaultUnitOfEffort) {
          return null
        }
        return (await readUnit({ id: record.defaultUnitOfEffort })).unit
      },
    } : {}),
  )
}
