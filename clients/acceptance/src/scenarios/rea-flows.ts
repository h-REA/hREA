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
const CREATE_EVENT_WITH_RESOURCE = gql`
  mutation ($e: EconomicEventCreateParams!, $r: EconomicResourceCreateParams) {
    res: createEconomicEvent(event: $e, newInventoriedResource: $r) {
      economicEvent { id action { id } }
      economicResource { id name accountingQuantity { hasNumericalValue } onhandQuantity { hasNumericalValue } }
    }
  }
`
const CREATE_EVENT = gql`
  mutation ($e: EconomicEventCreateParams!) {
    res: createEconomicEvent(event: $e) { economicEvent { id action { id } } }
  }
`
const GET_RESOURCE = gql`
  query ($id: ID!) {
    economicResource(id: $id) {
      id name
      accountingQuantity { hasNumericalValue }
      onhandQuantity { hasNumericalValue }
      conformsTo { id }
    }
  }
`

/**
 * Deep REA flows: resources born from events, accounting arithmetic, process
 * input/output, commitment fulfillment, intent satisfaction, agreements, plans.
 */
export async function runReaFlows(client: Client, r: Runner): Promise<void> {
  const { step, say, assert } = r

  let alice = '', bob = '', spec = ''
  await step('REA setup: two agents and a resource specification', async () => {
    alice = (await client.mutate({ mutation: CREATE_PERSON, variables: { p: { name: 'Rea Alice' } } })).data?.res?.agent?.id
    bob = (await client.mutate({ mutation: CREATE_PERSON, variables: { p: { name: 'Rea Bob' } } })).data?.res?.agent?.id
    spec = (await client.mutate({ mutation: CREATE_SPEC, variables: { rs: { name: 'Sourdough loaf' } } })).data?.res?.resourceSpecification?.id
    assert(alice && bob && spec, 'setup ids missing')
    return 'agents + spec created'
  })

  // ── EconomicResource born from a produce event ────────────────────────────
  let loafResource = ''
  await step('produce event creates an inventoried EconomicResource', async () => {
    const res = await client.mutate({
      mutation: CREATE_EVENT_WITH_RESOURCE,
      variables: {
        e: { action: 'produce', provider: alice, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 10 }, hasPointInTime: new Date('2026-07-03T08:00:00Z').toISOString() },
        r: { name: 'Batch #1 loaves' },
      },
    })
    loafResource = res.data?.res?.economicResource?.id
    assert(loafResource, 'no economicResource returned alongside the produce event')
    assert(res.data.res.economicResource.name === 'Batch #1 loaves', 'resource name did not round-trip')
    say('Alice baked: a produce event birthed "Batch #1 loaves" into inventory.')
    return `resource ${loafResource.slice(0, 12)}… created by event`
  })

  await step('explicit conformsTo on the resource params is NOT overridden by event inheritance', async () => {
    const otherSpec = (await client.mutate({ mutation: CREATE_SPEC, variables: { rs: { name: 'Rye loaf' } } })).data?.res?.resourceSpecification?.id
    assert(otherSpec, 'other spec id missing')
    const res = await client.mutate({
      mutation: gql`
        mutation ($e: EconomicEventCreateParams!, $r: EconomicResourceCreateParams) {
          res: createEconomicEvent(event: $e, newInventoriedResource: $r) {
            economicResource { id conformsTo { id } }
          }
        }
      `,
      variables: {
        e: { action: 'produce', provider: alice, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 1 } },
        r: { name: 'Rye batch', conformsTo: otherSpec },
      },
    })
    const got = res.data?.res?.economicResource?.conformsTo?.id
    assert(got === otherSpec, `explicit conformsTo was overridden: expected the resource's own spec, got ${got === spec ? "the event's spec" : got}`)
    return 'explicit conformsTo wins; inheritance only fills the gap'
  })

  await step('economicResource(id) + economicResources list reflect the new resource', async () => {
    const one = await client.query({ query: GET_RESOURCE, variables: { id: loafResource } })
    assert(one.data?.economicResource?.id === loafResource, 'economicResource(id) lookup failed')
    assert(one.data.economicResource.conformsTo?.id === spec, 'conformsTo does not point at the specification')
    const all = await client.query({ query: gql`query { economicResources { edges { node { id } } } }` })
    const ids = (all.data?.economicResources?.edges ?? []).map((e: any) => e.node.id)
    assert(ids.includes(loafResource), 'economicResources list misses the resource')
    return 'single + collection queries agree'
  })

  // ── Accounting arithmetic across the event sequence ───────────────────────
  await step('accounting arithmetic: produce 10 → raise 5 → lower 3 → consume 4 leaves 8', async () => {
    const evt = (action: string, n: number) => client.mutate({
      mutation: CREATE_EVENT,
      variables: { e: { action, provider: alice, receiver: alice, resourceInventoriedAs: loafResource, resourceQuantity: { hasNumericalValue: n } } },
    })
    await evt('raise', 5)
    await evt('lower', 3)
    await evt('consume', 4)
    const q = await client.query({ query: GET_RESOURCE, variables: { id: loafResource } })
    const acc = q.data?.economicResource?.accountingQuantity?.hasNumericalValue
    const onhand = q.data?.economicResource?.onhandQuantity?.hasNumericalValue
    assert(Number(acc) === 8, `accountingQuantity expected 8, got ${acc}`)
    assert(Number(onhand) === 8, `onhandQuantity expected 8, got ${onhand}`)
    say('The ledger holds: 10 baked + 5 found − 3 lost − 4 eaten = 8 loaves on both quantities.')
    return `accounting=8 onhand=8 after 4-event sequence`
  })

  // ── Process with observed input and output ────────────────────────────────
  await step('process observedInputs/observedOutputs resolve around consume/produce events', async () => {
    const proc = (await client.mutate({
      mutation: gql`mutation ($p: ProcessCreateParams!) { res: createProcess(process: $p) { process { id } } }`,
      variables: { p: { name: 'Bake bread' } },
    })).data?.res?.process?.id
    assert(proc, 'process id missing')
    const input = (await client.mutate({
      mutation: CREATE_EVENT,
      variables: { e: { action: 'consume', provider: alice, receiver: alice, resourceInventoriedAs: loafResource, resourceQuantity: { hasNumericalValue: 1 }, inputOf: proc } },
    })).data?.res?.economicEvent?.id
    const output = (await client.mutate({
      mutation: CREATE_EVENT_WITH_RESOURCE,
      variables: {
        e: { action: 'produce', provider: alice, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 2 }, outputOf: proc },
        r: { name: 'Croutons' },
      },
    })).data?.res?.economicEvent?.id
    assert(input && output, 'process event ids missing')
    const q = await client.query({
      query: gql`query ($id: ID!) { process(id: $id) { observedInputs { id } observedOutputs { id } } }`,
      variables: { id: proc },
    })
    const ins = (q.data?.process?.observedInputs ?? []).map((e: any) => e.id)
    const outs = (q.data?.process?.observedOutputs ?? []).map((e: any) => e.id)
    assert(ins.includes(input), 'observedInputs misses the consume event')
    assert(outs.includes(output), 'observedOutputs misses the produce event')
    say('A process consumed a loaf and produced croutons — both flows resolve from the process.')
    return 'process input/output round-trip'
  })

  // ── Commitment → fulfillment ──────────────────────────────────────────────
  let agreement = '', commitment = ''
  await step('commitment fulfilledBy: event fulfills commitment, reverse resolves', async () => {
    agreement = (await client.mutate({
      mutation: gql`mutation ($a: AgreementCreateParams!) { res: createAgreement(agreement: $a) { agreement { id } } }`,
      variables: { a: { name: 'Bread subscription' } },
    })).data?.res?.agreement?.id
    assert(agreement, 'agreement id missing')
    commitment = (await client.mutate({
      mutation: gql`mutation ($c: CommitmentCreateParams!) { res: createCommitment(commitment: $c) { commitment { id } } }`,
      variables: { c: { action: 'transfer', provider: alice, receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 2 }, clauseOf: agreement, due: new Date('2026-07-10T12:00:00Z').toISOString() } },
    })).data?.res?.commitment?.id
    assert(commitment, 'commitment id missing')
    const evt = (await client.mutate({
      mutation: gql`mutation ($e: EconomicEventCreateParams!) { res: createEconomicEvent(event: $e) { economicEvent { id } } }`,
      variables: { e: { action: 'transfer', provider: alice, receiver: bob, resourceInventoriedAs: loafResource, resourceQuantity: { hasNumericalValue: 2 }, fulfills: [commitment], realizationOf: agreement } },
    })).data?.res?.economicEvent?.id
    assert(evt, 'fulfillment event id missing')
    const q = await client.query({
      query: gql`query ($id: ID!) { commitment(id: $id) { fulfilledBy { id } clauseOf { id } } }`,
      variables: { id: commitment },
    })
    const fulfilled = (q.data?.commitment?.fulfilledBy ?? []).map((e: any) => e.id)
    assert(fulfilled.includes(evt), 'commitment.fulfilledBy misses the event')
    assert(q.data.commitment.clauseOf?.id === agreement, 'commitment.clauseOf does not resolve the agreement')
    say('Alice promised 2 loaves to Bob and delivered: the commitment shows its fulfilling event.')
    return 'fulfills → fulfilledBy round-trip'
  })

  // ── Intent → satisfaction ─────────────────────────────────────────────────
  await step('intent satisfiedBy: commitment satisfies intent, reverse resolves', async () => {
    const intent = (await client.mutate({
      mutation: gql`mutation ($i: IntentCreateParams!) { res: createIntent(intent: $i) { intent { id } } }`,
      variables: { i: { action: 'transfer', name: 'Bob wants bread', receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 2 } } },
    })).data?.res?.intent?.id
    assert(intent, 'intent id missing')
    const c2 = (await client.mutate({
      mutation: gql`mutation ($c: CommitmentCreateParams!) { res: createCommitment(commitment: $c) { commitment { id } } }`,
      variables: { c: { action: 'transfer', provider: alice, receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 2 }, satisfies: intent } },
    })).data?.res?.commitment?.id
    assert(c2, 'satisfying commitment id missing')
    const q = await client.query({
      query: gql`query ($id: ID!) { intent(id: $id) { name satisfiedBy { id } } }`,
      variables: { id: intent },
    })
    const sat = (q.data?.intent?.satisfiedBy ?? []).map((c: any) => c.id)
    assert(sat.includes(c2), 'intent.satisfiedBy misses the commitment')
    assert(q.data?.intent?.name === 'Bob wants bread', `Intent.name did not round-trip (got ${q.data?.intent?.name}) — the zome silently dropped it`)
    say('Bob\'s want got a promise: intent.satisfiedBy lists the commitment.')
    return 'satisfies → satisfiedBy round-trip'
  })

  // ── Agreement reverse fields ──────────────────────────────────────────────
  await step('agreement.commitments and agreement.economicEvents resolve', async () => {
    const q = await client.query({
      query: gql`query ($id: ID!) { agreement(id: $id) { commitments { id } economicEvents { id } } }`,
      variables: { id: agreement },
    })
    const cs = (q.data?.agreement?.commitments ?? []).map((c: any) => c.id)
    const es = q.data?.agreement?.economicEvents ?? []
    assert(cs.includes(commitment), 'agreement.commitments misses the clause commitment')
    assert(es.length >= 1, 'agreement.economicEvents empty despite realizationOf event')
    return `commitments=${cs.length} events=${es.length}`
  })

  // ── Plan wiring ───────────────────────────────────────────────────────────
  await step('plan: plannedWithin process + independentDemandOf commitment resolve from plan', async () => {
    const plan = (await client.mutate({
      mutation: gql`mutation ($p: PlanCreateParams!) { res: createPlan(plan: $p) { plan { id } } }`,
      variables: { p: { name: 'Q3 bakery plan' } },
    })).data?.res?.plan?.id
    assert(plan, 'plan id missing')
    const proc = (await client.mutate({
      mutation: gql`mutation ($p: ProcessCreateParams!) { res: createProcess(process: $p) { process { id } } }`,
      variables: { p: { name: 'Scale up ovens', plannedWithin: plan } },
    })).data?.res?.process?.id
    const demand = (await client.mutate({
      mutation: gql`mutation ($c: CommitmentCreateParams!) { res: createCommitment(commitment: $c) { commitment { id } } }`,
      variables: { c: { action: 'transfer', provider: alice, receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 20 }, independentDemandOf: plan } },
    })).data?.res?.commitment?.id
    assert(proc && demand, 'plan member ids missing')
    const q = await client.query({
      query: gql`query ($id: ID!) { plan(id: $id) { processes { id } independentDemands { id } } }`,
      variables: { id: plan },
    })
    const procs = (q.data?.plan?.processes ?? []).map((p: any) => p.id)
    const demands = (q.data?.plan?.independentDemands ?? []).map((c: any) => c.id)
    assert(procs.includes(proc), 'plan.processes misses the process')
    assert(demands.includes(demand), 'plan.independentDemands misses the commitment')
    say('The Q3 plan knows its process and its independent demand.')
    return 'plan reverse fields round-trip'
  })

  // ── Claim ─────────────────────────────────────────────────────────────────
  await step('claim: created against a past event, triggeredBy resolves, listed in claims', async () => {
    const event = (await client.mutate({
      mutation: CREATE_EVENT,
      variables: { e: { action: 'produce', provider: alice, receiver: alice, note: 'the triggering event' } },
    })).data?.res?.economicEvent?.id
    assert(event, 'triggering event id missing')

    const claim = (await client.mutate({
      mutation: gql`
        mutation ($c: ClaimCreateParams!) {
          res: createClaim(claim: $c) { claim { id action { id } triggeredBy { id } note finished } }
        }
      `,
      variables: { c: { action: 'produce', triggeredBy: event, note: 'reciprocity claim', finished: false } },
    })).data?.res?.claim
    assert(claim?.id, 'claim id missing')
    assert(claim.action?.id === 'produce', 'claim action did not round-trip')
    assert(claim.triggeredBy?.id === event, 'triggeredBy does not resolve to the triggering event')
    assert(claim.note === 'reciprocity claim', 'claim note did not round-trip')
    assert(claim.finished === false, 'claim finished did not round-trip')

    const q = await client.query({ query: gql`query { claims { edges { node { id } } } }` })
    const ids = (q.data?.claims?.edges ?? []).map((e: any) => e.node.id)
    assert(ids.includes(claim.id), 'claims collection misses the new claim')
    say('A claim points back at the event that earned it.')
    return 'claim round-trips and is listed'
  })

  // ── Update paths on the observation side ──────────────────────────────────
  await step('economicEvent and economicResource updates persist and advance revisions', async () => {
    const created = (await client.mutate({
      mutation: gql`
        mutation ($e: EconomicEventCreateParams!, $r: EconomicResourceCreateParams) {
          res: createEconomicEvent(event: $e, newInventoriedResource: $r) {
            economicEvent { id revisionId note }
            economicResource { id revisionId name note }
          }
        }
      `,
      variables: {
        e: { action: 'produce', provider: alice, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 3 }, note: 'first pass' },
        r: { name: 'Update batch' },
      },
    })).data?.res
    const event = created?.economicEvent
    const resource = created?.economicResource
    assert(event?.id && resource?.id, 'event or resource id missing')

    const updatedEvent = (await client.mutate({
      mutation: gql`
        mutation ($e: EconomicEventUpdateParams!) {
          res: updateEconomicEvent(event: $e) { economicEvent { id revisionId note } }
        }
      `,
      variables: { e: { revisionId: event.revisionId, note: 'second pass' } },
    })).data?.res?.economicEvent
    assert(updatedEvent?.id === event.id, 'event id changed on update')
    assert(updatedEvent.revisionId !== event.revisionId, 'event revisionId did not advance')
    assert(updatedEvent.note === 'second pass', 'event note did not update')

    const updatedResource = (await client.mutate({
      mutation: gql`
        mutation ($r: EconomicResourceUpdateParams!) {
          res: updateEconomicResource(resource: $r) { economicResource { id revisionId name note } }
        }
      `,
      variables: { r: { revisionId: resource.revisionId, note: 'counted by hand' } },
    })).data?.res?.economicResource
    assert(updatedResource?.id === resource.id, 'resource id changed on update')
    assert(updatedResource.revisionId !== resource.revisionId, 'resource revisionId did not advance')
    assert(updatedResource.note === 'counted by hand', 'resource note did not update')
    assert(updatedResource.name === 'Update batch', 'omitted name was wiped by the partial update')
    say('Both sides of an observation can be corrected after the fact.')
    return 'event and resource updates persist, revisions advance, omitted fields survive'
  })
}
