/**
 * Resolver callbacks for measurement struct sub-fields
 *
 * @package: HoloREA
 * @since:   2019-12-24
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'
import { mapZomeFn } from '../connection'

import {
  Maybe,
  Measure,
  Unit,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readUnit = mapZomeFn(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')

  return {
    hasUnit: async (record: Measure): Promise<Maybe<Unit>> => {
      if (!record.hasUnit) {
        return null
      }
      return (await readUnit({ id: record.hasUnit })).unit
    },
  }
}
