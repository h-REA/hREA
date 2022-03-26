#!/usr/bin/env bash
#
# Runs the Holochain DNA bundler utility against all configurations in the `dna_bundles` dir
#
# @package: Holo-REA
# @since:   2021-02-09
#
##

UTIL="${HOLOCHAIN_DNA_UTIL_PATH:-hc}"

# optimise all WASMs first
if [ $RUN_WASM_OPT -ne "0" ]; then
  for WASM in target/wasm32-unknown-unknown/release/*.wasm; do
    echo -e "\e[1mOptimising $WASM\e[0m..."
    wasm-opt -Oz "$WASM" --output "$WASM"
  done
fi

# compile DNAs by concatenating WASMs with properties
for DIR in bundles/dna/*; do
  if [[ -d "$DIR" ]]; then
    echo -e "\e[1mCompiling DNA in $DIR\e[0m"
    if "$UTIL" dna pack "$DIR" 2>/dev/null; then
      echo -e "\e[1;32m    packing succeeded.\e[0m"
    else
      echo -e "\e[1;31m    [FAIL]\e[0m"
    fi
  fi
done

# compile hApp bundles by concatenating DNAs and specifying any config
for DIR in bundles/app/*; do
  if [[ -d "$DIR" ]]; then
    echo -e "\e[1mBundling hApp in $DIR\e[0m"
    if "$UTIL" app pack "$DIR" 2>/dev/null; then
      echo -e "\e[1;32m    packing succeeded.\e[0m"
    else
      echo -e "\e[1;31m    [FAIL]\e[0m"
    fi
  fi
done
