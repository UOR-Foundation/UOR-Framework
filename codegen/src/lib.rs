//! UOR Foundation code generator.
//!
//! Reads the ontology from `uor_ontology::Ontology::full()` and generates the
//! `uor-foundation` Rust trait crate. The generated crate exports every ontology
//! class as a trait, every property as a method, and every named individual as a
//! constant — giving PRISM and other implementations a well-defined Rust interface.

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    missing_docs,
    clippy::missing_errors_doc
)]

pub mod emit;
pub mod enums;
pub mod individuals;
pub mod mapping;
pub mod traits;

use std::fmt::Write as FmtWrite;
use std::path::Path;

use anyhow::Result;
use uor_ontology::model::Space;
use uor_ontology::Ontology;

use emit::RustFile;
use mapping::namespace_mappings;

/// Report of what was generated.
#[derive(Debug, Default)]
pub struct GenerationReport {
    /// Number of traits generated.
    pub trait_count: usize,
    /// Number of methods generated.
    pub method_count: usize,
    /// Number of enums generated.
    pub enum_count: usize,
    /// Number of individual constants generated.
    pub const_count: usize,
    /// Files written.
    pub files: Vec<String>,
}

/// Generates the complete `uor-foundation` crate source into `out_dir`.
///
/// # Errors
///
/// Returns an error if any file cannot be written.
pub fn generate(ontology: &Ontology, out_dir: &Path) -> Result<GenerationReport> {
    let mut report = GenerationReport::default();
    let ns_map = namespace_mappings();

    // 1. Generate enums.rs
    let enums_content = enums::generate_enums_file(ontology);
    report.enum_count = enums::detect_enums(ontology).len();
    emit::write_file(&out_dir.join("enums.rs"), &enums_content)?;
    report.files.push("enums.rs".to_string());

    // 2. Generate per-namespace module files
    let mut kernel_modules = Vec::new();
    let mut bridge_modules = Vec::new();
    let mut user_modules = Vec::new();

    for module in &ontology.namespaces {
        let ns_iri = module.namespace.iri;
        let mapping = match ns_map.get(ns_iri) {
            Some(m) => m,
            None => continue,
        };

        let content = traits::generate_namespace_module(module, &ns_map);

        // Append PrimitiveOp impls to op.rs
        let content = if mapping.file_module == "op" {
            let op_impls = individuals::generate_primitive_op_impls(ontology);
            format!("{content}\n{op_impls}")
        } else {
            content
        };

        let file_path = out_dir
            .join(mapping.space_module)
            .join(format!("{}.rs", mapping.file_module));
        emit::write_file(&file_path, &content)?;

        let file_rel = format!("{}/{}.rs", mapping.space_module, mapping.file_module);
        report.files.push(file_rel);

        // Count traits and methods
        report.trait_count += module.classes.len();
        for prop in &module.properties {
            if prop.domain.is_some() && prop.kind != uor_ontology::PropertyKind::Annotation {
                report.method_count += 1;
            }
        }
        report.const_count += module
            .individuals
            .iter()
            .filter(|ind| {
                let t = mapping::local_name(ind.type_);
                t != "UnaryOp" && t != "BinaryOp" && t != "Involution" && t != "MetricAxis"
            })
            .count();

        match module.namespace.space {
            Space::Kernel => kernel_modules.push(mapping.file_module),
            Space::Bridge => bridge_modules.push(mapping.file_module),
            Space::User => user_modules.push(mapping.file_module),
        }
    }

    // 3. Generate space mod.rs files
    generate_mod_file(out_dir, "kernel", &kernel_modules, &mut report)?;
    generate_mod_file(out_dir, "bridge", &bridge_modules, &mut report)?;
    generate_mod_file(out_dir, "user", &user_modules, &mut report)?;

    // 4. Generate lib.rs
    let lib_content = generate_lib_rs(ontology);
    emit::write_file(&out_dir.join("lib.rs"), &lib_content)?;
    report.files.push("lib.rs".to_string());

    // 5. Generate README.md (written to parent of out_dir, i.e. foundation/)
    if let Some(crate_dir) = out_dir.parent() {
        let readme = generate_readme(ontology);
        emit::write_file(&crate_dir.join("README.md"), &readme)?;
    }

    Ok(report)
}

/// Generates a `mod.rs` file for a space module.
fn generate_mod_file(
    out_dir: &Path,
    space: &str,
    modules: &[&str],
    report: &mut GenerationReport,
) -> Result<()> {
    let mut f = RustFile::new(&format!("`{space}` space modules."));

    let mut sorted_modules: Vec<&str> = modules.to_vec();
    sorted_modules.sort_unstable();
    for module in &sorted_modules {
        let _ = writeln!(f.buf, "pub mod {module};");
    }

    let path = out_dir.join(space).join("mod.rs");
    emit::write_file(&path, &f.finish())?;
    report.files.push(format!("{space}/mod.rs"));
    Ok(())
}

