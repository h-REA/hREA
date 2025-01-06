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

    const exampleRecipeProcessEntry = {
      name: "lkasdf",
      // processConformsTo: rsId,
      note: "fffaa"
    }

    // t.ok(resp.data.rs.resourceSpecification.id, 'ResourceSpecification created')
    const rsId = resp.data.rs.resourceSpecification.id

    let respProcess1 = await alice.graphQL(`
      mutation(
        $rs: RecipeProcessCreateParams!,
        ) {
        rs: createRecipeProcess(recipeProcess: $rs) {
          recipeProcess {
            id
            revisionId
            name
            note
          }
        }
      }
    `, {
      rs: exampleRecipeProcessEntry,
    })
    let respProcess1Address = respProcess1.data.rs.recipeProcess.id
    await pause(100)

    let respProcess2 = await alice.graphQL(`
      mutation(
        $rs: RecipeProcessCreateParams!,
        ) {
        rs: createRecipeProcess(recipeProcess: $rs) {
          recipeProcess {
            id
            revisionId
            name
            note
          }
        }
      }
    `, {
      rs: exampleRecipeProcessEntry,
    })
    let respProcess2Address = respProcess2.data.rs.recipeProcess.id
    await pause(100)

    // get recipe process
    let getRespProcess1 = await graphQL(`
      query($id: ID!) {
        res: recipeProcess(id: $id) {
          id
          revisionId
          name
          note
        }
      }
    `, {
      id: respProcess1Address,
    })
    console.log("get resp process 1", JSON.stringify(getRespProcess1))
    
    const exampleEntry = {
      note: 'just testing',
      action: 'raise',
      resourceConformsTo: rsId,
      stage: respProcess1Address,
      recipeInputOf: respProcess1Address,
      recipeOutputOf: respProcess2Address,
      instructions: "do this",
      providerRole: "provider",
      receiverRole: "receiver",
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
            providerRole
            receiverRole
            instructions
            action {
              id
            }
            recipeInputOf {
              id
            }
            recipeOutputOf {
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
          providerRole
          receiverRole
          instructions
          action {
            id
          }
          recipeInputOf {
            id
          }
          recipeOutputOf {
            id
          }
          note
        }
      }
    `, {
      id: recipeId,
    })

    console.log("get resp", JSON.stringify(getResp.data.res))
    t.ok(getResp, 'recipe read')
    t.deepLooseEqual(getResp.data.res, createResp.data.rs.recipeFlow, 'record read OK')

    // query all
    let allResp = await graphQL(`
      query {
        res: recipeFlows {
          edges {
            node {
              id
              revisionId
              instructions
              action {
                id
              }
              recipeInputOf {
                id
              }
              recipeOutputOf {
                id
              }
              note
            }
          }
        }
      }
    `)

    console.log("get all resp", JSON.stringify(allResp.data.res))
    t.ok(allResp, 'recipe read all')
    t.ok(allResp.data.res.edges.length > 0, 'recipe read all OK')

    // query process recipeInputs
    const fetchProcess = await graphQL(`
      query($id: ID!) {
        res: recipeProcess(id: $id) {
          id
          recipeInputs {
            id
          }
          recipeOutputs {
            id
          }
        }
      }
    `, {
      id: respProcess1Address,
    })
    console.log("fetch process recipeInputs", JSON.stringify(fetchProcess))
    t.ok(fetchProcess, 'recipe process read')
    t.ok(fetchProcess.data.res, 'recipe process read OK')

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