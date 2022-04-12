#!/usr/bin/env bash
#
# Run nodejs integration tests
#
# @package: hREA
# @since:   2022-04-12
#
##

cd test

WASM_LOG=debug
RUST_LOG=error
RUST_BACKTRACE=1
GRAPHQL_DEBUG=1

# 3 tests
# 0 passed
# 3 failed
# echo 'running agent tests'
# npx tape agent/*.js | npx tap-dot

# 4 tests
# 47 passed
# 1 failed
# ..........................................x.....
# echo 'running economic-resource tests'
# npx tape economic-resource/*.js | npx tap-dot

# 3 tests
# 0 passed
# 3 failed
# echo 'running proposal tests'
# npx tape proposal/*.js | npx tap-dot

# 2 tests
# 0 passed
# 2 failed
# echo 'running agreement tests'
# npx tape agreement/*.js | npx tap-dot

# 1 tests
# 0 passed
# 1 failed
# echo 'running flows tests'
# npx tape flows/*.js | npx tap-dot

# 1 tests
# 0 passed
# 1 failed
# echo 'running satisfaction tests'
# npx tape satisfaction/*.js | npx tap-dot

# 11 tests
# 30 passed
# 6 failed
# .....................x.....xx....xxx
# echo 'running core-architecture tests'
# npx tape core-architecture/*.js | npx tap-dot

# 1 tests
# 2 passed
# 1 failed
# ..x
# echo 'running fulfillment tests'
# npx tape fulfillment/*.js | npx tap-dot

# 1 tests
# 0 passed
# 1 failed
# echo 'running social-architectures tests'
# npx tape social-architectures/*.js | npx tap-dot

# 2 tests
# 13 passed
# echo 'running economic-event tests'
# npx tape economic-event/*.js | npx tap-dot

# 2 tests
# 26 passed
# 24 failed
# .........xxxxxxxx.............xxxxxxxx....xxxxxxxx
# echo 'running process tests'
# npx tape process/*.js | npx tap-dot


# 5 tests
# 9 passed
# 4 failed
# ..x..x..x...x
# echo 'running specification tests'
# npx tape specification/*.js | npx tap-dot