load("@blackjack//:blackjack.bzl", "blackjack")

blackjack(name = "blackjack")

load("@rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "main",
    srcs = ["src/main.rs"],
    edition = "2018",
    deps = ["@crates_io_flate2//:flate2"],
)
