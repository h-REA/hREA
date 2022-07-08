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

import { APIOptions, ResolverOptions, DEFAULT_VF_MODULES, DNAIdMappings, CellId, VfModule } from './types.js'
import generateResolvers from './resolvers/index.js'
import * as hreaExtensionSchemas from './schema-extensions.js'
import { mapZomeFn, autoConnect, openConnection, sniffHolochainAppCells, remapCellId } from './connection.js'
// @ts-ignore
import { buildSchema, printSchema } from '@valueflows/vf-graphql'

export {
  // direct access to resolver callbacks generator for apps that need to bind to other GraphQL schemas
  generateResolvers,
  // schema extensions
  hreaExtensionSchemas,
  // connection handling methods
  autoConnect, openConnection, sniffHolochainAppCells,
  // direct access to Holochain zome method bindings for authoring own custom resolvers bound to non-REA DNAs
  mapZomeFn,
  // types that wrapper libraries may need to manage conductor DNA connection logic
  DNAIdMappings, CellId, APIOptions, VfModule, DEFAULT_VF_MODULES,

  // :TODO: remove this. After #266 clients should not need to think about differing IDs between Cells.
  remapCellId,
}

/**
 * A schema generator ready to be plugged in to a GraphQL client
 *
 * Required options: conductorUri, dnaConfig
 *
 * @return GraphQLSchema
 */
const bindSchema = async (options: APIOptions) => {
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

  // extend the base vf-graphql schema with one
  // or more holochain specific schema extensions.
  // add more here if more are added.
  const overriddenExtensionSchemas = [...extensionSchemas, hreaExtensionSchemas.associateMyAgentExtension]

  return makeExecutableSchema({
    typeDefs: printSchema(buildSchema(enabledVFModules, overriddenExtensionSchemas)),
    resolvers,
  })
}

export default bindSchema
