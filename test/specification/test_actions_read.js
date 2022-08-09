// :NOTE: this is a minimal test- actual actions are tested in Rust unit tests
import test from 'tape'
import {
  buildPlayer,
} from '../init.js'

test('Built-in action API', async (t) => {
  // display the filename for context in the terminal and use .warn
  // to override the tap testing log filters
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['specification'])
  try {
    const queryAllResp = await alice.graphQL(`
      {
        actions {
          id
        }
      }
    `, {})

    t.equal(queryAllResp.data.actions.length, 18, 'all action builtins present')

    const getResp = await alice.graphQL(`
      query($id: ID!) {
        action(id: $id) {
          id
          label
          resourceEffect
          inputOutput
          pairsWith
        }
      }
    `, {
      id: 'raise',
    })

    t.deepLooseEqual(getResp.data.action, {
      id: 'raise',
      label: 'raise',
      resourceEffect: 'increment',
      inputOutput: 'notApplicable',
      pairsWith: 'notApplicable',
    }, 'record read OK')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
