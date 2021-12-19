#!/bin/bash
set -e

echo "== BLACKJACK =="
echo | bazel run //:blackjack
bazel build //:all

echo "== PLATFORM SPECIFIC =="
pushd tests/platform_specific
echo | bazel run //:blackjack
bazel run //:main
popd

echo "== POPULAR CRATES =="
pushd tests/popular_crates
echo | bazel run //:blackjack
bazel build //:crates
popd

echo "== LINK C =="
pushd tests/linkc
echo | bazel run //:blackjack
bazel run //:main
popd

echo "== WORKSPACE =="
pushd tests/workspace
echo | bazel run //:blackjack
bazel run //crate1
bazel run //crate2
popd

echo "== NO DEPENDENCIES =="
pushd tests/no_dependencies
echo | bazel run //:blackjack
bazel run //:main
popd

