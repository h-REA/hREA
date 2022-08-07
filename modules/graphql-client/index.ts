/**
 * GraphQL client interface for Holochain connection
 *
 * @package  Holo-REA GraphQL client
 * @since    2020-07-14
 */

import { InMemoryCache, ApolloClient } from '@apollo/client'
import { SchemaLink } from '@apollo/link-schema'

import bindSchema, {
  autoConnect,
  BindSchemaOptions,
  DNAIdMappings,
  ExtensionOptions,
  ResolverOptions
} from '@valueflows/vf-graphql-holochain'

/* For external client. Are optional because
   `conductorUri` and `appID`
   may be autodetected or taken from environment variables
   and `dnaConfig` discovered through a functional connection
   to holochain
*/
interface AutoConnectionOptions {
  dnaConfig?: DNAIdMappings
  conductorUri?: string
  appID?: string
}

export type ClientOptions = Pick<ResolverOptions, 'traceAppSignals'>
  & Pick<BindSchemaOptions, 'enabledVFModules'>
  & ExtensionOptions
  & AutoConnectionOptions

/*
  This method creates the GraphQLSchema from
  the given options, and then builds an Graphql Apollo Client
  with the basic InMemoryCache with that schema.
*/
export async function initGraphQLClient(options: BindSchemaOptions) {
  const schema = await bindSchema(options)

  return new ApolloClient({
    cache: new InMemoryCache(),
    link: new SchemaLink({ schema })
  })
}

/*
  This is a zero-config option method (and the main export)
  where you can generate a GraphQL Apollo Client
  and have all configuration handled internally for you.
  It is intended for the less advanced/customized use cases,
  but still offers basic levels of configuration.
*/
async function connect(options: ClientOptions = {}) {
  let bindSchemaOptions: BindSchemaOptions
  // autodetect `CellId`s if no explicit `dnaConfig` is provided
  // also autodetect `conductorUri` since the caller can leave it undefined
  // in the options object (and pass it instead from either 
  // environment variables or introspected from the Holochain Launcher)
  if (!options.dnaConfig || !options.conductorUri) {
    let { dnaConfig, conductorUri } = await autoConnect(
      options.conductorUri,
      options.appID,
      // traceAppSignals needs to be passed during
      // the first initialization of the AppWebsocket
      options.traceAppSignals
    )
    bindSchemaOptions = {
      ...options,
      dnaConfig,
      conductorUri
    }
  } else {
    // the difference between BindSchemaOptions and ClientOptions
    // is that for BindSchemaOptions `dnaConfig` and `conductorUri` must
    // be defined, and in this logical branch
    // we've established that, making this a safe
    // type cast
    bindSchemaOptions = {
      ...options as BindSchemaOptions
    }
  }

  return await initGraphQLClient(bindSchemaOptions)
}

export default connect
