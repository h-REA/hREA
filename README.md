# hREA: The Holochain/REA coordination framework

<!-- MarkdownTOC -->

- [About](#about)
- [Documentation](#documentation)
- [hREA beyond Holochain](#hrea-beyond-holochain)
- [Other names](#other-names)
- [License](#license)

<!-- /MarkdownTOC -->


## About

[hREA](https://hrea.io) is a suite of functionally independent building blocks affording most functionality commonly used in supply chain systems, project management software, logistics management and enterprise resource planning; as well as post-capitalist economic paradigms such as gift and contributory economies.

Using [holochain](https://holochain.org) for data storage, data integrity, networking, and runtime, hREA comes with an [adapter library for GraphQL in Javascript](https://www.npmjs.com/package/@vf-ui/graphql-client-holochain), which is the main way to interface with the hREA suite.

What do we mean by "most functionality"?-

- **people & groups**: use the *agent* kit to keep track of people and groups and build trust in the network
- **scheduled deliverables**: use the *plan* kit to create a schedule of related operational processes with defined deliverables
- **agreements and contracts**: use the *agreement* kit when handling market exchanges (e.g. purchases) or other contracts and mutual obligations
- **event ledger**: use the *observation* kit to track the observed movements of resources, currencies and skills in any coordination space
- **coordination functions**: use the *planning* kit to decide on future plans, manage agreements or coordinate actions with other participants
- **needs matching**: use the *proposal* kit to group matched outcomes in order to describe bilateral and multilateral trade requests

All 6 of these come with the pre-packaged "full suite" that is the default configuration that is shipped.

These other capabilites are some of what's on the roadmap:
- **group management**: manage groups of collaborators and permission access between groups, sub-projects and across organisations
- **knowledge sharing**: use the *recipe* module to share structured production knowledge and easily repeat well-understood processes

A key aspect to note about these modules is that *they require no technical knowledge to remix or re-architect into different social organising patterns*. Different arrangements of network components can be used to address different use-cases in novel ways.

Most people making use of hREA will never have to delve into the software beyond this level. All modules in the suite have established APIs for interoperability and can be arranged into complex organisational patterns at runtime, like lego blocks.

Beyond this outer layer the system has been designed with flexibility, modularity and composability as core architectural concerns. The depth to which you will delve into the architecture depends on a project's needs; i.e. how much customisation is required. See [Repository Structure](#repository-structure) for a breakdown of how hREA fits together, how to customise it, and how to browse this repository.



## Documentation

Documentation for use of the hREA libraries and packages can be found at [docs.hrea.io](https://docs.hrea.io). 

Documentation for the underlying ValueFlows ontology can be found at [valueflo.ws](https://www.valueflo.ws).

Documentation (**warning: out of date**) for potential collaborators and entrepreneurs can be found in the project's [ecosystem wiki](https://github.com/h-REA/ecosystem/wiki/). This includes information on hREA's organisational goals, strategic mission, design philosophy, cultural background and ideological positioning.

For developers looking to work on the code in this repository, documentation can be found in the [`docs/`](docs/README.md) directory. We keep it within the codebase instead of in the wiki so that all contributors retain the information necessary to understand, configure and run the system. There is a [quick start guide](docs/README.md#quick-start) for those who want to spin up hREA locally for development or experimentation. To understand this whole repository and also how you might go about customising at different layers of the composable stack, check out [repository structure](docs/repository-structure.md).


## hREA beyond Holochain

hREA is built to implement the [ValueFlows protocol](https://valueflo.ws/)&mdash; a set of common vocabularies based on [REA Accounting theory](https://en.wikipedia.org/wiki/Resources,_events,_agents_(accounting_model)) to describe flows of economic resources of all kinds within distributed economic ecosystems.

By building to align with the [ValueFlows GraphQL spec](#valueflows-graphql-protocol-layer), UI applications built for hREA are automatically compatible with other ValueFlows-compatible system backends like our partner project [Bonfire](https://bonfirenetworks.org/).

The goal is to enable radical code reuse and cross-project interoperability between next-gen distributed backend systems and traditional web infrastructure, and to allow user interfaces to span multiple disparate apps.



## Other names

Previously, this work was referred to as *"Holo-REA"* or sometimes *"HoloREA"*. All these labels refer to the same project.


## 🌳 Oak Sponsors

- [Holo Ltd](https://holo.host)
- [Eric Meller](https://twitter.com/EricMeller)
- [Thomas Miller](https://www.linkedin.com/in/thomas-miller-3895833b)
- [Art Brock](https://www.artbrock.com/)


## License

Licensed under an [Apache 2.0 license](https://www.apache.org/licenses/LICENSE-2.0).
