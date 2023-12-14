import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
} from '../init.js'


test('Plan record API', async (t) => {
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['recipe', 'specification'])

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
      resourceConformsTo: rsId,
    }

    console.log(exampleEntry)

    let createResp = await alice.graphQL(`
      mutation(
        $rs: RecipeFlowCreateParams!,
        ) {
        rs: createRecipeFlow(recipeFlow: $rs) {
          recipeFlow {
            id
            revisionId
            action {
              id
            }
            note
          }
        }
      }
    `, {
      rs: exampleEntry,
    })
    await pause(100)
    t.ok(createResp.data.rs.recipeFlow.id, 'recipe created')
  } catch (e) {
    await alice.scenario.cleanUp()
    console.log(e)
  }
  await alice.scenario.cleanUp()
})