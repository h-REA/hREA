/**
 * Resolver callbacks for measurement struct sub-fields
 *
 * @package: HoloREA
 * @since:   2019-12-24
 */

import { DNAIdMappings } from '../types'
import { mapZomeFn } from '../connection'

import {
  Measure,
  Unit,
} from '@valueflows/vf-graphql'

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  const readUnit = mapZomeFn(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')

  return {
    hasUnit: async (record: Measure): Promise<Unit> => {
      return (await readUnit({ id: record.hasUnit })).unit
    },
  }
}
