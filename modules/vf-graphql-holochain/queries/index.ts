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

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasMeasurement = -1 !== enabledVFModules.indexOf(VfModule.Measurement)
  const hasKnowledge = -1 !== enabledVFModules.indexOf(VfModule.Knowledge)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasPlanning = -1 !== enabledVFModules.indexOf(VfModule.Planning)
  const hasProposal = -1 !== enabledVFModules.indexOf(VfModule.Proposal)
  const hasAgreement = -1 !== enabledVFModules.indexOf(VfModule.Agreement)

  return Object.assign({
      ...Action(dnaConfig, conductorUri),
    },
    (hasMeasurement ? { ...Unit(dnaConfig, conductorUri) } : {}),
    (hasKnowledge ? {
      ...ResourceSpecification(dnaConfig, conductorUri),
      ...ProcessSpecification(dnaConfig, conductorUri),
    } : {}),
    (hasAgent ? { ...Agent(dnaConfig, conductorUri) } : {}),
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
    (hasProposal ? { ...Proposal(dnaConfig, conductorUri) } : {}),
    (hasAgreement ? { ...Agreement(dnaConfig, conductorUri) } : {}),
  )
}
