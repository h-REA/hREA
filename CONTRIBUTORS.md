# Contribution guidelines


<!-- MarkdownTOC -->

- [Required software](#required-software)
- [Recommended dev tools](#recommended-dev-tools)
	- [Code quality](#code-quality)
		- [Editorconfig](#editorconfig)
		- [File headers](#file-headers)
	- [Version locking](#version-locking)
- [Git conventions](#git-conventions)
	- [Best practises](#best-practises)
	- [Branching workflow](#branching-workflow)
	- [Release management](#release-management)

<!-- /MarkdownTOC -->


## Required software

(This is a short version of the [official Holochain install instructions](https://developer.holochain.org/start.html)

**Nodejs**

1. For development, it is highly recommended to [install NVM](https://github.com/creationix/nvm) to manage nodejs versions. Once installed:

	nvm install $(cat .nvmrc)

Or if you wish to do it manually, ensure the version of node you're using corresponds with that indicated in the `.nvmrc` file.

2. It is also recommended to install a shell hook for `.nvmrc` files in order to ensure that your nodejs version remains synchronised with the project. Simply add this to `~/.bashrc`:

	#--------[ Ensure node version is correct everywhere

	cd () { builtin cd "$@" && chNodeVersion; }
	pushd () { builtin pushd "$@" && chNodeVersion; }
	popd () { builtin popd "$@" && chNodeVersion; }
	chNodeVersion() {
	    if [ -f ".nvmrc" ] ; then
	        nvm use;
	    fi
	}
	chNodeVersion;

3. Once nodejs is setup, install Yarn if you don't already have it: `npm i -g yarn`.

**Rust**

For development, it is highly recommended to install via RustUp:  

    curl https://sh.rustup.rs -sSf
    source $HOME/.cargo/env
    rustup toolchain install nightly-2019-02-04
    rustup default nightly-2019-02-04	# optional
    rustup target add wasm32-unknown-unknown --toolchain nightly-2019-02-04

We also recommend to set a default toolchain override for this directory when cloning. This is done automatically when running NPM setup- see `scripts/postinstall.sh` for details.
    
**ZeroMQ**

*(for Ubuntu users:)*
    
    cd /tmp
    wget https://github.com/zeromq/libzmq/releases/download/v4.3.1/zeromq-4.3.1.tar.gz
    tar -zxf ./zeromq-4.3.1.tar.gz
    cd zeromq-4.3.1

    sudo apt install libtool

    ./autogen.sh
    ./configure
    make -j 4
    sudo make install
    sudo ldconfig

**`hc` toolchain**

The Holochain toolchain will be installed for you at a known working version when initialising this repo. If  you see the error *"binary `hc` already exists"* upon installing or you wish to install yourself, you can do so with the following command, substituting `branch` or `ref` to target a specific version from git. Note that you must have Rust and ZeroMQ installed before proceeding to this step.

    cargo install hc --force --git https://github.com/holochain/holochain-rust.git --branch develop



## Recommended dev tools

### Code quality

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
			"email": "YOURNAME@consensys.net"
		},
	  ```
	- *(Note this configuration can also be specified on a per-project basis under `settings.FileHeader` in your project config JSON file.)*
	- Edit files in this folder to set the content to prepend to new files you create.
- **VSCode:**
	- *:TODO:*


### Version locking

This project uses [`.nvmrc` files](https://github.com/creationix/nvm#nvmrc) to specify the correct nodejs versions to run when developing. You can install some additional shell hooks into [zsh on OSX](https://github.com/creationix/nvm#zsh) or place this in your `.bashrc` on Linux to auto-switch to the correct node version as you move around:

```
cd () { builtin cd "$@" && chNodeVersion; }
pushd () { builtin pushd "$@" && chNodeVersion; }
popd () { builtin popd "$@" && chNodeVersion; }
chNodeVersion() {
    if [ -f ".nvmrc" ] ; then
        nvm use;
    fi
}
chNodeVersion;
```

To manage Rust versions, we presume that you are using [RustUp](https://rustup.rs/). With this under consideration, there is a toolchain override for the project pre-configured in `scripts/postinstall.sh`.






## Git conventions


### Best practises

- Commit messages should take the imperative form; ie. finish the sentence *"Applying this commit will [...]"*
- Use descriptive, single-line commit messages
- Make commits as atomic as possible. This makes life easier for a [variety of reasons](https://brainlessdeveloper.com/2018/02/19/git-best-practices-atomic-commits/). Heavily consider using `git add -p`, and *never* use `git add -A`.


### Branching workflow

We use a [gitflow](https://danielkummer.github.io/git-flow-cheatsheet/)-inspired but slightly less onerous process:

- We work off `master` and consider it our stable integration branch
- All work on improving the systems occurs in branches prefixed with `feature/XX-`, where `XX` is the related Github issue number
	- Feature branches are merged back to `master` *only* when fully integrated and tested
	- It is expected that developers working on new features merge in updates from `master` as work unfolds in order to minimise merge conflicts later
	- It is fine to merge work from another feature branch into your own in the case of interdependent features which cannot be completed without integrating
- Miscellaneous patches which don't fall under the usual improvement workflow can be undertaken in `hotfix/XX-` branches and merged back to master when tested and ready for deployment
- When finishing up work in any branch, it should be deleted and removed from Github.


### Release management

- A `release/` branch is opened to commit updates to package version numbers, README files and any other release prep work (eg. building documentation)
- Once ready, `release/` is merged to `master`.
- The resultant commit is tagged using semver with no prefix, eg. `0.13.2`.
