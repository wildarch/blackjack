use blackjack::Blackjack;
use cargo_lock::Lockfile;
use cargo_metadata::MetadataCommand;
use std::io::Write;
use std::path::{Path, PathBuf};

const CARGO_TOML_RUNFILES_PATH: &'static str = "Cargo.toml";
const CARGO_RUNFILES_PATH: &'static str = "external/blackjack_cargo/cargo";

fn workspace_path() -> PathBuf {
    // This is somewhat of an implementation detail
    let mut cargo_toml_path =
        std::fs::read_link(CARGO_TOML_RUNFILES_PATH).unwrap_or(PathBuf::from("Cargo.toml"));
    cargo_toml_path.pop();
    cargo_toml_path
}

fn set_cargo_path(metadata: &mut MetadataCommand) {
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
}

fn main() {
    let workspace_path = workspace_path();
    let cargo_toml_path = workspace_path.join("Cargo.toml");

    let mut metadata = MetadataCommand::new();
    metadata.manifest_path(&cargo_toml_path).other_options(vec![
        // TODO make this configurable
        "--filter-platform".to_string(),
        "x86_64-unknown-linux-gnu".to_string(),
    ]);
    set_cargo_path(&mut metadata);

    eprintln!("Blackjack will run `cargo metadata`, which may update your `Cargo.lock` file if it is not up to date");

    let output_path = workspace_path.join("cargo_dependencies.bzl");
    eprintln!("Writing output to {}", output_path.display());
    eprintln!("Press enter to continue, or Ctrl-C to abort");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read stdin");

    let metadata = metadata.exec().expect("cargo metadata failed");

    // Load Cargo.lock, but only after cargo metadata has run and it has been updated
    let lockfile =
        Lockfile::load(workspace_path.join("Cargo.lock")).expect("Failed to load Cargo.lock");

    let blackjack = Blackjack::new(metadata, lockfile);

    let mut temp_output = Vec::new();
    blackjack
        .render(&mut temp_output)
        // Writes to a Vec cannot fail
        .unwrap();

    let mut output = std::fs::File::create(output_path).expect("Could not open output file");
    output
        .write_all(&temp_output)
        .expect("Failed to write to output file");

    eprintln!("Done.");
}
