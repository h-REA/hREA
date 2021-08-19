/**
 * Finalise build process by preparing non-JS components for publishing.
 *
 * This also removes the private: true flag upon copying package.json so that publishing isn't blocked,
 * such config prevents people from publishing the incorrect package.
 *
 * @package: HoloREA
 * @since:   2020-01-31
 */

const fs = require('fs')
const path = require('path')

fs.copyFileSync(path.resolve(__dirname, '../../LICENSE'), path.resolve(__dirname, './build/LICENSE'))
fs.copyFileSync(path.resolve(__dirname, './README.md'), path.resolve(__dirname, './build/README.md'))
fs.copyFileSync(path.resolve(__dirname, './package.json'), path.resolve(__dirname, './build/package.json'))

const packageJson = require(path.resolve(__dirname, './build/package.json'))

delete packageJson['private']
delete packageJson['main']
delete packageJson['types']
delete packageJson.scripts['prepare']
packageJson.dependencies['@valueflows/vf-graphql-holochain'] = packageJson.dependencies['@valueflows/vf-graphql-holochain'].replace('../', '../../')

fs.writeFileSync(path.resolve(__dirname, './build/package.json'), JSON.stringify(packageJson, undefined, "  "))

