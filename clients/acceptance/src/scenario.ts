import { gql, type ApolloClient, type NormalizedCacheObject } from '@apollo/client/core'
import { type Runner, expectRejection } from './steps.js'

type Client = ApolloClient<NormalizedCacheObject>

const CREATE_PERSON = gql`
  mutation ($p: AgentCreateParams!) {
    res: createPerson(person: $p) { agent { id name } }
  }
`
const CREATE_RESOURCE_SPEC = gql`
  mutation ($rs: ResourceSpecificationCreateParams!) {
    res: createResourceSpecification(resourceSpecification: $rs) {
      resourceSpecification { id name substitutable mediumOfExchange }
    }
  }
`
const CREATE_INTENT = gql`
  mutation ($i: IntentCreateParams!) {
    res: createIntent(intent: $i) { intent { id action { id } } }
  }
`
const CREATE_PROPOSAL = gql`
  mutation ($p: ProposalCreateParams!) {
    res: createProposal(proposal: $p) {
      proposal { id name note purpose publishes { id } reciprocal { id } }
    }
  }
`
const GET_PROPOSAL = gql`
  query ($id: ID!) {
    proposal(id: $id) { id name note purpose publishes { id } }
  }
`
const OFFERS = gql`
  query { offers { edges { node { id purpose } } } }
`
const REQUESTS = gql`
  query { requests { edges { node { id purpose } } } }
`
// The unfiltered list queries — the surface Requests-and-Offers depends on but
// no existing step exercises. Verifies the get_all_proposals / get_all_intents
// zome fns resolve end-to-end through the GraphQL adapter (queries/index.ts).
const ALL_PROPOSALS = gql`
  query { proposals { edges { node { id name purpose publishes { id } } } } }
`
const ALL_INTENTS = gql`
  query { intents { edges { node { id action { id } name } } } }
`
const CREATE_EVENT = gql`
  mutation ($e: EconomicEventCreateParams!) {
    res: createEconomicEvent(event: $e) { economicEvent { id action { id } } }
  }
`
const CREATE_CLAIM = gql`
  mutation ($c: ClaimCreateParams!) {
    res: createClaim(claim: $c) { claim { id } }
  }
`
const GET_CLAIM = gql`
  query ($id: ID!) {
    claim(id: $id) { id triggeredBy { id } settledBy { id } }
  }
`

