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

fs.copyFileSync(path.resolve(__dirname, './README.md'), path.resolve(__dirname, './build/README.md'))

const packageJson = require(path.resolve(__dirname, './package.json'))
const dependentPackageJson = require(path.resolve(__dirname, '../vf-graphql-holochain/package.json'))

delete packageJson.scripts['prepare']
packageJson.dependencies['@valueflows/vf-graphql-holochain'] = dependentPackageJson.version

fs.writeFileSync(path.resolve(__dirname, './build/package.json'), JSON.stringify(packageJson, undefined, "  "))
