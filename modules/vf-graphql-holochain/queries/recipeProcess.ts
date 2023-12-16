/**
 * Top-level queries relating to RecipeProcess
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  RecipeProcess, RecipeProcessConnection, RecipeProcessResponse,
} from '@valueflows/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, RecipeProcessResponse>(dnaConfig, conductorUri, 'recipe', 'recipe_process', 'get_recipe_process')
  const readAll = mapZomeFn<PagingParams, RecipeProcessConnection>(dnaConfig, conductorUri, 'recipe', 'recipe_process_index', 'read_all_recipe_processes')

  return {
    recipeProcess: async (root, args): Promise<RecipeProcess> => {
      console.log("recipeProcess", args)
      return (await readOne({ address: args.id })).recipeProcess
    },
    recipeProcesses: async (root, args: PagingParams): Promise<RecipeProcessConnection> => {
      console.log("recipeProcesses", args)
      return await readAll(args)
    },
  }
}
