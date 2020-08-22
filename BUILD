load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "blackjack",
    srcs = glob(["src/bin/blackjack.rs"]),
    deps = ["@cargo_metadata", "@serde", "@serde_json"],
    edition = "2018",
    visibility = ["//visibility:public"],
)

genrule(
    name = "blackjack_output",
    srcs = ["Cargo.toml", "Cargo.lock"],
    outs = ["workspace"],
    cmd = "$(location //:blackjack) $(location @blackjack_cargo//:cargo) $$(readlink -e $(location Cargo.toml)) > $@",
    #cmd = "$(location @blackjack_cargo//:cargo) metadata --manifest-path=$$(readlink -e $(location Cargo.toml)) --frozen --offline > $@",
    tools = ["//:blackjack", "@blackjack_cargo//:cargo"],

)
