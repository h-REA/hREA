/**
 * Mutations manifest for HoloREA
 *
 * @package: HoloREA
 * @since:   2019-05-22
 */

import { DNAIdMappings, DEFAULT_VF_MODULES } from '../types'

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

// generic deletion calling format used by all mutations
export type deleteHandler = (root: any, args: { id: string }) => Promise<boolean>

export default (enabledVFModules: string[] = DEFAULT_VF_MODULES, dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const VFmodules = enabledVFModules || []
  const hasAgent = -1 !== VFmodules.indexOf("agent")
  const hasMeasurement = -1 !== VFmodules.indexOf("measurement")
  const hasKnowledge = -1 !== VFmodules.indexOf("knowledge")
  const hasObservation = -1 !== VFmodules.indexOf("observation")
  const hasPlanning = -1 !== VFmodules.indexOf("planning")
  const hasProposal = -1 !== VFmodules.indexOf("proposal")

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
  )
}
