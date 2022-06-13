/**
 * Simulates a simplified cross-border trade scenario.
 *
 * The following agents are involved:
 *
 * - "Customer" who orders a freight shipment of avocadoes from overseas
 * - "Exporter" who grows avocadoes and wants to sell them
 * - "Transporter" who takes the avocadoes from the farm, onto the ship,
 *                 and all the way to the foreign port (for simplicity)
 * - "Official" who checks the provenance of the shipment at import time
 * - "Courier" who delivers the final shipment to the customer (for simplicity)
 *
 * The following characteristics apply:
 *
 * - The "Customer" merely places an order, wishes to see it filled, and has no
 *   interest in any other matters.
 * - The "Exporter" subcontracts the "Transporter" to manage the operation, and
 *   the "Transporter" never deals with any "Official".
 * - The "Official" must view and assess the delivery of the shipment, then sign it
 *   and pass the shipment details along with the physical goods to their destination.
 * - The "Courier" takes on responsibility for the shipment data in transit.
 * - The "Customer" may request the full history of the shipment from the trusted
 *   "Courier" or "Official" at any time.
 * - The "Exporter", "Transporter" and other parties in the origin country have
 *   no need to know of the internal movements of the avocadoes once they enter
 *   the destination port, and sharing of such details is considered a liability
 *   to all parties in the supply chain.
 *
 * :TODO:
 *
 * - GraphQL API for "non-primary" networks
 *   @see https://github.com/holo-rea/holo-rea/issues/150
 *
 * @package: Holo-REA
 * @since:   2020-05-27
 */

 import test from "tape"
 import { pause } from "@holochain/tryorama"
import {
  getDNA,
  buildPlayer,
  buildGraphQL,
  bridge,
} from '../init.js'



// some constants to save us needing to think about ResourceSpecifications,
// Units & other fiddly stuff in this example

const AVOCADO_RESOURCE_CLASSIFICATION = 'http://aims.fao.org/aos/agrovoc/c_9022'
const USD_RESOURCE_CLASSIFICATION = 'currency:USD'
const KILOGRAMS_UNIT = 'kg'
const ONE_MONTH = 1000 * 3600 * 24 * 7 * 4

// init collaboration spaces

const DNAs = {
  // customer originating "marketplace" network
  customer_agent: getDNA('agent', 'customer_agent'),
  customer_planning: getDNA('planning', 'customer_planning'),
  customer_proposal: getDNA('proposal', 'customer_proposal'),

  // export management company network
  exporter_agent: getDNA('agent', 'exporter_agent'),
  exporter_planning: getDNA('planning', 'exporter_planning'),

  // subcontractor coordination network, provided by export management company
  ex_sub_agent: getDNA('agent', 'ex_sub_agent'),
  ex_sub_observation: getDNA('observation', 'ex_sub_observation'),
  ex_sub_planning: getDNA('planning', 'ex_sub_planning'),

  // importing official network
  importer_agent: getDNA('agent', 'importer_agent'),
  importer_observation: getDNA('observation', 'importer_observation'),
  importer_planning: getDNA('planning', 'importer_planning'),

  // courier logistics network
  courier_agent: getDNA('agent', 'courier_agent'),
  courier_observation: getDNA('observation', 'courier_observation'),
  courier_planning: getDNA('planning', 'courier_planning'),
}

