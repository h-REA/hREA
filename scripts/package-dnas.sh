#!/usr/bin/env bash
#
# Runs the Holochain DNA bundler utility against all configurations in the `happs` dir
#
# @package: Holo-REA
# @since:   2021-02-09
#
##

UTIL="${HOLOCHAIN_DNA_UTIL_PATH:-hc}"

for DIR in happs/*; do
  if [[ -d "$DIR" ]]; then
    echo "Compiling DNA in $DIR"
    if "$UTIL" dna pack "$DIR"; then
      echo "    packing succeeded."
    else
      echo -e "    [FAIL]"
    fi
  fi
done
