import { gql, type ApolloClient, type NormalizedCacheObject } from '@apollo/client/core'
import { type Runner } from '../steps.js'

type Client = ApolloClient<NormalizedCacheObject>

/**
 * Forward references, read back as objects.
 *
 * Every other module in this battery reads *reverse* links: `Plan.processes`,
 * `Commitment.fulfilledBy`, `Agreement.commitments`. Forward references were
 * only ever written, as mutation inputs, and never selected back. That left a
 * blind spot with a precise shape: a forward-reference resolver can name the
 * wrong entity type and nothing goes red, because `get_latest_rea_<type>` walks
 * update links generically and `formatResItem` decodes whatever entry it finds.
 * The result is a Process served under the `EconomicEvent` GraphQL type, which
 * is silently wrong data rather than an error.
 *
 * Two live defects were found that way and are asserted below: `containedIn`
 * resolved to null forever, and `organization(id)` returned a Person. Both were
 * confirmed by running this module against the unfixed adapter first.
 *
 * A third finding is recorded here rather than dressed up as a test. The
 * `EconomicEvent` resolvers for `inputOf`, `outputOf` and `resourceConformsTo`
 * passed a different entity string than the identical `Commitment` fields do,
 * naming `economic_event` where the value is a Process and `economic_resource`
 * where it is a ResourceSpecification. That reads like data corruption and is
 * not, today: `get_latest_rea_<type>` walks update links generically and
 * `formatResItem` decodes whatever entry it lands on, so the right record comes
 * back through the wrong door. Measured, not assumed: with the old strings in
 * place, the three checks below pass. They are kept because the wiring is still
 * wrong and would start returning nulls the moment those getters diverge, but
 * they are not evidence of a bug that was caught, and should not be read that
 * way.
 */
