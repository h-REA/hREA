/**
 * Intent record reference resolvers
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { zomeFunction } from '../connection'

import {
  Maybe,
  Agent,
  Intent,
  Satisfaction,
  Process,
  ResourceSpecification,
  ProposedIntent,
  Action,
} from '@valueflows/vf-graphql'

import { agent as loadAgent } from '../queries/agent'

// :TODO: how to inject DNA identifier?
const readSatisfactions = zomeFunction('planning', 'satisfaction', 'query_satisfactions')
const readProcesses = zomeFunction('observation', 'process', 'query_processes')
const readProposedIntent = zomeFunction('proposal', 'proposed_intent', 'get_proposed_intent')
const readResourceSpecification = zomeFunction('specification', 'resource_specification', 'get_resource_specification')
const readAction = zomeFunction('specification', 'action', 'get_action')

export const provider = async (record: Intent): Promise<Maybe<Agent>> => {
  return record.provider ? loadAgent(record, { id: record.provider }) : null
}

export const receiver = async (record: Intent): Promise<Maybe<Agent>> => {
  return record.receiver ? loadAgent(record, { id: record.receiver }) : null
}

export const inputOf = async (record: Intent): Promise<Process[]> => {
  return (await readProcesses({ params: { intendedInputs: record.id } })).pop()['process']
}

export const outputOf = async (record: Intent): Promise<Process[]> => {
  return (await readProcesses({ params: { intendedOutputs: record.id } })).pop()['process']
}

export const satisfiedBy = async (record: Intent): Promise<Satisfaction[]> => {
  return (await readSatisfactions({ params: { satisfies: record.id } })).map(({ satisfaction }) => satisfaction)
}

const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

export const publishedIn = async (record: Intent): Promise<ProposedIntent[]> => {
  return (await Promise.all((record.publishedIn || []).map((address)=>readProposedIntent({address})))).map(extractProposedIntent)
}

export const resourceConformsTo = async (record: Intent): Promise<ResourceSpecification> => {
  return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
}

export const action = async (record: Intent): Promise<Action> => {
  return (await readAction({ id: record.action }))
}
