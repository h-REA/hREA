# ValueFlows economic resource coordination software: Holochain implementation

<!-- MarkdownTOC -->

- [Packages](#packages)
- [Setup](#setup)
- [Contributing](#contributing)

<!-- /MarkdownTOC -->

## Packages

- `holo-rea-dht/`: Zome API code, written in Rust. Executes within a Holochain DHT runtime.
- `holo-rea-graphql/`: [ValueFlows](http://valueflo.ws/)-compliant GraphQL API wrapping the above Holochain DHT, written in TypeScript. Executes in a browser, desktop or nodejs environment.

## Setup

1. Ensure you have all necessary [required software](./CONTRIBUTORS.md#required-software) installed.
2. Run `yarn` from this directory to bootstrap the repo.


## Contributing

For information on our workflow and contribution guidelines, see [CONTRIBUTORS.md](./CONTRIBUTORS.md).

Developers wishing to contribute should also refer to [recommended dev tools](./CONTRIBUTORS.md#recommended-dev-tools) for assistance in configuring your workstation for the most streamlined and pleasant development experience.
