load("@blackjack//:blackjack.bzl", "blackjack")
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

blackjack(name = "blackjack")

rust_binary(
    name = "main",
    srcs = ["src/main.rs"],
    deps = ["@crates_io_battery//:battery"],
)
