/**
 * Resolver callbacks to bind vf-graphql with HoloREA DNAs
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { DNAIdMappings, ResolverOptions, URI, DateTime, DEFAULT_VF_MODULES } from '../types'

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

import Agreement from './agreement'

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

export default (options?: ResolverOptions) => {
  const {
    dnaConfig,
    enabledVFModules = DEFAULT_VF_MODULES,
    conductorUri = undefined,
    traceAppSignals = undefined,
  } = (options || {})

  const hasAgent = -1 !== enabledVFModules.indexOf("agent")
  const hasMeasurement = -1 !== enabledVFModules.indexOf("measurement")
  const hasKnowledge = -1 !== enabledVFModules.indexOf("knowledge")
  const hasObservation = -1 !== enabledVFModules.indexOf("observation")
  const hasPlanning = -1 !== enabledVFModules.indexOf("planning")
  const hasProposal = -1 !== enabledVFModules.indexOf("proposal")
  const hasAgreement = -1 !== enabledVFModules.indexOf("agreement")

  return Object.assign({
    // scalars
    URI, DateTime,
    // union type disambiguators
    EventOrCommitment, ProductionFlowItem, AccountingScope,
    // root schemas
    Query: Query(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
    Mutation: Mutation(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
  },
    // object field resolvers
    (hasAgent ? { Agent: Agent(enabledVFModules, dnaConfig, conductorUri, traceAppSignals) } : {}),
    (hasMeasurement ? { Measure: Measure(enabledVFModules, dnaConfig, conductorUri, traceAppSignals) } : {}),
    (hasKnowledge ? { ResourceSpecification: ResourceSpecification(enabledVFModules, dnaConfig, conductorUri, traceAppSignals) } : {}),
    (hasObservation ? {
      Process: Process(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
      EconomicEvent: EconomicEvent(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
      EconomicResource: EconomicResource(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
    } : {}),
    (hasPlanning ? {
      Commitment: Commitment(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
      Fulfillment: Fulfillment(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
      Intent: Intent(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
      Satisfaction: Satisfaction(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
    } : {}),
    (hasProposal ? {
      Proposal: Proposal(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
      ProposedTo: ProposedTo(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
      ProposedIntent: ProposedIntent(enabledVFModules, dnaConfig, conductorUri, traceAppSignals),
    } : {}),
    (hasAgreement ? { Agreement: Agreement(enabledVFModules, dnaConfig, conductorUri, traceAppSignals) } : {}),
  )
}
