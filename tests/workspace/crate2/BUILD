load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "crate2",
    srcs = ["src/main.rs"],
    edition = "2018",
    deps = ["@crates_io_bitflags//:bitflags"],
)
