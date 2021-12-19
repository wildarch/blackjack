# Blackjack - build cargo dependencies with Bazel
Generate bazel targets for crates.io dependencies in your `Cargo.toml`.

Blackjack reads standard `Cargo.toml`, so the same project can build with both `cargo` and Bazel.
Adding Bazel support to an existing cargo project is as simple as adding a `WORKSPACE` and `BUILD` file.

`WORKSPACE`:

```python
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Configure a Rust toolchain.
# See https://github.com/bazelbuild/rules_rust for more details.
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

# Set up blackjack
http_archive(
    name = "blackjack",
    url = "https://github.com/wildarch/blackjack/archive/f9d49ea9f93aabcea0fa9d8e90cb854c7a1de9ce.zip",
    sha256 = "496410e369ee6a986f223071d47c5f69302abaf41840a5f3451000ee5f4739bd",
    strip_prefix = "blackjack-f9d49ea9f93aabcea0fa9d8e90cb854c7a1de9ce",
)
load("@blackjack//:workspace.bzl", "blackjack_cargo")
blackjack_cargo()
```

`BUILD` file in the same directory as `Cargo.toml`:

```python
load("@blackjack//:blackjack.bzl", "blackjack")

blackjack(name = "blackjack")
```

Now run `bazel run //:blackjack`. You'll find a newly created `cargo_dependencies.bzl` file next to your `Cargo.toml`.

**Note:** Blackjack pulls in its own `cargo` executable, so you do not need to have it installed.

Import the generated dependencies into your `WORKSPACE`:

```python
load("//:cargo_dependencies.bzl", "cargo_dependencies")
cargo_dependencies()
```

Dependencies from `Cargo.toml` are now available as `rust_library` rules under the label `@crates_io_{name}//:{name}`.

For an example, see [here](https://github.com/wildarch/blackjack/tree/master/tests/popular_crates).

# Updates
If you make any changes to the dependency graph, simply re-run `bazel run //:blackjack` to build an updated `cargo_dependencies.bzl` file. 

# Configuration
Many crates will build out of the box. For more exotic crates (linking to C libraries, build scripts etc.), there are a few configuration options. This configuration is embedded in the `Cargo.toml` in the metadata section.

To pass extra flags to rustc when compiling a particular crate, add the following to the end of `Cargo.toml`:

```toml
[package.metadata.blackjack.proc-macro2]
rustc_flags = ["--cfg=use_proc_macro"]
```

**Note**: Blackjack provides default settings for a few common crates such as `proc-macro2`, so you don't always need to add them yourself. 
Please send PRs for default settings for other common crates!

If a dependency needs a build script, but does not otherwise do anything complicated, try adding something like:

```toml
[package.metadata.blackjack.proc-macro-nested]
build_script = true
```

If all else fails, you can replace the dependency with a custom `rust_library` target:

```toml
[package.metadata.blackjack.libz-sys]
replace = "@custom_libz_sys"
```

For a simple example of replacing a dependency and linking to a C library, see [here](https://github.com/wildarch/blackjack/tree/master/tests/linkc).

If you want to change the default `crates_io_` prefix for generated dependency targets, that is possible too:

```toml
[package.metadata.blackjack]
prefix = "blackjack_crates_io"
```

Now instead of `@crates_io_serde//:serde`, use `blackjack_crates_io_serde//:serde`.

# Things that don't work yet (but would gladly accept a PR for)
* Support for Windows. All that is really needed here is to add it to the list of supported platforms, and fix the platform specific workspace test.