export async function runForwardRefs(client: Client, { step, say, assert }: Runner) {
  let alice: string, bob: string, org: string
  let spec: string, unit: string, procIn: string, procOut: string

  await step('forward-ref setup: two people, an organization, a specification', async () => {
    alice = (await client.mutate({
      mutation: gql`mutation ($p: AgentCreateParams!) { res: createPerson(person: $p) { agent { id } } }`,
      variables: { p: { name: 'Alice Forward' } },
    })).data?.res?.agent?.id
    bob = (await client.mutate({
      mutation: gql`mutation ($p: AgentCreateParams!) { res: createPerson(person: $p) { agent { id } } }`,
      variables: { p: { name: 'Bob Forward' } },
    })).data?.res?.agent?.id
    org = (await client.mutate({
      mutation: gql`mutation ($o: OrganizationCreateParams!) { res: createOrganization(organization: $o) { agent { id } } }`,
      variables: { o: { name: 'Forward Co-op' } },
    })).data?.res?.agent?.id
    unit = (await client.mutate({
      mutation: gql`mutation ($u: UnitCreateParams!) { res: createUnit(unit: $u) { unit { id } } }`,
      variables: { u: { label: 'forward kilo', symbol: 'fkg', omUnitIdentifier: 'kilogram' } },
    })).data?.res?.unit?.id
    spec = (await client.mutate({
      mutation: gql`mutation ($rs: ResourceSpecificationCreateParams!) {
        res: createResourceSpecification(resourceSpecification: $rs) { resourceSpecification { id } } }`,
      variables: { rs: { name: 'Forward widget spec' } },
    })).data?.res?.resourceSpecification?.id
    procIn = (await client.mutate({
      mutation: gql`mutation ($p: ProcessCreateParams!) { res: createProcess(process: $p) { process { id } } }`,
      variables: { p: { name: 'Forward input process' } },
    })).data?.res?.process?.id
    procOut = (await client.mutate({
      mutation: gql`mutation ($p: ProcessCreateParams!) { res: createProcess(process: $p) { process { id } } }`,
      variables: { p: { name: 'Forward output process' } },
    })).data?.res?.process?.id
    assert(alice && bob && org && unit && spec && procIn && procOut, 'forward-ref setup ids missing')
    return 'agents, unit, specification and two processes created'
  })

  // ── EconomicEvent.inputOf / .outputOf must resolve to a Process ───────────
  await step('economicEvent inputOf and outputOf resolve to the Process, not to an event', async () => {
    const evtIn = (await client.mutate({
      mutation: gql`mutation ($e: EconomicEventCreateParams!) {
        res: createEconomicEvent(event: $e) { economicEvent { id } } }`,
      variables: { e: { action: 'consume', provider: alice, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 2, hasUnit: unit }, inputOf: procIn } },
    })).data?.res?.economicEvent?.id
    assert(evtIn, 'inputOf event id missing')

    const evtOut = (await client.mutate({
      mutation: gql`mutation ($e: EconomicEventCreateParams!) {
        res: createEconomicEvent(event: $e) { economicEvent { id } } }`,
      variables: { e: { action: 'produce', provider: alice, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 5, hasUnit: unit }, outputOf: procOut } },
    })).data?.res?.economicEvent?.id
    assert(evtOut, 'outputOf event id missing')

    // This pins the intended shape rather than catching a regression: see the
    // module comment on why a wrong entity string still returns the right
    // record through the generic getter.
    const a = await client.query({
      query: gql`query ($id: ID!) { economicEvent(id: $id) { id inputOf { id name } } }`,
      variables: { id: evtIn },
    })
    assert(a.data?.economicEvent?.inputOf?.id === procIn,
      `economicEvent.inputOf points at ${a.data?.economicEvent?.inputOf?.id}, not the process ${procIn}`)
    assert(a.data?.economicEvent?.inputOf?.name === 'Forward input process',
      `economicEvent.inputOf resolved something without the process name (got ${JSON.stringify(a.data?.economicEvent?.inputOf?.name)})`)

    const b = await client.query({
      query: gql`query ($id: ID!) { economicEvent(id: $id) { id outputOf { id name } } }`,
      variables: { id: evtOut },
    })
    assert(b.data?.economicEvent?.outputOf?.id === procOut,
      `economicEvent.outputOf points at ${b.data?.economicEvent?.outputOf?.id}, not the process ${procOut}`)
    assert(b.data?.economicEvent?.outputOf?.name === 'Forward output process',
      `economicEvent.outputOf resolved something without the process name (got ${JSON.stringify(b.data?.economicEvent?.outputOf?.name)})`)

    say('An event names the process it feeds, and reading that reference back returns the process itself.')
    return 'inputOf and outputOf both resolve to the named Process'
  })

  // ── EconomicEvent.resourceConformsTo must be a ResourceSpecification ──────
  await step('economicEvent resourceConformsTo resolves to the ResourceSpecification', async () => {
    const evt = (await client.mutate({
      mutation: gql`mutation ($e: EconomicEventCreateParams!) {
        res: createEconomicEvent(event: $e) { economicEvent { id } } }`,
      variables: { e: { action: 'transfer', provider: alice, receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 1, hasUnit: unit } } },
    })).data?.res?.economicEvent?.id
    assert(evt, 'event id missing')

    const q = await client.query({
      query: gql`query ($id: ID!) { economicEvent(id: $id) { id resourceConformsTo { id name } } }`,
      variables: { id: evt },
    })
    assert(q.data?.economicEvent?.resourceConformsTo?.id === spec,
      `resourceConformsTo points at ${q.data?.economicEvent?.resourceConformsTo?.id}, not the spec ${spec}`)
    assert(q.data?.economicEvent?.resourceConformsTo?.name === 'Forward widget spec',
      `resourceConformsTo resolved something that is not the specification (got ${JSON.stringify(q.data?.economicEvent?.resourceConformsTo?.name)})`)
    return 'resourceConformsTo resolves to the specification, by name'
  })

  // ── EconomicEvent.provider / .receiver, and the same pair on Commitment ───
  await step('provider and receiver resolve to the named agents on both event and commitment', async () => {
    const evt = (await client.mutate({
      mutation: gql`mutation ($e: EconomicEventCreateParams!) {
        res: createEconomicEvent(event: $e) { economicEvent { id } } }`,
      variables: { e: { action: 'transfer', provider: alice, receiver: bob, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 4, hasUnit: unit } } },
    })).data?.res?.economicEvent?.id

    const e = await client.query({
      query: gql`query ($id: ID!) { economicEvent(id: $id) { provider { id name } receiver { id name } } }`,
      variables: { id: evt },
    })
    assert(e.data?.economicEvent?.provider?.name === 'Alice Forward',
      `event provider resolved to ${JSON.stringify(e.data?.economicEvent?.provider?.name)}`)
    assert(e.data?.economicEvent?.receiver?.name === 'Bob Forward',
      `event receiver resolved to ${JSON.stringify(e.data?.economicEvent?.receiver?.name)}`)

    const com = (await client.mutate({
      mutation: gql`mutation ($c: CommitmentCreateParams!) {
        res: createCommitment(commitment: $c) { commitment { id } } }`,
      variables: { c: { action: 'transfer', provider: bob, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: 6, hasUnit: unit }, inputOf: procIn } },
    })).data?.res?.commitment?.id

    const c = await client.query({
      query: gql`query ($id: ID!) { commitment(id: $id) { provider { id name } receiver { id name } inputOf { id name } resourceConformsTo { id name } } }`,
      variables: { id: com },
    })
    assert(c.data?.commitment?.provider?.name === 'Bob Forward',
      `commitment provider resolved to ${JSON.stringify(c.data?.commitment?.provider?.name)}`)
    assert(c.data?.commitment?.receiver?.name === 'Alice Forward',
      `commitment receiver resolved to ${JSON.stringify(c.data?.commitment?.receiver?.name)}`)
    assert(c.data?.commitment?.inputOf?.name === 'Forward input process',
      `commitment inputOf resolved to ${JSON.stringify(c.data?.commitment?.inputOf?.name)}`)
    assert(c.data?.commitment?.resourceConformsTo?.name === 'Forward widget spec',
      `commitment resourceConformsTo resolved to ${JSON.stringify(c.data?.commitment?.resourceConformsTo?.name)}`)
    return 'all four agent and specification references resolve by name'
  })

  // ── EconomicResource.containedIn is a single resource, camelCased ─────────
  await step('economicResource containedIn resolves to the containing resource', async () => {
    const mk = async (qty: number) => {
      const r = await client.mutate({
        mutation: gql`mutation ($e: EconomicEventCreateParams!, $r: EconomicResourceCreateParams) {
          res: createEconomicEvent(event: $e, newInventoriedResource: $r) {
            economicEvent { id } economicResource { id } } }`,
        variables: {
          e: { action: 'raise', provider: alice, receiver: alice, resourceConformsTo: spec, resourceQuantity: { hasNumericalValue: qty, hasUnit: unit } },
          r: { name: `forward container ${qty}` },
        },
      })
      return r.data?.res?.economicResource?.id
    }
    const outer = await mk(10)
    const inner = await mk(3)
    assert(outer && inner, `resource ids missing (outer=${outer} inner=${inner})`)

    const rev = (await client.query({
      query: gql`query ($id: ID!) { economicResource(id: $id) { id revisionId } }`,
      variables: { id: inner },
    })).data?.economicResource?.revisionId
    assert(rev, 'inner resource revisionId missing')

    await client.mutate({
      mutation: gql`mutation ($r: EconomicResourceUpdateParams!) {
        res: updateEconomicResource(resource: $r) { economicResource { id } } }`,
      variables: { r: { revisionId: rev, containedIn: outer } },
    })

    const q = await client.query({
      query: gql`query ($id: ID!) { economicResource(id: $id) { id containedIn { id } } }`,
      variables: { id: inner },
    })
    // The schema declares containedIn as a single EconomicResource. The
    // resolver used to read record.contained_in, a key snakeToCamel had
    // already renamed, so this was permanently null.
    assert(q.data?.economicResource?.containedIn?.id === outer,
      `containedIn resolved to ${JSON.stringify(q.data?.economicResource?.containedIn)}, expected the outer resource ${outer}`)
    say('A resource placed inside another one reports its container.')
    return 'containedIn resolves to the container'
  })

  // ── organization(id) and person(id) must actually discriminate ────────────
  await step('organization and person queries accept their own kind and reject the other', async () => {
    const o = await client.query({
      query: gql`query ($id: ID!) { organization(id: $id) { id name } }`,
      variables: { id: org },
    })
    assert(o.data?.organization?.name === 'Forward Co-op',
      `organization(id) did not return the organization (got ${JSON.stringify(o.data?.organization)})`)

    const p = await client.query({
      query: gql`query ($id: ID!) { person(id: $id) { id name } }`,
      variables: { id: alice },
    })
    assert(p.data?.person?.name === 'Alice Forward',
      `person(id) did not return the person (got ${JSON.stringify(p.data?.person)})`)

    // The guards used to compare against a lowercase agentType the zome never
    // writes, and to return on mismatch rather than throw. Both bugs cancelled,
    // so organization(id) happily returned a Person. These two checks are the
    // ones that go red if either half comes back.
    let orgOnPerson: string | null = null
    try {
      const bad = await client.query({
        query: gql`query ($id: ID!) { organization(id: $id) { id name } }`,
        variables: { id: alice },
        fetchPolicy: 'no-cache',
      })
      if (bad.errors?.length) orgOnPerson = bad.errors[0].message
    } catch (e: any) { orgOnPerson = e.message ?? String(e) }
    assert(orgOnPerson !== null, 'organization(id) returned a Person instead of rejecting it')
    assert(/not an organization/i.test(orgOnPerson), `organization(id) rejected a Person for the wrong reason: ${orgOnPerson}`)

    let personOnOrg: string | null = null
    try {
      const bad = await client.query({
        query: gql`query ($id: ID!) { person(id: $id) { id name } }`,
        variables: { id: org },
        fetchPolicy: 'no-cache',
      })
      if (bad.errors?.length) personOnOrg = bad.errors[0].message
    } catch (e: any) { personOnOrg = e.message ?? String(e) }
    assert(personOnOrg !== null, 'person(id) returned an Organization instead of rejecting it')
    assert(/not a person/i.test(personOnOrg), `person(id) rejected an Organization for the wrong reason: ${personOnOrg}`)

    say('Asking for an organization by a person id is refused, and the other way round.')
    return 'both agent queries discriminate, in both directions'
  })
}
