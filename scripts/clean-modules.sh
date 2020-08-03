#!/usr/bin/env bash
#
# Clean script for node module folders in the repo
#
# @package: Holo-REA
# @since:   2020-07-21
#
##

rm -Rf node_modules

for DIR in **/node_modules; do
  rm -Rf "$DIR"
done
