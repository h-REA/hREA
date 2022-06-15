/**
 * Top-level agreement queries
 *
 * @package: Holo-REA
 * @since:   2020-06-16
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Agreement, AgreementResponse,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readRecord = mapZomeFn<ReadParams, AgreementResponse>(dnaConfig, conductorUri, 'agreement', 'agreement', 'get_agreement')

  return {
    agreement: async (root, args): Promise<Agreement> => {
      return (await readRecord({ address: args.id })).agreement
    },
  }
}
