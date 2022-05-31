/**
 * Resolvers for Commitment fields
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'
import { extractEdges, mapZomeFn } from '../connection'

import {
  Agent,
  Commitment,
  Fulfillment,
  Satisfaction,
  Process,
  ResourceSpecification,
  Action,
  Agreement,
  Plan,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent'
import agreementQueries from '../queries/agreement'
import planQueries from '../queries/plan'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasKnowledge = -1 !== enabledVFModules.indexOf(VfModule.Knowledge)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasAgreement = -1 !== enabledVFModules.indexOf(VfModule.Agreement)
  const hasPlan = -1 !== enabledVFModules.indexOf(VfModule.Plan)

  const readFulfillments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'fulfillment_index', 'query_fulfillments')
  const readSatisfactions = mapZomeFn(dnaConfig, conductorUri, 'planning', 'satisfaction_index', 'query_satisfactions')
  const readProcesses = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const readResourceSpecification = mapZomeFn(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readAction = mapZomeFn(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readPlan = planQueries(dnaConfig, conductorUri)['plan']
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']
  const readAgreement = agreementQueries(dnaConfig, conductorUri)['agreement']

  return Object.assign(
    {
      fulfilledBy: async (record: Commitment): Promise<Fulfillment[]> => {
        const results = await readFulfillments({ params: { fulfills: record.id } })
        return extractEdges(results)
      },

      satisfies: async (record: Commitment): Promise<Satisfaction[]> => {
        const results = await readSatisfactions({ params: { satisfiedBy: record.id } })
        return extractEdges(results)
      },
    },
    (hasAgent ? {
      provider: async (record: Commitment): Promise<Agent> => {
        return readAgent(record, { id: record.provider })
      },

      receiver: async (record: Commitment): Promise<Agent> => {
        return readAgent(record, { id: record.receiver })
      },
    } : {}),
    (hasObservation ? {
      inputOf: async (record: Commitment): Promise<Process[]> => {
        const results = await readProcesses({ params: { committedInputs: record.id } })
        return results.edges.pop()['node']
      },

      outputOf: async (record: Commitment): Promise<Process[]> => {
        const results = await readProcesses({ params: { committedOutputs: record.id } })
        return results.edges.pop()['node']
      },
    } : {}),
    (hasKnowledge ? {
      resourceConformsTo: async (record: Commitment): Promise<ResourceSpecification> => {
        return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
      },

      action: async (record: Commitment): Promise<Action> => {
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
  )
}
