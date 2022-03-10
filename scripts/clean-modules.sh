#!/usr/bin/env bash
#
# Clean script for node module folders in the repo
#
# @package: Holo-REA
# @since:   2020-07-21
#
##

rm -Rf node_modules
rm -Rf modules/**/build

for DIR in $(find -type d -iname node_modules); do
  echo "  Remove $DIR"
  rm -Rf "$DIR"
done
