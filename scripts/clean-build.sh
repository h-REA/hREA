#!/usr/bin/env bash
#
# Clean script for all build files
#
# @package: HoloREA
# @since:   2019-06-13
#
##

cargo clean

rm -Rf bundles/dna/
rm -Rf bundles/app/
rm bundles/web-app/*.webhapp
exit 0
