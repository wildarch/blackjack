workspace(name = "blackjack")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "cf2e82c56c73c3213bc2d94303aee88d7e44634ad1e1fe183befa85b17b5021d",
    strip_prefix = "rules_rust-0e3593fc5d839e4a74523f07e885b761ee19e662",
    urls = [
        # Master branch as of 2021-12-19
        "https://github.com/bazelbuild/rules_rust/archive/0e3593fc5d839e4a74523f07e885b761ee19e662.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

load("@blackjack//:workspace.bzl", "blackjack_cargo")

blackjack_cargo()

load("@//:cargo_dependencies.bzl", "cargo_dependencies")

cargo_dependencies()
