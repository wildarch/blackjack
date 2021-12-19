load("@blackjack//:blackjack.bzl", "blackjack")
load("@rules_rust//rust:defs.bzl", "rust_binary")

blackjack(name = "blackjack")

rust_binary(
    name = "main",
    srcs = ["src/main.rs"],
    rustc_flags = select({
        "@rules_rust//rust/platform:x86_64-apple-darwin": [
            "-lframework=IOKit",
            "-lframework=CoreFoundation",
        ],
        "//conditions:default": [],
    }),
    deps = ["@crates_io_battery//:battery"],
)
