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

import { GraphQLScalarType } from 'graphql'
import { parse } from 'fecha'
import { Kind } from 'graphql/language'

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

export const AnyType = new GraphQLScalarType({
  name: 'AnyType',
  description: 'A type which allows any arbitrary value to be set',
  serialize: (v) => JSON.stringify(v),
  parseValue: (v) => {
    try {
      return JSON.parse(v)
    } catch (e) {
      return v
    }
  },
  parseLiteral (ast) {
    if (ast.kind === Kind.STRING) {
      try {
        return JSON.parse(ast.value)
      } catch (e) {
        return ast.value
      }
    }
    return null
  },
})

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
