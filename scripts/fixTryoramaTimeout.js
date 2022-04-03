/// Temporary script to fix Tryorama websocket timeouts
/// for tests involving DNAs in excess of 16MB.of

const fs = require('fs')
const path = require('path')

const filePath = path.resolve(__dirname, '../node_modules/.pnpm/@holochain+client@0.3.2/node_modules/@holochain/client/lib/websocket/common.js')

if (!fs.existsSync(filePath)) {
  console.error('Unable to find Tryorama websocket file for patching. Was it updated? Is this script still needed?')
  process.exit(1)
}

const contents = fs.readFileSync(filePath) + ''

fs.writeFileSync(filePath, contents.replace(
  /exports\.DEFAULT_TIMEOUT\s*=\s*\d+/,
  'exports.DEFAULT_TIMEOUT = 50000'
))

console.log('Tryorama websocket timeout patched successfully!')
