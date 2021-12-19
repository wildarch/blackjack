load("@rules_rust//rust:defs.bzl", "rust_library")
load("@blackjack//:blackjack.bzl", "blackjack")

exports_files(
    ["src/bin/blackjack.rs"],
    visibility = ["//visibility:public"],
)

rust_library(
    name = "blackjack_lib",
    srcs = ["src/lib.rs"],
    edition = "2021",
    visibility = ["//visibility:public"],
    deps = [
        "@blackjack_crates_io_cargo_lock//:cargo_lock",
        "@blackjack_crates_io_cargo_metadata//:cargo_metadata",
        "@blackjack_crates_io_cfg_expr//:cfg_expr",
        "@blackjack_crates_io_serde//:serde",
        "@blackjack_crates_io_serde_json//:serde_json",
    ],
)

blackjack(name = "blackjack")
