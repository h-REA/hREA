#!/usr/bin/env bash
#
# Runs a Holochain conductor for development, using the `hc` utility.
#
# @package: hREA
# @author:  pospi <pospi@spadgos.com>
# @since:   2021-08-22
#
##

UTIL="${HOLOCHAIN_DNA_UTIL_PATH:-hc}"
APP="${HOLOCHAIN_APP_PORT:-4000}"
# ADM="${HOLOCHAIN_ADMIN_PORT:-4001}"

"$UTIL" s clean
"$UTIL" s create -n 1 -d tester network quic
"$UTIL" s call install-app-bundle ./bundles/full_suite/hrea_suite.happ
"$UTIL" s run --all -p $APP
