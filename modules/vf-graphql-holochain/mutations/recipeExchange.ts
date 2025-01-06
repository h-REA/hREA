/**
 * Recipe Process mutations
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { ByRevision, DNAIdMappings } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './index.js'

import {
  RecipeExchangeCreateParams,
  RecipeExchangeUpdateParams,
  RecipeExchangeResponse,
} from '@leosprograms/vf-graphql'

export interface CreateArgs {
  recipeExchange: RecipeExchangeCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<RecipeExchangeResponse>

export interface UpdateArgs {
  recipeExchange: RecipeExchangeUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<RecipeExchangeResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, RecipeExchangeResponse>(dnaConfig, conductorUri, 'combined', 'recipe_exchange', 'create_recipe_exchange')
  const runUpdate = mapZomeFn<UpdateArgs, RecipeExchangeResponse>(dnaConfig, conductorUri, 'combined', 'recipe_exchange', 'update_recipe_exchange')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'combined', 'recipe_exchange', 'delete_recipe_exchange')

  const createRecipeExchange: createHandler = async (root, args) => {
    console.log("createRecipeExchange **", args)
    const x = await runCreate(args)
    console.log("createRecipeExchange **", x)
    return x
  }

  const updateRecipeExchange: updateHandler = async (root, args) => {
    return await runUpdate(args)
  }

  const deleteRecipeExchange: deleteHandler = async (root, args) => {
    return await runDelete(args)
  }

  return {
    createRecipeExchange,
    updateRecipeExchange,
    deleteRecipeExchange,
  }
}
