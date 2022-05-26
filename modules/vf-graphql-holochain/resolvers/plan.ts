/**
 * Resolvers for Plan fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'
import { extractEdges, mapZomeFn } from '../connection'

import {
  Process,
  PlanProcessFilterParams,
  Plan,
  Commitment
} from '@valueflows/vf-graphql'


export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {

  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasPlanning = -1 !== enabledVFModules.indexOf(VfModule.Planning)

  const readProcesses = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const queryCommitments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')

  return Object.assign(
    (hasObservation ? {
      processes: async (filter: PlanProcessFilterParams): Promise<Process[]> => {
        const results = await readProcesses(filter)
        return extractEdges(results)
      },
    } : {}),
    (hasPlanning ? {
      independentDemands: async (record: Plan): Promise<Commitment[]> => {
        const commitments = await queryCommitments({ params: { independentDemandOf: record.id } })
        return extractEdges(commitments)
      },
    } : {}),
  )
}

