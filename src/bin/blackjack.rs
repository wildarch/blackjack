use cargo_metadata::{DependencyKind, Metadata, MetadataCommand, Package, PackageId};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

const CARGO_TOML_RUNFILES_PATH: &'static str = "Cargo.toml";
const CARGO_RUNFILES_PATH: &'static str = "external/blackjack_cargo/cargo";

#[derive(Debug, Deserialize, Default)]
struct BlackjackMetadataWrapper {
    blackjack: BlackjackMetadata,
}

#[derive(Debug, Deserialize, Default)]
struct BlackjackMetadata {
    rustc_flags: HashMap<String, Vec<String>>,
}

fn main() {
    // This is somewhat of an implementation detail
    let cargo_toml_path =
        std::fs::read_link(CARGO_TOML_RUNFILES_PATH).expect("Failed to read Cargo.toml symlink");
    let mut output_path = cargo_toml_path.clone();
    output_path.pop();
    output_path.push("cargo_dependencies.bzl");

    let mut metadata = MetadataCommand::new();
    metadata
        .manifest_path(cargo_toml_path)
        .other_options(vec!["--frozen".to_string(), "--offline".to_string()]);

    let cargo_runfiles_path = Path::new(CARGO_RUNFILES_PATH);
    if cargo_runfiles_path.exists() {
        eprintln!("Found cargo in runfiles: {}", cargo_runfiles_path.display());
        metadata.cargo_path(cargo_runfiles_path);
    } else {
        eprintln!(
            "Using default cargo in path. Working dir: {}",
            std::env::current_dir().unwrap().display()
        );
    }

    let metadata = metadata.exec().unwrap();

    eprintln!("Writing output to {}", output_path.display());
    eprintln!("Press enter to continue, or Ctrl-C to abort");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read stdin");
    let output = std::fs::File::create(output_path).expect("Could not open output file");
    let mut output = std::io::BufWriter::new(output);

    let root_id = metadata.resolve.as_ref().unwrap().root.as_ref().unwrap();
    let root_package = metadata.packages.iter().find(|p| &p.id == root_id).unwrap();
    let blackjack_metadata =
        serde_json::from_value::<BlackjackMetadataWrapper>(root_package.metadata.clone())
            .unwrap_or_default()
            .blackjack;

    writeln!(
        output,
        r#"
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def cargo_dependencies():
"#
    )
    .expect("Failed to write to output file");

    // Sort the list of packages by package id to make the output deterministic
    let mut packages = metadata.packages.clone();
    packages.sort_by_key(|p| p.id.clone());

    for package in packages {
        let archive = render_archive(&package, &metadata, &blackjack_metadata);
        writeln!(output, "{}", archive).expect("Failed to write to output file");
    }

    eprintln!("Done.");
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
    rustc_flags = ["--cap-lints=allow"] + {rustc_flags:?},
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
