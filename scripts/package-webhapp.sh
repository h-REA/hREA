#!/usr/bin/env bash
#
# Runs the Holochain webhapp bundler utility against the webhapp folder
#
# @package: Holo-REA
# @since:   2022-02-20
#
##

UTIL="${HOLOCHAIN_DNA_UTIL_PATH:-hc}"

echo -e "\e[1mPacking webhapp\e[0m"
if "$UTIL" web-app pack webhapp 2>/dev/null; then
  echo -e "\e[1;32m    packing succeeded.\e[0m"
else
  echo -e "\e[1;31m    [FAIL]\e[0m"
fi
