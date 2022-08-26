/**
 * Resolvers for Commitment fields
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ReadParams, ById, ByRevision, ResourceSpecificationAddress, AddressableIdentifier, AgentAddress, ProcessSpecificationAddress } from '../types.js'
import { extractEdges, mapZomeFn } from '../connection.js'

import {
  Agent,
  Commitment,
  CommitmentResponse,
  Fulfillment,
  Satisfaction,
  Process,
  ResourceSpecification,
  Action,
  Agreement,
  Plan,
  FulfillmentConnection,
  ProcessConnection,
  SatisfactionConnection,
  ResourceSpecificationResponse,
  AccountingScope,
  ProcessSpecification,
  EconomicResource,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent.js'
import agreementQueries from '../queries/agreement.js'
import planQueries from '../queries/plan.js'
import { FulfillmentSearchInput, ProcessSearchInput, SatisfactionSearchInput } from './zomeSearchInputTypes.js'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasProcess = -1 !== enabledVFModules.indexOf(VfModule.Process)
  const hasResourceSpecification = -1 !== enabledVFModules.indexOf(VfModule.ResourceSpecification)
  const hasProcessSpecification = -1 !== enabledVFModules.indexOf(VfModule.ProcessSpecification)
  const hasAction = -1 !== enabledVFModules.indexOf(VfModule.Action)
  const hasAgreement = -1 !== enabledVFModules.indexOf(VfModule.Agreement)
  const hasPlan = -1 !== enabledVFModules.indexOf(VfModule.Plan)
  const hasFulfillment = -1 !== enabledVFModules.indexOf(VfModule.Fulfillment)
  const hasSatisfaction = -1 !== enabledVFModules.indexOf(VfModule.Satisfaction)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)

  const readRevision = mapZomeFn<ByRevision, CommitmentResponse>(dnaConfig, conductorUri, 'planning', 'commitment', 'get_revision')
  const readFulfillments = mapZomeFn<FulfillmentSearchInput, FulfillmentConnection>(dnaConfig, conductorUri, 'planning', 'fulfillment_index', 'query_fulfillments')
  const readSatisfactions = mapZomeFn<SatisfactionSearchInput, SatisfactionConnection>(dnaConfig, conductorUri, 'planning', 'satisfaction_index', 'query_satisfactions')
  const readProcesses = mapZomeFn<ProcessSearchInput, ProcessConnection>(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const readResourceSpecification = mapZomeFn<ReadParams, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readAction = mapZomeFn<ById, Action>(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readPlan = planQueries(dnaConfig, conductorUri)['plan']
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']
  const readAgreement = agreementQueries(dnaConfig, conductorUri)['agreement']

  return Object.assign(
    (hasFulfillment ? {
      fulfilledBy: async (record: Commitment): Promise<Fulfillment[]> => {
        const results = await readFulfillments({ params: { fulfills: record.id } })
        return extractEdges(results)
      },
    } : {}),
    (hasSatisfaction ? {
      satisfies: async (record: Commitment): Promise<Satisfaction[]> => {
        const results = await readSatisfactions({ params: { satisfiedBy: record.id } })
        return extractEdges(results)
      },
    } : {}),
    (hasAgent ? {
      provider: async (record: Commitment): Promise<Agent> => {
        return readAgent(record, { id: record.provider })
      },

      receiver: async (record: Commitment): Promise<Agent> => {
        return readAgent(record, { id: record.receiver })
      },
      inScopeOf: async (record: { inScopeOf: AgentAddress[] }): Promise<AccountingScope[]> => {
        return (await Promise.all((record.inScopeOf || []).map((address)=>readAgent(record, {address}))))
      },
      involvedAgents: async (record: { involvedAgents: AgentAddress[] }): Promise<Agent[]> => {
        return (await Promise.all((record.involvedAgents || []).map((address)=>readAgent(record, {address}))))
      },
    } : {}),
    (hasProcess ? {
      inputOf: async (record: Commitment): Promise<Process> => {
        const results = await readProcesses({ params: { committedInputs: record.id } })
        return results.edges.pop()!['node']
      },

      outputOf: async (record: Commitment): Promise<Process> => {
        const results = await readProcesses({ params: { committedOutputs: record.id } })
        return results.edges.pop()!['node']
      },
    } : {}),
    (hasResourceSpecification ? {
      resourceConformsTo: async (record: { resourceConformsTo: ResourceSpecificationAddress }): Promise<ResourceSpecification> => {
        return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
      },
    } : {}),
    (hasProcessSpecification ? {
      stage: async (record: { stage: ProcessSpecificationAddress }): Promise<ProcessSpecification> => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasAction ? {
      action: async (record: { action: AddressableIdentifier }): Promise<Action> => {
        return (await readAction({ id: record.action }))
      },
    } : {}),
    (hasAgreement ? {
      clauseOf: async (record: Commitment): Promise<Agreement> => {
        return readAgreement(record, { id: record.clauseOf })
      },
    } : {}),
    (hasPlan ? {
      independentDemandOf: async (record: Commitment): Promise<Plan> => {
        return readPlan(record, { id: record.independentDemandOf })
      },
      plannedWithin: async (record: Commitment): Promise<Plan> => {
        return readPlan(record, { id: record.plannedWithin })
      },
    } : {}),
    (hasObservation ? {
      resourceInventoriedAs: async (record: Commitment): Promise<EconomicResource> => {
        throw new Error('resolver unimplemented')
      },
    } : {}),
    (hasHistory ? {
      revision: async (record: Commitment, args: { revisionId: AddressableIdentifier }): Promise<Commitment> => {
        return (await readRevision(args)).commitment
      },
    } : {}),
  )
}
