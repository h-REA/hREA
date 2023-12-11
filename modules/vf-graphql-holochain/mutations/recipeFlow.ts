/**
 * Recipe Flow mutations
 *
 * @package: HoloREA
 * @since:   2019-08-31
 */

import { ByRevision, DNAIdMappings } from '../types.js'
import { mapZomeFn } from '../connection.js'
import { deleteHandler } from './'

import {
  RecipeFlowCreateParams,
  RecipeFlowUpdateParams,
  RecipeFlowResponse,
} from '@valueflows/vf-graphql'

export interface CreateArgs {
  recipeFlow: RecipeFlowCreateParams,
}
export type createHandler = (root: any, args: CreateArgs) => Promise<RecipeFlowResponse>

export interface UpdateArgs {
  recipeFlow: RecipeFlowUpdateParams,
}
export type updateHandler = (root: any, args: UpdateArgs) => Promise<RecipeFlowResponse>

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const runCreate = mapZomeFn<CreateArgs, RecipeFlowResponse>(dnaConfig, conductorUri, 'recipe', 'recipe_flow', 'create_recipe_flow')
  const runUpdate = mapZomeFn<UpdateArgs, RecipeFlowResponse>(dnaConfig, conductorUri, 'recipe', 'recipe_flow', 'update_recipe_flow')
  const runDelete = mapZomeFn<ByRevision, boolean>(dnaConfig, conductorUri, 'recipe', 'recipe_flow', 'delete_recipe_flow')

  const createRecipeFlow: createHandler = async (root, args) => {
    console.log("=================CREATE A RECIPE FLOW===================")
    return runCreate(args)
  }

  const updateRecipeFlow: updateHandler = async (root, args) => {
    return runUpdate(args)
  }

  const deleteRecipeFlow: deleteHandler = async (root, args) => {
    return runDelete(args)
  }

  return {
    createRecipeFlow,
    updateRecipeFlow,
    deleteRecipeFlow,
  }
}
