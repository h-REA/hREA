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

import bindSchema from '@valueflows/vf-graphql-holochain'

async function initGraphQLClient(options) {
  const schema = await bindSchema(options/* modules, DNA id bindings */)

  return new ApolloClient({
    cache: new InMemoryCache(),
    link: new SchemaLink({ schema })
  });
}

export default initGraphQLClient;
