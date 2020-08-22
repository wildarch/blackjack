
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def cargo_dependencies():


    http_archive(
        name = "blackjack",
        url = "https://crates.io/api/v1/crates/blackjack/0.1.0/download",
        strip_prefix = "blackjack-0.1.0",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "blackjack",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@cargo_metadata//:cargo_metadata", "@serde//:serde", "@serde_json//:serde_json"],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = [],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "cargo_metadata",
        url = "https://crates.io/api/v1/crates/cargo_metadata/0.10.0/download",
        strip_prefix = "cargo_metadata-0.10.0",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "cargo_metadata",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@semver//:semver", "@serde//:serde", "@serde_json//:serde_json"],
    proc_macro_deps = ["@serde_derive//:serde_derive"],
    edition = "2015",
    crate_features = ["default"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "itoa",
        url = "https://crates.io/api/v1/crates/itoa/0.4.6/download",
        strip_prefix = "itoa-0.4.6",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "itoa",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2015",
    crate_features = [],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "proc_macro2",
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.18/download",
        strip_prefix = "proc-macro2-1.0.18",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "proc_macro2",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@unicode_xid//:unicode_xid"],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = ["default", "proc-macro"],
    rustc_flags = ["--cap-lints=allow"] + ["--cfg=use_proc_macro"],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "quote",
        url = "https://crates.io/api/v1/crates/quote/1.0.7/download",
        strip_prefix = "quote-1.0.7",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "quote",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@proc_macro2//:proc_macro2"],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = ["default", "proc-macro"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "ryu",
        url = "https://crates.io/api/v1/crates/ryu/1.0.5/download",
        strip_prefix = "ryu-1.0.5",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "ryu",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = [],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "semver",
        url = "https://crates.io/api/v1/crates/semver/0.9.0/download",
        strip_prefix = "semver-0.9.0",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "semver",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@semver_parser//:semver_parser", "@serde//:serde"],
    proc_macro_deps = [],
    edition = "2015",
    crate_features = ["default", "serde"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "semver_parser",
        url = "https://crates.io/api/v1/crates/semver-parser/0.7.0/download",
        strip_prefix = "semver-parser-0.7.0",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "semver_parser",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2015",
    crate_features = [],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "serde",
        url = "https://crates.io/api/v1/crates/serde/1.0.113/download",
        strip_prefix = "serde-1.0.113",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "serde",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = ["@serde_derive//:serde_derive"],
    edition = "2015",
    crate_features = ["default", "derive", "serde_derive", "std"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "serde_derive",
        url = "https://crates.io/api/v1/crates/serde_derive/1.0.113/download",
        strip_prefix = "serde_derive-1.0.113",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "serde_derive",
    srcs = glob(["**/*.rs"]),
    crate_type = "proc-macro",
    deps = ["@proc_macro2//:proc_macro2", "@quote//:quote", "@syn//:syn"],
    proc_macro_deps = [],
    edition = "2015",
    crate_features = ["default"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "serde_json",
        url = "https://crates.io/api/v1/crates/serde_json/1.0.55/download",
        strip_prefix = "serde_json-1.0.55",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "serde_json",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@itoa//:itoa", "@ryu//:ryu", "@serde//:serde"],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = ["default", "std"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "syn",
        url = "https://crates.io/api/v1/crates/syn/1.0.32/download",
        strip_prefix = "syn-1.0.32",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "syn",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@proc_macro2//:proc_macro2", "@quote//:quote", "@unicode_xid//:unicode_xid"],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = ["clone-impls", "default", "derive", "parsing", "printing", "proc-macro", "quote", "visit"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "unicode_xid",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.2.0/download",
        strip_prefix = "unicode-xid-0.2.0",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "unicode_xid",
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2015",
    crate_features = ["default"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    
