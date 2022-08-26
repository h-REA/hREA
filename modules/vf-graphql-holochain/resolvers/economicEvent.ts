/**
 * Resolvers for EconomicEvent fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ById, ByRevision, ReadParams, ResourceSpecificationAddress, AddressableIdentifier, AgentAddress, EconomicResourceAddress } from '../types.js'
import { extractEdges, mapZomeFn } from '../connection.js'

import {
  Agent,
  EconomicEvent,
  EconomicEventResponse,
  EconomicResource,
  Fulfillment,
  Satisfaction,
  Process,
  ResourceSpecification,
  Action,
  Agreement,
  FulfillmentConnection,
  SatisfactionConnection,
  ProcessConnection,
  ResourceSpecificationResponse,
  AccountingScope,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent.js'
import agreementQueries from '../queries/agreement.js'
import resourceQueries from '../queries/economicResource.js'
import { FulfillmentSearchInput, ProcessSearchInput, SatisfactionSearchInput } from './zomeSearchInputTypes.js'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasResourceSpecification = -1 !== enabledVFModules.indexOf(VfModule.ResourceSpecification)
  const hasAction = -1 !== enabledVFModules.indexOf(VfModule.Action)
  const hasFulfillment = -1 !== enabledVFModules.indexOf(VfModule.Fulfillment)
  const hasSatisfaction = -1 !== enabledVFModules.indexOf(VfModule.Satisfaction)
  const hasAgreement = -1 !== enabledVFModules.indexOf(VfModule.Agreement)
  const hasProcess = -1 !== enabledVFModules.indexOf(VfModule.Process)

  const readRevision = mapZomeFn<ByRevision, EconomicEventResponse>(dnaConfig, conductorUri, 'observation', 'economic_event', 'get_revision')
  const readFulfillments = mapZomeFn<FulfillmentSearchInput, FulfillmentConnection>(dnaConfig, conductorUri, 'observation', 'fulfillment_index', 'query_fulfillments')
  const readSatisfactions = mapZomeFn<SatisfactionSearchInput, SatisfactionConnection>(dnaConfig, conductorUri, 'observation', 'satisfaction_index', 'query_satisfactions')
  const readProcesses = mapZomeFn<ProcessSearchInput, ProcessConnection>(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const readAction = mapZomeFn<ById, Action>(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readResourceSpecification = mapZomeFn<ReadParams, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']
  const readAgreement = agreementQueries(dnaConfig, conductorUri)['agreement']
  const readResource = resourceQueries(dnaConfig, conductorUri)['economicResource']

  return Object.assign(
    {
      resourceInventoriedAs: async (record: { resourceInventoriedAs: EconomicResourceAddress }): Promise<EconomicResource | null> => {
        if (!record.resourceInventoriedAs) return null
        return await readResource(record, { id: record.resourceInventoriedAs })
      },
      toResourceInventoriedAs: async (record: { toResourceInventoriedAs: EconomicResourceAddress }): Promise<EconomicResource | null> => {
        if (!record.toResourceInventoriedAs) return null
        return await readResource(record, { id: record.toResourceInventoriedAs })
      },
      triggeredBy: () => {
        throw new Error('resolver unimplemented')
      },
      triggers: () => {
        throw new Error('resolver unimplemented')
      },
      previous: () => {
        throw new Error('resolver unimplemented')
      },
      next: () => {
        throw new Error('resolver unimplemented')
      },
      track: () => {
        throw new Error('resolver unimplemented')
      },
      trace: () => {
        throw new Error('resolver unimplemented')
      },
    },
    (hasProcess ? {
      inputOf: async (record: EconomicEvent): Promise<Process> => {
        const results = await readProcesses({ params: { observedInputs: record.id } })
        return results.edges.pop()!['node']
      },

      outputOf: async (record: EconomicEvent): Promise<Process> => {
        const results = await readProcesses({ params: { observedOutputs: record.id } })
        return results.edges.pop()!['node']
      },
    } : {}),
    (hasAgent ? {
      provider: async (record: EconomicEvent): Promise<Agent> => {
        return readAgent(record, { id: record.provider })
      },

      receiver: async (record: EconomicEvent): Promise<Agent> => {
        return readAgent(record, { id: record.receiver })
      },
      inScopeOf: async (record: { inScopeOf: AgentAddress[] }): Promise<AccountingScope[]> => {
        return (await Promise.all((record.inScopeOf || []).map((address)=>readAgent(record, {address}))))
      },
    } : {}),
    (hasFulfillment ? {
      fulfills: async (record: EconomicEvent): Promise<Fulfillment[]> => {
        const results = await readFulfillments({ params: { fulfilledBy: record.id } })
        return extractEdges(results)
      },
    } : {}),
    (hasSatisfaction ? {
      satisfies: async (record: EconomicEvent): Promise<Satisfaction[]> => {
        const results = await readSatisfactions({ params: { satisfiedBy: record.id } })
        return extractEdges(results)
      },
    } : {}),
    (hasResourceSpecification ? {
      resourceConformsTo: async (record: { resourceConformsTo: ResourceSpecificationAddress }): Promise<ResourceSpecification> => {
        // record isn't quite an `EconomicEvent` since it stores ids for linked types, not the type itself, right?
        return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
      },
    } : {}),
    (hasAction ? {
      action: async (record: { action: AddressableIdentifier }): Promise<Action> => {
        return (await readAction({ id: record.action }))
      },
    } : {}),
    (hasAgreement ? {
      realizationOf: async (record: EconomicEvent): Promise<Agreement> => {
        return readAgreement(record, { id: record.realizationOf })
      },
    } : {}),
    (hasHistory ? {
      revision: async (record: EconomicEvent, args: { revisionId: AddressableIdentifier }): Promise<EconomicEvent> => {
        return (await readRevision(args)).economicEvent
      },
    } : {}),
  )
}
