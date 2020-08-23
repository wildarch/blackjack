load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

def blackjack(name=None, manifest="Cargo.toml"):
  if not name:
    fail(msg="'name' must be set when calling blackjack(..)")

  rust_binary(
      name = name,
      srcs = ["@blackjack//:src/bin/blackjack.rs"],
      aliases = {"@blackjack//:blackjack_lib": "blackjack"},
      deps = [
          "@blackjack_crates_io_cargo_metadata//:cargo_metadata", 
          "@blackjack_crates_io_cargo_lock//:cargo_lock", 
          "@blackjack//:blackjack_lib",
      ],
      edition = "2018",
      visibility = ["//visibility:public"],
      data = [
        "@blackjack_cargo//:cargo",
        manifest,
      ],
  )

