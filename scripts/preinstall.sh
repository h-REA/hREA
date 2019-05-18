#!/usr/bin/env bash
#
# Repository pre-install setup script; runs to prepare
# system prior to Yarn setup commands running. This means bringing in
# remote referenced git submodules.
#
##

git submodule init
git submodule update
