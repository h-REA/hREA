#!/usr/bin/env bash
#
# Runs the Holochain DNA bundler utility against all configurations in the `dna_bundles` dir
#
# @package: hREA
# @since:   2021-02-09
#
##

UTIL="${HOLOCHAIN_DNA_UTIL_PATH:-hc}"

# determine repository root for substitution
ROOT_PATH=$(dirname "$0")
ROOT_PATH=$(cd "$ROOT_PATH" && pwd)
ROOT_PATH=$(dirname "$ROOT_PATH")
ROOT_PATH=$(printf '%s\n' "$ROOT_PATH" | sed -e 's/[\/&]/\\&/g') # make safe for sed

# optimise all WASMs first
if [[ $RUN_WASM_OPT -ne "0" ]]; then
  for WASM in target/wasm32-unknown-unknown/release/*.wasm; do
    echo -e "\e[1mOptimising $WASM\e[0m..."
    wasm-opt -Oz "$WASM" --output "$WASM"
  done
fi

# remove any stale DNA & app bundle files; refresh from templates
rm -Rf bundles/dna
cp -a bundles/dna_templates bundles/dna
rm -Rf bundles/app
cp -a bundles/app_templates bundles/app

# sed -i.bak works on both mac and linux
# https://stackoverflow.com/a/22084103/2132755

# compile DNAs by concatenating WASMs with properties
for DIR in bundles/dna/*; do
  if [[ -d "$DIR" ]]; then
    # @see https://github.com/holochain/holochain/issues/966
    # toggle `path`/`bundled` depending on build mode
    if [[ $BUNDLE_ZOMES -eq "1" ]]; then
      sed -i.bak "s/path:/bundled:/g" "$DIR/dna.yaml"
    fi
    # substitute absolute paths for compatibility with `path` or `bundled`
    sed -i.bak "s/<repository-path>/${ROOT_PATH}/g" "$DIR/dna.yaml"
    rm "$DIR/dna.yaml.bak"

    echo -e "\e[1mCompiling DNA in $DIR\e[0m"
    if "$UTIL" dna pack "$DIR"; then
      echo -e "\e[1;32m    packing succeeded.\e[0m"
    else
      echo -e "\e[1;31m    [FAIL]\e[0m"
    fi
  fi
done

# compile hApp bundles by concatenating DNAs and specifying any config
for DIR in bundles/app/*; do
  if [[ -d "$DIR" ]]; then
    sed -i.bak "s/<dna-build-path>/${ROOT_PATH}\/bundles\/dna/g" "$DIR/happ.yaml"
    rm "$DIR/happ.yaml.bak"

    echo -e "\e[1mBundling hApp in $DIR\e[0m"
    if "$UTIL" app pack "$DIR" 2>/dev/null; then
      echo -e "\e[1;32m    packing succeeded.\e[0m"
    else
      echo -e "\e[1;31m    [FAIL]\e[0m"
    fi
  fi
done
