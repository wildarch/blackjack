#!/bin/bash
set -e

echo | bazel run //:blackjack
bazel build //:all

pushd tests/popular_crates
echo | bazel run //:blackjack
bazel build //:crates
popd

pushd tests/linkc
echo | bazel run //:blackjack
bazel run //:main
popd

pushd tests/workspace
echo | bazel run //:blackjack
bazel run //crate1
bazel run //crate2
popd

pushd tests/platform_specific
echo | bazel run //:blackjack
bazel run //:main
popd
