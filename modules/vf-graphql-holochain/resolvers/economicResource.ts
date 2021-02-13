/**
 * Resolvers for EconomicResource fields
 *
 * @package: HoloREA
 * @since:   2019-10-31
 */

import { DNAIdMappings, DEFAULT_VF_MODULES } from '../types'
import { mapZomeFn } from '../connection'

import {
  EconomicResource,
  ResourceSpecification,
  Unit,
  ProcessSpecification,
  Action,
  Maybe,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: string[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri?: string, traceAppSignals?: AppSignalCb) => {
  const hasMeasurement = -1 !== enabledVFModules.indexOf("measurement")
  const hasKnowledge = -1 !== enabledVFModules.indexOf("knowledge")

  const readResources = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_resource', 'query_resources')
  const readUnit = mapZomeFn(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')
  const readProcessSpecification = mapZomeFn(dnaConfig, conductorUri, 'specification', 'process_specification', 'get_process_specification')
  const readAction = mapZomeFn(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readResourceSpecification = mapZomeFn(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')

  return Object.assign(
    {
      containedIn: async (record: EconomicResource): Promise<EconomicResource> => {
        return (await readResources({ params: { contains: record.id } })).pop()['economicResource']
      },

      contains: async (record: EconomicResource): Promise<EconomicResource[]> => {
        return (await readResources({ params: { containedIn: record.id } })).map(({ economicResource }) => economicResource)
      },
    },
    (hasKnowledge ? {
      conformsTo: async (record: EconomicResource): Promise<ResourceSpecification> => {
        return (await readResourceSpecification({ address: record.conformsTo})).resourceSpecification
      },

      stage: async (record: EconomicResource): Promise<ProcessSpecification> => {
        return (await readProcessSpecification({ address: record.stage })).processSpecification
      },

      state: async (record: EconomicResource): Promise<Action> => {
        return (await readAction({ address: record.state }))
      },
    } : {}),
    (hasMeasurement ? {
      unitOfEffort: async (record: EconomicResource): Promise<Maybe<Unit>> => {
        if (!record.unitOfEffort) {
          return null
        }
        return (await readUnit({ id: record.unitOfEffort })).unit
      },
    } : {}),
  )
}
