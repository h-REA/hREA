/**
 * Top-level agreement queries
 *
 * @package: Holo-REA
 * @since:   2020-06-16
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Agreement,
} from '@valueflows/vf-graphql'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readRecord = mapZomeFn(dnaConfig, conductorUri, 'agreement', 'agreement', 'get_agreement')

  return {
    agreement: async (root, { id }): Promise<Agreement> => {
      return (await readRecord({ address: id })).agreement
    },
  }
}