export async function runAcceptanceScenario(client: Client, r: Runner): Promise<void> {
  const { step, say, assert } = r

  // ── Agents ────────────────────────────────────────────────────────────────
  let aline = '', bob = ''
  await step('Create two Person agents (Aline, Bob)', async () => {
    const a = await client.mutate({ mutation: CREATE_PERSON, variables: { p: { name: 'Aline' } } })
    const b = await client.mutate({ mutation: CREATE_PERSON, variables: { p: { name: 'Bob' } } })
    aline = a.data?.res?.agent?.id
    bob = b.data?.res?.agent?.id
    assert(aline && bob, 'agent ids missing in response')
    assert(a.data.res.agent.name === 'Aline', `name round-trip failed: ${a.data.res.agent.name}`)
    say(`Aline (${aline.slice(0, 12)}…) will offer vegetables; Bob (${bob.slice(0, 12)}…) will request them.`)
    return 'both created, names round-trip'
  })

  // ── Resource specifications (VF 1.0 booleans #398 / #407) ────────────────
  let veggieSpec = '', currencySpec = ''
  await step('Create ResourceSpecifications with VF 1.0 booleans', async () => {
    const v = await client.mutate({
      mutation: CREATE_RESOURCE_SPEC,
      variables: { rs: { name: 'Fresh vegetables', substitutable: true, mediumOfExchange: false } },
    })
    const c = await client.mutate({
      mutation: CREATE_RESOURCE_SPEC,
      variables: { rs: { name: 'Local currency', substitutable: true, mediumOfExchange: true } },
    })
    veggieSpec = v.data?.res?.resourceSpecification?.id
    currencySpec = c.data?.res?.resourceSpecification?.id
    assert(veggieSpec && currencySpec, 'resource spec ids missing')
    assert(v.data.res.resourceSpecification.mediumOfExchange === false, 'mediumOfExchange=false did not round-trip')
    assert(c.data.res.resourceSpecification.mediumOfExchange === true, 'mediumOfExchange=true did not round-trip')
    assert(c.data.res.resourceSpecification.substitutable === true, 'substitutable=true did not round-trip')
    say('“Fresh vegetables” (a good) and “Local currency” (mediumOfExchange: true) both exist as specifications.')
    return 'substitutable + mediumOfExchange round-trip'
  })

  // ── Offer proposal (vf:Proposal.purpose = offer, #322) ────────────────────
  let offerId = ''
  await step('Publish an OFFER: primary + reciprocal intents under purpose="offer"', async () => {
    const give = await client.mutate({
      mutation: CREATE_INTENT,
      variables: { i: { action: 'transfer', name: 'Give vegetables', provider: aline, resourceConformsTo: veggieSpec, resourceQuantity: { hasNumericalValue: 5 } } },
    })
    const get = await client.mutate({
      mutation: CREATE_INTENT,
      variables: { i: { action: 'transfer', name: 'Receive currency', receiver: aline, resourceConformsTo: currencySpec, resourceQuantity: { hasNumericalValue: 10 } } },
    })
    const giveId = give.data?.res?.intent?.id
    const getId = get.data?.res?.intent?.id
    assert(giveId && getId, 'intent ids missing')
    const p = await client.mutate({
      mutation: CREATE_PROPOSAL,
      variables: { p: { name: 'Vegetable basket offer', note: 'Weekly basket', purpose: 'offer', publishes: [giveId], reciprocal: [getId] } },
    })
    offerId = p.data?.res?.proposal?.id
    assert(offerId, 'proposal id missing')
    assert(p.data.res.proposal.purpose === 'offer', `purpose round-trip failed: ${p.data.res.proposal.purpose}`)
    assert(p.data.res.proposal.publishes?.[0]?.id === giveId, 'publishes does not contain the primary intent')
    assert(p.data.res.proposal.reciprocal?.[0]?.id === getId, 'reciprocal does not contain the reciprocal intent')
    say('Aline published an offer: 5 vegetables for 10 local currency.')
    return 'purpose, publishes and reciprocal all round-trip'
  })

  // ── Round-trip re-read ────────────────────────────────────────────────────
  await step('Re-read the offer by id — stored fields equal inputs', async () => {
    const q = await client.query({ query: GET_PROPOSAL, variables: { id: offerId } })
    const p = q.data?.proposal
    assert(p, 'proposal not found by id')
    assert(p.name === 'Vegetable basket offer', `name mismatch: ${p.name}`)
    assert(p.note === 'Weekly basket', `note mismatch: ${p.note}`)
    assert(p.purpose === 'offer', `purpose mismatch: ${p.purpose}`)
    return 'name, note, purpose match created values'
  })

  // ── Purpose immutability (offer must not become a request) ───────────────
  await step('REJECTED: updateProposal changing purpose offer→request (immutability)', async () => {
    const q = await client.query({ query: gql`query ($id: ID!) { proposal(id: $id) { id revisionId publishes { id } } }`, variables: { id: offerId } })
    const rev = q.data?.proposal?.revisionId
    const pubIds = (q.data?.proposal?.publishes ?? []).map((i: any) => i.id)
    assert(rev, 'revisionId missing')
    const msg = await expectRejection(client, gql`
      mutation ($p: ProposalUpdateParams!) { res: updateProposal(proposal: $p) { proposal { id purpose } } }
    `, { p: { revisionId: rev, publishes: pubIds, purpose: 'request' } })
    assert(msg, 'purpose mutation was accepted — immutability rule failed')
    say(`Guard said no: ${msg!.split('\n')[0]}`)
    return 'rejected as expected'
  })

  // ── Request proposal ──────────────────────────────────────────────────────
  let requestId = ''
  await step('Publish a REQUEST: purpose="request"', async () => {
    const want = await client.mutate({
      mutation: CREATE_INTENT,
      variables: { i: { action: 'transfer', name: 'Need vegetables', receiver: bob, resourceConformsTo: veggieSpec, resourceQuantity: { hasNumericalValue: 3 } } },
    })
    const wantId = want.data?.res?.intent?.id
    assert(wantId, 'intent id missing')
    const p = await client.mutate({
      mutation: CREATE_PROPOSAL,
      variables: { p: { name: 'Vegetable request', purpose: 'request', publishes: [wantId] } },
    })
    requestId = p.data?.res?.proposal?.id
    assert(requestId, 'proposal id missing')
    assert(p.data.res.proposal.purpose === 'request', `purpose round-trip failed: ${p.data.res.proposal.purpose}`)
    say('Bob published a request: he needs 3 vegetables.')
    return 'request proposal created, purpose round-trips'
  })

  // ── Purpose index partition (offers / requests queries) ──────────────────
  await step('offers/requests queries partition by purpose index', async () => {
    const o = await client.query({ query: OFFERS })
    const r = await client.query({ query: REQUESTS })
    const offerNodes = (o.data?.offers?.edges ?? []).map((e: any) => e.node)
    const requestNodes = (r.data?.requests?.edges ?? []).map((e: any) => e.node)
    assert(offerNodes.find((n: any) => n.id === offerId), 'offers query misses the offer')
    assert(!offerNodes.find((n: any) => n.id === requestId), 'offers query leaks the request')
    assert(requestNodes.find((n: any) => n.id === requestId), 'requests query misses the request')
    assert(!requestNodes.find((n: any) => n.id === offerId), 'requests query leaks the offer')
    say(`offers → ${offerNodes.length} node(s), requests → ${requestNodes.length} node(s), no cross-leak.`)
    return `offers=${offerNodes.length} requests=${requestNodes.length}, disjoint`
  })

  // ── Unfiltered list queries (the surface R&O depends on, never before asserted) ──
  await step('Unfiltered proposals/intents list queries return created records', async () => {
    const pq = await client.query({ query: ALL_PROPOSALS })
    const proposals = (pq.data?.proposals?.edges ?? []).map((e: any) => e.node)
    assert(proposals.find((n: any) => n.id === offerId), 'proposals list misses the offer')
    assert(proposals.find((n: any) => n.id === requestId), 'proposals list misses the request')
    assert(proposals.length >= 2, `proposals list returned ${proposals.length}, expected >= 2`)
    // every listed proposal should have a name + a purpose round-tripped
    proposals.forEach((n: any, i: number) => {
      assert(typeof n.name === 'string', `proposal[${i}].name is not a string`)
      assert(n.purpose === 'offer' || n.purpose === 'request', `proposal[${i}].purpose unexpected: ${n.purpose}`)
    })

    const iq = await client.query({ query: ALL_INTENTS })
    const intents = (iq.data?.intents?.edges ?? []).map((e: any) => e.node)
    // At least the offer's "give" intent + the request's "want" intent exist now
    assert(intents.length >= 2, `intents list returned ${intents.length}, expected >= 2`)
    intents.forEach((n: any, i: number) => {
      assert(n.action?.id, `intent[${i}].action.id missing — resolver did not hydrate the action object`)
      assert(typeof n.name === 'string', `intent[${i}].name is not a string`)
    })

    say(`Unfiltered list serves ${proposals.length} proposal(s) and ${intents.length} intent(s) — the path consumers rely on.`)
    return `proposals=${proposals.length} intents=${intents.length}`
  })

  // ── Happy-path economic event (VF transfer, two agents) ───────────────────
  let transferEventId = ''
  await step('Record a transfer EconomicEvent (Aline → Bob)', async () => {
    const e = await client.mutate({
      mutation: CREATE_EVENT,
      variables: { e: { action: 'transfer', provider: aline, receiver: bob, resourceConformsTo: veggieSpec, resourceQuantity: { hasNumericalValue: 5 }, hasPointInTime: new Date('2026-07-01T12:00:00Z').toISOString() } },
    })
    transferEventId = e.data?.res?.economicEvent?.id
    assert(transferEventId, 'event id missing')
    assert(e.data.res.economicEvent.action.id === 'transfer', 'action round-trip failed')
    say('The basket physically moved: transfer of 5 vegetables from Aline to Bob was recorded.')
    return 'transfer event accepted with both agents'
  })

  // ── Claim + settlement (VF 1.0 settlement shape) ──────────────────────────
  await step('Claim triggered by the transfer, settled by a reciprocal event (Claim.settledBy)', async () => {
    const c = await client.mutate({
      mutation: CREATE_CLAIM,
      variables: { c: { action: 'transfer', triggeredBy: transferEventId, resourceClassifiedAs: ['https://example.org/currency'], resourceQuantity: { hasNumericalValue: 10 } } },
    })
    const claimId = c.data?.res?.claim?.id
    assert(claimId, 'claim id missing')
    const settle = await client.mutate({
      mutation: CREATE_EVENT,
      variables: { e: { action: 'transfer', provider: bob, receiver: aline, resourceConformsTo: currencySpec, resourceQuantity: { hasNumericalValue: 10 }, settles: claimId, hasPointInTime: new Date('2026-07-02T12:00:00Z').toISOString() } },
    })
    const settleId = settle.data?.res?.economicEvent?.id
    assert(settleId, 'settling event id missing')
    const q = await client.query({ query: GET_CLAIM, variables: { id: claimId } })
    const settledBy = (q.data?.claim?.settledBy ?? []).map((e: any) => e.id)
    assert(settledBy.includes(settleId), `Claim.settledBy (${settledBy.length} events) does not contain the settling event`)
    say('Bob paid 10 currency; the claim shows it via Claim.settledBy — the VF 1.0 settlement shape, no reified Settlement entity.')
    return 'settles → settledBy reverse relation round-trips'
  })

  // ── Process planning links (intendedInputs / intendedOutputs) ────────────
  await step('Process.intendedOutputs round-trips (missing zome fn fixed)', async () => {
    const p = await client.mutate({
      mutation: gql`mutation ($p: ProcessCreateParams!) { res: createProcess(process: $p) { process { id } } }`,
      variables: { p: { name: 'Grow vegetables' } },
    })
    const processId = p.data?.res?.process?.id
    assert(processId, 'process id missing')
    const outIntent = await client.mutate({
      mutation: CREATE_INTENT,
      variables: { i: { action: 'produce', name: 'Harvest', outputOf: processId, resourceConformsTo: veggieSpec, resourceQuantity: { hasNumericalValue: 20 } } },
    })
    const outId = outIntent.data?.res?.intent?.id
    assert(outId, 'output intent id missing')
    const q = await client.query({
      query: gql`query ($id: ID!) { process(id: $id) { id intendedOutputs { id } } }`,
      variables: { id: processId },
    })
    const outs = (q.data?.process?.intendedOutputs ?? []).map((i: any) => i.id)
    assert(outs.includes(outId), `intendedOutputs (${outs.length}) does not contain the intent`)
    say('A process plans its harvest: Process.intendedOutputs resolves — this zome fn did not exist before this review.')
    return 'intendedOutputs resolves via get_rea_intents_for_rea_process_outputs'
  })

  // ── Negative paths ────────────────────────────────────────────────────────
  await step('REJECTED: proposal with purpose="banana" (ProposalPurpose guard)', async () => {
    const msg = await expectRejection(client, gql`
      mutation ($p: ProposalCreateParams!) { res: createProposal(proposal: $p) { proposal { id } } }
    `, { p: { name: 'Bad', purpose: 'banana', publishes: [] } })
    assert(msg, 'invalid purpose was accepted — enum/validation guard failed')
    say(`Guard said no: ${msg!.split('\n')[0]}`)
    return 'rejected as expected'
  })

  await step('REJECTED: EconomicEvent with action="banana" (VF action vocabulary gate)', async () => {
    const msg = await expectRejection(client, CREATE_EVENT, {
      e: { action: 'banana', provider: aline, receiver: bob, resourceQuantity: { hasNumericalValue: 1 } },
    })
    assert(msg, 'invalid action was accepted — VF_BUILTIN_ACTIONS gate failed')
    say(`Guard said no: ${msg!.split('\n')[0]}`)
    return 'rejected as expected'
  })

  await step('ACCEPTED: EconomicEvent with action="copy" (VF 1.0 vocabulary completeness)', async () => {
    const e = await client.mutate({
      mutation: CREATE_EVENT,
      variables: { e: { action: 'copy', provider: aline, receiver: aline, resourceConformsTo: veggieSpec, resourceQuantity: { hasNumericalValue: 1 } } },
    })
    const id = e.data?.res?.economicEvent?.id
    assert(id, 'copy event id missing')
    assert(e.data.res.economicEvent.action.id === 'copy', 'action round-trip failed')
    say('A canonical VF 1.0 action (copy — unstable tier) is accepted by the DHT gate, alongside combine and separate.')
    return 'canonical 1.0 action accepted by integrity gate'
  })

  await step('Action vocabulary parity: 21 ids served; combine/separate exercised end-to-end', async () => {
    const EXPECTED = ['dropoff','pickup','consume','use','work','cite','produce','accept','modify','pass','fail','combine','separate','copy','deliver-service','transfer-all-rights','transfer-custody','transfer','move','raise','lower']
    const q = await client.query({ query: gql`query { actions { id } }` })
    const served = (q.data?.actions ?? []).map((a: any) => a.id).sort()
    assert(served.length === EXPECTED.length, `actions query served ${served.length} ids, expected ${EXPECTED.length}`)
    const missing = EXPECTED.filter(id => !served.includes(id))
    assert(missing.length === 0, `actions query missing: ${missing.join(', ')}`)
    for (const action of ['combine', 'separate']) {
      const e = await client.mutate({
        mutation: CREATE_EVENT,
        variables: { e: { action, provider: aline, receiver: aline, resourceConformsTo: veggieSpec, resourceQuantity: { hasNumericalValue: 1 } } },
      })
      assert(e.data?.res?.economicEvent?.action?.id === action, `${action} event did not round-trip through integrity + adapter`)
    }
    say('The zome serves all 21 actions (19 canonical VF 1.0 + 2 legacy), and combine/separate round-trip through every layer.')
    return '21-id set exact; combine + separate accepted and readable'
  })

  await step('REJECTED: EconomicEvent with hasBeginning > hasEnd (VF temporal rule)', async () => {
    const msg = await expectRejection(client, CREATE_EVENT, {
      e: {
        action: 'transfer', provider: aline, receiver: bob,
        resourceConformsTo: veggieSpec, resourceQuantity: { hasNumericalValue: 1 },
        hasBeginning: new Date('2026-07-05T12:00:00Z').toISOString(),
        hasEnd: new Date('2026-07-01T12:00:00Z').toISOString(),
      },
    })
    assert(msg, 'temporal violation was accepted — has_beginning <= has_end rule failed')
    say(`Guard said no: ${msg!.split('\n')[0]}`)
    return 'rejected as expected'
  })

}
