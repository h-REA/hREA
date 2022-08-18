/**
 * Resolvers for Fulfillment fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, injectTypename, DEFAULT_VF_MODULES, VfModule, EconomicEventAddress, ByRevision, AddressableIdentifier } from '../types.js'
import { mapZomeFn, remapCellId } from '../connection.js'

import {
  Fulfillment,
  FulfillmentResponse,
  EconomicEvent,
  Commitment,
  EconomicEventConnection,
  CommitmentConnection,
} from '@valueflows/vf-graphql'
import { CommitmentSearchInput, EconomicEventSearchInput } from './zomeSearchInputTypes.js'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)

  const readRevision = mapZomeFn<ByRevision, FulfillmentResponse>(dnaConfig, conductorUri, 'planning', 'fulfillment', 'get_revision')
  const readEvents = mapZomeFn<EconomicEventSearchInput, EconomicEventConnection>(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'query_economic_events')
  const readCommitments = mapZomeFn<CommitmentSearchInput, CommitmentConnection>(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')

  return Object.assign(
    (hasCommitment ? {
      fulfills: injectTypename('Commitment', async (record: Fulfillment): Promise<Commitment> => {
        const results = await readCommitments({ params: { fulfilledBy: record.id } })
        return results.edges.pop()!['node']
      }),
    } : {}),
    (hasObservation ? {
      fulfilledBy: injectTypename('EconomicEvent', async (record: Fulfillment): Promise<EconomicEvent> => {
        const associatedId = remapCellId(record.id, record.fulfilledBy)
        const results = await readEvents({ params: { fulfills: associatedId } })
        return results.edges.pop()!['node']
      }),
    } : {}),
    (hasHistory ? {
      revision: async (record: Fulfillment, args: { revisionId: AddressableIdentifier }): Promise<Fulfillment> => {
        return (await readRevision(args)).fulfillment
      },
    } : {}),
  )
}
