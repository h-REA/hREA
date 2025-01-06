/**
 * Recipe Process record reference resolvers
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ReadParams, ById, ByRevision, ProcessSpecificationAddress, AddressableIdentifier, RecipeFlowAddress } from '../types.js'
import { extractEdges, mapZomeFn } from '../connection.js'

import {
  RecipeExchange,
  RecipeExchangeResponse,
  Action,
  ProcessSpecification,
  ProcessSpecificationResponse,
  RecipeFlow,
  RecipeFlowResponse,
  RecipeFlowConnection,
} from '@leosprograms/vf-graphql'

import { RecipeFlowSearchInput } from './zomeSearchInputTypes.js'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasHistory = -1 !== enabledVFModules.indexOf(VfModule.History)
  const hasProcessSpecification = -1 !== enabledVFModules.indexOf(VfModule.ProcessSpecification)
  
  const readRevision = mapZomeFn<ByRevision, RecipeExchangeResponse>(dnaConfig, conductorUri, 'combined', 'recipe_exchange', 'get_revision')
  const readRecipeFlows = mapZomeFn<RecipeFlowSearchInput, RecipeFlowConnection>(dnaConfig, conductorUri, 'combined', 'indexing', 'query_recipe_flows')

  return Object.assign(
    {
      recipeClauses: async (record: RecipeExchange): Promise<RecipeFlow[]> => {
        const recipeFlows = await readRecipeFlows({ params: { recipeClauseOf: record.id } })
        return extractEdges(recipeFlows)
      },
      recipeReciprocalClauses: async (record: RecipeExchange): Promise<RecipeFlow[]> => {
        const recipeFlows = await readRecipeFlows({ params: { recipeReciprocalClauseOf: record.id } })
        return extractEdges(recipeFlows)
      }
    },    
    (hasHistory ? {
      revision: async (record: RecipeExchange, args: { revisionId: AddressableIdentifier }): Promise<RecipeExchange> => {
        return (await readRevision(args)).recipeExchange
      },
    } : {}),
  )
}
