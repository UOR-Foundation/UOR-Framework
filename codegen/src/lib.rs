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
pub mod enforcement;
pub mod enums;
pub mod individuals;
pub mod mapping;
pub mod traits;

use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::path::Path;

use anyhow::Result;
use uor_ontology::model::Space;
use uor_ontology::{Ontology, Property};

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

    // Build cross-namespace property-by-domain lookup for inherited associated type detection.
    let all_props_by_domain: HashMap<&str, Vec<&Property>> = {
        let mut map: HashMap<&str, Vec<&Property>> = HashMap::new();
        for module in &ontology.namespaces {
            for prop in &module.properties {
                if let Some(domain) = prop.domain {
                    map.entry(domain).or_default().push(prop);
                }
            }
        }
        map
    };

    for module in &ontology.namespaces {
        let ns_iri = module.namespace.iri;
        let mapping = match ns_map.get(ns_iri) {
            Some(m) => m,
            None => continue,
        };

        let content = traits::generate_namespace_module(module, &ns_map, &all_props_by_domain);

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

    // 6. Generate enforcement.rs (declarative enforcement types)
    let enforcement_content = enforcement::generate_enforcement_module();
    let enforcement_path = out_dir.join("enforcement.rs");
    emit::write_file(&enforcement_path, &enforcement_content)?;
    // Run rustfmt on the generated file to ensure it matches cargo fmt output.
    let _ = std::process::Command::new("rustfmt")
        .arg(&enforcement_path)
        .status();
    report.files.push("enforcement.rs".to_string());

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
         //! - [`user`] — Runtime declarations: types, morphisms, state\n\
         //!\n\
         //! # Enforcement Layer\n\
         //!\n\
         //! The [`enforcement`] module provides concrete types (not generic over\n\
         //! `Primitives`) for declarative validation. These form a three-layer\n\
         //! pipeline:\n\
         //!\n\
         //! **Layer 1 — Opaque Witnesses.** [`enforcement::Datum`],\n\
         //! [`enforcement::Validated`], [`enforcement::Derivation`],\n\
         //! [`enforcement::FreeRank`]: sealed types with private fields that\n\
         //! prove a value passed through the reduction evaluator or the two-phase\n\
         //! minting boundary. Prism code consumes these but cannot fabricate them.\n\
         //!\n\
         //! **Layer 2 — Declarative Builders.** [`enforcement::CompileUnitBuilder`]\n\
         //! and 8 others collect the declarations required by each conformance\n\
         //! shape, then call `validate()` to get a `Validated<T>` or a\n\
         //! [`enforcement::ShapeViolation`] with machine-readable IRIs.\n\
         //!\n\
         //! **Layer 3 — Term AST.** [`enforcement::Term`] and\n\
         //! [`enforcement::TermArena`]: stack-resident, `#![no_std]`-safe\n\
         //! expression trees. Builders accept `Term` references (not closures),\n\
         //! keeping Prism declarations within the term language.\n\
         //!\n\
         //! # The `uor!` Macro\n\
         //!\n\
         //! The re-exported [`uor!`] macro parses EBNF surface syntax at compile\n\
         //! time and produces typed `Term` ASTs. Ground assertions (no free\n\
         //! variables) are evaluated at compile time using the foundation's\n\
         //! `const fn` ring arithmetic.\n\
         //!\n\
         //! ```rust,ignore\n\
         //! use uor_foundation::uor;\n\
         //!\n\
         //! // Type declaration with constraints:\n\
         //! let pixel = uor! {{ type Pixel {{ residue: 255; hamming: 8; }} }};\n\
         //!\n\
         //! // Operation composition (produces a TermArena):\n\
         //! let expr = uor! {{ add(mul(3, 5), 7) }};\n\
         //!\n\
         //! // Ground assertion — checked at compile time:\n\
         //! uor! {{ assert add(1, 2) = 3; }};\n\
         //! ```\n\
         //!\n\
         //! # Getting Started\n\
         //!\n\
         //! 1. Implement [`Primitives`] for your concrete type family.\n\
         //! 2. Use the [`enforcement`] builders to declare your types, effects,\n\
         //!    and boundaries.\n\
         //! 3. Use the [`uor!`] macro for term-language expressions.\n\
         //! 4. The reduction evaluator validates and evaluates your declarations,\n\
         //!    producing [`enforcement::Datum`] and [`enforcement::Derivation`]\n\
         //!    witnesses.",
        ontology.version,
    ));

    f.line("#![no_std]");
    f.blank();
    f.line("pub mod bridge;");
    f.line("pub mod enforcement;");
    f.line("pub mod enums;");
    f.line("pub mod kernel;");
    f.line("pub mod user;");
    f.blank();
    f.line("pub use enums::*;");
    f.line("pub use uor_foundation_macros::uor;");
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
    let ns_map = namespace_mappings();

    // Build module table rows dynamically from the ontology
    let mut rows = String::new();
    for module in &ontology.namespaces {
        if let Some(mapping) = ns_map.get(module.namespace.iri) {
            let space_label = match module.namespace.space {
                Space::Kernel => "Kernel",
                Space::Bridge => "Bridge",
                Space::User => "User",
            };
            // Use first sentence of namespace comment as description
            let desc = module
                .namespace
                .comment
                .split('.')
                .next()
                .unwrap_or(module.namespace.label);
            let _ = writeln!(
                rows,
                "| `{}::{}` | {} | {} |",
                mapping.space_module, mapping.file_module, space_label, desc
            );
        }
    }

    format!(
        r#"# uor-foundation

The complete [UOR Foundation](https://uor.foundation/) ontology encoded as
typed Rust traits. Import and implement.

## Contents

- {ns} namespaces
- {classes} OWL classes (one trait each)
- {props} OWL properties (one method each)
- {inds} named individuals (constants and enums)
- `enforcement` module with declarative builders and opaque witnesses
- `uor!` proc macro for compile-time term-language DSL

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
use uor_foundation::bridge::partition::FreeRank;

impl FreeRank<MyImpl> for MyFreeRank {{
    // ...
}}
```

## Module structure

| Module | Space | Description |
|--------|-------|-------------|
{module_rows}| `enums` | — | Controlled vocabulary enums (WittLevel, PrimitiveOp, etc.) |
| `enforcement` | — | Opaque witnesses, declarative builders, Term AST |

## Features

This crate is `#![no_std]` with zero mandatory dependencies. The `uor!`
proc macro (from `uor-foundation-macros`) parses term-language expressions
at compile time.

## License

Apache-2.0 — see [LICENSE](https://github.com/UOR-Foundation/UOR-Framework/blob/main/LICENSE).
"#,
        version = ontology.version,
        ns = ontology.namespaces.len(),
        classes = ontology.class_count(),
        props = ontology.property_count(),
        inds = ontology.individual_count(),
        module_rows = rows,
    )
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn generation_produces_correct_trait_count() {
        let ontology = uor_ontology::Ontology::full();
        let tmp = std::env::temp_dir().join("uor_codegen_test");
        let _ = std::fs::create_dir_all(&tmp);
        match generate(ontology, &tmp) {
            Ok(report) => {
                assert_eq!(
                    report.trait_count,
                    uor_ontology::counts::CLASSES,
                    "Trait count should match CLASSES"
                );
                assert!(
                    report.method_count >= uor_ontology::counts::METHODS,
                    "Method count ({}) should be >= METHODS ({})",
                    report.method_count,
                    uor_ontology::counts::METHODS
                );
            }
            Err(e) => panic!("Code generation failed: {e}"),
        }
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
