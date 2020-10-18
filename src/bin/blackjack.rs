use blackjack::Blackjack;
use cargo_lock::Lockfile;
use cargo_metadata::MetadataCommand;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args().skip(1);
    let cargo_path = args.next().expect("No cargo path provided (1st argument)");
    let mut cargo_toml_path: PathBuf = args
        .next()
        .expect("No Cargo.toml path provided (2nd argument)")
        .into();
    // If the Cargo.toml path is a symlink, resolve it first
    if let Ok(p) = std::fs::read_link(&cargo_toml_path) {
        cargo_toml_path = p;
    }

    let mut metadata = MetadataCommand::new();
    metadata
        .cargo_path(cargo_path)
        .manifest_path(&cargo_toml_path);

    eprintln!("Blackjack will run `cargo metadata`, which may update your `Cargo.lock` file if it is not up to date");

    let workspace_path = {
        cargo_toml_path.pop();
        cargo_toml_path
    };
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
