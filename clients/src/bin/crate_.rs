//! `uor-crate` — Generates the `uor-foundation` Rust library crate from the ontology.
//!
//! Reads `uor_ontology::Ontology::full()` and writes generated Rust source files
//! to the `foundation/src/` directory.
//!
//! **Usage:**
//! ```
//! uor-crate [--out <path>]
//! ```

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    missing_docs,
    clippy::missing_errors_doc
)]

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

/// Generate the uor-foundation Rust library crate from the ontology.
#[derive(Parser)]
#[command(
    name = "uor-crate",
    about = "Generate the uor-foundation Rust trait crate"
)]
struct Args {
    /// Output directory for generated source files.
    #[arg(long, default_value = "foundation/src")]
    out: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let ontology = uor_ontology::Ontology::full();

    println!(
        "Generating uor-foundation from ontology v{}: {} namespaces, {} classes, {} properties, {} individuals",
        ontology.version,
        ontology.namespaces.len(),
        ontology.class_count(),
        ontology.property_count(),
        ontology.individual_count()
    );

    let report = uor_codegen::generate(ontology, &args.out)?;

    println!(
        "Generated {} traits, {} methods, {} enums, {} constants",
        report.trait_count, report.method_count, report.enum_count, report.const_count
    );
    println!("Files written ({}):", report.files.len());
    for file in &report.files {
        println!("  {}", file);
    }

    println!("Generation complete.");
    Ok(())
}
