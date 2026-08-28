import { makeExecutableSchema } from '@graphql-tools/schema';
import { type GraphQLSchema } from 'graphql';
import { generateResolvers } from './src/resolvers/index.js';
// @ts-ignore
import { buildSchema, printSchema } from '@valueflows/vf-graphql'

type hREASchemaParams = {
  appWebSocket: any;
  roleName: string;
  cell?: any; // If you want to pass a cell instead of appWebSocket and roleName
}

export function createHolochainSchema(params: hREASchemaParams): GraphQLSchema {
  const cell = {
    callZome: async function(args) {
      return params.appWebSocket.callZome({
        cap_secret: null,
        role_name: params.roleName,
        zome_name: args.zome_name,
        fn_name: args.fn_name,
        payload: args.payload,
      }, 999999);
    }
  }

  const resolvers = generateResolvers(params.cell || cell)
  const enabledVFModules = [
    'util',
    'pagination',
    'history',
    'agent',
    'action',
    'plan',
    'util',
    'commitment',
    'proposal',
    'recipe',
    'process',
    'measurement',
    'observation',
    'process_specification',
    'resource_specification',
    'agreement',
    'intent',
  ]

  const overriddenExtensionSchemas = []
  const executableSchema = makeExecutableSchema({
    typeDefs: printSchema(buildSchema(enabledVFModules, overriddenExtensionSchemas)),
    resolvers,
  });

  return executableSchema
}
