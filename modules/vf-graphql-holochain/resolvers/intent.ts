/**
 * Intent record reference resolvers
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { zomeFunction } from '../connection'

import {
  Intent,
  Satisfaction,
  Process,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readSatisfactions = zomeFunction('planning', 'satisfaction', 'query_satisfactions')
const readProcesses = zomeFunction('observation', 'process', 'query_processes')

export const inputOf = async (record: Intent): Promise<[Process]> => {
  return (await readProcesses({ params: { intendedInputs: record.id } })).pop()['process']
}

export const outputOf = async (record: Intent): Promise<[Process]> => {
  return (await readProcesses({ params: { intendedOutputs: record.id } })).pop()['process']
}

export const satisfiedBy = async (record: Intent): Promise<[Satisfaction]> => {
  return (await readSatisfactions({ params: { satisfies: record.id } })).map(({ satisfaction }) => satisfaction)
}
