/**
 * Resolvers for EconomicEvent fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'
import { extractEdges, mapZomeFn } from '../connection'

import {
  Process,
  PlanProcessFilterParams
} from '@valueflows/vf-graphql'


export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {

  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasKnowledge = -1 !== enabledVFModules.indexOf(VfModule.Knowledge)
  const hasPlanning = -1 !== enabledVFModules.indexOf(VfModule.Planning)
  const hasAgreement = -1 !== enabledVFModules.indexOf(VfModule.Agreement)

  const readProcesses = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')

  return Object.assign(
    (hasPlanning ? {
      processes: async (filter: PlanProcessFilterParams): Promise<Process[]> => {
        const results = await readProcesses(filter)
        return extractEdges(results)
      },
    } : {}),
  )
}

