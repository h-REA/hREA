#!/usr/bin/env bash
#
# Workaround to run tests, which must be separate from the main workspace
# due to issues with dependencies in Holochain crates.

for TEST in lib/**/tests/Cargo.toml; do
  echo -e "\e[1mRunning unit tests in $TEST\e[0m"

  # duplicate the root workspace lockfile first, or package versions may mismatch
  cp Cargo.lock $(dirname $TEST)

  # --nocapture makes sure the logging output is visible
  CARGO_TARGET_DIR=target cargo test --manifest-path $TEST --lib --features="mock" -- --nocapture
  [ $? -eq 0 ] || exit 1
done
