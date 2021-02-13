/**
 * Resolvers for Commitment fields
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { DNAIdMappings, DEFAULT_VF_MODULES } from '../types'
import { mapZomeFn } from '../connection'

import {
  Agent,
  Commitment,
  Fulfillment,
  Satisfaction,
  Process,
  ResourceSpecification,
  Action,
  Agreement,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent'
import agreementQueries from '../queries/agreement'

export default (enabledVFModules: string[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri?: string, traceAppSignals?: AppSignalCb) => {
  const hasAgent = -1 !== enabledVFModules.indexOf("agent")
  const hasKnowledge = -1 !== enabledVFModules.indexOf("knowledge")
  const hasObservation = -1 !== enabledVFModules.indexOf("observation")
  const hasAgreement = -1 !== enabledVFModules.indexOf("agreement")

  const readFulfillments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'fulfillment', 'query_fulfillments')
  const readSatisfactions = mapZomeFn(dnaConfig, conductorUri, 'planning', 'satisfaction', 'query_satisfactions')
  const readProcesses = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process', 'query_processes')
  const readResourceSpecification = mapZomeFn(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readAction = mapZomeFn(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']
  const readAgreement = agreementQueries(dnaConfig, conductorUri)['agreement']

  return Object.assign(
    {
      fulfilledBy: async (record: Commitment): Promise<Fulfillment[]> => {
        return (await readFulfillments({ params: { fulfills: record.id } })).map(({ fulfillment }) => fulfillment)
      },

      satisfies: async (record: Commitment): Promise<Satisfaction[]> => {
        return (await readSatisfactions({ params: { satisfiedBy: record.id } })).map(({ satisfaction }) => satisfaction)
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
        return (await readProcesses({ params: { committedInputs: record.id } })).pop()['process']
      },

      outputOf: async (record: Commitment): Promise<Process[]> => {
        return (await readProcesses({ params: { committedOutputs: record.id } })).pop()['process']
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
  )
}
