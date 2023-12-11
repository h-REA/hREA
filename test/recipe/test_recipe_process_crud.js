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
        $rs: ProcessSpecificationCreateParams!,
      ) {
        rs: createProcessSpecification(processSpecification: $rs) {
          processSpecification {
            id
          }
        }
      }
    `, {
      rs: {
        name: 'test process spec',
      },
    })
    await pause(100)

    // t.ok(resp.data.rs.resourceSpecification.id, 'ResourceSpecification created')
    const rsId = resp.data.rs.processSpecification.id
    
    const exampleEntry = {
      name: "String!",
      processClassifiedAs: rsId,
      note: "String"    
    }

    console.log(exampleEntry)

    let createResp = await alice.graphQL(`
      mutation(
        $rs: RecipeProcessCreateParams!,
        ) {
        rs: createRecipeProcess(recipeProcess: $rs) {
          recipeProcess {
            id
            revisionId
            name
            processClassifiedAs
            note
          }
        }
      }
    `, {
      rs: exampleEntry,
    })
    await pause(100)
    t.ok(createResp.data.rs.recipeProcess.id, 'recipe created')
  } catch (e) {
    await alice.scenario.cleanUp()
    console.log(e)
  }
  await alice.scenario.cleanUp()
})