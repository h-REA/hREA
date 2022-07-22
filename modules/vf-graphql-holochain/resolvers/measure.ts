/**
 * Resolver callbacks for measurement struct sub-fields
 *
 * @package: HoloREA
 * @since:   2019-12-24
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ById, AddressableIdentifier } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Maybe,
  Measure,
  Unit,
  UnitResponse,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readUnit = mapZomeFn<ById, UnitResponse>(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')

  return {
    hasUnit: async (record: { hasUnit: AddressableIdentifier }): Promise<Maybe<Unit>> => {
      if (!record.hasUnit) {
        return null
      }
      return (await readUnit({ id: record.hasUnit })).unit
    },
  }
}
