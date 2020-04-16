# ValueFlows economic resource coordination software: Holochain implementation

A resource accounting framework and suite of apps for building economic coordination applications of any kind.

Implements a highly modular [Resource / Event / Agent (REA)](https://en.wikipedia.org/wiki/Resources,_events,_agents_(accounting_model)) network accounting system, based on the [ValueFlows](https://valueflo.ws/) protocol and built on [Holochain](https://holochain.org/).

<!-- MarkdownTOC -->

- [About](#about)
- [Documentation](#documentation)
- [Repository structure](#repository-structure)
- [License](#license)

<!-- /MarkdownTOC -->

## About

This Holochain application is actually a *suite of cooperating applications*, similar to 'microservices' in traditional client/server web architecture. Each app is packaged as a *'DNA'*- a concept specific to Holochain apps which essentially refers to a content-addressable network with its own application logic; in other words, Holochain apps are protocols defined as self-contained code bundles and are referenced by the hash of that code.

HoloREA is built to align with the [Open App Ecosystem](https://github.com/open-app/)'s ideologies and goals as well as that of the [Free Software Movement](https://www.gnu.org/philosophy/free-software-intro.en.html). Thus HoloREA's *'DNAs'* can be thought of as a framework for composing more complex end-user Holochain apps. For example, you might create a Holochain app to manage the internal logic of your cooperative or business and have it publish events out to separate HoloREA *'observation'* networks in order to share resources with distributors and suppliers. You might also choose to swap out HoloREA's *'agreements'* module with your own logic for managing agreements; or even combine HoloREA's own modules in nonstandard arrangements, like having multiple separate *'observation'* modules communicating with a shared *'planning'* space.

## Documentation

[`docs/`](docs/README.md) contains the developer documentation, written in markdown. We keep it within the codebase instead of in the wiki so that all contributors retain the information necessary to understand, configure and run the system.

There is a [quick start guide](docs/README.md#quick-start) for those who want to spin up Holo-REA locally for development or experimentation.

See the [ecosystem wiki](https://github.com/holo-rea/ecosystem/wiki/) for more information on Holo-REA's organisational goals, strategic mission, design philosophy, cultural background and ideological positioning.

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
