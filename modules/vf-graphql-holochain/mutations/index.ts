/**
 * Mutations manifest for HoloREA
 *
 * @package: HoloREA
 * @since:   2019-05-22
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'

import ResourceSpecification from './resourceSpecification'
import ProcessSpecification from './processSpecification'
import Unit from './unit'

import Process from './process'
import EconomicResource from './economicResource'
import EconomicEvent from './economicEvent'

import Commitment from './commitment'
import Fulfillment from './fulfillment'

import Intent from './intent'
import Satisfaction from './satisfaction'

import Proposal from './proposal'
import ProposedTo from './proposedTo'
import ProposedIntent from './proposedIntent'

import Agreement from './agreement'
import Plan from './plan'

// generic deletion calling format used by all mutations
export type deleteHandler = (root: any, args: { revisionId: string }) => Promise<boolean>

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasMeasurement = -1 !== enabledVFModules.indexOf(VfModule.Measurement)
  const hasKnowledge = -1 !== enabledVFModules.indexOf(VfModule.Knowledge)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasPlanning = -1 !== enabledVFModules.indexOf(VfModule.Planning)
  const hasProposal = -1 !== enabledVFModules.indexOf(VfModule.Proposal)
  const hasAgreement = -1 !== enabledVFModules.indexOf(VfModule.Agreement)
  const hasPlan = -1 !== enabledVFModules.indexOf(VfModule.Plan)

  return Object.assign(
    (hasMeasurement ? { ...Unit(dnaConfig, conductorUri) } : {}),
    (hasKnowledge ? {
      ...ResourceSpecification(dnaConfig, conductorUri),
      ...ProcessSpecification(dnaConfig, conductorUri),
    } : {}),
    (hasObservation ? {
      ...Process(dnaConfig, conductorUri),
      ...EconomicResource(dnaConfig, conductorUri),
      ...EconomicEvent(dnaConfig, conductorUri),
    } : {}),
    (hasPlanning ? {
      ...Commitment(dnaConfig, conductorUri),
      ...Fulfillment(dnaConfig, conductorUri),
      ...Intent(dnaConfig, conductorUri),
      ...Satisfaction(dnaConfig, conductorUri),
    } : {}),
    (hasProposal ? {
      ...Proposal(dnaConfig, conductorUri),
      ...ProposedTo(dnaConfig, conductorUri),
      ...ProposedIntent(dnaConfig, conductorUri),
    } : {}),
    (hasAgreement ? { ...Agreement(dnaConfig, conductorUri) } : {}),
    (hasPlan ? { ...Plan(dnaConfig, conductorUri) } : {}),
  )
}
