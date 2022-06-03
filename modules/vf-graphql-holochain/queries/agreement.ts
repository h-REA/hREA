/**
 * Top-level agreement queries
 *
 * @package: Holo-REA
 * @since:   2020-06-16
 */

import { DNAIdMappings, ReadParams } from '../types'
import { mapZomeFn } from '../connection'

import {
  Agreement, AgreementResponse,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readRecord = mapZomeFn<ReadParams, AgreementResponse>(dnaConfig, conductorUri, 'agreement', 'agreement', 'get_agreement')

  return {
    // why does agreement query not return AgreementResponse like other queries?
    agreement: async (root, args): Promise<Agreement> => {
      return (await readRecord(args)).agreement
    },
  }
}
