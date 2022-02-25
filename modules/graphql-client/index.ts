/**
 * GraphQL client interface for Holochain connection
 *
 * :TODO: sniff active DNA configuration from conductor
 *
 * @package  Holo-REA GraphQL client
 * @since    2020-07-14
 */

import { InMemoryCache, ApolloClient } from '@apollo/client'
import { SchemaLink } from '@apollo/link-schema'

import bindSchema, { autoConnect, APIOptions, DNAIdMappings } from '@valueflows/vf-graphql-holochain'

// Same as OpenConnectionOptions but for external client where dnaConfig may be autodetected
interface AutoConnectionOptions {
  dnaConfig?: DNAIdMappings,
}

export type ClientOptions = APIOptions & AutoConnectionOptions

export async function initGraphQLClient(options: APIOptions) {
  const schema = await bindSchema(options/* modules, DNA id bindings */)

  return new ApolloClient({
    cache: new InMemoryCache(),
    link: new SchemaLink({ schema })
  });
}

async function connect(options: ClientOptions) {
  // autodetect `CellId`s if no explicit `dnaConfig` is provided
  if (!options.dnaConfig) {
    let { dnaConfig } = await autoConnect(options.conductorUri)
    options.dnaConfig = dnaConfig
  }

  return await initGraphQLClient(options)
}

export default connect;
