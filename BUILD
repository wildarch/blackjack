exports_files(["src/bin/blackjack.rs"], visibility = ["//visibility:public"],)

load("@rules_rust//rust:rust.bzl", "rust_library")
load("@blackjack//:blackjack.bzl", "blackjack")

rust_library(
    name = "blackjack_lib",
    srcs = ["src/lib.rs"],
    deps = [
        "@blackjack_crates_io_cargo_metadata//:cargo_metadata", 
        "@blackjack_crates_io_serde//:serde", 
        "@blackjack_crates_io_serde_json//:serde_json",
        "@blackjack_crates_io_cargo_lock//:cargo_lock",
        "@blackjack_crates_io_cfg_expr//:cfg_expr",
    ],
    edition = "2018",
    visibility = ["//visibility:public"],
)

blackjack(name = "blackjack")
