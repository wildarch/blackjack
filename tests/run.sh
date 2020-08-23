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
