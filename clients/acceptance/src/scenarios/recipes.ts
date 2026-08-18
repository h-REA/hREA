import { gql, type ApolloClient, type NormalizedCacheObject } from '@apollo/client/core'
import { type Runner } from '../steps.js'

type Client = ApolloClient<NormalizedCacheObject>

const CREATE_RECIPE_EXCHANGE = gql`
  mutation ($r: RecipeExchangeCreateParams!) {
    res: createRecipeExchange(recipeExchange: $r) { recipeExchange { id revisionId name note } }
  }
`

const CREATE_RECIPE_PROCESS = gql`
  mutation ($r: RecipeProcessCreateParams!) {
    res: createRecipeProcess(recipeProcess: $r) { recipeProcess { id revisionId name note } }
  }
`

const CREATE_RECIPE_FLOW = gql`
  mutation ($r: RecipeFlowCreateParams!) {
    res: createRecipeFlow(recipeFlow: $r) {
      recipeFlow {
        id revisionId note instructions providerRole receiverRole
        action { id }
        resourceQuantity { hasNumericalValue hasUnit { id label } }
        recipeInputOf { id }
        recipeOutputOf { id }
      }
    }
  }
`

const READ_RECIPE_FLOW = gql`
  query ($id: ID!) {
    res: recipeFlow(id: $id) {
      id revisionId note instructions providerRole receiverRole
      action { id }
      resourceQuantity { hasNumericalValue hasUnit { id label } }
      recipeInputOf { id }
      recipeOutputOf { id }
    }
  }
`

/**
 * The recipe vocabulary: RecipeExchange with its two clause families,
 * RecipeProcess, and RecipeFlow CRUD including the quantity and role fields.
 *
 * Migrated from the Tryorama suite's `recipes/` tests, which were the only
 * coverage this domain had.
 */
