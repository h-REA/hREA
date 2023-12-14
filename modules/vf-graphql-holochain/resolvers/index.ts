/**
 * Resolver callbacks to bind vf-graphql with HoloREA DNAs
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { DNAIdMappings, ResolverOptions, URI, Decimal, DEFAULT_VF_MODULES, VfModule } from '../types.js'
import { DateTimeResolver as DateTime } from 'graphql-scalars'

import { autoConnect, openConnection } from '../connection.js'

import Query from '../queries/index.js'
import Mutation from '../mutations/index.js'

import Revision from './revision.js'

import Measure from './measure.js'
import Unit from './unit.js'
import ResourceSpecification from './resourceSpecification.js'

import Agent from './agent.js'

import Process from './process.js'
import ProcessSpecification from './processSpecification.js'
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

import RecipeFlow from './recipeFlow.js'
import RecipeProcess from './recipeProcess.js'

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
const TrackTraceItem = {
  __resolveType: (obj, ctx, info) => obj.__typename,
}

// the default export
const generateResolvers = async (options: ResolverOptions) => {
  const {
    enabledVFModules,
    conductorUri,
    adminConductorUri,
    appId,
    dnaConfig,
    traceAppSignals = undefined,
  } = options

  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasMeasurement = -1 !== enabledVFModules.indexOf(VfModule.Measurement)
  const hasResourceSpecification = -1 !== enabledVFModules.indexOf(VfModule.ResourceSpecification)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasProcess = -1 !== enabledVFModules.indexOf(VfModule.Process)
  const hasProcessSpecification = -1 !== enabledVFModules.indexOf(VfModule.ProcessSpecification)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)
  const hasFulfillment = -1 !== enabledVFModules.indexOf(VfModule.Fulfillment)
  const hasIntent = -1 !== enabledVFModules.indexOf(VfModule.Intent)
  const hasSatisfaction = -1 !== enabledVFModules.indexOf(VfModule.Satisfaction)
  const hasProposal = -1 !== enabledVFModules.indexOf(VfModule.Proposal)
  const hasAgreement = -1 !== enabledVFModules.indexOf(VfModule.Agreement)
  const hasPlan = -1 !== enabledVFModules.indexOf(VfModule.Plan)
  const hasRecipe = -1 !== enabledVFModules.indexOf(VfModule.Recipe)

  // prefetch connection for this API schema
  // and also this makes calls to the admin conductor to authorize a set of signing
  // credentials for each Cell
  await autoConnect(conductorUri, adminConductorUri, appId, traceAppSignals)

  return Object.assign({
    // scalars
    URI, DateTime,
    // root schemas
    Query: Query(enabledVFModules, dnaConfig, conductorUri),
    Mutation: Mutation(enabledVFModules, dnaConfig, conductorUri),
  },
  // union type disambiguators
  (hasObservation && hasProcess ? { ProductionFlowItem } : {}),
  (hasAgent ? { AccountingScope } : {}),
  (hasSatisfaction ? { EventOrCommitment } : {}),
  (hasObservation ? { TrackTraceItem } : {}),
    // object field resolvers
    (hasHistory ? {
      Revision: Revision(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasAgent ? {
      Agent: Agent(enabledVFModules, dnaConfig, conductorUri),
      Person: Agent(enabledVFModules, dnaConfig, conductorUri),
      Organization: Agent(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasMeasurement ? {
      Measure: Measure(enabledVFModules, dnaConfig, conductorUri),
      Unit: Unit(enabledVFModules, dnaConfig, conductorUri)
    } : {}),
    (hasResourceSpecification ? { ResourceSpecification: ResourceSpecification(enabledVFModules, dnaConfig, conductorUri) } : {}),
    (hasObservation ? {
      EconomicEvent: EconomicEvent(enabledVFModules, dnaConfig, conductorUri),
      EconomicResource: EconomicResource(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasProcess ? {
      Process: Process(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasProcessSpecification ? {
      ProcessSpecification: ProcessSpecification(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasCommitment ? {
      Commitment: Commitment(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasFulfillment ? {
      Fulfillment: Fulfillment(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasIntent ? {
      Intent: Intent(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasSatisfaction ? {
      Satisfaction: Satisfaction(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasProposal ? {
      Proposal: Proposal(enabledVFModules, dnaConfig, conductorUri),
      ProposedIntent: ProposedIntent(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasProposal && hasAgent ? {
      ProposedTo: ProposedTo(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
    (hasAgreement ? { Agreement: Agreement(enabledVFModules, dnaConfig, conductorUri) } : {}),
    (hasPlan ? { Plan: Plan(enabledVFModules, dnaConfig, conductorUri) } : {}),
    (hasRecipe ? { 
      RecipeFlow: RecipeFlow(enabledVFModules, dnaConfig, conductorUri),
      RecipeProcess: RecipeProcess(enabledVFModules, dnaConfig, conductorUri),
    } : {}),
  )
}

export default generateResolvers
