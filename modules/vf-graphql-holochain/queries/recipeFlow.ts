/**
 * Top-level queries relating to RecipeFlow
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  RecipeFlow, RecipeFlowConnection, RecipeFlowResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, RecipeFlowResponse>(dnaConfig, conductorUri, 'recipe', 'recipe_flow', 'get_recipe_flow')
  const readAll = mapZomeFn<PagingParams, RecipeFlowConnection>(dnaConfig, conductorUri, 'recipe', 'recipe_flow_index', 'read_all_recipe_flows')

  return {
    recipeFlow: async (root, args): Promise<RecipeFlow> => {
      console.log("recipeFlow", args)
      return (await readOne({ address: args.id })).recipeFlow
    },
    recipeFlows: async (root, args: PagingParams): Promise<RecipeFlowConnection> => {
      return await readAll(args)
    },
  }
}
