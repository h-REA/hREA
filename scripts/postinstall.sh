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
HDK_RUST_REVID=v0.0.27-alpha1

# :IMPORTANT: must also be changed in package.json scripts
HOLONIX_VER="0.0.29"  # Note that this determines $HDK_RUST_REVID when running via Nix

DEP_ERR_OUTTRO="Please see README for setup instructions."

HAS_NIX=$(command -v nix-shell >/dev/null 2>&1)
HAS_NIX=$?

if [[ $HAS_NIX ]]; then
  echo "Nix installed, using standad install."

  if [[ -d "./holonix-${HOLONIX_VER}" ]]; then
    echo "Holonix already downloaded, skipping!"
  else
    echo "Holonix not cached yet, downloading latest supported version..."

    wget "https://github.com/holochain/holonix/archive/${HOLONIX_VER}.tar.gz" -O "./holonix-${HOLONIX_VER}.tar.gz"
    tar -zxf "./holonix-${HOLONIX_VER}.tar.gz"
    rm "./holonix-${HOLONIX_VER}.tar.gz"

    echo "Done."
  fi

  echo "Initial shell will now boot to install dependencies."
  echo ""
  echo "Use 'npm run shell' or this command in future:"
  echo "    nix-shell holonix-${HOLONIX_VER}"
  echo ""
  echo "Hit CTRL-D to continue once first boot is complete."

  nix-shell holonix-${HOLONIX_VER}
else
  echo -e "Nix not installed... attempting Rust toolchain for advanced install...">&2;

  command -v rustup >/dev/null 2>&1 || {
    echo -e "\e[1m\e[31mRust not installed! $DEP_ERR_OUTTRO">&2;
    exit 1
  }

  # RUST-BASED INSTALL

  # activate & lock rust toolchain for this repo
  rustup override set nightly-2019-02-04

  # install rust deps
  rustup target add wasm32-unknown-unknown
  rustup component add clippy
  cargo install rustfmt
  # install Holochain CLI and runtime at known compatible version
  cargo install holochain hc --git https://github.com/holochain/holochain-rust.git --tag $HDK_RUST_REVID

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

# munge some stuff for development ease-of-use
# (note CWD is root folder as this script is driven by NPM)
node scripts/fixReactAppSourceMaps.js
