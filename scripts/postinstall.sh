#!/usr/bin/env bash
#
# Repository setup script
#
# @package: HoloREA
# @since:   2019-02-04
#
##

# :DUPE: hdk-rust-revid
HDK_RUST_REVID=v0.0.50-alpha4

DEP_ERR_OUTTRO="Please see README for setup instructions."

HAS_NIX=$(command -v nix >/dev/null 2>&1)
HAS_NIX=$?

if [[ $HAS_NIX ]]; then
  echo "Nix is installed- simply run \`nix develop\` to begin developing!"
else
  echo -e "Nix not installed... attempting Rust toolchain for advanced install...">&2;

  command -v rustup >/dev/null 2>&1 || {
    echo -e "\e[1m\e[31mRust not installed! $DEP_ERR_OUTTRO">&2;
    exit 1
  }

  # RUST-BASED INSTALL

  # install rust deps
  rustup target add wasm32-unknown-unknown
  rustup component add clippy
  cargo install rustfmt
  # install Holochain CLI and runtime at known compatible version
  cargo install holochain hc --git https://github.com/holochain/holochain.git --tag $HDK_RUST_REVID

  # END INSTALL

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
fi
