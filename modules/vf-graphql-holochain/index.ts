/**
 * HoloREA GraphQL schema interface
 *
 * A GraphQL schema (suitable for use with `apollo-link-schema`) which defines
 * bindings between the ValueFlows protocol and a Holochain backend.
 *
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { makeExecutableSchema } from '@graphql-tools/schema'

import { APIOptions, ResolverOptions, DEFAULT_VF_MODULES, DNAIdMappings, CellId, VfModule } from './types'
import generateResolvers from './resolvers'
import { mapZomeFn, autoConnect, openConnection, sniffHolochainAppCells } from './connection'
const { buildSchema, printSchema } = require('@valueflows/vf-graphql')

export {
  // direct access to resolver callbacks generator for apps that need to bind to other GraphQL schemas
  generateResolvers,
  // connection handling methods
  autoConnect, openConnection, sniffHolochainAppCells,
  // direct access to Holochain zome method bindings for authoring own custom resolvers bound to non-REA DNAs
  mapZomeFn,
  // types that wrapper libraries may need to manage conductor DNA connection logic
  DNAIdMappings, CellId, APIOptions, VfModule
}

/**
 * A schema generator ready to be plugged in to a GraphQL client
 *
 * Required options: conductorUri, dnaConfig
 *
 * @return GraphQLSchema
 */
export default async (options: APIOptions) => {
  const {
    // use full supported set of modules by default
    enabledVFModules = DEFAULT_VF_MODULES,
    extensionSchemas = [],
    extensionResolvers = {},
  } = options

  const coreResolvers = await generateResolvers(options as ResolverOptions)
  const resolvers = {
    ...coreResolvers,
    ...extensionResolvers,
    Query: {
      ...(coreResolvers.Query || {}),
      ...(extensionResolvers.Query || {})
    },
    Mutation: {
      ...(coreResolvers.Mutation || {}),
      ...(extensionResolvers.Mutation || {})
    }
  }

  return makeExecutableSchema({
    typeDefs: printSchema(buildSchema(enabledVFModules, extensionSchemas)),
    resolvers,
  })
}
