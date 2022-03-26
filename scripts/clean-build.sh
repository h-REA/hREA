#!/usr/bin/env bash
#
# Clean script for all build files
#
# @package: HoloREA
# @since:   2019-06-13
#
##

nix-shell --run hn-node-flush
nix-shell --run hn-rust-flush

rm bundles/dna/**/*.dna
rm bundles/app/**/*.happ
rm bundles/web-app/*.webhapp
