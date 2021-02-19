load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@blackjack//:cargo_dependencies.bzl", "cargo_dependencies")

def _cargo_target(os_name):
  if os_name.startswith("mac os"):
    return "x86_64-apple-darwin"
  if os_name.find("windows") != -1:
    return "x86_64-pc-windows-msvc"
  else:
    return "x86_64-unknown-linux-gnu"

def _cargo_binary_name(os_name):
  if os_name.find("windows") != -1:
    return "cargo.exe"
  else:
    return "cargo"

def _blackjack_cargo_impl(ctx):
  os_name = ctx.os.name.lower()
  target = _cargo_target(os_name)
  ctx.download_and_extract(
    url = "https://static.rust-lang.org/dist/2021-02-11/cargo-1.50.0-%s.tar.gz" % target,
    stripPrefix = "cargo-1.50.0-%s/cargo/bin" % target,
  )
  ctx.file("BUILD.bazel", """alias(name="blackjack_cargo.exe", actual = "%s", visibility  = ["//visibility:public"])""" % _cargo_binary_name(os_name))

_blackjack_cargo = repository_rule(
  implementation = _blackjack_cargo_impl,
)

def blackjack_cargo():
  _blackjack_cargo(name = "blackjack_cargo")
  cargo_dependencies()
