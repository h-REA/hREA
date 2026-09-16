# Architecture

hREA implements the [ValueFlows](https://valueflo.ws) economic vocabulary as a Holochain DNA.
This document explains the conceptual model, how it maps onto the zomes, the record types, the
action system, and the patterns the coordinator uses.

## The ValueFlows / REA model

ValueFlows is built on REA accounting theory (Resources, Events, Agents). The core idea is a
three-layer view of economic activity, from concrete to speculative:

- **Observation** records what actually happened. An `EconomicEvent` (for example, "produced 5
  widgets") changes the quantity of an `EconomicResource`.
- **Planning** records what is promised or intended. A `Commitment` is a promise to perform a
  future event; an `Intent` is a desired future event not yet committed to.
- **Coordination** records proposals and agreements. A `Proposal` bundles intents into an offer
  or request; an `Agreement` records mutual obligations.

Agents (people or organizations) provide and receive in these flows. Specifications
(`ResourceSpecification`, `ProcessSpecification`, `Unit`) describe the kinds of things and
measures involved. Recipes describe reusable templates for processes and exchanges.

The canonical ontology is defined at [valueflo.ws](https://www.valueflo.ws); this document
only covers how hREA realizes it.

## Integrity zome vs coordinator zome

Holochain splits a DNA into integrity and coordinator zomes. hREA has exactly one of each.

### Integrity zome (`hrea_integrity`)

Located at `dnas/hrea/zomes/integrity/hrea/`. It defines every entry type and link type and
their validation. The entry and link enums are declared in `src/lib.rs`, and each record type
has its own source file (for example `rea_economic_event.rs`, `rea_agent.rs`).

### Coordinator zome (`hrea`)

Located at `dnas/hrea/zomes/coordinator/hrea/`. It contains all callable logic: the public
`#[hdk_extern]` functions for CRUD and queries, link traversal, collection indexing, and the
post-commit signal emission that lets UIs react to changes. Each record type has a matching
source file mirroring the integrity layer.

## Record types

The integrity zome defines eighteen entry types, listed in `EntryTypes` in `dnas/hrea/zomes/integrity/hrea/src/lib.rs`. Rust structs are prefixed `Rea`. The table below is the whole set.

| ValueFlows type | Entry struct | Source file (integrity) | Notable fields |
|-----------------|--------------|--------------------------|----------------|
| Agent | `ReaAgent` | `rea_agent.rs` | name, agent_type, classified_as |
| Agreement | `ReaAgreement` | `rea_agreement.rs` | name, created |
| Plan | `ReaPlan` | `rea_plan.rs` | name, created, due, in_scope_of |
| Process | `ReaProcess` | `rea_process.rs` | name, has_beginning, has_end, based_on, planned_within |
| ProcessSpecification | `ReaProcessSpecification` | `rea_process_specification.rs` | name, note, image |
| EconomicEvent | `ReaEconomicEvent` | `rea_economic_event.rs` | rea_action, provider, receiver, resource_quantity, input_of, output_of, fulfills, satisfies |
| EconomicResource | `ReaEconomicResource` | `rea_economic_resource.rs` | conforms_to, accounting_quantity, onhand_quantity, primary_accountable, contained_in |
| ResourceSpecification | `ReaResourceSpecification` | `rea_resource_specification.rs` | name, default_unit_of_resource, default_unit_of_effort, substitutable, medium_of_exchange |
| Unit | `ReaUnit` | `rea_unit.rs` | label, symbol, om_unit_identifier |
| Commitment | `ReaCommitment` | `rea_commitment.rs` | rea_action, provider, receiver, input_of, output_of, satisfies, clause_of |
| Intent | `ReaIntent` | `rea_intent.rs` | rea_action, provider, receiver, available_quantity, minimum_quantity |
| Proposal | `ReaProposal` | `rea_proposal.rs` | purpose, publishes, reciprocal, proposed_to, unit_based |
| RecipeProcess | `ReaRecipeProcess` | `rea_recipe_process.rs` | process_conforms_to |
| RecipeExchange | `ReaRecipeExchange` | `rea_recipe_exchange.rs` | name, note |
| RecipeFlow | `ReaRecipeFlow` | `rea_recipe_flow.rs` | rea_action, resource_quantity (a QuantityValue), effort_quantity (a String, not a quantity), recipe_input_of, recipe_output_of |
| Claim | `ReaClaim` | `rea_claim.rs` | rea_action, triggered_by, resource_quantity, effort_quantity, due, finished, agreed_in |
| SpatialThing | `ReaSpatialThing` | `rea_spatial_thing.rs` | name, mappable_address, lat, long, alt |
| AgreementBundle | `ReaAgreementBundle` | `rea_agreement_bundle.rs` | name, note, agreements |

A shared `QuantityValue` struct (defined in `rea_unit.rs`) pairs a numeric value with a `Unit`
and is reused across commitments, intents, recipe flows, events, and resources.

## Links

The integrity zome (`src/lib.rs`) defines three families of link types:

- **Update links** (for example `ReaAgentUpdates`, `ReaEconomicEventUpdates`) form the
  revision chain for each entry, so the coordinator can resolve the latest version.
- **Collection links** (for example `AllAgents`, `AllEconomicEvents`) index every instance of a
  type for collection queries.
- **Relationship links** wire the domain graph together, for example
  `ProviderToReaCommitments`, `ReaProcessToReaEconomicEventInputs`,
  `CommitmentToFulfillingEconomicEvents`, `IntentToSatisfyingCommitments`, and
  `ReaEconomicResourceToReaEconomicResources` (resource nesting).

## The action system (`vf_actions`)

EconomicEvents, Commitments, Intents, and RecipeFlows all carry a `rea_action` string that
names a ValueFlows action. Actions are defined in the embedded `vf_actions` crate
(`dnas/hrea/zomes/coordinator/hrea/vf_actions/`).

An `Action` declares how it affects inventory through two effects:

- `accounting_effect` and `onhand_effect`, each an `ActionEffect`
  (`NoEffect`, `Increment`, `Decrement`, or `DecrementIncrement`).
- An `input_output` `ProcessType` (`Input`, `Output`, or `NotApplicable`).

`vf_actions/src/builtins.rs` defines the built-in ValueFlows actions (such as `produce`,
`consume`, `use`, `work`, `transfer`, `move`, `pickup`, `dropoff`). The crate exposes
`get_builtin_action(key)` and `get_all_builtin_actions()`, plus validators
(`validate_flow_action`, `validate_move_inventories`).

When an EconomicEvent is created, the coordinator applies the action's effects to the linked
EconomicResource's `accounting_quantity` and `onhand_quantity`. This is the mechanism by which
recording an event updates inventory.

## Coordinator patterns

Every record type's coordinator module follows a consistent shape:

- `create_rea_<type>(entry) -> Record`
- `get_latest_rea_<type>(original_hash) -> Option<Record>` (walks the update chain from the original)
- `get_original_rea_<type>(original_hash) -> Option<Record>`

  Both read getters take the **original** action hash, not a revision id. Passing a revision id returns nothing, because they follow `Rea<Type>Updates` links hanging off the original. Only `update_*` and `delete_*` take `revision_id`.
- `update_rea_<type>({ revision_id, entry }) -> Record` (merges fields, extends the chain)
- `delete_rea_<type>(revision_id) -> ActionHash` and, for agents only, the batch helper `get_rea_agents_from_action_hashes(...)`. It is the single `from_action_hashes` function in the coordinator; the general batch fetch is `get_generic_entries` in `collections.rs`.

Two cross-cutting behaviors live in `src/lib.rs`:

- **Post-commit signals.** After a successful commit the coordinator emits a signal so
  subscribed clients can update reactively.
- **EconomicResource is event-driven.** Resources are generally not created directly; creating
  an EconomicEvent with the right action creates or mutates the associated resource.

For how this surfaces over GraphQL (where these zome functions become queries and mutations),
see [GraphQL API & Integration](./graphql-api.md).
