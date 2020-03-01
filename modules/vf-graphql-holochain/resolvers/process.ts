/**
 * Resolvers for Process fields
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { zomeFunction } from '../connection'
import { injectTypename } from '../types'

import {
  Process,
  EconomicEvent,
  Commitment,
  Intent,
  ProcessSpecification
} from '@valueflows/vf-graphql'

// :TODO: how to inject DNA identifier?
const readEvents = zomeFunction('observation', 'economic_event', 'query_events')
const readCommitments = zomeFunction('planning', 'commitment', 'query_commitments')
const readIntents = zomeFunction('planning', 'intent', 'query_intents')
const readProcessBasedOn = zomeFunction('specification', 'process_specification', 'get_process_specification')

export const inputs = injectTypename('EconomicEvent', async (record: Process): Promise<EconomicEvent[]> => {
  return (await readEvents({ params: { inputOf: record.id } })).map(({ economicEvent }) => economicEvent)
})

export const outputs = injectTypename('EconomicEvent', async (record: Process): Promise<EconomicEvent[]> => {
  return (await readEvents({ params: { outputOf: record.id } })).map(({ economicEvent }) => economicEvent)
})

export const committedInputs = injectTypename('Commitment', async (record: Process): Promise<Commitment[]> => {
  return (await readCommitments({ params: { inputOf: record.id } })).map(({ commitment }) => commitment)
})

export const committedOutputs = injectTypename('Commitment', async (record: Process): Promise<Commitment[]> => {
  return (await readCommitments({ params: { outputOf: record.id } })).map(({ commitment }) => commitment)
})

export const intendedInputs = async (record: Process): Promise<Intent[]> => {
  return (await readIntents({ params: { inputOf: record.id } })).map(({ intent }) => intent)
}

export const intendedOutputs = async (record: Process): Promise<Intent[]> => {
  return (await readIntents({ params: { outputOf: record.id } })).map(({ intent }) => intent)
}

export const basedOn = async (record: Process): Promise<ProcessSpecification> => {
  return (await readProcessBasedOn({ address: record.basedOn })).processSpecification
}
