import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
} from '../init.js'


test('Plan record API', async (t) => {
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['planning', 'specification'])

  try {
    const { graphQL } = alice

    let resp = await graphQL(`
      mutation(
        $rs: ResourceSpecificationCreateParams!,
      ) {
        rs: createResourceSpecification(resourceSpecification: $rs) {
          resourceSpecification {
            id
          }
        }
      }
    `, {
      rs: {
        name: 'test resource spec',
      },
    })
    await pause(100)

    // t.ok(resp.data.rs.resourceSpecification.id, 'ResourceSpecification created')
    const rsId = resp.data.rs.resourceSpecification.id
    
    const exampleEntry = {
      note: 'just testing',
      action: 'raise',
      recipeFlowResource: rsId,
    }

    let createResp = await alice.graphQL(`
      mutation($rec: RecipeFlowCreateParams!) {
        res: createRecipeFlow(recipeFlow: $rec) {
          recipeFlow {
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
    await alice.scenario.cleanUp()
    console.log(e)
  }
  await alice.scenario.cleanUp()
})