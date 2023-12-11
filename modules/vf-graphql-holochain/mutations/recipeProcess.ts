/**
 * Recipe Process mutations
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { ByRevision, DNAIdMappings } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './'

import {
  RecipeProcessCreateParams,
  RecipeProcessUpdateParams,
  RecipeProcessResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  recipeProcess: RecipeProcessCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<RecipeProcessResponse>

export interface UpdateArgs {
  recipeProcess: RecipeProcessUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<RecipeProcessResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, RecipeProcessResponse>(dnaConfig, conductorUri, 'recipe', 'recipe_process', 'create_recipe_process')
  const runUpdate = mapZomeFn<UpdateArgs, RecipeProcessResponse>(dnaConfig, conductorUri, 'recipe', 'recipe_process', 'update_recipe_process')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'recipe', 'recipe_process', 'delete_recipe_process')

  const createRecipeProcess: createHandler = async (root, args) => {
    return runCreate(args)
  }

  const updateRecipeProcess: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteRecipeProcess: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createRecipeProcess,
    updateRecipeProcess,
    deleteRecipeProcess,
  }
}
