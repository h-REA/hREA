#!/usr/bin/env bash
#
# Clean script for all build files
#
# @package: HoloREA
# @since:   2019-06-13
#
##

rm -Rf happs/**/dist
rm -Rf happs/**/zomes/**/code/target

# :IMPORTANT: after updating Holochain this can be needed to avoid unmet dependency errors
cargo update
