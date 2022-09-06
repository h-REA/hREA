/**
 * Top-level agreement queries
 *
 * @package: hREA
 * @since:   2020-06-16
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Agreement, AgreementConnection, AgreementResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readRecord = mapZomeFn<ReadParams, AgreementResponse>(dnaConfig, conductorUri, 'agreement', 'agreement', 'get_agreement')
  const readAll = mapZomeFn<PagingParams, AgreementConnection>(dnaConfig, conductorUri, 'agreement', 'agreement_index', 'read_all_agreements')

  return {
    agreement: async (root, args): Promise<Agreement> => {
      return (await readRecord({ address: args.id })).agreement
    },
    agreements: async (root, args: PagingParams): Promise<AgreementConnection> => {
      return await readAll(args)
    },
  }
}
