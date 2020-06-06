/**
 * Queries manifest for HoloREA
 *
 * @package: HoloREA
 * @since:   2019-05-27
 */

import { DNAIdMappings } from '../types'

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

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => ({
  ...Action(dnaConfig, conductorUri),
  ...Unit(dnaConfig, conductorUri),
  ...ResourceSpecification(dnaConfig, conductorUri),
  ...ProcessSpecification(dnaConfig, conductorUri),
  ...Agent(dnaConfig, conductorUri),
  ...Process(dnaConfig, conductorUri),
  ...EconomicResource(dnaConfig, conductorUri),
  ...EconomicEvent(dnaConfig, conductorUri),
  ...Commitment(dnaConfig, conductorUri),
  ...Fulfillment(dnaConfig, conductorUri),
  ...Intent(dnaConfig, conductorUri),
  ...Satisfaction(dnaConfig, conductorUri),
  ...Proposal(dnaConfig, conductorUri),
})
