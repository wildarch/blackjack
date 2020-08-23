#!/bin/bash

yes | bazel run //:blackjack
bazel build //:all

pushd tests/popular_crates
yes | bazel run //:blackjack
bazel build //:crates
popd