export async function runRecipes(client: Client, r: Runner): Promise<void> {
  const { step, say, assert } = r

  let exchange = '', spec = '', unit = '', processA = '', processB = ''
  let flowId = '', flowRev = ''

  await step('recipeExchange create + read round-trip', async () => {
    const created = (await client.mutate({
      mutation: CREATE_RECIPE_EXCHANGE,
      variables: { r: { name: 'Bread for labour', note: 'weekly standing swap' } },
    })).data?.res?.recipeExchange
    assert(created?.id, 'recipeExchange id missing')
    exchange = created.id

    const read = (await client.query({
      query: gql`query ($id: ID!) { res: recipeExchange(id: $id) { id revisionId name note } }`,
      variables: { id: exchange },
    })).data?.res
    assert(read?.name === 'Bread for labour', 'name did not round-trip')
    assert(read?.note === 'weekly standing swap', 'note did not round-trip')
    assert(read?.revisionId === created.revisionId, 'read revisionId differs from create')
    return 'exchange created, fields round-trip'
  })

  await step('recipeProcess create + read round-trip', async () => {
    spec = (await client.mutate({
      mutation: gql`mutation ($rs: ResourceSpecificationCreateParams!) { res: createResourceSpecification(resourceSpecification: $rs) { resourceSpecification { id } } }`,
      variables: { rs: { name: 'Sourdough loaf' } },
    })).data?.res?.resourceSpecification?.id

    unit = (await client.mutate({
      mutation: gql`mutation ($u: UnitCreateParams!) { res: createUnit(unit: $u) { unit { id label } } }`,
      variables: { u: { label: 'loaf', symbol: 'lf', omUnitIdentifier: 'one' } },
    })).data?.res?.unit?.id

    processA = (await client.mutate({
      mutation: CREATE_RECIPE_PROCESS,
      variables: { r: { name: 'Mix', note: 'flour, water, salt' } },
    })).data?.res?.recipeProcess?.id
    processB = (await client.mutate({
      mutation: CREATE_RECIPE_PROCESS,
      variables: { r: { name: 'Bake', note: 'the hot bit' } },
    })).data?.res?.recipeProcess?.id
    assert(spec && unit && processA && processB, 'recipe process fixtures missing')

    const read = (await client.query({
      query: gql`query ($id: ID!) { res: recipeProcess(id: $id) { id name note } }`,
      variables: { id: processA },
    })).data?.res
    assert(read?.name === 'Mix', 'recipeProcess name did not round-trip')
    return 'two recipe processes, a spec and a unit'
  })

  await step('recipeExchange lists its clauses and reciprocal clauses separately', async () => {
    const clause = (await client.mutate({
      mutation: CREATE_RECIPE_FLOW,
      variables: { r: { note: 'the bread side', action: 'raise', resourceConformsTo: spec, recipeClauseOf: exchange } },
    })).data?.res?.recipeFlow?.id
    const reciprocal = (await client.mutate({
      mutation: CREATE_RECIPE_FLOW,
      variables: { r: { note: 'the labour side', action: 'raise', resourceConformsTo: spec, recipeReciprocalClauseOf: exchange } },
    })).data?.res?.recipeFlow?.id
    assert(clause && reciprocal, 'clause ids missing')

    const q = await client.query({
      query: gql`
        query {
          res: recipeExchanges {
            edges { node { id recipeClauses { id note } recipeReciprocalClauses { id note } } }
          }
        }
      `,
    })
    const node = (q.data?.res?.edges ?? []).map((e: any) => e.node).find((n: any) => n.id === exchange)
    assert(node, 'exchange absent from the recipeExchanges collection')
    const clauses = (node.recipeClauses ?? []).map((c: any) => c.id)
    const reciprocals = (node.recipeReciprocalClauses ?? []).map((c: any) => c.id)
    assert(clauses.includes(clause), 'recipeClauses misses the clause')
    assert(reciprocals.includes(reciprocal), 'recipeReciprocalClauses misses the reciprocal clause')
    assert(!clauses.includes(reciprocal), 'a reciprocal clause leaked into recipeClauses')
    say('An exchange knows both halves of the bargain, and keeps them apart.')
    return 'both clause families resolve, and do not cross'
  })

  await step('recipeFlow create carries quantity, roles and both process links', async () => {
    const created = (await client.mutate({
      mutation: CREATE_RECIPE_FLOW,
      variables: {
        r: {
          note: 'one loaf out of the bake',
          action: 'raise',
          resourceConformsTo: spec,
          stage: processA,
          recipeInputOf: processA,
          recipeOutputOf: processB,
          resourceQuantity: { hasUnit: unit, hasNumericalValue: 10 },
          instructions: 'prove overnight',
          providerRole: 'baker',
          receiverRole: 'eater',
        },
      },
    })).data?.res?.recipeFlow
    assert(created?.id, 'recipeFlow id missing')
    flowId = created.id
    flowRev = created.revisionId

    assert(created.action?.id === 'raise', 'action did not round-trip')
    assert(created.providerRole === 'baker' && created.receiverRole === 'eater', 'roles did not round-trip')
    assert(created.instructions === 'prove overnight', 'instructions did not round-trip')
    assert(created.resourceQuantity?.hasNumericalValue === 10, 'quantity value did not round-trip')
    assert(created.resourceQuantity?.hasUnit?.id === unit, 'quantity unit did not resolve')
    assert(created.recipeInputOf?.id === processA, 'recipeInputOf did not resolve')
    assert(created.recipeOutputOf?.id === processB, 'recipeOutputOf did not resolve')

    const read = (await client.query({ query: READ_RECIPE_FLOW, variables: { id: flowId } })).data?.res
    assert(JSON.stringify(read) === JSON.stringify(created), 'read-back differs from the created record')
    return 'flow created, every field round-trips through a re-read'
  })

  await step('recipeProcess recipeInputs/recipeOutputs resolve from both ends', async () => {
    const asInput = (await client.query({
      query: gql`query ($id: ID!) { res: recipeProcess(id: $id) { recipeInputs { id } recipeOutputs { id } } }`,
      variables: { id: processA },
    })).data?.res
    const asOutput = (await client.query({
      query: gql`query ($id: ID!) { res: recipeProcess(id: $id) { recipeInputs { id } recipeOutputs { id } } }`,
      variables: { id: processB },
    })).data?.res
    assert((asInput?.recipeInputs ?? []).some((f: any) => f.id === flowId), 'Mix does not list the flow as an input')
    assert((asOutput?.recipeOutputs ?? []).some((f: any) => f.id === flowId), 'Bake does not list the flow as an output')
    return 'the flow resolves as input of one process and output of the other'
  })

  await step('recipeFlow update advances the revision and keeps the id', async () => {
    // RecipeFlow has no sparse-update support: the coordinator deserialises the
    // whole entry, so a payload carrying only the changed field fails inside
    // the zome. Unit and Intent got `merge_fields`; RecipeFlow has not. Send
    // the full entry, the way the Tryorama test this replaces did.
    const updated = (await client.mutate({
      mutation: gql`mutation ($r: RecipeFlowUpdateParams!) { res: updateRecipeFlow(recipeFlow: $r) { recipeFlow { id revisionId note } } }`,
      variables: {
        r: {
          revisionId: flowRev,
          note: 'two loaves out of the bake',
          action: 'raise',
          resourceConformsTo: spec,
          stage: processA,
          recipeInputOf: processA,
          recipeOutputOf: processB,
          resourceQuantity: { hasUnit: unit, hasNumericalValue: 10 },
          instructions: 'prove overnight',
          providerRole: 'baker',
          receiverRole: 'eater',
        },
      },
    })).data?.res?.recipeFlow
    assert(updated?.id === flowId, 'record id changed on update')
    assert(updated.revisionId !== flowRev, 'revisionId did not advance')
    assert(updated.note === 'two loaves out of the bake', 'updated note not persisted')
    flowRev = updated.revisionId
    return 'update persists and advances the revision'
  })

  await step('recipeFlow delete succeeds against the current revision', async () => {
    const res = await client.mutate({
      mutation: gql`mutation ($rev: ID!) { res: deleteRecipeFlow(revisionId: $rev) }`,
      variables: { rev: flowRev },
    })
    assert(res.data?.res === true, 'delete did not report success')
    say('The recipe was written, revised, and torn up — full RecipeFlow CRUD.')
    return 'deleted'
  })
}
