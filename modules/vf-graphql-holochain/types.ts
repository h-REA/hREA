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
  name: '[any value]',
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
  name: 'DateTime (ISO8601)',
  serialize: parseDate,
  parseValue: parseDate,
  parseLiteral (ast) {
    if (ast.kind === Kind.STRING && ast.value.match(isoDateRegex)) {
      return parseDate(ast.value)
    }
    return null
  }
})
