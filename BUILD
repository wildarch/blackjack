load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "blackjack",
    srcs = glob(["src/bin/blackjack.rs"]),
    deps = ["@cargo_metadata", "@serde", "@serde_json"],
    edition = "2018",
    visibility = ["//visibility:public"],
)
