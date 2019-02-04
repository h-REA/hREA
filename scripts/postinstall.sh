#!/usr/bin/env bash
#
# Repository setup script
#
# @package: HoloREA
# @author:  pospi <pospi@spadgos.com>
# @since:   2019-02-04
#
##

DEP_ERR_OUTTRO="Please see README for setup instructions."

command -v rustup >/dev/null 2>&1 || {
  echo -e "\e[1m\e[31mRust not installed! $DEP_ERR_OUTTRO">&2;
  exit 1
}

pushd holo-rea-dht
  # activate & lock rust toolchain for this repo
  rustup override set nightly-2019-02-04
  # install Holochain CLI at known compatible version
  cargo install hc --git https://github.com/holochain/holochain-rust.git --rev 4c442073
popd

command -v hc >/dev/null 2>&1 || {
  echo -e "\e[1m\e[31mHolochain CLI not installed! $DEP_ERR_OUTTRO">&2;
  exit 1
}
