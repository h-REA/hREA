/**
 * Queries manifest for HoloREA
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'

import Action from './action'
import Unit from './unit'
import ResourceSpecification from './resourceSpecification'
import ProcessSpecification from './processSpecification'

import Agent from './agent'

import Process from './process'
import EconomicResource from './economicResource'
import EconomicEvent from './economicEvent'

import Commitment from './commitment'
import Fulfillment from './fulfillment'

import Intent from './intent'
import Satisfaction from './satisfaction'

import Proposal from './proposal'

import Agreement from './agreement'
import Plan from './plan'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasMeasurement = -1 !== enabledVFModules.indexOf(VfModule.Measurement)
  const hasAction = -1 !== enabledVFModules.indexOf(VfModule.Action)
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
    (hasAction ? {
      ...Action(dnaConfig, conductorUri),
    } : {}),
    (hasMeasurement ? { ...Unit(dnaConfig, conductorUri) } : {}),
    (hasResourceSpecification ? {
      ...ResourceSpecification(dnaConfig, conductorUri),
    } : {}),
    (hasProcessSpecification ? {
      ...ProcessSpecification(dnaConfig, conductorUri),
    } : {}),
    (hasAgent ? { ...Agent(dnaConfig, conductorUri) } : {}),
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
    (hasProposal ? { ...Proposal(dnaConfig, conductorUri) } : {}),
    (hasAgreement ? { ...Agreement(dnaConfig, conductorUri) } : {}),
    (hasPlan ? { ...Plan(dnaConfig, conductorUri) } : {}),
  )
}
