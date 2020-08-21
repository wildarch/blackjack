use cargo_metadata::{DependencyKind, Metadata, MetadataCommand, Package, PackageId};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
struct BlackjackMetadataWrapper {
    blackjack: BlackjackMetadata,
}

#[derive(Debug, Deserialize, Default)]
struct BlackjackMetadata {
    rustc_flags: HashMap<String, Vec<String>>,
}

fn main() {
    let metadata = MetadataCommand::new()
        .other_options(vec!["--frozen".to_string(), "--offline".to_string()])
        .exec()
        .unwrap();

    let root_id = metadata.resolve.as_ref().unwrap().root.as_ref().unwrap();
    eprintln!("root id: {}", root_id);
    let root_package = metadata.packages.iter().find(|p| &p.id == root_id).unwrap();
    let blackjack_metadata =
        serde_json::from_value::<BlackjackMetadataWrapper>(root_package.metadata.clone())
            .unwrap_or_default()
            .blackjack;

    println!(
        r#"
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

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")
        "#
    );

    for package in &metadata.packages {
        let archive = render_archive(&package, &metadata, &blackjack_metadata);
        println!("{}", archive);
    }
}

#[derive(PartialEq)]
enum CrateType {
    Lib,
    ProcMacro,
}

fn crate_type(package_id: &PackageId, metadata: &Metadata) -> CrateType {
    let package = metadata
        .packages
        .iter()
        .find(|p| &p.id == package_id)
        .unwrap();
    match package.targets[0].crate_types[0].as_ref() {
        "proc-macro" => CrateType::ProcMacro,
        _ => CrateType::Lib,
    }
}

fn render_build_file(
    package: &Package,
    metadata: &Metadata,
    blackjack_metadata: &BlackjackMetadata,
) -> String {
    let target = package
        .targets
        .iter()
        .filter(|t| t.kind == vec!["lib".to_string()] || t.kind == vec!["proc-macro".to_string()])
        .next()
        .expect("Can't find library");
    let dependencies = &metadata
        .resolve
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find(|n| package.id == n.id)
        .unwrap()
        .deps;
    let deps: Vec<String> = dependencies
        .iter()
        .filter(|d| {
            d.dep_kinds.iter().any(|k| k.kind == DependencyKind::Normal)
                && crate_type(&d.pkg, metadata) == CrateType::Lib
        })
        .map(|d| format!("@{name}//:{name}", name = sanitize_name(&d.name)))
        .collect();
    let proc_macro_deps: Vec<String> = dependencies
        .iter()
        .filter(|d| {
            d.dep_kinds.iter().any(|k| k.kind == DependencyKind::Normal)
                && crate_type(&d.pkg, metadata) == CrateType::ProcMacro
        })
        .map(|d| format!("@{name}//:{name}", name = sanitize_name(&d.name)))
        .collect();
    let features = &metadata
        .resolve
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find(|n| n.id == package.id)
        .unwrap()
        .features;
    let rustc_flags = blackjack_metadata
        .rustc_flags
        .get(&package.name)
        .cloned()
        .unwrap_or_default();
    format!(
        r#"
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "{name}",
    srcs = glob(["**/*.rs"]),
    crate_type = "{crate_type}",
    deps = {deps:?},
    proc_macro_deps = {proc_macro_deps:?},
    edition = "{edition}",
    crate_features = {crate_features:?},
    rustc_flags = {rustc_flags:?},
    visibility = ["//visibility:public"],
)
    "#,
        name = sanitize_name(&package.name),
        crate_type = target.crate_types[0],
        deps = deps,
        proc_macro_deps = proc_macro_deps,
        edition = target.edition,
        crate_features = features,
        rustc_flags = rustc_flags,
    )
}

fn render_archive(
    package: &Package,
    metadata: &Metadata,
    blackjack_metadata: &BlackjackMetadata,
) -> String {
    format!(
        r#"
http_archive(
    name = "{sanitized_name}",
    url = "https://crates.io/api/v1/crates/{name}/{version}/download",
    strip_prefix = "{name}-{version}",
    type = "tar.gz",
    build_file_content = """{build_file_content}""",
)
    "#,
        sanitized_name = sanitize_name(&package.name),
        name = package.name,
        version = package.version,
        build_file_content = render_build_file(package, metadata, blackjack_metadata),
    )
}

fn sanitize_name(s: &str) -> String {
    s.replace("-", "_")
}
