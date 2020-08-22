load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def blackjack_cargo():
  http_archive(
      name = "blackjack_cargo",
      # Taken from https://static.rust-lang.org/dist/channel-rust-stable.toml.
      url = "https://static.rust-lang.org/dist/2020-08-03/cargo-0.46.1-x86_64-unknown-linux-gnu.tar.gz",
      sha256 = "ac2746e3d3bab7301b8aa747eff7c4d66f9c88a61f9117a4d6669c40317b69cc",
      strip_prefix = "cargo-0.46.1-x86_64-unknown-linux-gnu/cargo/bin",
      build_file_content = """exports_files(["cargo"], visibility = ["//visibility:public"])""",
  )
