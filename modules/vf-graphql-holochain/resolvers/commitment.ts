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
  Maybe,
  Plan,
  FulfillmentConnection,
  ProcessConnection,
  SatisfactionConnection,
  ResourceSpecificationResponse,
  ProcessSpecificationResponse,
  AccountingScope,
  ProcessSpecification,
  EconomicResource,
} from '@leosprograms/vf-graphql'

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

  const readRevision = mapZomeFn<ByRevision, CommitmentResponse>(dnaConfig, conductorUri, 'combined', 'commitment', 'get_revision')
  const readFulfillments = mapZomeFn<FulfillmentSearchInput, FulfillmentConnection>(dnaConfig, conductorUri, 'combined', 'indexing', 'query_fulfillments')
  const readSatisfactions = mapZomeFn<SatisfactionSearchInput, SatisfactionConnection>(dnaConfig, conductorUri, 'combined', 'indexing', 'query_satisfactions')
  const readProcesses = mapZomeFn<ProcessSearchInput, ProcessConnection>(dnaConfig, conductorUri, 'combined', 'indexing', 'query_processes')
  const readResourceSpecification = mapZomeFn<ReadParams, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'combined', 'resource_specification', 'get_resource_specification')
  const readProcessSpecification = mapZomeFn<ReadParams, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'combined', 'process_specification', 'get_process_specification')
  const readAction = mapZomeFn<ById, Action>(dnaConfig, conductorUri, 'combined', 'action', 'get_action')
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
      providerId: async (record: Commitment): Promise<any> => {
        return record.provider ? record.provider : null
      },

      provider: async (record: Commitment): Promise<Agent> => {
        return readAgent(record, { id: record.provider })
      },

      receiverId: async (record: Commitment): Promise<any> => {
        return record.receiver ? record.receiver : null
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
      inputOf: async (record: Commitment): Promise<Process | null> => {
        try {
          const results = await readProcesses({ params: { committedInputs: record.id } })
          console.log("inputOf", results)
          return results.edges.pop()!['node']
        } catch (e) {
          console.error(`Error fetching inputOf for Commitment ${record.id}`, e)
          return null
        }
      },

      outputOf: async (record: Commitment): Promise<Process | null> => {
        try {
          const results = await readProcesses({ params: { committedOutputs: record.id } })
          console.log("outputOf", results)
          return results.edges.pop()!['node']
        } catch (e) {
          console.error(`Error fetching outputOf for Commitment ${record.id}`, e)
          return null
        }
      },
    } : {}),
    (hasResourceSpecification ? {
      resourceConformsTo: async (record: { resourceConformsTo: ResourceSpecificationAddress }): Promise<ResourceSpecification> => {
        return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
      },
    } : {}),
    (hasProcessSpecification ? {
      stageId: async (record: Commitment): Promise<any> => {
        return record.stage ? record.stage : "undefined"
      },
      stage: async (record: { stage: ProcessSpecificationAddress }): Promise<ProcessSpecification | {}> => {
        return record.stage ? (await readProcessSpecification({ address: record.stage })).processSpecification : {}
      },
    } : {}),
    (hasAction ? {
      action: async (record: { action: AddressableIdentifier }): Promise<Action> => {
        return (await readAction({ id: record.action }))
      },
    } : {}),
    (hasAgreement ? {
      clauseOf: async (record: Commitment): Promise<Maybe<Agreement>> => {
        if (!record.clauseOf) { return null }
        try {
          return await readAgreement(record, { id: record.clauseOf })
        } catch (e) {
          console.error(`Error fetching clauseOf for Commitment ${record.id}`, e)
          return null
        }
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
