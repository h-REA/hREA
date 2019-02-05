// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape')
const { container } = require('./__init')

const alice = container.makeCaller('alice', dnaPath)

test('description of example test', (t) => {
  // Make a call to a Zome function
  // indicating the capability and function, and passing it an input
    const addr = alice.call("my_zome", "create_my_entry", {"entry" : {"content":"sample content"}})

    const result = alice.call("my_zome", "get_my_entry", {"address": addr.Ok})

  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: { App: [ 'my_entry', '{"content":"sample content"}' ] } })

  // ends this test
  t.end()
})

