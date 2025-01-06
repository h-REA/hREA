/**
 * Top-level queries relating to RecipeExchange
 *
 * @package: HoloREA
 * @since:   2019-09-12
 */

import { DNAIdMappings, ReadParams } from '../types.js'
import { mapZomeFn } from '../connection.js'

import {
  RecipeExchange, RecipeExchangeConnection, RecipeExchangeResponse,
} from '@leosprograms/vf-graphql'
import { PagingParams } from '../resolvers/zomeSearchInputTypes.js'

export default (dnaConfig: DNAIdMappings, conductorUri: string) => {
  const readOne = mapZomeFn<ReadParams, RecipeExchangeResponse>(dnaConfig, conductorUri, 'combined', 'recipe_exchange', 'get_recipe_exchange')
  const readAll = mapZomeFn<PagingParams, RecipeExchangeConnection>(dnaConfig, conductorUri, 'combined', 'indexing', 'read_all_recipe_exchanges')

  return {
    recipeExchange: async (root, args): Promise<RecipeExchange> => {
      console.log("recipeExchange query", args)
      const x = await readOne({ address: args.id })
      console.log("recipeExchange query result", x)
      return (await readOne({ address: args.id })).recipeExchange

    },
    recipeExchanges: async (root, args: PagingParams): Promise<RecipeExchangeConnection> => {
      return await readAll(args)
    },
  }
}
