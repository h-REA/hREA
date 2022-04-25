/**
 * Intent record reference resolvers
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'
import { mapZomeFn } from '../connection'

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

import agentQueries from '../queries/agent'

const extractProposedIntent = (data): ProposedIntent => data.proposedIntent

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasKnowledge = -1 !== enabledVFModules.indexOf(VfModule.Knowledge)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasProposal = -1 !== enabledVFModules.indexOf(VfModule.Proposal)

  const readSatisfactions = mapZomeFn(dnaConfig, conductorUri, 'planning', 'satisfaction_index', 'query_satisfactions')
  const readProcesses = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const readProposedIntent = mapZomeFn(dnaConfig, conductorUri, 'proposal', 'proposed_intent', 'get_proposed_intent')
  const readResourceSpecification = mapZomeFn(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readAction = mapZomeFn(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']

  return Object.assign(
    {
      satisfiedBy: async (record: Intent): Promise<Satisfaction[]> => {
        return (await readSatisfactions({ params: { satisfies: record.id } })).map(({ satisfaction }) => satisfaction)
      },
    },
    (hasAgent ? {
      provider: async (record: Intent): Promise<Maybe<Agent>> => {
        return record.provider ? readAgent(record, { id: record.provider }) : null
      },

      receiver: async (record: Intent): Promise<Maybe<Agent>> => {
        return record.receiver ? readAgent(record, { id: record.receiver }) : null
      },
    } : {}),
    (hasObservation ? {
      inputOf: async (record: Intent): Promise<Process[]> => {
        return (await readProcesses({ params: { intendedInputs: record.id } })).pop()['process']
      },

      outputOf: async (record: Intent): Promise<Process[]> => {
        return (await readProcesses({ params: { intendedOutputs: record.id } })).pop()['process']
      },
    } : {}),
    (hasProposal ? {
      publishedIn: async (record: Intent): Promise<ProposedIntent[]> => {
        return (await Promise.all((record.publishedIn || []).map((address)=>readProposedIntent({address})))).map(extractProposedIntent)
      },
    } : {}),
    (hasKnowledge ? {
      resourceConformsTo: async (record: Intent): Promise<ResourceSpecification> => {
        return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
      },

      action: async (record: Intent): Promise<Action> => {
        return (await readAction({ id: record.action }))
      },
    } : {}),
  )
}
