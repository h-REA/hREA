#!/usr/bin/env bash
#
# Repository setup script
#
# @package: HoloREA
# @author:  pospi <pospi@spadgos.com>
# @since:   2019-02-04
#
##

# :DUPE: hdk-rust-revid
HDK_RUST_REVID=v0.0.19-alpha1

DEP_ERR_OUTTRO="Please see README for setup instructions."

command -v rustup >/dev/null 2>&1 || {
  echo -e "\e[1m\e[31mRust not installed! $DEP_ERR_OUTTRO">&2;
  exit 1
}

# activate & lock rust toolchain for this repo
rustup override set nightly-2019-02-04

# install rust deps
rustup target add wasm32-unknown-unknown
rustup component add clippy
cargo install rustfmt
# install Holochain CLI and runtime at known compatible version
cargo install holochain hc --git https://github.com/holochain/holochain-rust.git --tag $HDK_RUST_REVID

# munge some stuff for development ease-of-use
# (note CWD is root folder as this script is driven by NPM)
node scripts/fixReactAppSourceMaps.js

# post-install check
command -v rustfmt >/dev/null 2>&1 || {
  echo -e "\e[1m\e[31mRustfmt not installed! $DEP_ERR_OUTTRO">&2;
  exit 1
}
command -v hc >/dev/null 2>&1 || {
  echo -e "\e[1m\e[31mHolochain CLI not installed! $DEP_ERR_OUTTRO">&2;
  exit 1
}
command -v holochain >/dev/null 2>&1 || {
  echo -e "\e[1m\e[31mHolochain runtime not installed! $DEP_ERR_OUTTRO">&2;
  exit 1
}
