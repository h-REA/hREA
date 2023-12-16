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

    const recipeId = createResp.data.rs.recipeFlow.id
    const recipeRev = createResp.data.rs.recipeFlow.revisionId
    console.log("recipe id", recipeId)

    let getResp = await graphQL(`
      query($id: ID!) {
        res: recipeFlow(id: $id) {
          id
          revisionId
          action {
            id
          }
          note
        }
      }
    `, {
      id: recipeId,
    })

    console.log("get resp", JSON.stringify(getResp.data.res))
    console.log("other", JSON.stringify(exampleEntry))

    t.ok(getResp, 'recipe read')

    t.deepLooseEqual(getResp.data.res, createResp.data.rs.recipeFlow, 'record read OK')

    const updateResp = await graphQL(`
      mutation($rs: RecipeFlowUpdateParams!) {
        res: updateRecipeFlow(recipeFlow: $rs) {
          recipeFlow {
            id
            revisionId
          }
        }
      }
    `, {
      rs: { revisionId: recipeRev, ...exampleEntry },
    })

    await pause(100)
    t.equal(updateResp.data.res.recipeFlow.id, recipeId, 'record ID consistent')
    t.notEqual(updateResp.data.res.recipeFlow.revisionId, recipeRev, 'record updated')

    const deleteResult = await graphQL(`
      mutation($revisionId: ID!) {
        res: deleteRecipeFlow(revisionId: $revisionId)
      }
    `, {
      revisionId: recipeRev,
    })
    await pause(100)

    t.equal(deleteResult.data.res, true, "delete successful")
  } catch (e) {
    await alice.scenario.cleanUp()
    console.log(e)
  }
  await alice.scenario.cleanUp()
})