/// Generates the crate root `lib.rs`.
fn generate_lib_rs(ontology: &Ontology) -> String {
    let mut f = RustFile::new(&format!(
        "UOR Foundation — typed Rust traits for the complete ontology.\n\
         //!\n\
         //! Version: {}\n\
         //!\n\
         //! This crate exports every ontology class as a trait, every property as a\n\
         //! method, and every named individual as a constant. Implementations (like\n\
         //! PRISM) import these traits and provide concrete types.\n\
         //!\n\
         //! # Primitives\n\
         //!\n\
         //! All traits are generic over [`Primitives`], a type family that lets\n\
         //! implementors choose their own concrete representations for XSD types.\n\
         //!\n\
         //! ```rust,ignore\n\
         //! struct MyImpl;\n\
         //! impl uor_foundation::Primitives for MyImpl {{\n\
         //!     type String = str;\n\
         //!     type Integer = i64;\n\
         //!     type NonNegativeInteger = u64;\n\
         //!     type PositiveInteger = u64;\n\
         //!     type Decimal = f64;\n\
         //!     type Boolean = bool;\n\
         //! }}\n\
         //! ```\n\
         //!\n\
         //! # Module Structure\n\
         //!\n\
         //! - [`kernel`] — Immutable foundation: addressing, schema, operations\n\
         //! - [`bridge`] — Kernel-computed, user-consumed: queries, resolution, partitions, proofs\n\
         //! - [`user`] — Runtime declarations: types, morphisms, state",
        ontology.version,
    ));

    f.line("#![no_std]");
    f.blank();
    f.line("pub mod bridge;");
    f.line("pub mod enums;");
    f.line("pub mod kernel;");
    f.line("pub mod user;");
    f.blank();
    f.line("pub use enums::*;");
    f.blank();

    // Primitives trait
    f.doc_comment("XSD primitive type family.");
    f.doc_comment("");
    f.doc_comment("Implementors choose concrete representations for each XSD type.");
    f.doc_comment("PRISM might use `u64` for integers at Q0, `u128` at higher quantum");
    f.doc_comment("levels, or a bignum library. The foundation does not constrain this choice.");
    f.line("pub trait Primitives {");
    f.indented_doc_comment(
        "String type (`xsd:string`). Use `str` for borrowed, `String` for owned.",
    );
    f.line("    type String: ?Sized;");
    f.indented_doc_comment("Integer type (`xsd:integer`).");
    f.line("    type Integer;");
    f.indented_doc_comment("Non-negative integer type (`xsd:nonNegativeInteger`).");
    f.line("    type NonNegativeInteger;");
    f.indented_doc_comment("Positive integer type (`xsd:positiveInteger`).");
    f.line("    type PositiveInteger;");
    f.indented_doc_comment("Decimal type (`xsd:decimal`).");
    f.line("    type Decimal;");
    f.indented_doc_comment("Boolean type (`xsd:boolean`).");
    f.line("    type Boolean;");
    f.line("}");
    f.blank();

    f.finish()
}

/// Generates `README.md` for the published crate.
fn generate_readme(ontology: &Ontology) -> String {
    format!(
        r#"# uor-foundation

The complete [UOR Foundation](https://uor.foundation/) ontology encoded as
typed Rust traits. Import and implement.

## Contents

- {ns} namespaces
- {classes} OWL classes (one trait each)
- {props} OWL properties (one method each)
- {inds} named individuals (constants and enums)

## Quick start

```toml
[dependencies]
uor-foundation = "{version}"
```

```rust
use uor_foundation::Primitives;

struct MyImpl;
impl Primitives for MyImpl {{
    type String = str;
    type Integer = i64;
    type NonNegativeInteger = u64;
    type PositiveInteger = u64;
    type Decimal = f64;
    type Boolean = bool;
}}
```

Then implement any foundation trait with your chosen primitives:

```rust,ignore
use uor_foundation::bridge::partition::FiberBudget;

impl FiberBudget<MyImpl> for MyFiberBudget {{
    // ...
}}
```

## Module structure

| Module | Space | Description |
|--------|-------|-------------|
| `kernel::address` | Kernel | Content-addressed glyph space |
| `kernel::schema` | Kernel | Ring schema: Datum, Term, Ring |
| `kernel::op` | Kernel | Primitive operations and the dihedral group |
| `bridge::query` | Bridge | Query hierarchy |
| `bridge::resolver` | Bridge | Resolution state machine |
| `bridge::partition` | Bridge | Irreducibility partitions and fiber budgets |
| `bridge::observable` | Bridge | Observable measurements |
| `bridge::proof` | Bridge | Proof and witness data |
| `bridge::derivation` | Bridge | Derivation and rewrite steps |
| `bridge::trace` | Bridge | Computation traces |
| `bridge::cert` | Bridge | Certificate hierarchy |
| `user::type_` | User | Type definitions and constraints |
| `user::morphism` | User | Transforms and composition laws |
| `user::state` | User | Context, bindings, frames, transitions |

## Features

This crate is `#![no_std]` with zero mandatory dependencies.

## License

Apache-2.0 — see [LICENSE](https://github.com/UOR-Foundation/UOR-Framework/blob/main/LICENSE).
"#,
        version = ontology.version,
        ns = ontology.namespaces.len(),
        classes = ontology.class_count(),
        props = ontology.property_count(),
        inds = ontology.individual_count(),
    )
}
