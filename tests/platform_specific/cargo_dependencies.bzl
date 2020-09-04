"""
DO NOT EDIT!

This file is automatically @generated by blackjack.
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def cargo_dependencies():


    http_archive(
        name = "crates_io_const_fn_0.4.2",
        url = "https://crates.io/api/v1/crates/const_fn/0.4.2/download",
        sha256 = "ce90df4c658c62f12d78f7508cf92f9173e5184a539c10bfe54a3107b3ffd0f2",
        strip_prefix = "const_fn-0.4.2",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "const_fn",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "proc-macro",
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
        name = "crates_io_libc_0.2.76",
        url = "https://crates.io/api/v1/crates/libc/0.2.76/download",
        sha256 = "755456fae044e6fa1ebbbd1b3e902ae19e73097ed4ed87bb79934a867c007bc3",
        strip_prefix = "libc-0.2.76",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "libc",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2015",
    crate_features = ["default", "std"],
    rustc_flags = ["--cap-lints=allow"] + ["--cfg=libc_priv_mod_use", "--cfg=libc_union", "--cfg=libc_const_size_of", "--cfg=libc_align", "--cfg=libc_core_cvoid", "--cfg=libc_packedN", "--cfg=libc_cfg_target_vendor"],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_proc_macro_hack_0.5.18",
        url = "https://crates.io/api/v1/crates/proc-macro-hack/0.5.18/download",
        sha256 = "99c605b9a0adc77b7211c6b1f722dcb613d68d66859a44f3d485a6da332b0598",
        strip_prefix = "proc-macro-hack-0.5.18",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "proc_macro_hack",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "proc-macro",
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
        name = "crates_io_proc_macro2_1.0.20",
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.20/download",
        sha256 = "175c513d55719db99da20232b06cda8bab6b83ec2d04e3283edf0213c37c1a29",
        strip_prefix = "proc-macro2-1.0.20",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "proc_macro2",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@crates_io_unicode_xid_0.2.1//:unicode_xid"],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = ["default", "proc-macro"],
    rustc_flags = ["--cap-lints=allow"] + ["--cfg=use_proc_macro"],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_quote_1.0.7",
        url = "https://crates.io/api/v1/crates/quote/1.0.7/download",
        sha256 = "aa563d17ecb180e500da1cfd2b028310ac758de548efdd203e18f283af693f37",
        strip_prefix = "quote-1.0.7",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "quote",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@crates_io_proc_macro2_1.0.20//:proc_macro2"],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = ["default", "proc-macro"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_standback_0.2.10",
        url = "https://crates.io/api/v1/crates/standback/0.2.10/download",
        sha256 = "33a71ea1ea5f8747d1af1979bfb7e65c3a025a70609f04ceb78425bc5adad8e6",
        strip_prefix = "standback-0.2.10",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "standback",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = ["std"],
    rustc_flags = ["--cap-lints=allow"] + ["--cfg=__standback_since_1_31", "--cfg=__standback_since_1_32", "--cfg=__standback_since_1_33", "--cfg=__standback_since_1_34", "--cfg=__standback_since_1_35", "--cfg=__standback_since_1_36", "--cfg=__standback_since_1_37", "--cfg=__standback_since_1_38", "--cfg=__standback_since_1_39", "--cfg=__standback_since_1_40", "--cfg=__standback_since_1_41", "--cfg=__standback_since_1_42", "--cfg=__standback_since_1_43", "--cfg=__standback_since_1_44", "--cfg=__standback_before_1_45", "--cfg=__standback_before_1_46"],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_syn_1.0.39",
        url = "https://crates.io/api/v1/crates/syn/1.0.39/download",
        sha256 = "891d8d6567fe7c7f8835a3a98af4208f3846fba258c1bc3c31d6e506239f11f9",
        strip_prefix = "syn-1.0.39",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "syn",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@crates_io_proc_macro2_1.0.20//:proc_macro2", "@crates_io_quote_1.0.7//:quote", "@crates_io_unicode_xid_0.2.1//:unicode_xid"],
    proc_macro_deps = [],
    edition = "2018",
    crate_features = ["clone-impls", "default", "derive", "full", "parsing", "printing", "proc-macro", "quote", "visit"],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_time",
        url = "https://crates.io/api/v1/crates/time/0.2.17/download",
        sha256 = "ca7ec98a72285d12e0febb26f0847b12d54be24577618719df654c66cadab55d",
        strip_prefix = "time-0.2.17",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "time",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = ["@crates_io_libc_0.2.76//:libc", "@crates_io_standback_0.2.10//:standback", "@crates_io_time_macros_0.1.0//:time_macros"],
    proc_macro_deps = ["@crates_io_const_fn_0.4.2//:const_fn"],
    edition = "2018",
    crate_features = ["default", "deprecated", "libc", "std", "stdweb", "winapi"],
    rustc_flags = ["--cap-lints=allow"] + ["--cfg=__time_02_supports_non_exhaustive", "--cfg=__time_02_instant_checked_ops", "--cfg=__time_02_nonzero_signed", "--cfg=__time_02_use_trait_as_underscore", "--cfg=__time_02_cargo_web"],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_time_macros_0.1.0",
        url = "https://crates.io/api/v1/crates/time-macros/0.1.0/download",
        sha256 = "9ae9b6e9f095bc105e183e3cd493d72579be3181ad4004fceb01adbe9eecab2d",
        strip_prefix = "time-macros-0.1.0",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "time_macros",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "lib",
    deps = [],
    proc_macro_deps = ["@crates_io_proc_macro_hack_0.5.18//:proc_macro_hack", "@crates_io_time_macros_impl_0.1.1//:time_macros_impl"],
    edition = "2018",
    crate_features = [],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_time_macros_impl_0.1.1",
        url = "https://crates.io/api/v1/crates/time-macros-impl/0.1.1/download",
        sha256 = "e5c3be1edfad6027c69f5491cf4cb310d1a71ecd6af742788c6ff8bced86b8fa",
        strip_prefix = "time-macros-impl-0.1.1",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "time_macros_impl",
    aliases = {},
    srcs = glob(["**/*.rs"]),
    crate_type = "proc-macro",
    deps = ["@crates_io_proc_macro2_1.0.20//:proc_macro2", "@crates_io_quote_1.0.7//:quote", "@crates_io_standback_0.2.10//:standback", "@crates_io_syn_1.0.39//:syn"],
    proc_macro_deps = ["@crates_io_proc_macro_hack_0.5.18//:proc_macro_hack"],
    edition = "2018",
    crate_features = [],
    rustc_flags = ["--cap-lints=allow"] + [],
    visibility = ["//visibility:public"],
)
    """,
    )
    

    http_archive(
        name = "crates_io_unicode_xid_0.2.1",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.2.1/download",
        sha256 = "f7fe0bb3479651439c9112f72b6c505038574c9fbb575ed1bf3b797fa39dd564",
        strip_prefix = "unicode-xid-0.2.1",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "unicode_xid",
    aliases = {},
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
    

    http_archive(
        name = "crates_io_version_check_0.9.2",
        url = "https://crates.io/api/v1/crates/version_check/0.9.2/download",
        sha256 = "b5a972e5669d67ba988ce3dc826706fb0a8b01471c088cb0b6110b805cc36aed",
        strip_prefix = "version_check-0.9.2",
        type = "tar.gz",
        build_file_content = """
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "version_check",
    aliases = {},
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
    
