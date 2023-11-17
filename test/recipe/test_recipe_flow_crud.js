import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
} from '../init.js'

const exampleEntry = {
  name: 'test recipe flow',
  note: 'just testing',
}

test('Plan record API', async (t) => {
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['planning'])

  try {
    let createResp = await alice.graphQL(`
      mutation($rec: RecipeFlowCreateParams!) {
        res: createRecipeFlow(recipe: $rec) {
          recipeFlow {
            id
            revisionId
          }
        }
      }
    `, {
      rec: exampleEntry,
    })
    await pause(100)
    t.ok(createResp.data.res.recipeFlow.id, 'recipe created')
  } catch (e) {
    console.log(e)
  }
})