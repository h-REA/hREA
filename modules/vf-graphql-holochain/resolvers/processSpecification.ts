/**
 * Resolvers for ProcessSpecification fields
 *
 * @package: HoloREA
 * @since:   2022-07-11
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types.js'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasProcess = -1 !== enabledVFModules.indexOf(VfModule.Process)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)

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
  )
}
