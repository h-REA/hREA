/**
 * Dirty hack to fix Create React App config so that sourcemaps from compiled dependencies are picked up
 *
 * @package: HoloREA
 * @since:   2019-05-28
 * @flow
 */

const path = require('path')
const fs = require('fs')

const configFile = path.resolve(__dirname,
  // Yarn path
  // '../node_modules/react-scripts/config/webpack.config.js'
  // pNPM path
  '../node_modules/.pnpm/react-scripts@3.0.1_typescript@3.8.3/node_modules/react-scripts/config/webpack.config.js'
)

console.info("Hacking react-scripts Webpack config...")

const contents = fs.readFileSync(configFile)

const MODULE_DIRS_WITH_SOURCEMAPS = [
  path.resolve(__dirname, '../modules/vf-graphql-holochain')
]

fs.writeFileSync(configFile, contents.toString().replace(
  /^\s*({\s*$\s*test: \/\\\.\(js\|mjs\|jsx\|ts\|tsx\)\$\/,\s*$\s*enforce: 'pre',)/gm,
  `{
          test: /\.(js|mjs|jsx|ts|tsx)$/,
          enforce: 'pre',
          use: ['source-map-loader'],
          include: [
            '${MODULE_DIRS_WITH_SOURCEMAPS.join("',\n'")}'
          ]
        },$1`
  )
  .replace('sourceMaps: false', 'sourceMaps: true')
  .replace('cheap-module-source-map', 'inline-source-map')
)

console.info("react-scripts Webpack config fixed!")
