/**
 * Resolver callbacks to bind vf-graphql with HoloREA DNAs
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { DNAIdMappings, ResolverOptions, URI, DEFAULT_VF_MODULES, VfModule } from '../types'
import { DateTimeResolver as DateTime } from 'graphql-scalars'

import { openConnection } from '../connection'

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

export default async (options: ResolverOptions) => {
  const {
    enabledVFModules = DEFAULT_VF_MODULES,
    conductorUri,
    dnaConfig,
    traceAppSignals = undefined,
  } = options

  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasMeasurement = -1 !== enabledVFModules.indexOf(VfModule.Measurement)
  const hasKnowledge = -1 !== enabledVFModules.indexOf(VfModule.Knowledge)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasPlanning = -1 !== enabledVFModules.indexOf(VfModule.Planning)
  const hasProposal = -1 !== enabledVFModules.indexOf(VfModule.Proposal)
  const hasAgreement = -1 !== enabledVFModules.indexOf(VfModule.Agreement)

  // prefetch connection for this API schema
  await openConnection(conductorUri, traceAppSignals)

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
    (hasAgent ? { Agent: Agent(enabledVFModules, dnaConfig, conductorUri) } : {}),
    (hasMeasurement ? { Measure: Measure(enabledVFModules, dnaConfig, conductorUri) } : {}),
    (hasKnowledge ? { ResourceSpecification: ResourceSpecification(enabledVFModules, dnaConfig, conductorUri) } : {}),
    (hasObservation ? {
      Process: Process(enabledVFModules, dnaConfig, conductorUri),
      EconomicEvent: EconomicEvent(enabledVFModules, dnaConfig, conductorUri),
      EconomicResource: EconomicResource(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasPlanning ? {
      Commitment: Commitment(enabledVFModules, dnaConfig, conductorUri),
      Fulfillment: Fulfillment(enabledVFModules, dnaConfig, conductorUri),
      Intent: Intent(enabledVFModules, dnaConfig, conductorUri),
      Satisfaction: Satisfaction(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasProposal ? {
      Proposal: Proposal(enabledVFModules, dnaConfig, conductorUri),
      ProposedTo: ProposedTo(enabledVFModules, dnaConfig, conductorUri),
      ProposedIntent: ProposedIntent(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasAgreement ? { Agreement: Agreement(enabledVFModules, dnaConfig, conductorUri) } : {}),
  )
}
