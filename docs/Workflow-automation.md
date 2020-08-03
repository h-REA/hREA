# Development workflow automation

> Helpful commands to make your life easier.

<!-- MarkdownTOC -->

- [Task automation](#task-automation)
- [Creating new DNAs](#creating-new-dnas)
- [Creating new zomes](#creating-new-zomes)
- [Creating new cross-DNA indexes](#creating-new-cross-dna-indexes)
- [Updating the Holochain platform](#updating-the-holochain-platform)
	- [Updating your local workspace after an upgrade](#updating-your-local-workspace-after-an-upgrade)

<!-- /MarkdownTOC -->


## Task automation

We use [Hygen](https://www.hygen.io/) to manage repetitive tasks within the codebase. The `_templates` folder in the root of the repository contains [ejs templates](https://github.com/mde/ejs) that deploy sets of parameterised files into the correct directory structures.

Whenever you find yourself doing something repetitive, consider adding a Hygen template for it. This will speed up your work, make it easier for others to contribute and also creates a set of self-documenting code patterns used in the project.



## Creating new DNAs

1. `cd happs/`
2. `hc init <NEW_DNA_NAME>` scaffolds a new DNA folder named `NEW_DNA_NAME`.
3. Edit `app.json` in the newly created folder as appropriate.
4. Remove these generated files from the newly created directory:
	- `test/` (integration tests are all contained in the top-level `test` directory)
	- `.gitignore` (already taken care of via project-global ignore file)
5. Wire up a new `build` sub-command in the toplevel `package.json`; eg. `"build:dna_obs": "cd happs/observation && hc package"`. Do not forget to add the new build step to the base NPM `build` script.
6. Edit `conductor-config.toml` as appropriate to include instance configuration & bridging for any new DHTs to be loaded from this DNA in the local test environment.


## Creating new zomes

1. From the root directory, run `npx hygen init-zome mixin-zome-libs`. Answering the questions will yield a generated directory structure that defines the zome code, optimally organised for zome integration, re-use and recomposition.
2. After generating the zome module crates, you usually want to generate the zome definition itself, using `npx hygen init-zome new-zome`. This will yield a zome directory configuration suitable for placement inside an existing DNA directory that has some initial imports configured to load a corresponding *'mixin zome lib'*.
3. Add the new zome & library paths to the `members` section of `Cargo.toml` workspace in *this* directory.
4. Begin filling in the blanks or removing CRUD API routes as needed.


## Creating new cross-DNA indexes

*(This presumes you have already created the zomes for both the source and destination record types, and that those zomes reside in different DNAs.)*

1. Use the generator command `npx hygen init-zome new-index-zome` and answer the prompts.
2. Edit the necessary constants in the generated `lib.rs` file, taking care to not to confuse the `FWD` (usually many:1) and `RECIPROCAL` (1:many) link types.


## Updating the Holochain platform

The instructions apply to the officially supported Nix release of Holochain. An upgrade for natively installed Rust packages is much the same, except for use of `cargo install` commands instead of the Nix-specific steps.

1. Upgrade Holonix according to the "how to upgrade holonix" section on https://docs.holochain.love/docs/configure/
	- Change `ref` in `config.nix` to the latest Holonix release tag on Github and alter `sha256` to invalidate its cache
	- Attempt to drop into the nix shell, it will error with “hash mismatch”
	- Copy the “got:” hash for the new ref to `holonix.github.sha256`
2. Run `nix-shell` to boot into the Nix environment
3. `npm run clean:build` from the root directory to wipe Rust build files and refresh the cargo cache to match the new HC version.
4. Change `HDK_RUST_REVID` in `scripts/postinstall.sh` to match the version you have updated to so that new contributors have their tooling configured properly. The appropriate HDK revision ID can be found in `dist/config.nix` in the Holonix repository.
5. Locate all other references to the old Holochain dependency versions in `Cargo.toml` files and update to the new `HDK_RUST_REVID` version. All instances should be locateable by searching the codebase for the string `:DUPE: hdk-rust-revid`.
	- **Important:** if the repo is using git submodules for dependency management, ensure that the changes to the manifest files in the submodule are also committed & pushed to their remotes; and that the changed submodule reference in the toplevel Holo-REA repository is also commited and pushed.
6. Ensure the latest available version of [Try-o-rama](https://www.npmjs.com/package/@holochain/try-o-rama) is also configured in `test/package.json`.


### Updating your local workspace after an upgrade

When another contributor upgrades the Nix or Node configurations in this repository, you will need to perform some steps to re-sync your working copy with the new version after pulling such changes.

If in doubt, you can generally get back up and running again using `npm run clean` and then re-entering `nix-shell`, but often this is overkill (and will result in you unnecessarily downloading a lot of data).

**After an update to `config.nix`:**

1. Enter the project's `nix-shell` as usual.
2. `npm run clean:build` from the root directory to wipe all Rust build artifacts and caches completely.
3. Exit the Nix shell.
4. Re-enter the Nix shell, and you are ready to re-build the DNA modules.

**After an update to any `package.json`:**

*(Note: this is only necessary if `dependencies` or `devDependencies` have changed.)*

1. Run `pnpm install` again.
2. Node-based commands (eg. `build:graphql-adapter`, `dev:graphql-explorer`) should all run fine now.
3. If any errors persist, try `npm run clean:modules && pnpm install` to ensure your module folders have been pruned.
