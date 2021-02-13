#!/usr/bin/env bash
#
# Runs the Holochain DNA bundler utility against all configurations in the `happs` dir
#
# @package: Holo-REA
# @since:   2021-02-09
#
##

UTIL="${HOLOCHAIN_DNA_UTIL_PATH:-dna-util}"

for DIR in happs/*.dna.workdir; do
  echo "Compiling DNA in $DIR"
  "$UTIL" -c "$DIR"
done
