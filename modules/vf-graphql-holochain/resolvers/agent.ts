/**
 * Resolvers for agent fields
 *
 * @package: Holo-REA
 * @since:   2020-05-28
 */

import { DNAIdMappings } from '../types'

export default (dnaConfig?: DNAIdMappings, conductorUri?: string) => {
  return {
    __resolveType: (obj, ctx, info) => obj.__typename
  }
}
