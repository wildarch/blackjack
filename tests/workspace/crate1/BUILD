load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "crate1",
    srcs = ["src/main.rs"],
    edition = "2018",
    deps = ["@crates_io_rand//:rand"],
)
