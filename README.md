# Blackjack
Bazel Build file generator for crates.io dependencies.

Blackjack reads standard `Cargo.toml`, so the same project can build with both `cargo` and Bazel.
Adding Bazel support to an existing project is as simple as adding a `WORKSPACE` and `BUILD` file.

# TODO
* Support custom build files for crates
* Support replacing a crate with a different target

# Would appreciate PR for:
* Support for Windows and Mac
* Workspace support
