#!/bin/bash
set -e

echo | bazel run //:blackjack
bazel build //:all

pushd tests/popular_crates
echo | bazel run //:blackjack
bazel build //:crates
popd
