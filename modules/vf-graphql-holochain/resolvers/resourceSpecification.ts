/**
 * Resolvers for ResourceSpecification fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ById, AddressableIdentifier } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Maybe,
  EconomicResourceConnection,
  ResourceSpecification,
  Unit,
  UnitResponse,
} from '@valueflows/vf-graphql'
import { EconomicResourceSearchInput } from './zomeSearchInputTypes.js'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasMeasurement = -1 !== enabledVFModules.indexOf(VfModule.Measurement)
  const hasIntent = -1 !== enabledVFModules.indexOf(VfModule.Intent)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)

  const queryResources = mapZomeFn<EconomicResourceSearchInput, EconomicResourceConnection>(dnaConfig, conductorUri, 'observation', 'economic_resource_index', 'query_economic_resources')
  const readUnit = mapZomeFn<ById, UnitResponse>(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')

  return Object.assign(
    (hasObservation ? {
      conformingResources: async (record: ResourceSpecification): Promise<EconomicResourceConnection> => {
        return await queryResources({ params: { conformsTo: record.id } })
      },
      economicEvents: () => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasCommitment ? {
      commitments: () => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasIntent ? {
      intents: () => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasMeasurement ? {
      defaultUnitOfResource: async (record: { defaultUnitOfResource: AddressableIdentifier }) => {
        if (!record.defaultUnitOfResource) {
          return null
        }
        return (await readUnit({ id: record.defaultUnitOfResource })).unit
      },
      defaultUnitOfEffort: async (record: { defaultUnitOfEffort: AddressableIdentifier }): Promise<Maybe<Unit>> => {
        if (!record.defaultUnitOfEffort) {
          return null
        }
        return (await readUnit({ id: record.defaultUnitOfEffort })).unit
      },
    } : {}),
  )
}
