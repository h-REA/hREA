/**
 * Resolvers for EconomicEvent fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { zomeFunction } from '../connection'

import {
  EconomicEvent,
  Fulfillment,
  Satisfaction,
  Process,
  ResourceSpecification,
  Action
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readFulfillments = zomeFunction('observation', 'fulfillment', 'query_fulfillments')
const readSatisfactions = zomeFunction('observation', 'satisfaction', 'query_satisfactions')
const readProcesses = zomeFunction('observation', 'process', 'query_processes')
const readAction = zomeFunction('specification', 'action', 'get_action')
const readResourceSpecification = zomeFunction('specification', 'resource_specification', 'get_resource_specification')

export const inputOf = async (record: EconomicEvent): Promise<Process[]> => {
  return (await readProcesses({ params: { inputs: record.id } })).pop()['process']
}

export const outputOf = async (record: EconomicEvent): Promise<Process[]> => {
  return (await readProcesses({ params: { outputs: record.id } })).pop()['process']
}

export const fulfills = async (record: EconomicEvent): Promise<Fulfillment[]> => {
  return (await readFulfillments({ params: { fulfilledBy: record.id } })).map(({ fulfillment }) => fulfillment)
}

export const satisfies = async (record: EconomicEvent): Promise<Satisfaction[]> => {
  return (await readSatisfactions({ params: { satisfiedBy: record.id } })).map(({ satisfaction }) => satisfaction)
}

export const resourceConformsTo = async (record: EconomicEvent): Promise<ResourceSpecification> => {
  return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
}

export const action = async (record: EconomicEvent): Promise<Action> => {
  return (await readAction({ id: record.action }))
}
