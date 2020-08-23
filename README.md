# Blackjack
Bazel Build file generator for Rust.

Blackjack reads standard `Cargo.toml`, so the same project can build with both `cargo` and Bazel.
Adding Bazel support to an existing project is as simple as adding a `WORKSPACE` and `BUILD` file.

## Limitations
* Only one version of a particular dependency is supported
