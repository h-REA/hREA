/**
 * Resolver callbacks to bind vf-graphql with HoloREA DNAs
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { DNAIdMappings, URI, DateTime } from '../types'

import Query from '../queries'
import Mutation from '../mutations'

import Measure from './measure'
import ResourceSpecification from './resourceSpecification'

import Agent from './agent'

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

// union type disambiguation
const EventOrCommitment = {
  __resolveType: (obj, ctx, info) => obj.__typename,
}
const AccountingScope = {
  __resolveType: (obj, ctx, info) => obj.__typename,
}

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => ({
  // scalars
  URI, DateTime,
  // root schemas
  Query: Query(dnaConfig, conductorUri),
  Mutation: Mutation(dnaConfig, conductorUri),
  // object field resolvers
  Agent: Agent(dnaConfig, conductorUri),
  Measure: Measure(dnaConfig, conductorUri),
  ResourceSpecification: ResourceSpecification(dnaConfig, conductorUri),
  Process: Process(dnaConfig, conductorUri),
  EconomicResource: EconomicResource(dnaConfig, conductorUri),
  EconomicEvent: EconomicEvent(dnaConfig, conductorUri),
  Commitment: Commitment(dnaConfig, conductorUri),
  Fulfillment: Fulfillment(dnaConfig, conductorUri),
  Intent: Intent(dnaConfig, conductorUri),
  Satisfaction: Satisfaction(dnaConfig, conductorUri),
  Proposal: Proposal(dnaConfig, conductorUri),
  ProposedTo: ProposedTo(dnaConfig, conductorUri),
  ProposedIntent: ProposedIntent(dnaConfig, conductorUri),
  // union type disambiguators
  EventOrCommitment,
  AccountingScope,
})
