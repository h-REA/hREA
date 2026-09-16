import { gql, type ApolloClient, type NormalizedCacheObject } from '@apollo/client/core'
import { type Runner } from '../steps.js'

type Client = ApolloClient<NormalizedCacheObject>

const CREATE_PERSON = gql`
  mutation ($p: AgentCreateParams!) { res: createPerson(person: $p) { agent { id } } }
`
const CREATE_SPEC = gql`
  mutation ($rs: ResourceSpecificationCreateParams!) {
    res: createResourceSpecification(resourceSpecification: $rs) { resourceSpecification { id } }
  }
`
const CREATE_AGREEMENT = gql`
  mutation ($a: AgreementCreateParams!) {
    res: createAgreement(agreement: $a) { agreement { id revisionId name note } }
  }
`
const UPDATE_AGREEMENT = gql`
  mutation ($a: AgreementUpdateParams!) {
    res: updateAgreement(agreement: $a) { agreement { id revisionId name note } }
  }
`
const GET_AGREEMENT = gql`
  query ($id: ID!) { agreement(id: $id) { id name note } }
`
const CREATE_COMMITMENT = gql`
  mutation ($c: CommitmentCreateParams!) {
    res: createCommitment(commitment: $c) { commitment { id revisionId note } }
  }
`
const UPDATE_COMMITMENT = gql`
  mutation ($c: CommitmentUpdateParams!) {
    res: updateCommitment(commitment: $c) { commitment { id revisionId note } }
  }
`
const CREATE_PLAN = gql`
  mutation ($p: PlanCreateParams!) { res: createPlan(plan: $p) { plan { id revisionId name note } } }
`
const UPDATE_PLAN = gql`
  mutation ($p: PlanUpdateParams!) { res: updatePlan(plan: $p) { plan { id revisionId name note } } }
`
const CREATE_EVENT = gql`
  mutation ($e: EconomicEventCreateParams!) {
    res: createEconomicEvent(event: $e) { economicEvent { id } }
  }
`
const CREATE_EVENT_WITH_RESOURCE = gql`
  mutation ($e: EconomicEventCreateParams!, $r: EconomicResourceCreateParams) {
    res: createEconomicEvent(event: $e, newInventoriedResource: $r) {
      economicEvent { id }
      economicResource { id }
    }
  }
`

/**
 * The checks the Tryorama retirement would otherwise have dropped.
 *
 * Each step here maps to a behaviour that the old `tests/src` suite asserted
 * and that neither `crud.ts`, `rea-flows.ts`, `recipes.ts` nor the Sweettest
 * crate picked up. They live in their own file so the mapping stays auditable:
 * this is the difference between "the count is close" and "the same rules are
 * covered".
 */
