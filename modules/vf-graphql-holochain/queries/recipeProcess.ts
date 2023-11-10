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
  const readOne = mapZomeFn<ReadParams, RecipeProcessResponse>(dnaConfig, conductorUri, 'planning', 'recipe_process', 'get_recipe_process')
  const readAll = mapZomeFn<PagingParams, RecipeProcessConnection>(dnaConfig, conductorUri, 'planning', 'recipe_process_index', 'read_all_recipe_processs')

  return {
    recipeProcess: async (root, args): Promise<RecipeProcess> => {
      return (await readOne({ address: args.id })).recipeProcess
    },
    recipeProcesss: async (root, args: PagingParams): Promise<RecipeProcessConnection> => {
      return await readAll(args)
    },
  }
}
