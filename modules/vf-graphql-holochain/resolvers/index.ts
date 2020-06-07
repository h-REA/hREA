/**
 * Resolver callbacks to bind vf-graphql with HoloREA DNAs
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { DNAIdMappings, ResolverOptions, URI, DateTime } from '../types'

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
const ProductionFlowItem = {
  __resolveType: (obj, ctx, info) => obj.__typename,
}
const AccountingScope = {
  __resolveType: (obj, ctx, info) => obj.__typename,
}

export const DEFAULT_VF_MODULES = [
  'knowledge', 'measurement',
  'agent',
  'observation', 'planning', 'proposal',
]

export default (options?: ResolverOptions) => {
  const {
    enabledVFModules = DEFAULT_VF_MODULES,
    dnaConfig = undefined,
    conductorUri = undefined,
  } = (options || {})

  const hasAgent = -1 !== enabledVFModules.indexOf("agent")
  const hasMeasurement = -1 !== enabledVFModules.indexOf("measurement")
  const hasKnowledge = -1 !== enabledVFModules.indexOf("knowledge")
  const hasObservation = -1 !== enabledVFModules.indexOf("observation")
  const hasPlanning = -1 !== enabledVFModules.indexOf("planning")
  const hasProposal = -1 !== enabledVFModules.indexOf("proposal")

  return Object.assign({
    // scalars
    URI, DateTime,
    // union type disambiguators
    EventOrCommitment, ProductionFlowItem, AccountingScope,
    // root schemas
    Query: Query(enabledVFModules, dnaConfig, conductorUri),
    Mutation: Mutation(enabledVFModules, dnaConfig, conductorUri),
  },
    // object field resolvers
    (hasAgent ? { Agent: Agent(dnaConfig, conductorUri) } : {}),
    (hasMeasurement ? { Measure: Measure(dnaConfig, conductorUri) } : {}),
    (hasKnowledge ? { ResourceSpecification: ResourceSpecification(dnaConfig, conductorUri) } : {}),
    (hasObservation ? {
      Process: Process(dnaConfig, conductorUri),
      EconomicEvent: EconomicEvent(dnaConfig, conductorUri),
      EconomicResource: EconomicResource(dnaConfig, conductorUri),
    } : {}),
    (hasPlanning ? {
      Commitment: Commitment(dnaConfig, conductorUri),
      Fulfillment: Fulfillment(dnaConfig, conductorUri),
      Intent: Intent(dnaConfig, conductorUri),
      Satisfaction: Satisfaction(dnaConfig, conductorUri),
    } : {}),
    (hasProposal ? {
      Proposal: Proposal(dnaConfig, conductorUri),
      ProposedTo: ProposedTo(dnaConfig, conductorUri),
      ProposedIntent: ProposedIntent(dnaConfig, conductorUri),
    } : {}),
  )
}
