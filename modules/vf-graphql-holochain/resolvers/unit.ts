/**
 * Resolver callbacks for Unit
 *
 * @package: hREA
 * @since:   2022-08-18
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ById, ByRevision, AddressableIdentifier } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  Unit,
  UnitResponse,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)

  const readRevision = mapZomeFn<ByRevision, UnitResponse>(dnaConfig, conductorUri, 'specification', 'unit', 'get_revision')

  return Object.assign({},
    (hasHistory ? {
      revision: async (record: Unit, args: { revisionId: AddressableIdentifier }): Promise<Unit> => {
        return (await readRevision(args)).unit
      },
    } : {}),
  )
}
