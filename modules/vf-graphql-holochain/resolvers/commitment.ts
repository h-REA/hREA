/**
 * Resolvers for Commitment fields
 *
 * @package: HoloREA
 * @since:   2019-08-28
 */

import { zomeFunction } from '../connection'

import {
  Commitment,
  Fulfillment,
  Satisfaction,
  Process,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readFulfillments = zomeFunction('planning', 'fulfillment', 'query_fulfillments')
const readSatisfactions = zomeFunction('planning', 'satisfaction', 'query_satisfactions')
const readProcesses = zomeFunction('observation', 'process', 'query_processes')

export const inputOf = async (record: Commitment): Promise<[Process]> => {
  return (await readProcesses({ params: { committedInputs: record.id } })).pop()['process']
}

export const outputOf = async (record: Commitment): Promise<[Process]> => {
  return (await readProcesses({ params: { committedOutputs: record.id } })).pop()['process']
}

export const fulfilledBy = async (record: Commitment): Promise<[Fulfillment]> => {
  return (await readFulfillments({ params: { fulfills: record.id } })).map(({ fulfillment }) => fulfillment)
}

export const satisfies = async (record: Commitment): Promise<[Satisfaction]> => {
  return (await readSatisfactions({ params: { satisfiedBy: record.id } })).map(({ satisfaction }) => satisfaction)
}
