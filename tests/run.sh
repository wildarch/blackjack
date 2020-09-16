#!/bin/bash
set -e

# CI runs on ubuntu-latest
target=x86_64-unknown-linux-gnu

echo | bazel run //:blackjack -- --target="$target"
bazel build //:all

pushd tests/popular_crates
echo | bazel run //:blackjack -- --target="$target"
bazel build //:crates
popd

pushd tests/linkc
echo | bazel run //:blackjack -- --target="$target"
bazel run //:main
popd

pushd tests/workspace
echo | bazel run //:blackjack -- --target="$target"
bazel run //crate1
bazel run //crate2
popd