test('Cross-border trade scenario', async (t) => {
  // init agents

  const customer = await buildPlayer(s, 'customer', buildConfig({
    // 'native' marketplace network
    agent: DNAs['customer_agent'],
    planning: DNAs['customer_planning'],
    proposal: DNAs['customer_proposal'],
    // auxilliary networks with a presence for auditing purposes
    courier_observation: DNAs['courier_observation'],
    official_observation: DNAs['importer_observation'],
  }, {
    vf_planning: ['proposal', 'planning'],
  }))
  customer.courierGQL = buildGraphQL(customer, { dnaConfig: {
    observation: 'courier_observation',
  } })
  customer.importOfficialGQL = buildGraphQL(customer, { dnaConfig: {
    observation: 'official_observation',
  } })

  const exporter = await buildPlayer(s, 'exporter', buildConfig({
    agent: DNAs['exporter_agent'],
    planning: DNAs['exporter_planning'],
    // auxilliary sub-network for managing subcontractor
    sub_agent: DNAs['ex_sub_agent'],
    sub_observation: DNAs['ex_sub_observation'],
    sub_planning: DNAs['ex_sub_planning'],
  }, [
    bridge('vf_observation', 'sub_planning', 'sub_observation'),
  ]))
  exporter.contractorGQL = buildGraphQL(exporter, { dnaConfig: {
    agent: 'sub_agent',
    observation: 'sub_observation',
    planning: 'sub_planning',
  } })

  const transporter = await buildPlayer(s, 'transporter', buildConfig({
    agent: DNAs['ex_sub_agent'],
    observation: DNAs['ex_sub_observation'],
    planning: DNAs['ex_sub_planning'],
    // auxilliary access to exporter planning space for fulfilling commitments
    exporter_planning: DNAs['exporter_planning'],
  }, [
    bridge('vf_observation', 'planning', 'observation'),
    bridge('vf_observation', 'exporter_planning', 'observation'),
  ]))
  transporter.employerGQL = buildGraphQL(transporter, { dnaConfig: {
    planning: 'exporter_planning',
  } })

  const official = await buildPlayer(s, 'official', buildConfig({
    agent: DNAs['importer_agent'],
    observation: DNAs['importer_observation'],
    planning: DNAs['importer_planning'],
    // auxilliary access to exporter network for checking provenance
    exporter_observation: DNAs['ex_sub_observation'],
    // auxilliary access to courier network for transferring custody documentation
    courier_observation: DNAs['courier_observation'],
    courier_planning: DNAs['courier_planning'],
  }, [
    bridge('vf_observation', 'planning', 'observation'),
    bridge('vf_observation', 'courier_planning', 'courier_observation'),
  ]))
  official.exporterGQL = buildGraphQL(official, { dnaConfig: {
    observation: 'exporter_observation',
  } })
  official.courierGQL = buildGraphQL(official, { dnaConfig: {
    observation: 'courier_observation',
    planning: 'courier_planning',
  } })

  const courier = await buildPlayer(s, 'courier', buildConfig({
    agent: DNAs['courier_agent'],
    observation: DNAs['courier_observation'],
    planning: DNAs['courier_planning'],
    // auxilliary access to customer marketplace network for managing trades
    customer_agent: DNAs['customer_agent'],
    customer_planning: DNAs['customer_planning'],
    customer_proposal: DNAs['customer_proposal'],
  }, {
    vf_observation: ['planning', 'observation'],
    vf_planning: ['customer_proposal', 'customer_planning'],
  }))
  courier.marketplaceGQL = buildGraphQL(courier, { dnaConfig: {
    agent: 'customer_agent',
    proposal: 'customer_proposal',
    planning: 'customer_planning',
  } })

  // :TODO: this probably differs from the agent ID that counts, the agent ID inside the... proposal? network
  //        but potentially better to use a consistent singular agent ID within each agent-network set... needs thought...
  const customerAddress = customer.instance('agent').agentAddress
  const courierAddress = courier.instance('agent').agentAddress

  // ---------------------------------------------------------------------------
  // ---------------------------[ BEGIN SCENARIO ]------------------------------
  // ---------------------------------------------------------------------------

  // The customer places an initial order to the marketplace
  // =======================================================

  let resp = await customer.graphQL(`
    mutation($i1: IntentCreateParams!, $i2: IntentCreateParams!, $p: ProposalCreateParams!) {
      i1: createIntent(intent: $i1) {
        intent {
          id
        }
      }
      i2: createIntent(intent: $i2) {
        intent {
          id
        }
      }
      p: createProposal(proposal: $p) {
        proposal {
          id
        }
      }
    }
  `, {
    i1: {
      name: 'initial order for 1000kg avocados',
      action: 'transfer',
      receiver: customerAddress,
      resourceClassifiedAs: [AVOCADO_RESOURCE_CLASSIFICATION],
      resourceQuantity: { hasNumericalValue: 1000, hasUnit: KILOGRAMS_UNIT },
      due: new Date(Date.now() + ONE_MONTH),
    },
    i2: {
      name: 'offer for $12000USD sale',
      action: 'transfer',
      provider: customerAddress,
      resourceClassifiedAs: [USD_RESOURCE_CLASSIFICATION],
      resourceQuantity: { hasNumericalValue: 12000 },
    },
    p: {
      name: 'listing to buy avocado shipment',
    },
  })
  await pause(100)
  t.ok(resp.data.i1.intent.id, 'customer order created')
  t.ok(resp.data.i2.intent.id, 'customer offer created')
  t.ok(resp.data.p.proposal.id, 'customer listing created')

  const proposalId = resp.data.p.proposal.id

  // link intents to the proposal on the marketplace

  resp = await customer.graphQL(`
    mutation() {
      pi1: proposeIntent(publishedIn: "${proposalId}", publishes: "${resp.data.i1.intent.id}", reciprocal: false)
      pi2: proposeIntent(publishedIn: "${proposalId}", publishes: "${resp.data.i2.intent.id}", reciprocal: true)
    ` +
  // publish the proposal to the courier
    `
      pt1: proposeTo(proposed: "${proposalId}", proposedTo: "${courierAddress}")
    }
  `, {})

  // Courier receives the order and accepts the offer
  // ================================================

  // Courier puts listing to exporter to manage first leg of shipment
  // ================================================================

  // Exporter accepts offers, enlists transporter subcontractor
  // ==========================================================

  // Transporter tracks receipt and handover of goods
  // ================================================

  // Exporter hands audit log over to import official
  // ================================================

  // Import official checks and signs logs
  // =====================================

  // Official passes transport logs over to courier
  // ==============================================

  // Courier delivers final shipment to customer
  // ===========================================

  await alice.scenario.cleanUp()
})


