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

import fecha from 'fecha'

import { GraphQLScalarType } from 'graphql'
import { Kind } from 'graphql/language'

// base types

const isoDateRegex = /^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d[+-]\d\d:\d\d$/
const parseDate = (val) => fecha.parse(val, 'YYYY-MM-DDTHH:mm:ssZZ')

export const AnyType = new GraphQLScalarType({
  name: 'AnyType',
  description: 'A type which allows any arbitrary value to be set',
  serialize: (v) => JSON.stringify(v),
  parseValue: (v) => JSON.parse(v),
  parseLiteral (ast) {
    if (ast.kind === Kind.STRING) {
      return JSON.parse(ast.value)
    }
    return null
  }
})

export const URL = new GraphQLScalarType({
  name: 'URL',
  description: 'The `URL` type declares a reference to any resolvable resource.',
  serialize: (v) => v,
  parseValue: (v) => v,
  parseLiteral (ast) {
    if (ast.kind === Kind.STRING) {
      if (!ast.value.match(/^\w+:/)) {  // :TODO: this matches URIs too, should we rename the type?
        throw new Error('Unable to parse URL- invalid format')
      }
      return ast.value
    }
    return null
  }
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
  }
})
