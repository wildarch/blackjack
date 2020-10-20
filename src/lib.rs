use cargo_lock::Lockfile;
use cargo_metadata::{DependencyKind, Metadata, Node, Package, PackageId, Target};
use cfg_expr::targets::get_builtin_target_by_triple;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::io::Write;

#[derive(Debug, Deserialize, Default)]
struct BlackjackMetadataWrapper {
    #[serde(default)]
    blackjack: BlackjackMetadata,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct CrateOpts {
    #[serde(default)]
    build_script: bool,
    #[serde(default)]
    rustc_flags: Vec<String>,
    #[serde(default)]
    replace: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct BlackjackMetadata {
    prefix: Option<String>,

    #[serde(flatten)]
    crate_opts: HashMap<String, CrateOpts>,
}

const DEFAULT_PREFIX: &str = "crates_io";
const SUPPORTED_TARGETS: &[&str] = &[
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
];

fn default_crate_opts() -> Vec<(String, CrateOpts)> {
    vec![
        (
            "indexmap".to_string(),
            CrateOpts {
                rustc_flags: vec!["--cfg=has_std".to_string()],
                ..Default::default()
            },
        ),
        (
            "proc-macro2".to_string(),
            CrateOpts {
                rustc_flags: vec!["--cfg=use_proc_macro".to_string()],
                ..Default::default()
            },
        ),
        (
            "proc-macro-nested".to_string(),
            CrateOpts {
                build_script: true,
                ..Default::default()
            },
        ),
        (
            "libc".to_string(),
            CrateOpts {
                rustc_flags: vec![
                    "--cfg=libc_priv_mod_use".to_string(),
                    "--cfg=libc_union".to_string(),
                    "--cfg=libc_const_size_of".to_string(),
                    "--cfg=libc_align".to_string(),
                    "--cfg=libc_core_cvoid".to_string(),
                    "--cfg=libc_packedN".to_string(),
                    "--cfg=libc_cfg_target_vendor".to_string(),
                ],
                ..Default::default()
            },
        ),
        (
            "typenum".to_string(),
            CrateOpts {
                build_script: true,
                ..Default::default()
            },
        ),
    ]
}

impl BlackjackMetadata {
    pub fn new(package: &Package) -> BlackjackMetadata {
        let mut blackjack_metadata = if package.metadata.is_null() {
            BlackjackMetadata::default()
        } else {
            serde_json::from_value::<BlackjackMetadataWrapper>(package.metadata.clone())
                .expect("Failed to parse metadata")
                .blackjack
        };
        if let Some(p) = blackjack_metadata.prefix.as_ref() {
            if p == "" {
                blackjack_metadata.prefix = None;
            }
        }
        blackjack_metadata
    }

    pub fn merge(mut self, other: BlackjackMetadata) -> Self {
        self.prefix = match (self.prefix, other.prefix) {
            (None, None) => None,
            (Some(p), None) => Some(p),
            (None, Some(p)) => Some(p),
            (Some(a), Some(b)) => panic!(
                "Cannot merge metadata, two different prefixes given ({} vs {})",
                a, b
            ),
        };
        self.crate_opts.extend(other.crate_opts);
        self
    }
}

pub struct Blackjack {
    metadata: Metadata,
    blackjack_metadata: BlackjackMetadata,
    packages: HashMap<PackageId, Package>,
    direct_dependencies: HashSet<PackageId>,
    lockfile: Lockfile,
}

impl Blackjack {
    pub fn new(mut metadata: Metadata, lockfile: Lockfile) -> Blackjack {
        // Sort the nodes to make traversal deterministic.
        metadata
            .resolve
            .as_mut()
            .unwrap()
            .nodes
            .sort_by(|a, b| a.id.cmp(&b.id));
        let packages: HashMap<PackageId, Package> = metadata
            .packages
            .iter()
            .map(|p| (p.id.clone(), p.clone()))
            .collect();
        // Combine all metadata from all packages in the workspace.
        let mut blackjack_metadata = metadata
            .workspace_members
            .iter()
            .map(|p| packages.get(p).unwrap())
            .map(BlackjackMetadata::new)
            .fold(BlackjackMetadata::default(), |a, b| a.merge(b));
        // Add in relevant default crate options
        for (crate_name, opts) in default_crate_opts() {
            // Only add the crate options if the crate appears in the dependency graph
            if metadata.packages.iter().any(|p| p.name == crate_name) {
                blackjack_metadata
                    .crate_opts
                    .entry(crate_name)
                    .or_insert(opts);
            }
        }
        eprintln!("{:#?}", blackjack_metadata);
        let direct_dependencies = direct_dependencies(&metadata);
        Blackjack {
            metadata,
            packages,
            blackjack_metadata,
            lockfile,
            direct_dependencies,
        }
    }

    pub fn render<W: Write>(&self, mut output: W) -> Result<(), std::io::Error> {
        writeln!(
            output,
            r#""""
DO NOT EDIT!

This file is automatically @generated by blackjack.
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def cargo_dependencies():
"#
        )?;

        for node in self.nodes() {
            let package = self.packages.get(&node.id).unwrap();
            if !package
                .source
                .as_ref()
                .map(|s| s.is_crates_io())
                .unwrap_or(false)
            {
                // Skip packages not sourced from crates.io
                continue;
            }
            writeln!(output, "{}", self.render_archive(node, package))?;
        }
        Ok(())
    }

    fn nodes(&self) -> impl Iterator<Item = &Node> + '_ {
        self.metadata.resolve.as_ref().unwrap().nodes.iter()
    }

    fn crate_type(&self, package: &Package) -> CrateType {
        match package.targets[0].crate_types[0].as_ref() {
            "proc-macro" => CrateType::ProcMacro,
            _ => CrateType::Lib,
        }
    }

    fn render_archive(&self, node: &Node, package: &Package) -> String {
        let archive_name = if self.direct_dependencies.contains(&node.id) {
            self.prefixed_name(package)
        } else {
            format!(
                "{prefixed_name}_{version}",
                prefixed_name = self.prefixed_name(package),
                version = sanitize_version(&package.version.to_string()),
            )
        };
        format!(
            r#"
    http_archive(
        name = "{archive_name}",
        url = "https://crates.io/api/v1/crates/{name}/{version}/download",
        sha256 = "{checksum}",
        strip_prefix = "{name}-{version}",
        type = "tar.gz",
        build_file_content = """{build_file_content}""",
    )
    "#,
            archive_name = archive_name,
            name = package.name,
            version = package.version,
            checksum = self.checksum(package),
            build_file_content = self.render_build_file(node, package),
        )
    }

    fn prefixed_name(&self, package: &Package) -> String {
        format!(
            "{prefix}_{name}",
            prefix = self
                .blackjack_metadata
                .prefix
                .as_deref()
                .unwrap_or(DEFAULT_PREFIX),
            name = sanitize_name(&package.name)
        )
    }

    fn dep_label(&self, package: &Package) -> String {
        // Check if a replacement label has been configured
        if let Some(CrateOpts {
            replace: Some(label),
            ..
        }) = self.blackjack_metadata.crate_opts.get(&package.name)
        {
            return label.clone();
        }

        if self.direct_dependencies.contains(&package.id) {
            format!(
                "@{prefixed_name}//:{name}",
                prefixed_name = self.prefixed_name(package),
                name = sanitize_name(&package.name)
            )
        } else {
            format!(
                "@{prefixed_name}_{version}//:{name}",
                prefixed_name = self.prefixed_name(package),
                version = sanitize_version(&package.version.to_string()),
                name = sanitize_name(&package.name),
            )
        }
    }

    // Add a package to the given DependencySet, with an optional target predicate.
    fn add_dep<T: fmt::Display>(
        &self,
        dep_set: &mut DependencySet,
        target: &Option<T>,
        package: &Package,
    ) {
        let dep_label = self.dep_label(package);
        if let Some(target_expr) = target {
            if let Some(_) = get_builtin_target_by_triple(&target_expr.to_string()) {
                // The target expr is a target triple
                if SUPPORTED_TARGETS.contains(&target_expr.to_string().as_str()) {
                    dep_set
                        .platform_specific_deps
                        .entry(triple_to_condition(target_expr))
                        .or_insert_with(Vec::new)
                        .push(dep_label);
                }
            } else {
                // The target triple is a more complex cfg(..) expression
                let target_expr = cfg_expr::Expression::parse(&target_expr.to_string())
                    .expect("Failed to parse target");
                // Check to which targets the target expression applies
                for target in SUPPORTED_TARGETS {
                    let target = get_builtin_target_by_triple(target).unwrap();
                    let uses_dep = target_expr.eval(|pred| match pred {
                        cfg_expr::Predicate::Target(tp) => tp.matches(target),
                        _ => false,
                    });
                    if uses_dep {
                        dep_set
                            .platform_specific_deps
                            .entry(triple_to_condition(target.triple))
                            .or_insert_with(Vec::new)
                            .push(dep_label.clone());
                    }
                }
            }
        } else {
            // No target specified, add to the common dependencies for all platforms
            dep_set.common_deps.push(dep_label);
        }
    }

    fn crate_deps(&self, node: &Node) -> CrateDependencies {
        let mut crate_deps = CrateDependencies::default();
        for dep in &node.deps {
            let package = self.packages.get(&dep.pkg).unwrap();
            let mut use_dep = false;
            for dep_kind in &dep.dep_kinds {
                // The dependency kind determines to which dependency set we need to add the
                // package.
                let dep_set = match dep_kind.kind {
                    DependencyKind::Build => Some(&mut crate_deps.build_deps),
                    DependencyKind::Normal if self.crate_type(&package) == CrateType::ProcMacro => {
                        Some(&mut crate_deps.proc_macro_deps)
                    }
                    DependencyKind::Normal => Some(&mut crate_deps.normal_deps),
                    _ => None,
                };
                if let Some(dep_set) = dep_set {
                    use_dep = true;
                    self.add_dep(dep_set, &dep_kind.target, &package);
                }
            }
            if use_dep {
                let dep_name = sanitize_name(&dep.name);
                let package_name = sanitize_name(&package.name);
                if dep_name != package_name {
                    crate_deps.aliases.insert(self.dep_label(package), dep_name);
                }
            }
        }
        // If any dependency sets have platform specific dependencies, they need to have a default
        // condition for platforms that do not need any platform specific dependencies.
        for dep_set in [
            &mut crate_deps.build_deps,
            &mut crate_deps.proc_macro_deps,
            &mut crate_deps.normal_deps,
        ]
        .iter_mut()
        .filter(|ds| !ds.platform_specific_deps.is_empty())
        {
            // If there are any platform specific deps, add the default empty condition
            dep_set
                .platform_specific_deps
                .insert("//conditions:default".to_string(), Vec::new());
        }
        crate_deps
    }

    fn render_build_file(&self, node: &Node, package: &Package) -> String {
        let target = library_target(package);
        let mut crate_deps = self.crate_deps(node);
        let crate_opts = self
            .blackjack_metadata
            .crate_opts
            .get(&package.name)
            .cloned()
            .unwrap_or_default();
        let build_script = if crate_opts.build_script {
            crate_deps
                .normal_deps
                .common_deps
                .push(":build_script".to_string());
            format!(
                r#"
load("@io_bazel_rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")

cargo_build_script(
    name = "build_script",
    srcs = glob(["build.rs", "build/*.rs"]),
    deps = {build_deps},
)
                "#,
                build_deps = crate_deps.build_deps
            )
        } else {
            "".to_string()
        };
        format!(
            r#"
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")
{build_script}
rust_library(
    name = "{name}",
    aliases = {aliases:?},
    srcs = glob(["**/*.rs"]),
    crate_type = "{crate_type}",
    deps = {deps},
    proc_macro_deps = {proc_macro_deps},
    edition = "{edition}",
    crate_features = {crate_features:?},
    rustc_flags = ["--cap-lints=allow"] + {rustc_flags:?},
    visibility = ["//visibility:public"],
)
    "#,
            build_script = build_script,
            name = sanitize_name(&package.name),
            aliases = crate_deps.aliases,
            crate_type = target.crate_types[0],
            deps = crate_deps.normal_deps,
            proc_macro_deps = crate_deps.proc_macro_deps,
            edition = target.edition,
            crate_features = node.features,
            rustc_flags = crate_opts.rustc_flags,
        )
    }

    fn checksum(&self, package: &Package) -> String {
        let lockfile_package = self
            .lockfile
            .packages
            .iter()
            .find(|p| p.name.as_str() == package.name && p.version == package.version)
            .expect("No matching lockfile entry for package");
        lockfile_package
            .checksum
            .as_ref()
            .expect("package in lockfile is missing a checksum")
            .to_string()
    }
}

/// All dependencies of a crate
#[derive(Default)]
struct CrateDependencies {
    aliases: BTreeMap<String, String>,
    normal_deps: DependencySet,
    proc_macro_deps: DependencySet,
    build_deps: DependencySet,
}

/// Represents the common and platform specific dependencies of a particular class (normal,
/// proc_macro, build) for a crate.
#[derive(Default)]
struct DependencySet {
    common_deps: Vec<String>,
    platform_specific_deps: BTreeMap<String, Vec<String>>,
}

// Renders the dependencies as a valid bazel dependency set
impl fmt::Display for DependencySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.platform_specific_deps.is_empty() {
            write!(f, "{:?}", self.common_deps)
        } else {
            write!(
                f,
                "{:?} + select({:?})",
                self.common_deps, self.platform_specific_deps
            )
        }
    }
}

#[derive(PartialEq)]
enum CrateType {
    Lib,
    ProcMacro,
}

fn sanitize_name(s: &str) -> String {
    s.replace("-", "_")
}

fn sanitize_version(s: &str) -> String {
    s.replace("+", "--PLUS--")
}

fn library_target(package: &Package) -> &Target {
    package
        .targets
        .iter()
        .find(|target| {
            target
                .kind
                .iter()
                .any(|kind| kind == "lib" || kind == "proc-macro")
        })
        .expect("dependency provides no lib or proc-macro target")
}

fn direct_dependencies(metadata: &Metadata) -> HashSet<PackageId> {
    let workspace_nodes = metadata
        .resolve
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .filter(|n| metadata.workspace_members.contains(&n.id));
    workspace_nodes
        .flat_map(|n| n.deps.iter().map(|d| d.pkg.clone()))
        .collect()
}

fn triple_to_condition<T: fmt::Display>(triple: T) -> String {
    format!("@io_bazel_rules_rust//rust/platform:{}", triple)
}
