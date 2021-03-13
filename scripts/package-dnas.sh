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
    echo -e "\e[1mCompiling DNA in $DIR\e[0m"
    if "$UTIL" dna pack "$DIR" 2>/dev/null; then
      echo -e "\e[1;32m    packing succeeded.\e[0m"
    else
      echo -e "\e[1;31m    [FAIL]\e[0m"
    fi
  fi
done
