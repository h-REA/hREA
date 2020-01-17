#!/usr/bin/env bash
#
# Builds the stock observation DNA and copies binaries in for DNA compilation.
#
# @package: Holo-REA
# @since:   2020-01-17
#
##

cd ../../
npm run build:dna_obs

cp /tmp/holochain/target/wasm32-unknown-unknown/release/hc_zome_rea_process.wasm \
   example/custom-resource-attributes/zomes/process/code/hc_zome_rea_process.wasm
cp /tmp/holochain/target/wasm32-unknown-unknown/release/hc_zome_rea_fulfillment_index.wasm \
   example/custom-resource-attributes/zomes/fulfillment/code/hc_zome_rea_fulfillment_index.wasm
cp /tmp/holochain/target/wasm32-unknown-unknown/release/hc_zome_rea_satisfaction_index.wasm \
   example/custom-resource-attributes/zomes/satisfaction/code/hc_zome_rea_satisfaction_index.wasm
