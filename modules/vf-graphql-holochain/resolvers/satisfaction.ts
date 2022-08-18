/**
 * Resolvers for Satisfaction record relationships
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, addTypename, DEFAULT_VF_MODULES, VfModule, ByRevision, AddressableIdentifier } from '../types.js'
import { mapZomeFn, remapCellId } from '../connection.js'

import {
  Satisfaction,
  SatisfactionResponse,
  EventOrCommitment,
  Intent,
  EconomicEventConnection,
  CommitmentConnection,
  IntentConnection,
} from '@valueflows/vf-graphql'
import { CommitmentSearchInput, EconomicEventSearchInput, IntentSearchInput } from './zomeSearchInputTypes.js'

async function extractRecordsOrFail (query): Promise<any> {
  const val = await query
  if (!val || !val.edges || !val.edges.length || !val.edges[0].node) {
    throw new Error('Reference not found')
  }
  return val.edges[0].node
}

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)
  const hasIntent = -1 !== enabledVFModules.indexOf(VfModule.Intent)

  const readRevision = mapZomeFn<ByRevision, SatisfactionResponse>(dnaConfig, conductorUri, 'planning', 'satisfaction', 'get_revision')
  const readEvents = mapZomeFn<EconomicEventSearchInput, EconomicEventConnection>(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'query_economic_events')
  const readCommitments = mapZomeFn<CommitmentSearchInput, CommitmentConnection>(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')
  const readIntents = mapZomeFn<IntentSearchInput, IntentConnection>(dnaConfig, conductorUri, 'planning', 'intent_index', 'query_intents')

  return Object.assign(
    (hasObservation || hasCommitment ? {
      satisfiedBy: async (record: Satisfaction): Promise<EventOrCommitment> => {
        const associatedId = remapCellId(record.id, record.satisfiedBy)
        // :NOTE: this presumes a satisfaction will never be erroneously linked to 2 records
        return (
          await Promise.all((hasCommitment ? [
            extractRecordsOrFail(readCommitments({ params: { satisfies: associatedId } }))
              .then(addTypename('Commitment'))
              .catch((e) => e),
          ] : []).concat(hasObservation ? [
            extractRecordsOrFail(readEvents({ params: { satisfies: associatedId } }))
              .then(addTypename('EconomicEvent'))
              .catch((e) => e),
          ] : []))
        )
        .filter(r => !(r instanceof Error))
        .pop()
      }
    } : {}),
    (hasIntent ? {
      satisfies: async (record: Satisfaction): Promise<Intent> => {
        const results = await readIntents({ params: { satisfiedBy: record.id } })
        return results.edges.pop()!['node']
      }
    } : {}),
    (hasHistory ? {
      revision: async (record: Satisfaction, args: { revisionId: AddressableIdentifier }): Promise<Satisfaction> => {
        return (await readRevision(args)).satisfaction
      },
    } : {}),
  )
}
