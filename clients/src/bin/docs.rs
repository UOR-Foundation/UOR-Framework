//! `uor-docs` — Generates documentation from the UOR spec and verified content.
//!
//! **Outputs:**
//! - `<out>/index.html` — Ontology inventory page
//! - `<out>/namespaces/*.html` — Per-namespace reference pages (auto-generated)
//! - `<out>/concepts/*.html` — Concept explanation pages
//! - `<out>/guides/*.html` — How-to guide pages
//! - `docs/DEVELOPER_REFERENCE.md` — Machine-generated developer reference
//!
//! **Usage:**
//! ```
//! uor-docs [--out <path>] [--reference <path>]
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
use uor_docs::generate;

/// Generate UOR Foundation documentation artifacts.
#[derive(Parser)]
#[command(
    name = "uor-docs",
    about = "Generate UOR Foundation documentation artifacts"
)]
struct Args {
    /// Output directory for generated documentation.
    #[arg(long, default_value = "public/docs")]
    out: PathBuf,

    /// Path for the generated developer reference; --readme is a legacy alias.
    #[arg(long, visible_alias = "readme", default_value = "docs/DEVELOPER_REFERENCE.md")]
    reference: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    generate(&args.out, &args.reference)?;

    println!("Documentation generated successfully.");
    println!("  Docs: {}", args.out.display());
    println!("  Reference: {}", args.reference.display());

    Ok(())
}
