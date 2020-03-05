# Developer documentation

<!-- MarkdownTOC -->

- [Quick start](#quick-start)
	- [Install Nix](#install-nix)
	- [Init the project](#init-the-project)
- [Running](#running)
	- [Advanced execution](#advanced-execution)
	- [Debugging](#debugging)
- [Contributing](#contributing)
	- [Known issues](#known-issues)
	- [Gotchas](#gotchas)
- [Multi-project setup](#multi-project-setup)
- [Recommended dev tools](#recommended-dev-tools)
	- [Code quality](#code-quality)
		- [Linters](#linters)
		- [Editorconfig](#editorconfig)
		- [File headers](#file-headers)

<!-- /MarkdownTOC -->


## Quick start

(This is a short version of the [official Holochain install instructions](https://developer.holochain.org/start.html).)

### Install Nix

You need to run your Holochain tooling (`hc` & `holochain` binaries, `cargo`, `rustc`, `node` etc **and your editor**) from within a Nix shell in order to have access to all the CLI applications you'll need in development. It is installed via:

    curl https://nixos.org/nix/install | sh

You should now have `nix-shell` available in your PATH and be able to proceed with running this package's installation steps.

**Linux users:** if you get warnings about `libgtk3-nocsd.so.0`, you should add this line to your `~/.profile` (or other) file before the `nix.sh` line:

    export LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libgtk3-nocsd.so.0

### Init the project

1. Run `nix-shell` from within the project directory to enter the development environment. **All editor tooling should be booted from within this shell!**
2. Run `yarn` to install project dependencies.

Once configured, you should run `nix-shell` any time you're working on this project to bring all tooling online.




## Running

Once installation has completed you can run `nix-shell` (if you have not already done so) followed by `npm start` to boot up the following services.

**DO NOT USE https://holochain.love WITH THIS REPOSITORY!!** If you do, you will be using the wrong version of Holochain core and may encounter errors.

- [GraphiQL query interface](apps/holorea-graphql-explorer) backed by the [ValueFlows GraphQL spec](https://github.com/valueflows/vf-graphql/) at `http://localhost:3000`
- Holochain DNA HTTP interface at `http://localhost:4000`
- Holochain DNA websocket RPC interface at `ws://localhost:4001`
- TypeScript compiler daemon for rebuilding `vf-graphql-holochain` browser module upon changes

### Advanced execution

If you look at the commands in `package.json` you will see that they are namespaced into groups of functionality. You can also see which commands depend on each other. Most of the time it will be more efficient to understand the command structure and run individual commands than it will be to boot the whole system together.

There are some key commands you should be aware of in order to best understand and utilise this repository without encountering confusion:

- `dht:sim2h`: this command **must be running** in the background in order for any Holochain apps to function. If the sim2h network backend is not active you will experience hangs & timeouts in tests and in running the conductor. Note that the higher-level test commands (eg. `test:integration`) all boot sim2h as they execute.
- Therefore, if running tests directly / individually with `npx tape test/**/*.js` (or other test file globs), you will have to `npm run dht:sim2h` in another terminal.

Something you may find painful when debugging is that the `react-scripts` Webpack configuration used by some UI apps clears the terminal when it is active. To work around this, you can run these commands in separate terminals so that the output is not truncated. Running the system like this would be a case of:

- Running `npm run build` first
- `npm run dht` in a separate terminal to boot the network backend & Holochain conductor
- `npm run dev:graphql-adapter` in its own terminal if you plan on editing the GraphQL code & want realtime feedback on your changes
- `npm run dev:graphql-explorer` to boot up the GraphiQL app UI to interact with the DNAs, or boot any other UI apps instead

### Debugging

Most of the time during development, you won't want to run the whole test suite but rather just those tests you're currently working on. The usual workflow is:

1. `npm run build` or one of the sub-commands (eg. `npm run build:dna_obs`) to rebuild the module(s) you are working on.
2. `npx tape test/**/*.js` to run specific tests, substituting a path to an individual file.

Test output from the Holochain conductor can be noisy. We recommend using a unique logging prefix and grepping the output, whilst printing JavaScript-level debug logs to stderr. In other words:

- In your Rust code, prefix any debug logging with some string:
  ```rust
  let _ = hdk::debug(format!("WARGH {:?}", something));
  ```
- In JavaScript code, use `console.error` instead of `console.log`:
  ```javascript
  console.error(require('util').inspect(something, { depth: null, colors: true }))
  ```
- Now run tests similarly to `npx tape test/**/*.js | grep WARGH` and you should only be seeing what's of interest.

Another useful command to pipe test output to is `npx faucet`, which will hide all of the test output except for the failures. A nice trick to use here is that STDERR will still be printed, so if you use `console.error` in your tests you will still see that output even when hiding logging output with `faucet`.

For more complex debugging situations there is also an environment variable `VERBOSE_DNA_DEBUG=1` which can be used to show additional debug output from the conductor.





## Contributing

If you plan on contributing to HoloREA's development, please read the following after you have configured your development environment as above:

- [Contributor workflow](Contributor-workflow.md) (contribution protocol, git best practises & coding standards)
- [Workflow automation](Workflow-automation.md) (how to perform common development tasks)
- "[For new code contributors](https://github.com/holo-rea/ecosystem/wiki/For-new-code-contributors)" on the project ecosystem wiki has further information on how to engage with the project.

### Known issues

- The Visual Studio Code terminal can cause issues with Nix, especially on Windows. Use a standalone terminal instead of the one built in to the editor avoid potential problems.
- If you get `Bad owner or permissions on $HOME/.ssh/config` when attempting to use git remote commands or SSH from within the Nix shell, ensure your `~/.ssh/config` has `0644` permissions and not `0664`.

### Gotchas

- Inconsistent state behaviours in tests:
	- This is most often due to mis-use of `await s.consistency()` in test code. Ensure that consistency checks are *only* present after `mutation` GraphQL operations and JSONRPC calls which modify the source-chain state; i.e. after a GraphQL `query` one should *not* perform a consistency wait.
- Receiving incorrect record IDs when retrieving records:
	- These errors are often encountered when confusing cross-DNA link fields for same-DNA links. Check that you are using the appropriate helpers for the link type (`local_index` vs `remote_index` helpers).
- Generic internal errors of *"Unknown entry type"*:
	- This happens when attempting to create an entry link with a type that has not been defined for the entry. Ensure your `link_type` values defined for the entry match those being used elsewhere in the code.
- Receiving errors like *"Could not convert Entry result to requested type"* when creating or modifying entries:
	- This is usually due to an incorrect entry type definition in an entry's `validation` callback. The `hdk::EntryValidationData` must be declared with the appropriate entry's type.



## Multi-project setup

For developers who need to work on other ValueFlows-related codebases whilst developing Holo-REA, check out the [ValueFlows project metarepo](https://github.com/holo-rea/valueflows-project-metarepo/).



## Recommended dev tools

### Code quality

#### Linters

For Rust, install [Clippy]. `rustup component add clippy` is executed after setting up the repo, so you should not need to do anything other than setup Rust for your editor:

- **Sublime Text:**
  - `Rust Enhanced` and `SublimeLinter-contrib-rustc` via Package Control will give you autocomplete and error output upon saving files.
- **VSCode:**
  - Install the `Rust (rls)` extension via the marketplace
  - Set `rust-client.disableRustup = false` in the editor configuration (Rust versions are managed by Nix)
  - For advanced users you can also setup a language server to get realtime code hinting & errors as you type, [for more info, see here](https://hoverbear.org/2017/03/03/setting-up-a-rust-devenv/).

For JavaScript, install [eslint]. All necessary dependencies should be installed via NPM upon initialising the repository, but you must still configure your editor to show linter output:

- **Sublime Text:**
    - `SublimeLinter-eslint` and `SublimeLinter-tslint` are both used, depending on whether editing JS or TS files.
- **VSCode:**
    - Install the `ESLint` extension via the marketplace.

#### Editorconfig

This ensures consistency in file formatting. Install a plugin for your editor according to the following:

- **Sublime Text:**
  - `EditorConfig` via Package Control
- **VSCode:**
  - `EditorConfig for VSCode` via the marketplace

#### File headers

You can configure your editor to automatically add new header comment blocks to files you create.

- **Sublime Text:**
  - Install `FileHeader` via Package Control
  - Go to *Preferences > Package Settings > FileHeader > Settings - User* to locate your `custom_template_header_path`
  - Also add this block to your settings:
    ```
    "Default": {
      "author": "YOURNAME",
      "email": "YOURNAME@example.com"
    },
    ```
  - *(Note this configuration can also be specified on a per-project basis under `settings.FileHeader` in your project config JSON file.)*
  - Edit files in this folder to set the content to prepend to new files you create.
- **VSCode:**
  - *:TODO:*
