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
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readFulfillments = zomeFunction('observation', 'fulfillment', 'query_fulfillments')
const readSatisfactions = zomeFunction('planning', 'satisfaction', 'query_satisfactions')
const readProcesses = zomeFunction('observation', 'process', 'query_processes')

export const inputOf = async (record: EconomicEvent): Promise<[Process]> => {
  return (await readProcesses({ params: { inputs: record.id } })).pop()['process']
}

export const outputOf = async (record: EconomicEvent): Promise<[Process]> => {
  return (await readProcesses({ params: { outputs: record.id } })).pop()['process']
}

export const fulfills = async (record: EconomicEvent): Promise<[Fulfillment]> => {
  return (await (await readFulfillments)({ economic_event: record.id })).map(({ fulfillment }) => fulfillment)
}

export const satisfies = async (record: EconomicEvent): Promise<[Satisfaction]> => {
  return (await (await readSatisfactions)({ satisfied_by: record.id })).map(({ satisfaction }) => satisfaction)
}
