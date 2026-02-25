//! `uor-build` — Assembles the UOR Foundation ontology from the `uor-ontology` library
//! and writes the artifacts to the output directory.
//!
//! **Outputs:**
//! - `<out>/uor.foundation.json` — JSON-LD 1.1
//! - `<out>/uor.foundation.ttl` — Turtle 1.1
//! - `<out>/uor.foundation.nt` — N-Triples
//!
//! **Usage:**
//! ```
//! uor-build [--out <path>]
//! ```

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    missing_docs,
    clippy::missing_errors_doc
)]

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use uor_ontology::serializer::{jsonld, ntriples, turtle};
use uor_ontology::Ontology;

/// Build the UOR Foundation ontology artifacts.
#[derive(Parser)]
#[command(name = "uor-build", about = "Build UOR Foundation ontology artifacts")]
struct Args {
    /// Output directory for generated artifacts.
    #[arg(long, default_value = "public")]
    out: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let out = &args.out;

    fs::create_dir_all(out)
        .with_context(|| format!("Failed to create output directory: {}", out.display()))?;

    let ontology = Ontology::full();

    // Print summary
    println!(
        "UOR Foundation ontology v{}: {} namespaces, {} classes, {} properties, {} individuals",
        ontology.version,
        ontology.namespaces.len(),
        ontology.class_count(),
        ontology.property_count(),
        ontology.individual_count()
    );

    // JSON-LD
    let json_path = out.join("uor.foundation.json");
    let json_value = jsonld::to_json_ld(ontology);
    let json_str = serde_json::to_string_pretty(&json_value)
        .context("Failed to serialize ontology to JSON-LD")?;
    fs::write(&json_path, &json_str)
        .with_context(|| format!("Failed to write {}", json_path.display()))?;
    println!("  Written: {}", json_path.display());

    // Turtle
    let ttl_path = out.join("uor.foundation.ttl");
    let ttl_str = turtle::to_turtle(ontology);
    fs::write(&ttl_path, &ttl_str)
        .with_context(|| format!("Failed to write {}", ttl_path.display()))?;
    println!("  Written: {}", ttl_path.display());

    // N-Triples
    let nt_path = out.join("uor.foundation.nt");
    let nt_str = ntriples::to_ntriples(ontology);
    fs::write(&nt_path, &nt_str)
        .with_context(|| format!("Failed to write {}", nt_path.display()))?;
    println!("  Written: {}", nt_path.display());

    println!("Build complete.");
    Ok(())
}
