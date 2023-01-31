#!/usr/bin/env node

/**
 * Dirty hack to fix Create React App config so that some dependencies using non-suffixed import paths can compile
 *
 * @package: hREA
 * @since:   2023-01-31
 */

const path = require('path')
const fs = require('fs')

const configFile = path.resolve(__dirname,
  '../apps/hrea-graphql-explorer/node_modules/react-scripts/config/webpack.config.js'
)

console.info("Hacking react-scripts Webpack config...")

const contents = fs.readFileSync(configFile)

fs.writeFileSync(configFile, contents.toString().replace(
// revert any existing replacements first to make idempotent
`              test: /\\.(js|mjs)$/,
              exclude: /@babel(?:\\/|\\\\{1,2})runtime/,
              loader: require.resolve('babel-loader'),
              resolve: {
                fullySpecified: false,
              },`,
`              test: /\\.(js|mjs)$/,
              exclude: /@babel(?:\\/|\\\\{1,2})runtime/,
              loader: require.resolve('babel-loader'),`
).replace(
`              test: /\\.(js|mjs)$/,
              exclude: /@babel(?:\\/|\\\\{1,2})runtime/,
              loader: require.resolve('babel-loader'),`,
`              test: /\\.(js|mjs)$/,
              exclude: /@babel(?:\\/|\\\\{1,2})runtime/,
              loader: require.resolve('babel-loader'),
              resolve: {
                fullySpecified: false,
              },`
  )
)

console.info("react-scripts Webpack config fixed!")
