/**
 * base types for GraphQL query layer
 *
 * @see https://github.com/valueflows/vf-graphql/blob/master/schemas/structs.gql
 *
 * @package: HoloREA prototype
 * @since:   2019-01-03
 * @package: HoloREA
 * @since:   2019-05-20
 */

import { IResolvers } from '@graphql-tools/utils'
import { GraphQLScalarType } from 'graphql'
import { parse } from 'fecha'
import { Kind } from 'graphql/language'

// Configuration object to allow specifying custom conductor DNA IDs to bind to.
// Default is to use a DNA with the same ID as the mapping ID (ie. agent = "agent")

interface DNAMappings {
  agent: string,
  observation: string,
  planning: string,
  proposal: string,
  specification: string,
}

export type DNAIdMappings = DNAMappings | undefined



// Options for resolver generator
export interface ResolverOptions {
  // Array of ValueFlows module names to include in the schema
  // @see https://github.com/valueflows/vf-graphql/#generating-schemas
  enabledVFModules?: string[],

  // Mapping of DNA identifiers to runtime instance names to bind to.
  // If not specified, DNAs are bound to the default instance names which are the same as the DNA IDs.
  dnaConfig?: DNAIdMappings,

  // Custom Holochain conductor URI to use with this instance, to support connecting to multiple conductors.
  // If not specified, connects to the local conductor or the URI stored in `process.env.REACT_APP_HC_CONN_URL`.
  conductorUri?: string,
}

// Schema generation options to be passed to vf-graphql
export interface ExtensionOptions {
  // Array of additional SDL schema strings to bundle into the resultant schema in addition to the
  // specified modular sub-section of the ValueFlows spec.
  // @see https://github.com/valueflows/vf-graphql/#generating-schemas
  extensionSchemas?: string[],

  // Additional resolver callbacks to inject into the schema in addition to the specified bound
  // set of Holo-REA DNA resolvers.
  // Used for injecting implementation logic for `extensionSchemas`: not recommended for overriding
  // parts of the Holo-REA core behaviour!
  extensionResolvers?: IResolvers,
}

export type APIOptions = ResolverOptions & ExtensionOptions





// helpers for resolvers to inject __typename parameter for union type disambiguation
// ...this might be unnecessarily present due to lack of familiarity with GraphQL?

type ObjDecorator<T> = (obj: T) => T
type Resolver<T> = (root, args) => Promise<T>

export function addTypename<T> (name: string): ObjDecorator<T> {
  return (obj) => {
    obj['__typename'] = name
    return obj
  }
}

export function injectTypename<T> (name: string, fn: Resolver<T>): Resolver<T> {
  return async (root, args): Promise<T> => {
    const data = await fn(root, args)
    data['__typename'] = name
    return data
  }
}

// base types

const isoDateRegex = /^\d{4}-\d\d-\d\d(T\d\d:\d\d:\d\d(\.\d\d\d)?)?([+-]\d\d:\d\d)?$/
const parseDate = (val) => {
  if (val instanceof Date) {
    return val
  }
  // :TODO: try progressively more specific dates
  return parse(val, 'YYYY-MM-DDTHH:mm:ss.SSSZZ')
}

export const URI = new GraphQLScalarType({
  name: 'URI',
  description: 'The `URI` type declares a reference to any resolvable resource.',
  serialize: (v) => v,
  parseValue: (v) => v,
  parseLiteral (ast) {
    if (ast.kind === Kind.STRING) {
      if (!ast.value.match(/^\w+:/)) {
        throw new Error('Unable to parse URI- invalid format')
      }
      return ast.value
    }
    return null
  },
})

export const DateTime = new GraphQLScalarType({
  name: 'DateTime',
  description: 'The `DateTime` scalar type represents a DateTime value as specified by [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601).',
  serialize: parseDate,
  parseValue: parseDate,
  parseLiteral (ast) {
    if (ast.kind === Kind.STRING && ast.value.match(isoDateRegex)) {
      return parseDate(ast.value)
    }
    return null
  },
})
