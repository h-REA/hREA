#!/usr/bin/env bash
#
# Runs a Holochain conductor for development, using the `hc` utility.
#
# @package: hREA
# @author:  pospi <pospi@spadgos.com>
# @since:   2021-08-22
#
##

set -e

UTIL="${HOLOCHAIN_DNA_UTIL_PATH:-hc}"
APP="${HOLOCHAIN_APP_PORT:-4000}"
ADM="${HOLOCHAIN_ADMIN_PORT:-4001}"

"$UTIL" s clean
echo \"pass\" | "$UTIL" s --piped create -n 1 -d hrea_tester network quic
echo \"pass\" | "$UTIL" s --piped call install-app ./bundles/app/full_suite/hrea_suite.happ
echo \"pass\" | "$UTIL" s --piped -f=$ADM run --all -p $APP
