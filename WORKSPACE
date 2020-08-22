workspace(name = "blackjack")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "io_bazel_rules_rust",
    strip_prefix = "rules_rust-fdf9655ba95616e0314b4e0ebab40bb0c5fe005c",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/fdf9655ba95616e0314b4e0ebab40bb0c5fe005c.zip",
    ],
)
http_archive(
    name = "bazel_skylib",
    sha256 = "12ee3a5732e8c353fce4a710dbe045a16a161c49c79622faa1f2813f668bb442",
    strip_prefix = "bazel-skylib-8f3151fb4a91d5f2ae4cad5901ea72fe30a2aba0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/8f3151fb4a91d5f2ae4cad5901ea72fe30a2aba0.tar.gz",  # 2020-07-10
)
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

load("@blackjack//:workspace.bzl", "blackjack_cargo")
blackjack_cargo()

load("@//:Cargo.bzl", "cargo_dependencies")
cargo_dependencies()
