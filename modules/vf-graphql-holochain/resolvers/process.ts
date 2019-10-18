/**
 * Resolvers for Process fields
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'

import {
  Process,
  EconomicEvent,
  Commitment,
  Intent,
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readEvents = zomeFunction('observation', 'economic_event', 'query_events')
const readCommitments = zomeFunction('planning', 'commitment', 'query_commitments')
const readIntents = zomeFunction('planning', 'intent', 'query_intents')

export const inputs = async (record: Process): Promise<[EconomicEvent]> => {
  return (await (await readEvents)({ params: { inputOf: record.id } })).map(({ economicEvent }) => economicEvent)
}

export const outputs = async (record: Process): Promise<[EconomicEvent]> => {
  return (await (await readEvents)({ params: { outputOf: record.id } })).map(({ economicEvent }) => economicEvent)
}

export const committedInputs = async (record: Process): Promise<[Commitment]> => {
  return (await (await readCommitments)({ params: { inputOf: record.id } })).map(({ commitment }) => commitment)
}

export const committedOutputs = async (record: Process): Promise<[Commitment]> => {
  return (await (await readCommitments)({ params: { outputOf: record.id } })).map(({ commitment }) => commitment)
}

export const intendedInputs = async (record: Process): Promise<[Intent]> => {
  return (await (await readIntents)({ params: { inputOf: record.id } })).map(({ intent }) => intent)
}

export const intendedOutputs = async (record: Process): Promise<[Intent]> => {
  return (await (await readIntents)({ params: { outputOf: record.id } })).map(({ intent }) => intent)
}
