/**
 * Mutations manifest for HoloREA
 *
 * @package: HoloREA
 * @since:   2019-05-22
 */

import { DNAIdMappings } from '../types'

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

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => ({
  ...ResourceSpecification(dnaConfig, conductorUri),
  ...ProcessSpecification(dnaConfig, conductorUri),
  ...Unit(dnaConfig, conductorUri),
  ...Process(dnaConfig, conductorUri),
  ...EconomicResource(dnaConfig, conductorUri),
  ...EconomicEvent(dnaConfig, conductorUri),
  ...Commitment(dnaConfig, conductorUri),
  ...Fulfillment(dnaConfig, conductorUri),
  ...Intent(dnaConfig, conductorUri),
  ...Satisfaction(dnaConfig, conductorUri),
  ...Proposal(dnaConfig, conductorUri),
  ...ProposedTo(dnaConfig, conductorUri),
  ...ProposedIntent(dnaConfig, conductorUri),
})
