/**
 * Mutations manifest for HoloREA
 *
 * @package: HoloREA
 * @since:   2019-05-22
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ByRevision } from '../types.js'

import ResourceSpecification from './resourceSpecification.js'
import ProcessSpecification from './processSpecification.js'
import Unit from './unit.js'
import Process from './process.js'
import EconomicResource from './economicResource.js'
import EconomicEvent from './economicEvent.js'
import Commitment from './commitment.js'
import Fulfillment from './fulfillment.js'
import Intent from './intent.js'
import Satisfaction from './satisfaction.js'
import Proposal from './proposal.js'
import ProposedTo from './proposedTo.js'
import ProposedIntent from './proposedIntent.js'
import Agreement from './agreement.js'
import Plan from './plan.js'
import Agent from './agent.js'

// generic deletion calling format used by all mutations
export type deleteHandler = (root: any, args: ByRevision) => Promise<boolean>

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasMeasurement = -1 !== enabledVFModules.indexOf(VfModule.Measurement)
  const hasProcessSpecification = -1 !== enabledVFModules.indexOf(VfModule.ProcessSpecification)
  const hasResourceSpecification = -1 !== enabledVFModules.indexOf(VfModule.ResourceSpecification)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasProcess = -1 !== enabledVFModules.indexOf(VfModule.Process)
  const hasFulfillment = -1 !== enabledVFModules.indexOf(VfModule.Fulfillment)
  const hasIntent = -1 !== enabledVFModules.indexOf(VfModule.Intent)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)
  const hasSatisfaction = -1 !== enabledVFModules.indexOf(VfModule.Satisfaction)
  const hasProposal = -1 !== enabledVFModules.indexOf(VfModule.Proposal)
  const hasAgreement = -1 !== enabledVFModules.indexOf(VfModule.Agreement)
  const hasPlan = -1 !== enabledVFModules.indexOf(VfModule.Plan)

  return Object.assign(
    (hasMeasurement ? { ...Unit(dnaConfig, conductorUri) } : {}),
    (hasResourceSpecification ? {
      ...ResourceSpecification(dnaConfig, conductorUri),
    } : {}),
    (hasProcessSpecification ? {
      ...ProcessSpecification(dnaConfig, conductorUri),
    } : {}),
    (hasObservation ? {
      ...EconomicResource(dnaConfig, conductorUri),
      ...EconomicEvent(dnaConfig, conductorUri),
    } : {}),
    (hasProcess ? {
      ...Process(dnaConfig, conductorUri),
    } : {}),
    (hasCommitment ? {
      ...Commitment(dnaConfig, conductorUri),
    } : {}),
    (hasFulfillment ? {
      ...Fulfillment(dnaConfig, conductorUri),
    } : {}),
    (hasIntent ? {
      ...Intent(dnaConfig, conductorUri),
    } : {}),
    (hasSatisfaction ? {
      ...Satisfaction(dnaConfig, conductorUri),
    } : {}),
    (hasProposal ? {
      ...Proposal(dnaConfig, conductorUri),
      ...ProposedIntent(dnaConfig, conductorUri),
    } : {}),
    (hasProposal && hasAgent ? {
      ...ProposedTo(dnaConfig, conductorUri),
    } : {}),
    (hasAgreement ? { ...Agreement(dnaConfig, conductorUri) } : {}),
    (hasPlan ? { ...Plan(dnaConfig, conductorUri) } : {}),
    (hasAgent ? { ...Agent(dnaConfig, conductorUri) } : {}),
  )
}