export async function runRegressions(client: Client, r: Runner): Promise<void> {
  const { step, say, assert } = r

  let alice = '', bob = '', spec = ''
  await step('regression setup: two agents and a specification', async () => {
    alice = (await client.mutate({ mutation: CREATE_PERSON, variables: { p: { name: 'Reg Alice' } } })).data?.res?.agent?.id
    bob = (await client.mutate({ mutation: CREATE_PERSON, variables: { p: { name: 'Reg Bob' } } })).data?.res?.agent?.id
    spec = (await client.mutate({ mutation: CREATE_SPEC, variables: { rs: { name: 'Regression widget' } } })).data?.res?.resourceSpecification?.id
    assert(alice && bob && spec, 'setup ids missing')
    return 'agents and specification created'
  })

  // ── Agreement: create fields, read-back, and two chained updates ──────────
  let agreement = '', agreementRev = ''
  await step('agreement create round-trips name and note, and reads back by id', async () => {
    const a = (await client.mutate({
      mutation: CREATE_AGREEMENT,
      variables: { a: { name: 'Winter supply', note: 'six deliveries' } },
    })).data?.res?.agreement
    agreement = a?.id
    agreementRev = a?.revisionId
    assert(agreement && agreementRev, 'agreement ids missing')
    assert(a.name === 'Winter supply', `name did not round-trip on create (got ${a.name})`)
    assert(a.note === 'six deliveries', `note did not round-trip on create (got ${a.note})`)
    const q = await client.query({ query: GET_AGREEMENT, variables: { id: agreement } })
    assert(q.data?.agreement?.name === 'Winter supply', `Agreement.name is wrong on read-back (got ${q.data?.agreement?.name})`)
    assert(q.data?.agreement?.note === 'six deliveries', `Agreement.note is wrong on read-back (got ${q.data?.agreement?.note})`)
    return 'name and note survive create and read-back'
  })

  await step('agreement updates twice in a row, the second built on the first revision', async () => {
    const first = (await client.mutate({
      mutation: UPDATE_AGREEMENT,
      variables: { a: { revisionId: agreementRev, name: 'Winter supply 2026' } },
    })).data?.res?.agreement
    assert(first?.name === 'Winter supply 2026', `first update did not persist the name (got ${first?.name})`)
    assert(first.id === agreement, 'first update forked the agreement id')
    assert(first.revisionId !== agreementRev, 'revisionId did not advance on the first update')
    assert(first.note === 'six deliveries', `omitted note was wiped by the sparse update (got ${first.note})`)

    const second = (await client.mutate({
      mutation: UPDATE_AGREEMENT,
      variables: { a: { revisionId: first.revisionId, name: 'Winter supply 2027', note: 'eight deliveries' } },
    })).data?.res?.agreement
    assert(second?.id === agreement, 'second update forked the agreement id')
    assert(second.revisionId !== first.revisionId, 'revisionId did not advance on the second update')

    const q = await client.query({ query: GET_AGREEMENT, variables: { id: agreement } })
    assert(q.data?.agreement?.name === 'Winter supply 2027', `the collection shows the first update, not the second (got ${q.data?.agreement?.name})`)
    assert(q.data?.agreement?.note === 'eight deliveries', `second update did not persist the note (got ${q.data?.agreement?.note})`)
    say('An agreement was renamed twice and the second rename is what survives.')
    return 'chained update converges on the latest revision'
  })

  await step('an original id can still update and delete an entity after it has revisions', async () => {
    const created = (await client.mutate({
      mutation: CREATE_AGREEMENT,
      variables: { a: { name: 'Stable-id agreement', note: 'original' } },
    })).data?.res?.agreement
    assert(created?.id && created?.revisionId, 'stable-id agreement ids missing')

    const first = (await client.mutate({
      mutation: UPDATE_AGREEMENT,
      variables: { a: { revisionId: created.revisionId, name: 'Stable-id revision one' } },
    })).data?.res?.agreement
    assert(first?.revisionId !== created.revisionId, 'first stable-id update did not advance the revision')

    const second = (await client.mutate({
      mutation: UPDATE_AGREEMENT,
      variables: { a: { revisionId: created.id, name: 'Stable-id revision two' } },
    })).data?.res?.agreement
    assert(second?.id === created.id, 'update through the original id changed the stable id')
    assert(second?.name === 'Stable-id revision two', 'update through the original id did not persist')

    const read = await client.query({
      query: GET_AGREEMENT,
      variables: { id: created.id },
      fetchPolicy: 'no-cache',
    })
    assert(read.data?.agreement?.name === 'Stable-id revision two', 'stable-id read did not resolve the latest update')

    const deleted = await client.mutate({
      mutation: gql`mutation ($rev: ID!) { res: deleteAgreement(revisionId: $rev) }`,
      variables: { rev: created.id },
    })
    assert(deleted.data?.res === true, 'delete through the original id did not report success')
    const all = await client.query({
      query: gql`query { agreements { edges { node { id } } } }`,
      fetchPolicy: 'no-cache',
    })
    const ids = (all.data?.agreements?.edges ?? []).map((edge: any) => edge.node.id)
    assert(!ids.includes(created.id), 'agreement deleted through its original id remains in the collection')
    return 'original id resolves updates and deletion after revision'
  })

  // ── Commitment update ────────────────────────────────────────────────────
  let commitment = '', commitmentRev = ''
  await step('commitment update persists the note, keeps its id, advances the revision', async () => {
    const c = (await client.mutate({
      mutation: CREATE_COMMITMENT,
      variables: { c: { action: 'transfer', provider: alice, receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 3 }, note: 'as agreed', clauseOf: agreement } },
    })).data?.res?.commitment
    commitment = c?.id
    commitmentRev = c?.revisionId
    assert(commitment && commitmentRev, 'commitment ids missing')

    const u = (await client.mutate({
      mutation: UPDATE_COMMITMENT,
      variables: { c: { revisionId: commitmentRev, note: 'renegotiated' } },
    })).data?.res?.commitment
    assert(u?.note === 'renegotiated', `updated note not persisted (got ${u?.note})`)
    assert(u.id === commitment, 'update forked the commitment id')
    assert(u.revisionId !== commitmentRev, 'revisionId did not advance on update')
    commitmentRev = u.revisionId

    const q = await client.query({
      query: gql`query ($id: ID!) { commitment(id: $id) { id note } }`,
      variables: { id: commitment },
    })
    assert(q.data?.commitment?.note === 'renegotiated', `re-read does not show the update (got ${q.data?.commitment?.note})`)
    return 'commitment note updated and re-read'
  })

  // ── Plan update, twice ───────────────────────────────────────────────────
  let plan = ''
  await step('plan updates twice in a row and the latest name is what reads back', async () => {
    const p = (await client.mutate({
      mutation: CREATE_PLAN,
      variables: { p: { name: 'Spring plan', note: 'draft' } },
    })).data?.res?.plan
    plan = p?.id
    assert(plan && p?.revisionId, 'plan ids missing')
    assert(p.name === 'Spring plan', `plan name did not round-trip on create (got ${p.name})`)

    const first = (await client.mutate({
      mutation: UPDATE_PLAN,
      variables: { p: { revisionId: p.revisionId, name: 'Spring plan rev A' } },
    })).data?.res?.plan
    assert(first?.name === 'Spring plan rev A', `first plan update did not persist (got ${first?.name})`)
    assert(first.id === plan, 'first plan update forked the id')

    const second = (await client.mutate({
      mutation: UPDATE_PLAN,
      variables: { p: { revisionId: first.revisionId, name: 'Spring plan rev B' } },
    })).data?.res?.plan
    assert(second?.id === plan, 'second plan update forked the id')

    const q = await client.query({
      query: gql`query ($id: ID!) { plan(id: $id) { id name note } }`,
      variables: { id: plan },
    })
    assert(q.data?.plan?.name === 'Spring plan rev B', `plan re-read shows ${q.data?.plan?.name}, not the second update`)
    assert(q.data?.plan?.note === 'draft', `omitted note was wiped across two sparse updates (got ${q.data?.plan?.note})`)
    return 'plan chained update converges'
  })

  // ── Intent.observedBy: satisfaction by a direct event ────────────────────
  await step('intent observedBy: an event satisfies an intent and the reverse resolves', async () => {
    const intent = (await client.mutate({
      mutation: gql`mutation ($i: IntentCreateParams!) { res: createIntent(intent: $i) { intent { id } } }`,
      variables: { i: { action: 'transfer', name: 'Bob wants widgets', receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 3 } } },
    })).data?.res?.intent?.id
    assert(intent, 'intent id missing')

    const satisfyingCommitment = (await client.mutate({
      mutation: CREATE_COMMITMENT,
      variables: { c: { action: 'transfer', provider: alice, receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 3 }, satisfies: intent } },
    })).data?.res?.commitment?.id
    assert(satisfyingCommitment, 'satisfying commitment id missing')

    const evt = (await client.mutate({
      mutation: CREATE_EVENT,
      variables: { e: { action: 'transfer', provider: alice, receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 3 }, satisfies: [intent] } },
    })).data?.res?.economicEvent?.id
    assert(evt, 'satisfying event id missing')

    const q = await client.query({
      query: gql`query ($id: ID!) { intent(id: $id) { id observedBy { id } satisfiedBy { id } } }`,
      variables: { id: intent },
    })
    const observed = (q.data?.intent?.observedBy ?? []).map((e: any) => e.id)
    const satisfied = (q.data?.intent?.satisfiedBy ?? []).map((c: any) => c.id)
    assert(observed.length === 1 && observed[0] === evt, `intent.observedBy did not contain exactly the event (got ${observed.length} entries)`)
    assert(satisfied.length === 1 && satisfied[0] === satisfyingCommitment, `intent.satisfiedBy did not contain exactly the commitment (got ${satisfied.length} entries)`)
    assert(!observed.includes(satisfyingCommitment), 'the satisfying commitment leaked into observedBy')
    assert(!satisfied.includes(evt), 'the satisfying event leaked into satisfiedBy')
    say('An intent was satisfied by a direct event rather than a promise, and the reverse link resolves.')
    return 'satisfiedBy and observedBy remain isolated on the same intent'
  })

  // ── Plan.nonProcessCommitments and Process committed inputs/outputs ──────
  await step('plan nonProcessCommitments and process committedInputs/committedOutputs resolve', async () => {
    const direct = (await client.mutate({
      mutation: CREATE_COMMITMENT,
      variables: { c: { action: 'transfer', provider: alice, receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 7 }, plannedWithin: plan } },
    })).data?.res?.commitment?.id
    assert(direct, 'non-process commitment id missing')

    const proc = (await client.mutate({
      mutation: gql`mutation ($p: ProcessCreateParams!) { res: createProcess(process: $p) { process { id } } }`,
      variables: { p: { name: 'Assemble widgets', plannedWithin: plan } },
    })).data?.res?.process?.id
    assert(proc, 'process id missing')

    const cIn = (await client.mutate({
      mutation: CREATE_COMMITMENT,
      variables: { c: { action: 'consume', provider: alice, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 2 }, inputOf: proc } },
    })).data?.res?.commitment?.id
    const cOut = (await client.mutate({
      mutation: CREATE_COMMITMENT,
      variables: { c: { action: 'produce', provider: alice, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 1 }, outputOf: proc } },
    })).data?.res?.commitment?.id
    assert(cIn && cOut, 'committed input/output ids missing')

    const qp = await client.query({
      query: gql`query ($id: ID!) { plan(id: $id) { nonProcessCommitments { id } } }`,
      variables: { id: plan },
    })
    const nonProcess = (qp.data?.plan?.nonProcessCommitments ?? []).map((c: any) => c.id)
    assert(nonProcess.includes(direct), 'plan.nonProcessCommitments misses the commitment planned directly against the plan')

    const qproc = await client.query({
      query: gql`query ($id: ID!) { process(id: $id) { committedInputs { id } committedOutputs { id } } }`,
      variables: { id: proc },
    })
    const ins = (qproc.data?.process?.committedInputs ?? []).map((c: any) => c.id)
    const outs = (qproc.data?.process?.committedOutputs ?? []).map((c: any) => c.id)
    assert(ins.includes(cIn), 'process.committedInputs misses the input commitment')
    assert(outs.includes(cOut), 'process.committedOutputs misses the output commitment')
    assert(!ins.includes(cOut), 'committedInputs leaked the output commitment')
    say('A plan counts a commitment made outside any process, and a process knows what it has promised in and out.')
    return 'nonProcessCommitments + committedInputs/Outputs resolve without cross-leak'
  })

  // ── A resource stays listed exactly once across repeated events ──────────
  await step('a resource is listed exactly once after repeated events against it', async () => {
    const made = (await client.mutate({
      mutation: CREATE_EVENT_WITH_RESOURCE,
      variables: {
        e: { action: 'produce', provider: alice, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 10 } },
        r: { name: 'Widget bin' },
      },
    })).data?.res
    const resource = made?.economicResource?.id
    assert(resource, 'resource id missing')

    let latestEvent = ''
    for (const [action, qty] of [['raise', 5], ['lower', 3], ['consume', 4]] as Array<[string, number]>) {
      const e = await client.mutate({
        mutation: CREATE_EVENT,
        variables: { e: { action, provider: alice, receiver: alice, resourceInventoriedAs: resource, resourceQuantity: { hasNumericalValue: qty } } },
      })
      latestEvent = e.data?.res?.economicEvent?.id
      assert(latestEvent, `${action} event was not created`)
    }

    const throughEvent = await client.query({
      query: gql`
        query ($id: ID!) {
          economicEvent(id: $id) {
            resourceInventoriedAs {
              id revisionId
              accountingQuantity { hasNumericalValue }
              onhandQuantity { hasNumericalValue }
            }
          }
        }
      `,
      variables: { id: latestEvent },
      fetchPolicy: 'no-cache',
    })
    const resolvedResource = throughEvent.data?.economicEvent?.resourceInventoriedAs
    assert(resolvedResource?.id === resource, 'event did not retain the resource stable id')
    assert(resolvedResource?.revisionId, 'event did not resolve a resource revision')
    assert(Number(resolvedResource?.accountingQuantity?.hasNumericalValue) === 8, 'event did not resolve the latest accounting quantity')
    assert(Number(resolvedResource?.onhandQuantity?.hasNumericalValue) === 8, 'event did not resolve the latest onhand quantity')

    const q = await client.query({
      query: gql`query { economicResources { edges { node { id } } } }`,
    })
    const ids = (q.data?.economicResources?.edges ?? []).map((e: any) => e.node.id)
    const occurrences = ids.filter((id: string) => id === resource).length
    assert(occurrences === 1, `the resource appears ${occurrences} times in economicResources after three further events; a duplicated resource-to-event link is back`)
    say('Three more events hit the same bin and it is still one row, not four.')
    return 'event resolves the latest resource revision without duplicating its row'
  })

  // ── Deep plan graph under collection fanout ─────────────────────────────
  await step('a nested plan query resolves multiple processes and commitment families', async () => {
    const fullPlan = (await client.mutate({
      mutation: CREATE_PLAN,
      variables: { p: { name: 'Fanout plan' } },
    })).data?.res?.plan?.id
    assert(fullPlan, 'fanout plan id missing')

    const processCount = 5
    const commitmentsPerDirection = 4
    const processIds: string[] = []
    for (let processIndex = 0; processIndex < processCount; processIndex++) {
      const processId = (await client.mutate({
        mutation: gql`mutation ($p: ProcessCreateParams!) { res: createProcess(process: $p) { process { id } } }`,
        variables: { p: { name: `Fanout process ${processIndex + 1}`, plannedWithin: fullPlan } },
      })).data?.res?.process?.id
      assert(processId, `fanout process ${processIndex + 1} id missing`)
      processIds.push(processId)

      for (let commitmentIndex = 0; commitmentIndex < commitmentsPerDirection; commitmentIndex++) {
        const input = await client.mutate({
          mutation: CREATE_COMMITMENT,
          variables: { c: { action: 'consume', provider: alice, receiver: bob, inputOf: processId, note: `Input ${processIndex}-${commitmentIndex}` } },
        })
        const output = await client.mutate({
          mutation: CREATE_COMMITMENT,
          variables: { c: { action: 'produce', provider: alice, receiver: bob, outputOf: processId, note: `Output ${processIndex}-${commitmentIndex}` } },
        })
        assert(input.data?.res?.commitment?.id && output.data?.res?.commitment?.id, 'fanout process commitment missing')
      }
    }

    const independentDemandCount = 5
    for (let index = 0; index < independentDemandCount; index++) {
      const demand = await client.mutate({
        mutation: CREATE_COMMITMENT,
        variables: { c: { action: 'raise', provider: alice, receiver: bob, independentDemandOf: fullPlan, note: `Demand ${index}` } },
      })
      assert(demand.data?.res?.commitment?.id, `independent demand ${index} missing`)
    }

    const nonProcessCount = 10
    for (let index = 0; index < nonProcessCount; index++) {
      const direct = await client.mutate({
        mutation: CREATE_COMMITMENT,
        variables: { c: { action: 'raise', provider: alice, receiver: bob, plannedWithin: fullPlan, note: `Direct ${index}` } },
      })
      assert(direct.data?.res?.commitment?.id, `non-process commitment ${index} missing`)
    }

    const graph = await client.query({
      query: gql`
        query ($id: ID!) {
          plan(id: $id) {
            id
            processes { id committedInputs { id } committedOutputs { id } }
            independentDemands { id }
            nonProcessCommitments { id }
          }
        }
      `,
      variables: { id: fullPlan },
      fetchPolicy: 'no-cache',
    })
    const result = graph.data?.plan
    assert(result?.id === fullPlan, 'nested fanout plan did not resolve')
    assert(result.processes?.length === processCount, `expected ${processCount} processes, got ${result.processes?.length}`)
    assert(result.independentDemands?.length === independentDemandCount, `expected ${independentDemandCount} independent demands, got ${result.independentDemands?.length}`)
    assert(result.nonProcessCommitments?.length === nonProcessCount, `expected ${nonProcessCount} non-process commitments, got ${result.nonProcessCommitments?.length}`)
    for (const process of result.processes ?? []) {
      assert(processIds.includes(process.id), `nested query returned unknown process ${process.id}`)
      assert(process.committedInputs?.length === commitmentsPerDirection, `process ${process.id} has the wrong input fanout`)
      assert(process.committedOutputs?.length === commitmentsPerDirection, `process ${process.id} has the wrong output fanout`)
    }
    return `${processCount} processes and ${processCount * commitmentsPerDirection * 2 + independentDemandCount + nonProcessCount} commitments resolved in one graph`
  })

  // ── Collection queries that lost their only caller ──────────────────────
  await step('economicEvents collection lists events with their own fields', async () => {
    const q = await client.query({
      query: gql`query { economicEvents { edges { node { id revisionId action { id } } } } }`,
    })
    const nodes = (q.data?.economicEvents?.edges ?? []).map((e: any) => e.node)
    assert(nodes.length >= 1, 'economicEvents collection is empty despite events created in this run')
    assert(nodes.every((n: any) => n.id && n.revisionId && n.action?.id), 'an economicEvents node came back without id, revisionId or action')
    return `economicEvents listed ${nodes.length} events`
  })

  await step('spatialThings collection lists a created location', async () => {
    const st = (await client.mutate({
      mutation: gql`mutation ($s: SpatialThingCreateParams!) { res: createSpatialThing(spatialThing: $s) { spatialThing { id name } } }`,
      variables: { s: { name: 'Regression warehouse', mappableAddress: '1 Rue Test, Montreal', lat: 45.5, long: -73.6 } },
    })).data?.res?.spatialThing
    assert(st?.id, 'spatial thing id missing')
    const q = await client.query({ query: gql`query { spatialThings { edges { node { id name } } } }` })
    const ids = (q.data?.spatialThings?.edges ?? []).map((e: any) => e.node.id)
    assert(ids.includes(st.id), 'spatialThings collection does not list the created location')
    return 'spatialThings lists the new location'
  })

  await step('agreementBundles collection lists a created bundle with its name', async () => {
    const other = (await client.mutate({
      mutation: CREATE_AGREEMENT,
      variables: { a: { name: 'Second agreement' } },
    })).data?.res?.agreement?.id
    assert(other, 'second agreement id missing')
    const bundle = (await client.mutate({
      mutation: gql`
        mutation ($b: AgreementBundleCreateParams!) {
          res: createAgreementBundle(agreementBundle: $b) { agreementBundle { id name agreements { id } } }
        }
      `,
      variables: { b: { name: 'Winter bundle', agreements: [agreement, other] } },
    })).data?.res?.agreementBundle
    assert(bundle?.id, 'bundle id missing')
    assert(bundle.name === 'Winter bundle', `bundle name did not round-trip on create (got ${bundle.name})`)
    const q = await client.query({ query: gql`query { agreementBundles { edges { node { id name } } } }` })
    const found = (q.data?.agreementBundles?.edges ?? []).map((e: any) => e.node).find((n: any) => n.id === bundle.id)
    assert(found, 'agreementBundles collection does not list the created bundle')
    assert(found.name === 'Winter bundle', `bundle name is wrong in the collection (got ${found.name})`)
    return 'agreementBundles lists the bundle and its name'
  })

  // ── Relay backwards pagination on agents ────────────────────────────────
  await step('agents pagination preserves exact positional and cursor boundaries', async () => {
    const all = await client.query({
      query: gql`query { agents { edges { cursor node { id } } } }`,
      fetchPolicy: 'no-cache',
    })
    const edges = all.data?.agents?.edges ?? []
    assert(edges.length >= 4, `need at least 4 agents to exercise backwards paging, found ${edges.length}`)
    const allIds = edges.map((e: any) => e.node.id)

    const bounded = await client.query({
      query: gql`query { agents(first: 3, last: 5) { edges { node { id } } } }`,
      fetchPolicy: 'no-cache',
    })
    const boundedIds = (bounded.data?.agents?.edges ?? []).map((e: any) => e.node.id)
    assert(JSON.stringify(boundedIds) === JSON.stringify(allIds.slice(2, 5)), 'first=3,last=5 returned the wrong positional slice')

    const firstThree = await client.query({
      query: gql`query { agents(last: 3) { edges { node { id } } } }`,
      fetchPolicy: 'no-cache',
    })
    const firstThreeIds = (firstThree.data?.agents?.edges ?? []).map((e: any) => e.node.id)
    assert(JSON.stringify(firstThreeIds) === JSON.stringify(allIds.slice(0, 3)), 'last=3 returned the wrong positional slice')

    const beforeCursor = edges[edges.length - 1].cursor
    const before = await client.query({
      query: gql`query ($c: String!) { agents(before: $c) { edges { node { id } } } }`,
      variables: { c: beforeCursor },
      fetchPolicy: 'no-cache',
    })
    const head = (before.data?.agents?.edges ?? []).map((e: any) => e.node.id)
    assert(JSON.stringify(head) === JSON.stringify(allIds.slice(0, -1)), 'before cursor returned the wrong exact prefix')

    const afterCursor = edges[1].cursor
    const after = await client.query({
      query: gql`query ($c: String!) { agents(after: $c) { edges { node { id } } } }`,
      variables: { c: afterCursor },
      fetchPolicy: 'no-cache',
    })
    const afterIds = (after.data?.agents?.edges ?? []).map((e: any) => e.node.id)
    assert(JSON.stringify(afterIds) === JSON.stringify(allIds.slice(2)), 'after cursor returned the wrong exact suffix')

    const rangeEnd = Math.min(5, edges.length - 1)
    const between = await client.query({
      query: gql`query ($after: String!, $before: String!) { agents(after: $after, before: $before) { edges { node { id } } } }`,
      variables: { after: edges[1].cursor, before: edges[rangeEnd].cursor },
      fetchPolicy: 'no-cache',
    })
    const betweenIds = (between.data?.agents?.edges ?? []).map((e: any) => e.node.id)
    assert(JSON.stringify(betweenIds) === JSON.stringify(allIds.slice(2, rangeEnd)), 'after+before returned the wrong exact range')
    say('Positional and cursor bounds return the exact ordered agent slices.')
    return 'first+last, last, before, after and after+before preserve exact order'
  })
}
