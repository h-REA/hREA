import { gql, type ApolloClient, type NormalizedCacheObject } from '@apollo/client/core'
import { type Runner, expectRejection } from '../steps.js'

type Client = ApolloClient<NormalizedCacheObject>

const CREATE_PERSON = gql`
  mutation ($p: AgentCreateParams!) { res: createPerson(person: $p) { agent { id name } } }
`

/**
 * CRUD depth, collections and negatives: units, organizations, relationships,
 * updates, deletes, pagination, spatial bounds, bundles, temporal round-trips.
 */
export async function runCrudSuite(client: Client, r: Runner): Promise<void> {
  const { step, say, assert } = r

  // ── Unit CRUD ─────────────────────────────────────────────────────────────
  let unitId = '', unitRev = ''
  await step('unit create + read round-trip (label/symbol/omUnitIdentifier)', async () => {
    const res = await client.mutate({
      mutation: gql`mutation ($u: UnitCreateParams!) { res: createUnit(unit: $u) { unit { id revisionId label symbol omUnitIdentifier } } }`,
      variables: { u: { label: 'kilogram', symbol: 'kg', omUnitIdentifier: 'kilogram' } },
    })
    unitId = res.data?.res?.unit?.id
    unitRev = res.data?.res?.unit?.revisionId
    assert(unitId && unitRev, 'unit ids missing')
    assert(res.data.res.unit.symbol === 'kg', 'symbol did not round-trip')
    return 'unit created, fields round-trip'
  })

  await step('unit partial update: changed field persists, omitted fields survive', async () => {
    const res = await client.mutate({
      mutation: gql`mutation ($u: UnitUpdateParams!) { res: updateUnit(unit: $u) { unit { id revisionId label symbol omUnitIdentifier } } }`,
      variables: { u: { revisionId: unitRev, label: 'kilogramme' } },
    })
    assert(res.data?.res?.unit?.label === 'kilogramme', 'updated label not persisted')
    assert(res.data.res.unit.revisionId !== unitRev, 'revisionId did not advance on update')
    unitRev = res.data.res.unit.revisionId
    const q = await client.query({ query: gql`query ($id: ID!) { unit(id: $id) { label symbol omUnitIdentifier } }`, variables: { id: unitId } })
    assert(q.data?.unit?.label === 'kilogramme', 're-read does not show the update')
    assert(q.data.unit.symbol === 'kg', `omitted symbol was wiped by the partial update (got ${q.data.unit.symbol})`)
    assert(q.data.unit.omUnitIdentifier === 'kilogram', `omitted omUnitIdentifier was wiped (got ${q.data.unit.omUnitIdentifier})`)
    return 'sparse payload: label changed, symbol + omUnitIdentifier untouched'
  })

  await step('unit delete removes it from the units collection', async () => {
    await client.mutate({
      mutation: gql`mutation ($rev: ID!) { res: deleteUnit(revisionId: $rev) }`,
      variables: { rev: unitRev },
    })
    const q = await client.query({ query: gql`query { units { edges { node { id } } } }` })
    const ids = (q.data?.units?.edges ?? []).map((e: any) => e.node.id)
    assert(!ids.includes(unitId), 'deleted unit still listed')
    say('The kilogram came, got a French spelling, and left — full unit CRUD.')
    return 'deleted unit absent from collection'
  })

  // ── People / organizations partition ──────────────────────────────────────
  let org = '', person = ''
  await step('people/organizations/agents queries partition agent types', async () => {
    person = (await client.mutate({ mutation: CREATE_PERSON, variables: { p: { name: 'Crud Carol' } } })).data?.res?.agent?.id
    org = (await client.mutate({
      mutation: gql`mutation ($o: OrganizationCreateParams!) { res: createOrganization(organization: $o) { agent { id } } }`,
      variables: { o: { name: 'Bakery Coop', classifiedAs: ['https://example.org/coop'] } },
    })).data?.res?.agent?.id
    assert(person && org, 'agent ids missing')
    const people = (await client.query({ query: gql`query { people { edges { node { id } } } }` })).data?.people?.edges.map((e: any) => e.node.id)
    const orgs = (await client.query({ query: gql`query { organizations { edges { node { id } } } }` })).data?.organizations?.edges.map((e: any) => e.node.id)
    const agents = (await client.query({ query: gql`query { agents { edges { node { id } } } }` })).data?.agents?.edges.map((e: any) => e.node.id)
    assert(people.includes(person) && !people.includes(org), 'people partition wrong')
    assert(orgs.includes(org) && !orgs.includes(person), 'organizations partition wrong')
    assert(agents.includes(org) && agents.includes(person), 'agents union missing members')
    return `people=${people.length} orgs=${orgs.length} agents=${agents.length}, partitions clean`
  })

  // ── Agent relationships: KNOWN GAP tripwire ───────────────────────────────
  // The GraphQL schema advertises agentRelationship mutations/queries but the
  // DNA implements none of them (no entry types, no zome fns). This step
  // asserts the gap so it starts FAILING the day someone implements them —
  // at which point it must be rewritten as a real round-trip test.
  await step('KNOWN GAP: agentRelationship mutations advertised but unimplemented in the DNA', async () => {
    const msg = await expectRejection(client, gql`
      mutation ($r: AgentRelationshipRoleCreateParams!) { res: createAgentRelationshipRole(agentRelationshipRole: $r) { agentRelationshipRole { id } } }
    `, { r: { roleLabel: 'member of' } }, /./)
    say('Tripwire holds: the social-graph surface is schema-only for now (documented in the review report).')
    return 'gap documented and guarded'
  })

  // ── Update round-trips across entities ────────────────────────────────────
  await step('updateResourceSpecification / updateIntent / updateProposal persist changes', async () => {
    const spec = (await client.mutate({
      mutation: gql`mutation ($rs: ResourceSpecificationCreateParams!) { res: createResourceSpecification(resourceSpecification: $rs) { resourceSpecification { id revisionId } } }`,
      variables: { rs: { name: 'Widget', substitutable: false } },
    })).data?.res?.resourceSpecification
    const specUp = await client.mutate({
      mutation: gql`mutation ($rs: ResourceSpecificationUpdateParams!) { res: updateResourceSpecification(resourceSpecification: $rs) { resourceSpecification { name substitutable } } }`,
      variables: { rs: { revisionId: spec.revisionId, name: 'Widget v2', substitutable: true } },
    })
    assert(specUp.data?.res?.resourceSpecification?.name === 'Widget v2', 'spec name update lost')
    assert(specUp.data.res.resourceSpecification.substitutable === true, 'spec boolean update lost')

    const intent = (await client.mutate({
      mutation: gql`mutation ($i: IntentCreateParams!) { res: createIntent(intent: $i) { intent { id revisionId } } }`,
      variables: { i: { action: 'raise', name: 'Draft intent' } },
    })).data?.res?.intent
    const intentUp = await client.mutate({
      mutation: gql`mutation ($i: IntentUpdateParams!) { res: updateIntent(intent: $i) { intent { name note } } }`,
      variables: { i: { revisionId: intent.revisionId, name: 'Final intent', note: 'polished' } },
    })
    assert(intentUp.data?.res?.intent?.name === 'Final intent', 'intent update lost')

    const pubIntent = (await client.mutate({
      mutation: gql`mutation ($i: IntentCreateParams!) { res: createIntent(intent: $i) { intent { id } } }`,
      variables: { i: { action: 'raise', name: 'To publish' } },
    })).data?.res?.intent?.id
    const prop = (await client.mutate({
      mutation: gql`mutation ($p: ProposalCreateParams!) { res: createProposal(proposal: $p) { proposal { id revisionId } } }`,
      variables: { p: { name: 'Draft proposal', purpose: 'offer', publishes: [pubIntent] } },
    })).data?.res?.proposal
    const propUp = await client.mutate({
      mutation: gql`mutation ($p: ProposalUpdateParams!) { res: updateProposal(proposal: $p) { proposal { name purpose } } }`,
      variables: { p: { revisionId: prop.revisionId, name: 'Final proposal', publishes: [pubIntent], purpose: 'offer' } },
    })
    assert(propUp.data?.res?.proposal?.name === 'Final proposal', 'proposal update lost')
    assert(propUp.data.res.proposal.purpose === 'offer', 'unchanged purpose must survive update')
    return 'three entity updates persist (incl. same-purpose proposal update)'
  })

  // ── Delete round-trips ────────────────────────────────────────────────────
  await step('deleted proposal and intent disappear from collections', async () => {
    const i = (await client.mutate({
      mutation: gql`mutation ($i: IntentCreateParams!) { res: createIntent(intent: $i) { intent { id revisionId } } }`,
      variables: { i: { action: 'raise', name: 'Ephemeral intent' } },
    })).data?.res?.intent
    const p = (await client.mutate({
      mutation: gql`mutation ($p: ProposalCreateParams!) { res: createProposal(proposal: $p) { proposal { id revisionId } } }`,
      variables: { p: { name: 'Ephemeral proposal', publishes: [i.id] } },
    })).data?.res?.proposal
    await client.mutate({ mutation: gql`mutation ($rev: ID!) { res: deleteProposal(revisionId: $rev) }`, variables: { rev: p.revisionId } })
    await client.mutate({ mutation: gql`mutation ($rev: ID!) { res: deleteIntent(revisionId: $rev) }`, variables: { rev: i.revisionId } })
    const props = (await client.query({ query: gql`query { proposals { edges { node { id } } } }` })).data?.proposals?.edges.map((e: any) => e.node.id)
    const intents = (await client.query({ query: gql`query { intents { edges { node { id } } } }` })).data?.intents?.edges.map((e: any) => e.node.id)
    assert(!props.includes(p.id), 'deleted proposal still listed')
    assert(!intents.includes(i.id), 'deleted intent still listed')
    return 'both deletions took effect in collections'
  })

  // ── Pagination ────────────────────────────────────────────────────────────
  await step('pagination: cursor walk over agents visits all, no duplicates', async () => {
    for (let n = 0; n < 7; n++) {
      await client.mutate({ mutation: CREATE_PERSON, variables: { p: { name: `Page Agent ${n}` } } })
    }
    const seen = new Set<string>()
    let after: string | undefined
    let pages = 0
    for (;;) {
      const q: any = await client.query({
        query: gql`query ($first: Int, $after: String) { agents(first: $first, after: $after) { pageInfo { hasNextPage endCursor } edges { cursor node { id } } } }`,
        variables: { first: 4, after },
      })
      const edges = q.data?.agents?.edges ?? []
      for (const e of edges) {
        assert(!seen.has(e.node.id), `duplicate node across pages: ${e.node.id}`)
        seen.add(e.node.id)
      }
      pages++
      const pi = q.data?.agents?.pageInfo
      if (!pi?.hasNextPage || !pi?.endCursor || pages > 20) break
      after = pi.endCursor
    }
    assert(seen.size >= 7, `pagination walk saw only ${seen.size} agents`)
    say(`Walked ${pages} pages of agents (${seen.size} total), cursors clean, no duplicates.`)
    return `${seen.size} agents over ${pages} pages, no duplicates`
  })

  // ── SpatialThing bounds ───────────────────────────────────────────────────
  await step('spatialThing round-trip through the adapter', async () => {
    const ok = await client.mutate({
      mutation: gql`mutation ($s: SpatialThingCreateParams!) { res: createSpatialThing(spatialThing: $s) { spatialThing { id lat long mappableAddress } } }`,
      variables: { s: { name: 'Bakery', lat: 45.5, long: -73.6, mappableAddress: '123 Bread St, Montréal' } },
    })
    assert(ok.data?.res?.spatialThing?.id, 'valid spatial thing rejected')
    assert(Number(ok.data.res.spatialThing.lat) === 45.5, 'lat did not round-trip')
    say('Montréal round-trips through the adapter; the WGS84 bounds themselves are asserted in tests/sweettest.')
    return 'valid spatial thing round-trips'
  })

  // ── AgreementBundle membership ────────────────────────────────────────────
  await step('agreementBundle lists its member agreements', async () => {
    const a1 = (await client.mutate({
      mutation: gql`mutation ($a: AgreementCreateParams!) { res: createAgreement(agreement: $a) { agreement { id } } }`,
      variables: { a: { name: 'Order line A' } },
    })).data?.res?.agreement?.id
    const a2 = (await client.mutate({
      mutation: gql`mutation ($a: AgreementCreateParams!) { res: createAgreement(agreement: $a) { agreement { id } } }`,
      variables: { a: { name: 'Order line B' } },
    })).data?.res?.agreement?.id
    const bundle = (await client.mutate({
      mutation: gql`mutation ($b: AgreementBundleCreateParams!) { res: createAgreementBundle(agreementBundle: $b) { agreementBundle { id agreements { id } } } }`,
      variables: { b: { name: 'Order #42', agreements: [a1, a2] } },
    })).data?.res?.agreementBundle
    assert(bundle?.id, 'bundle id missing')
    const memberIds = (bundle.agreements ?? []).map((a: any) => a.id)
    assert(memberIds.includes(a1) && memberIds.includes(a2), 'bundle does not list both agreements')
    return 'bundle membership round-trip'
  })

  // ── Temporal round-trip ───────────────────────────────────────────────────
  await step('temporal fields round-trip on proposal and process', async () => {
    const begin = '2026-08-01T09:00:00.000Z', end = '2026-08-31T17:00:00.000Z'
    const pubIntent = (await client.mutate({
      mutation: gql`mutation ($i: IntentCreateParams!) { res: createIntent(intent: $i) { intent { id } } }`,
      variables: { i: { action: 'raise', name: 'August window' } },
    })).data?.res?.intent?.id
    const prop = await client.mutate({
      mutation: gql`mutation ($p: ProposalCreateParams!) { res: createProposal(proposal: $p) { proposal { hasBeginning hasEnd } } }`,
      variables: { p: { name: 'August offer', publishes: [pubIntent], hasBeginning: begin, hasEnd: end } },
    })
    assert(new Date(prop.data?.res?.proposal?.hasBeginning).toISOString() === begin, 'proposal hasBeginning drifted')
    assert(new Date(prop.data.res.proposal.hasEnd).toISOString() === end, 'proposal hasEnd drifted')
    const proc = await client.mutate({
      mutation: gql`mutation ($p: ProcessCreateParams!) { res: createProcess(process: $p) { process { hasBeginning hasEnd } } }`,
      variables: { p: { name: 'August production', hasBeginning: begin, hasEnd: end } },
    })
    assert(new Date(proc.data?.res?.process?.hasBeginning).toISOString() === begin, 'process hasBeginning drifted')
    return 'timestamps preserved to the millisecond'
  })

  // ── Negative action vocabulary on Commitment and Claim ────────────────────
  await step('the adapter surfaces an integrity rejection with its reason intact', async () => {
    // The vocabulary gate itself is asserted per entity in
    // tests/sweettest/tests/integrity_gate.rs. What this step is for is the
    // layer above: that a zome-side rejection still reads as its own message
    // by the time it comes back through GraphQL.
    const evt = (await client.mutate({
      mutation: gql`mutation ($e: EconomicEventCreateParams!) { res: createEconomicEvent(event: $e) { economicEvent { id } } }`,
      variables: { e: { action: 'raise', provider: person, receiver: person, resourceClassifiedAs: ['https://example.org/thing'], resourceQuantity: { hasNumericalValue: 1 } } },
    })).data?.res?.economicEvent?.id
    const msg = await expectRejection(client, gql`
      mutation ($c: ClaimCreateParams!) { res: createClaim(claim: $c) { claim { id } } }
    `, { c: { action: 'banana', triggeredBy: evt } }, 'is not a valid ValueFlows action')
    say(`The reason survives the trip up: ${msg.split('\n')[0]}`)
    return 'rejection reason reaches the GraphQL caller'
  })

  // ── Action vocabulary surface ─────────────────────────────────────────────
  await step('actions collection lists the VF vocabulary and each is fetchable by id', async () => {
    const all = await client.query({
      query: gql`query { actions { id label resourceEffect onhandEffect inputOutput pairsWith } }`,
    })
    const actions = all.data?.actions ?? []
    assert(actions.length > 0, 'actions collection is empty')
    const first = actions[0]
    assert(first?.id, 'first action has no id')
    const one = await client.query({
      query: gql`query ($id: ID!) { action(id: $id) { id resourceEffect } }`,
      variables: { id: first.id },
    })
    assert(one.data?.action?.id === first.id, 'action(id) returned a different action')
    assert(one.data.action.resourceEffect, 'action has no resourceEffect')
    return `${actions.length} actions listed, ${first.id} fetchable by id`
  })

  // ── VF 1.0 booleans on ResourceSpecification ──────────────────────────────
  await step('resourceSpecification substitutable/mediumOfExchange round-trip both ways', async () => {
    const make = (name: string, substitutable: boolean, mediumOfExchange: boolean) => client.mutate({
      mutation: gql`
        mutation ($rs: ResourceSpecificationCreateParams!) {
          res: createResourceSpecification(resourceSpecification: $rs) {
            resourceSpecification { id name substitutable mediumOfExchange }
          }
        }
      `,
      variables: { rs: { name, substitutable, mediumOfExchange } },
    })
    const usd = (await make('USD', true, true)).data?.res?.resourceSpecification
    assert(usd?.substitutable === true, 'substitutable did not round-trip true')
    assert(usd?.mediumOfExchange === true, 'mediumOfExchange did not round-trip true')
    const widget = (await make('Hand-carved widget', false, false)).data?.res?.resourceSpecification
    assert(widget?.substitutable === false, 'substitutable did not round-trip false')
    assert(widget?.mediumOfExchange === false, 'mediumOfExchange did not round-trip false')
    say('A currency and a one-off carving, told apart by two booleans.')
    return 'both booleans round-trip in both directions'
  })

  // ── Revision metadata ─────────────────────────────────────────────────────
  await step('meta.retrievedRevision advances on update and matches on re-read', async () => {
    const created = (await client.mutate({
      mutation: gql`
        mutation ($o: OrganizationCreateParams!) {
          res: createOrganization(organization: $o) {
            agent { id revisionId meta { retrievedRevision { id time } } }
          }
        }
      `,
      variables: { o: { name: 'Revision Co-op' } },
    })).data?.res?.agent
    assert(created?.meta?.retrievedRevision?.time, 'create carried no retrievedRevision time')

    const updated = (await client.mutate({
      mutation: gql`
        mutation ($o: OrganizationUpdateParams!) {
          res: updateOrganization(organization: $o) {
            agent { id revisionId meta { retrievedRevision { id time } } }
          }
        }
      `,
      variables: { o: { revisionId: created.revisionId, name: 'Revision Co-op renamed' } },
    })).data?.res?.agent
    const before = created.meta.retrievedRevision.time
    const after = updated?.meta?.retrievedRevision?.time
    assert(after > before, `retrievedRevision time did not advance on update (${before} -> ${after})`)

    const q = await client.query({
      query: gql`
        query { agents { edges { node { id meta { retrievedRevision { id time } } } } } }
      `,
    })
    const node = (q.data?.agents?.edges ?? []).map((e: any) => e.node).find((n: any) => n.id === created.id)
    assert(node, 'updated agent absent from the agents collection')
    assert(node.meta?.retrievedRevision?.time === after, 'collection read shows a different revision than the update')
    return 'revision metadata advances and stays consistent across reads'
  })

  // ── Delete paths not covered elsewhere ────────────────────────────────────
  await step('agreement and plan deletes remove them from their collections', async () => {
    const agreement = (await client.mutate({
      mutation: gql`mutation ($a: AgreementCreateParams!) { res: createAgreement(agreement: $a) { agreement { id revisionId } } }`,
      variables: { a: { name: 'Cancelled order' } },
    })).data?.res?.agreement
    const plan = (await client.mutate({
      mutation: gql`mutation ($p: PlanCreateParams!) { res: createPlan(plan: $p) { plan { id revisionId } } }`,
      variables: { p: { name: 'Abandoned plan' } },
    })).data?.res?.plan
    assert(agreement?.id && plan?.id, 'agreement or plan id missing')

    const deletedAgreement = (await client.mutate({
      mutation: gql`mutation ($rev: ID!) { res: deleteAgreement(revisionId: $rev) }`,
      variables: { rev: agreement.revisionId },
    })).data?.res
    const deletedPlan = (await client.mutate({
      mutation: gql`mutation ($rev: ID!) { res: deletePlan(revisionId: $rev) }`,
      variables: { rev: plan.revisionId },
    })).data?.res
    assert(deletedAgreement === true, 'deleteAgreement did not report success')
    assert(deletedPlan === true, 'deletePlan did not report success')

    const agreements = (await client.query({ query: gql`query { agreements { edges { node { id } } } }` }))
      .data?.agreements?.edges?.map((e: any) => e.node.id) ?? []
    const plans = (await client.query({ query: gql`query { plans { edges { node { id } } } }` }))
      .data?.plans?.edges?.map((e: any) => e.node.id) ?? []
    assert(!agreements.includes(agreement.id), 'deleted agreement still listed')
    assert(!plans.includes(plan.id), 'deleted plan still listed')
    return 'both deletes remove the record from its collection'
  })

  await step('agent delete removes the organization from the agents collection', async () => {
    const created = (await client.mutate({
      mutation: gql`mutation ($o: OrganizationCreateParams!) { res: createOrganization(organization: $o) { agent { id revisionId } } }`,
      variables: { o: { name: 'Dissolved Co-op' } },
    })).data?.res?.agent
    assert(created?.id, 'organization id missing')
    const deleted = (await client.mutate({
      mutation: gql`mutation ($rev: ID!) { res: deleteOrganization(revisionId: $rev) }`,
      variables: { rev: created.revisionId },
    })).data?.res
    assert(deleted === true, 'deleteOrganization did not report success')
    const q = await client.query({ query: gql`query { agents { edges { node { id } } } }` })
    const ids = (q.data?.agents?.edges ?? []).map((e: any) => e.node.id)
    assert(!ids.includes(created.id), 'deleted organization still listed among agents')
    say('The co-op dissolved, and the register forgot it.')
    return 'deleted organization absent from agents'
  })

  // ── Sparse updates across every entity that has a required field ──────────
  await step('sparse update leaves omitted fields alone on every entity with a required field', async () => {
    // Any entry struct with a non-Option field used to fail here: the extern
    // boundary decodes `entry: ReaX` before the merge runs, so a payload
    // carrying only the changed field died with a Deserialize wasm error.
    // Unit, Intent and EconomicEvent were fixed first; these seven followed.
    const survives = async (label: string, create: any, createVars: any, update: any, read: any,
                            pick: (d: any) => any, keptField: string, keptValue: any) => {
      const made = pick((await client.mutate({ mutation: create, variables: createVars })).data)
      assert(made?.revisionId, `${label}: create returned no revisionId`)
      const updated = pick((await client.mutate({
        mutation: update, variables: { u: { revisionId: made.revisionId, note: `${label} note` } },
      })).data)
      assert(updated?.id === made.id, `${label}: id changed on sparse update`)
      const back = (await client.query({ query: read, variables: { id: made.id } })).data?.res
      assert(back?.[keptField] === keptValue,
        `${label}: omitted ${keptField} was lost by the sparse update (got ${JSON.stringify(back?.[keptField])})`)
      assert(back?.note === `${label} note`, `${label}: the changed note did not persist`)
    }

    await survives('organization',
      gql`mutation ($c: OrganizationCreateParams!) { res: createOrganization(organization: $c) { agent { id revisionId } } }`,
      { c: { name: 'Sparse Co-op' } },
      gql`mutation ($u: OrganizationUpdateParams!) { res: updateOrganization(organization: $u) { agent { id revisionId } } }`,
      gql`query ($id: ID!) { res: organization(id: $id) { name note } }`,
      (d: any) => d?.res?.agent, 'name', 'Sparse Co-op')

    await survives('process',
      gql`mutation ($c: ProcessCreateParams!) { res: createProcess(process: $c) { process { id revisionId } } }`,
      { c: { name: 'Sparse Process' } },
      gql`mutation ($u: ProcessUpdateParams!) { res: updateProcess(process: $u) { process { id revisionId } } }`,
      gql`query ($id: ID!) { res: process(id: $id) { name note } }`,
      (d: any) => d?.res?.process, 'name', 'Sparse Process')

    await survives('processSpecification',
      gql`mutation ($c: ProcessSpecificationCreateParams!) { res: createProcessSpecification(processSpecification: $c) { processSpecification { id revisionId } } }`,
      { c: { name: 'Sparse Baking' } },
      gql`mutation ($u: ProcessSpecificationUpdateParams!) { res: updateProcessSpecification(processSpecification: $u) { processSpecification { id revisionId } } }`,
      gql`query ($id: ID!) { res: processSpecification(id: $id) { name note } }`,
      (d: any) => d?.res?.processSpecification, 'name', 'Sparse Baking')

    await survives('resourceSpecification',
      gql`mutation ($c: ResourceSpecificationCreateParams!) { res: createResourceSpecification(resourceSpecification: $c) { resourceSpecification { id revisionId } } }`,
      { c: { name: 'Sparse Widget' } },
      gql`mutation ($u: ResourceSpecificationUpdateParams!) { res: updateResourceSpecification(resourceSpecification: $u) { resourceSpecification { id revisionId } } }`,
      gql`query ($id: ID!) { res: resourceSpecification(id: $id) { name note } }`,
      (d: any) => d?.res?.resourceSpecification, 'name', 'Sparse Widget')

    await survives('spatialThing',
      gql`mutation ($c: SpatialThingCreateParams!) { res: createSpatialThing(spatialThing: $c) { spatialThing { id revisionId } } }`,
      { c: { name: 'Sparse Bakery', lat: 45.5, long: -73.6 } },
      gql`mutation ($u: SpatialThingUpdateParams!) { res: updateSpatialThing(spatialThing: $u) { spatialThing { id revisionId } } }`,
      gql`query ($id: ID!) { res: spatialThing(id: $id) { name note } }`,
      (d: any) => d?.res?.spatialThing, 'name', 'Sparse Bakery')

    // Claim and RecipeFlow keep an action rather than a name.
    const evt = (await client.mutate({
      mutation: gql`mutation ($e: EconomicEventCreateParams!) { res: createEconomicEvent(event: $e) { economicEvent { id } } }`,
      variables: { e: { action: 'produce', provider: person, receiver: person, resourceClassifiedAs: ['https://example.org/thing'], resourceQuantity: { hasNumericalValue: 1 } } },
    })).data?.res?.economicEvent?.id
    const claim = (await client.mutate({
      mutation: gql`mutation ($c: ClaimCreateParams!) { res: createClaim(claim: $c) { claim { id revisionId } } }`,
      variables: { c: { action: 'produce', triggeredBy: evt } },
    })).data?.res?.claim
    assert(claim?.revisionId, 'claim: create returned no revisionId')
    await client.mutate({
      mutation: gql`mutation ($u: ClaimUpdateParams!) { res: updateClaim(claim: $u) { claim { id revisionId } } }`,
      variables: { u: { revisionId: claim.revisionId, note: 'claim note' } },
    })
    const claimBack = (await client.query({
      query: gql`query ($id: ID!) { res: claim(id: $id) { action { id } note } }`, variables: { id: claim.id },
    })).data?.res
    assert(claimBack?.action?.id === 'produce', `claim: omitted action was lost (got ${JSON.stringify(claimBack?.action)})`)
    assert(claimBack?.note === 'claim note', 'claim: the changed note did not persist')

    const rspec = (await client.mutate({
      mutation: gql`mutation ($rs: ResourceSpecificationCreateParams!) { res: createResourceSpecification(resourceSpecification: $rs) { resourceSpecification { id } } }`,
      variables: { rs: { name: 'Flow spec' } },
    })).data?.res?.resourceSpecification?.id
    const flow = (await client.mutate({
      mutation: gql`mutation ($f: RecipeFlowCreateParams!) { res: createRecipeFlow(recipeFlow: $f) { recipeFlow { id revisionId } } }`,
      variables: { f: { action: 'raise', resourceConformsTo: rspec, note: 'before' } },
    })).data?.res?.recipeFlow
    assert(flow?.revisionId, 'recipeFlow: create returned no revisionId')
    await client.mutate({
      mutation: gql`mutation ($u: RecipeFlowUpdateParams!) { res: updateRecipeFlow(recipeFlow: $u) { recipeFlow { id revisionId } } }`,
      variables: { u: { revisionId: flow.revisionId, note: 'after' } },
    })
    const flowBack = (await client.query({
      query: gql`query ($id: ID!) { res: recipeFlow(id: $id) { action { id } note } }`, variables: { id: flow.id },
    })).data?.res
    assert(flowBack?.action?.id === 'raise', `recipeFlow: omitted action was lost (got ${JSON.stringify(flowBack?.action)})`)
    assert(flowBack?.note === 'after', 'recipeFlow: the changed note did not persist')

    say('Seven entities, each updated by one field, each keeping the rest.')
    return 'sparse updates preserve omitted fields on all seven'
  })
}
