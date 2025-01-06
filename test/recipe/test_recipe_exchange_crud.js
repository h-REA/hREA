import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  buildPlayer,
} from '../init.js'


test('Plan record API', async (t) => {
  console.warn(`\n\n${import.meta.url}`)
  const alice = await buildPlayer(['combined'])

  try {
    const { graphQL } = alice

    const exampleEntry = {
      name: "lkasdf",
      note: "fffaa"
    }

    console.log(exampleEntry)

    let createResp = await alice.graphQL(`
      mutation(
        $rs: RecipeExchangeCreateParams!,
        ) {
        rs: createRecipeExchange(recipeExchange: $rs) {
          recipeExchange {
            id
            revisionId
            name
            note
          }
        }
      }
    `, {
      rs: exampleEntry,
    })
    console.log("createResp", JSON.stringify(createResp))
    await pause(500)
    t.ok(createResp.data.rs.recipeExchange.id, 'recipe created')
    const recipeId = createResp.data.rs.recipeExchange.id
    console.log("recipe id", recipeId)

    let readResp = await graphQL(`
      query($id: ID!) {
        res: recipeExchange(id: $id) {
          id
          revisionId
          name
          note
        }
      }
    `, {
      id: recipeId,
    })

    console.log("response", JSON.stringify(readResp))

    t.ok(readResp, 'recipe read')
    t.deepLooseEqual(readResp.data.res, createResp.data.rs.recipeExchange, 'recipe read matches create')


    let flowResp = await alice.graphQL(`
      mutation(
        $rs: RecipeFlowCreateParams!,
        ) {
        rs: createRecipeFlow(recipeFlow: $rs) {
          recipeFlow {
            id
            revisionId
            note
          }
        }
      }
    `, {
      rs: {
        note: 'recipe flow 1',
        action: 'raise',
        resourceConformsTo: recipeId,
        stage: recipeId,
        recipeClauseOf: recipeId,
      },
    })

    pause(500)

    let flowResp2 = await alice.graphQL(`
      mutation(
        $rs: RecipeFlowCreateParams!,
        ) {
        rs: createRecipeFlow(recipeFlow: $rs) {
          recipeFlow {
            id
            revisionId
            note
          }
        }
      }
    `, {
      rs: {
        note: 'recipe flow 1',
        action: 'raise',
        resourceConformsTo: recipeId,
        stage: recipeId,
        recipeReciprocalClauseOf: recipeId,
      },
    })

    console.log("flowResp", JSON.stringify(flowResp), JSON.stringify(flowResp2))
    const recipeFlowId = flowResp.data.rs.recipeFlow.id
    const recipeFlowId2 = flowResp2.data.rs.recipeFlow.id
    await pause(500)

    // fetch all
    let fetchAllResp = await graphQL(`
      query {
        res: recipeExchanges {
          edges {
            node {
              id
              revisionId
              name
              note
              recipeClauses {
                id
                revisionId
                note
              }
              recipeReciprocalClauses {
                id
                revisionId
                note
              }
            }
          }
        }
      }
    `)
    console.log(JSON.stringify(fetchAllResp))
    t.ok(fetchAllResp, 'recipeExchanges fetched')
    t.ok(fetchAllResp.data.res.edges.length > 0, 'recipeExchanges not empty')
    const firstRecipeExchange = fetchAllResp.data.res.edges[0].node
    t.ok(firstRecipeExchange.recipeClauses.length > 0, 'recipeClauses not empty')
    t.ok(firstRecipeExchange.recipeReciprocalClauses.length > 0, 'recipeReciprocalClauses not empty')

  } catch (e) {
    await alice.scenario.cleanUp()
    console.log(e)
  }
  await alice.scenario.cleanUp()
})