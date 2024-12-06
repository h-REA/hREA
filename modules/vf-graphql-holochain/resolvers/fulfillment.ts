/**
 * Resolvers for Fulfillment fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */
// @ts-nocheck

import { DNAIdMappings, injectTypename, DEFAULT_VF_MODULES, VfModule, EconomicEventAddress, ByRevision, AddressableIdentifier, ReadParams } from '../types.js'
import { mapZomeFn, remapCellId } from '../connection.js'

import {
  Fulfillment,
  FulfillmentResponse,
  EconomicEvent,
  Commitment,
  EconomicEventConnection,
  CommitmentConnection,
  CommitmentResponse,
  EconomicEventResponse,
} from '@leosprograms/vf-graphql'
import { CommitmentSearchInput, EconomicEventSearchInput } from './zomeSearchInputTypes.js'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasCommitment = -1 !== enabledVFModules.indexOf(VfModule.Commitment)

  const readRevision = mapZomeFn<ByRevision, FulfillmentResponse>(dnaConfig, conductorUri, 'combined', 'fulfillment', 'get_revision')
  const readEvents = mapZomeFn<EconomicEventSearchInput, EconomicEventConnection>(dnaConfig, conductorUri, 'combined', 'indexing', 'query_economic_events')
  const readCommitments = mapZomeFn<CommitmentSearchInput, CommitmentConnection>(dnaConfig, conductorUri, 'combined', 'indexing', 'query_commitments')
  const readOneCommitment = mapZomeFn<ReadParams, CommitmentResponse>(dnaConfig, conductorUri, 'combined', 'commitment', 'get_commitment')
  const readOneEconomicEvent = mapZomeFn<ReadParams, EconomicEventResponse>(dnaConfig, conductorUri, 'combined', 'economic_event', 'get_economic_event')

  return Object.assign(
    (hasCommitment ? {
      fulfills: injectTypename('Commitment', async (record: Fulfillment): Promise<Commitment> => {
        const results = await readOneCommitment({ address: record.fulfills })
        return results.commitment
      }),
    } : {}),
    (hasObservation ? {
      fulfilledBy: injectTypename('EconomicEvent', async (record: Fulfillment): Promise<EconomicEvent> => {
        const results = await readOneEconomicEvent({ address: record.fulfilledBy })
        return results.economicEvent
      }),
    } : {}),
    (hasHistory ? {
      revision: async (record: Fulfillment, args: { revisionId: AddressableIdentifier }): Promise<Fulfillment> => {
        return (await readRevision(args)).fulfillment
      },
    } : {}),
  )
}
