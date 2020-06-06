/**
 * Resolvers for EconomicEvent fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Agent,
  EconomicEvent,
  Fulfillment,
  Satisfaction,
  Process,
  ResourceSpecification,
  Action,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent'

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const readFulfillments = mapZomeFn(dnaConfig, conductorUri, 'observation', 'fulfillment', 'query_fulfillments')
  const readSatisfactions = mapZomeFn(dnaConfig, conductorUri, 'observation', 'satisfaction', 'query_satisfactions')
  const readProcesses = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process', 'query_processes')
  const readAction = mapZomeFn(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readResourceSpecification = mapZomeFn(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']

  return {
    provider: async (record: EconomicEvent): Promise<Agent> => {
      return readAgent(record, { id: record.provider })
    },

    receiver: async (record: EconomicEvent): Promise<Agent> => {
      return readAgent(record, { id: record.receiver })
    },

    inputOf: async (record: EconomicEvent): Promise<Process[]> => {
      return (await readProcesses({ params: { inputs: record.id } })).pop()['process']
    },

    outputOf: async (record: EconomicEvent): Promise<Process[]> => {
      return (await readProcesses({ params: { outputs: record.id } })).pop()['process']
    },

    fulfills: async (record: EconomicEvent): Promise<Fulfillment[]> => {
      return (await readFulfillments({ params: { fulfilledBy: record.id } })).map(({ fulfillment }) => fulfillment)
    },

    satisfies: async (record: EconomicEvent): Promise<Satisfaction[]> => {
      return (await readSatisfactions({ params: { satisfiedBy: record.id } })).map(({ satisfaction }) => satisfaction)
    },

    resourceConformsTo: async (record: EconomicEvent): Promise<ResourceSpecification> => {
      return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
    },

    action: async (record: EconomicEvent): Promise<Action> => {
      return (await readAction({ id: record.action }))
    },
  }
}
