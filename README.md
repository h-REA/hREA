# The Holo-REA coordination framework (hREA)

A suite of functionally independent building blocks for creating distributed social, economic & resource coordination applications of any kind, implemented as [Holochain](https://holochain.org/) "[DNA modules](https://developer.holochain.org/docs/concepts/2_application_architecture/)" (fully decentralised, agent-centric application microservices).

<!-- MarkdownTOC -->

- [About](#about)
- [Documentation](#documentation)
- [Holo-REA beyond Holochain](#holo-rea-beyond-holochain)
- [Repository structure](#repository-structure)
- [License](#license)

<!-- /MarkdownTOC -->

## About

The modular building blocks offered by Holo-REA out of the box cover most functionality commonly used in supply chain systems, project management software, logistics management and enterprise resource planning; as well as post-capitalist economic paradigms such as gift and contributory economies. Some of the core functionality offered includes:

- **group management**: manage groups of collaborators and permission access between groups, sub-projects and across organisations
- **event ledger**: use the *observation* module to track the observed movements of resources, currencies and skills in any coordination space
- **coordination functions**: use *planning* modules to decide on future plans, manage agreements or coordinate actions with other participants
- **needs matching**: use *proposal* modules to group matched outcomes in order to describe bilateral and multilateral trade requests
- **knowledge sharing**: use the *recipe* module to share structured production knowledge and easily repeat well-understood processes

This is just scratching the surface- there is so much more! For a more detailed list of other modules and features planned for development, see [modules in the Holo-REA framework](https://github.com/holo-rea/ecosystem/wiki/Modules-in-the-HoloREA-framework).

A key aspect to note about these modules is that *they require no technical knowledge to remix or re-architect into different social organising patterns*. Different arrangements of network components can be used to address different use-cases in novel ways.

This is not to say that Holo-REA can only be customised in so much depth. Modularity is a key design goal at every layer of the framework- and in addition to rearranging the modules themselves, each component can be extended with custom domain-specific logic or used as helper libraries to service other functions.

## Documentation

**High-level documentation** for integrators, potential collaborators and entrepreneurs can be found in the project's [ecosystem wiki](https://github.com/holo-rea/ecosystem/wiki/). This includes information on Holo-REA's organisational goals, strategic mission, design philosophy, cultural background and ideological positioning.

**Developer documentation** can be found in the [`docs/`](docs/README.md) directory. We keep it within the codebase instead of in the wiki so that all contributors retain the information necessary to understand, configure and run the system.

There is a [quick start guide](docs/README.md#quick-start) for those who want to spin up Holo-REA locally for development or experimentation.

## Holo-REA beyond Holochain

Holo-REA is built to implement the [ValueFlows protocol](https://valueflo.ws/)&mdash; a set of common vocabularies based on [REA Accounting theory](https://en.wikipedia.org/wiki/Resources,_events,_agents_(accounting_model)) to describe flows of economic resources of all kinds within distributed economic ecosystems.

By building to align with the [ValueFlows GraphQL spec](https://github.com/valueflows/vf-graphql/), UI applications built for Holo-REA can be made compatible with other ValueFlows-compatible system backends like [CommonsPub](https://github.com/commonspub/CommonsPub-Server), [Scuttlebutt](https://github.com/open-app/economic-sentences-graphql) and the original [NRP](https://github.com/django-rea/nrp) with little to no effort.

The goal is to enable radical code reuse and cross-project interoperability between next-gen distributed backend systems and traditional web infrastructure, and to allow user interfaces to span multiple disparate apps.

## Repository structure

- `apps/` is filled with end-user applications built on the HoloREA framework.
	- `holorea-graphql-explorer/` is a [GraphiQL](https://github.com/graphql/graphiql) interface to the system with some added [additions to assist with comprehension](https://github.com/OneGraph/graphiql-explorer-example). Wired up to the development DNAs by default&mdash; super handy for testing and getting to know the ValueFlows [data structure](https://github.com/valueflows/vf-graphql/).
- `example/` contains contrived implementations for particular use-cases and domain-specific applications. If you're interested in seeing what building on Holo-REA looks like, the projects in these folders will be quite illuminating.
- [`happs/`](happs/README.md) contains subdirectories corresponding to separate Holochain app DNA packages, which expose their data as independent, isolated DHTs. Each DNA is composed of multiple *zomes* which describe semi-independent bits of functionality within that app DNA. Zomes are mostly just scaffolding- the bulk of their logic is broken up into several packages to promote modularisation, and kept in the `lib/` directory.
- `lib/` contains library code that is used by the various hApps (Holochain apps). Note that shared code is necessary to facilitate sharing of data between DNAs, as the de/serialisation logic is defined by Rust structs- hence this separation.
	- `hdk_graph_helpers/` is the result of abstracting away common functionality used in zome callback handlers to reduce boilerplate.
	- The `rea_*/` folders in this directory correspond to the actual ValueFlows record structures and their behaviours used in zome code. Most of them contain at minimum their underlying storage data structures and CRUD API behaviours.
- `modules/` is home to the JavaScript modules used in binding the Holochain backend to UIs, servers etc
	- [`vf-graphql-holochain/`](modules/vf-graphql-holochain/README.md) contains the complete GraphQL schema adapter with bindings to Holochain DNA conductors (which expose the app DNAs defined in `happs/`).
- `scripts/` just has utility script files used for DevOps tasks and configuring the repo.
- `test/` contains JavaScript integration tests for the system as a whole&mdash; they cover zome API tests and interactions between different app DNAs.
- The NPM scripts in `package.json` at the top level of the repo are used to orchestrate test commands & run the apps for development. Please see the file for reference.

## License

Licensed under an Apache 2.0 license.
