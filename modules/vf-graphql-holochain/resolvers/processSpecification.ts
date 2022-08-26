/**
 * Resolvers for ProcessSpecification fields
 *
 * @package: HoloREA
 * @since:   2022-07-11
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ByRevision, AddressableIdentifier } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  ProcessSpecification,
  ProcessSpecificationResponse,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasProcess = -1 !== enabledVFModules.indexOf(VfModule.Process)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)

  const readRevision = mapZomeFn<ByRevision, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'process_specification', 'get_revision')

  return Object.assign(
    (hasObservation ? {
      resourcesCurrentlyAtStage: () => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasCommitment ? {
      commitmentsRequiringStage: () => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasProcess ? {
      conformingProcesses: () => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasHistory ? {
      revision: async (record: ProcessSpecification, args: { revisionId: AddressableIdentifier }): Promise<ProcessSpecification> => {
        return (await readRevision(args)).processSpecification
      },
    } : {}),
  )
}
