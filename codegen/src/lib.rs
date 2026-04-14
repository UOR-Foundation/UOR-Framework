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
pub mod pipeline;
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
                !mapping::ENUM_VARIANT_CLASS_NAMES.contains(&t)
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
    let enforcement_content = enforcement::generate_enforcement_module(ontology);
    let enforcement_path = out_dir.join("enforcement.rs");
    emit::write_file(&enforcement_path, &enforcement_content)?;
    // Run rustfmt on the generated file to ensure it matches cargo fmt output.
    let _ = std::process::Command::new("rustfmt")
        .arg(&enforcement_path)
        .status();
    report.files.push("enforcement.rs".to_string());

    // 7. Generate pipeline.rs (v0.2.1 reduction pipeline driver)
    let pipeline_content = pipeline::generate_pipeline_module(ontology);
    let pipeline_path = out_dir.join("pipeline.rs");
    emit::write_file(&pipeline_path, &pipeline_content)?;
    let _ = std::process::Command::new("rustfmt")
        .arg(&pipeline_path)
        .status();
    report.files.push("pipeline.rs".to_string());

    // v0.2.2 W15: the `uor-foundation-macros` crate is deleted in v0.2.2.
    // No macro-crate ontology assets to generate. The proc macros it housed
    // (`uor!`, `uor_ground!`, `#[derive(ConstrainedType)]`, `#[derive(CompileUnit)]`,
    // `#[uor_grounded]`) are removed: the contract is enforced at the type and
    // visibility level (sealed traits, `pub(crate)` constructors, the W6
    // dylint group, the public-API snapshot), and the canonical builder
    // surface is the public `Term::*` enum variants directly.

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
         //! method, and every named individual as a constant. Implementations import\n\
         //! these traits and provide concrete types.\n\
         //!\n\
         //! # Principal data path\n\
         //!\n\
         //! v0.2.2 establishes a single sanctioned API path. Everything else has been\n\
         //! deleted (no proc-macro back-doors, no second constructor for sealed types):\n\
         //!\n\
         //! ```text\n\
         //!  host bytes  ──▶  impl Grounding<Map = …>  ──▶  Datum<L>   [W4: kind-typed]\n\
         //!                                                  │\n\
         //!                                                  ▼\n\
         //!  builder.validate_const() │ .validate()  ──▶  Validated<T, Phase>\n\
         //!                                                  │            [W2 + W13]\n\
         //!                                                  ▼\n\
         //!  pipeline::run::<T, P>(unit)  ──▶  Grounded<T>\n\
         //!                                       │            [W14]\n\
         //!                                       ▼\n\
         //!                            .triad() → Triad<L>     [W8]\n\
         //!                            .certificate()          [W11: Certified<C>]\n\
         //! ```\n\
         //!\n\
         //! Every contract is enforced at the type and visibility level. Sealed traits,\n\
         //! `pub(crate)` constructors, and the v0.2.2 conformance suite (W5 ψ-leakage gate,\n\
         //! W6 public-API snapshot) catch any deviation.\n\
         //!\n\
         //! # HostTypes (v0.2.2 W10)\n\
         //!\n\
         //! Downstream chooses representations only for the four slots that genuinely\n\
         //! vary across host environments. Witt-level integers, booleans, IRIs, and\n\
         //! canonical bytes are foundation-owned and derived from `WittLevel`.\n\
         //!\n\
         //! ```rust,ignore\n\
         //! use uor_foundation::{{HostTypes, DefaultHostTypes}};\n\
         //!\n\
         //! // Use the canonical defaults: f64 / i64 / str / [u8].\n\
         //! type H = DefaultHostTypes;\n\
         //!\n\
         //! // Or override one slot:\n\
         //! struct EmbeddedHost;\n\
         //! impl HostTypes for EmbeddedHost {{\n\
         //!     type Decimal = f32;        // override: tighter precision budget\n\
         //!     type DateTime = i64;       // default\n\
         //!     type HostString = str;     // default\n\
         //!     type WitnessBytes = [u8];  // default\n\
         //! }}\n\
         //! ```\n\
         //!\n\
         //! `Primitives` is retained as a deprecated alias for v0.2.1 backwards compatibility.\n\
         //!\n\
         //! # Module structure\n\
         //!\n\
         //! - [`kernel`] — Immutable foundation: addressing, schema, operations\n\
         //! - [`bridge`] — Kernel-computed, user-consumed: queries, resolution, partitions, proofs\n\
         //! - [`user`] — Runtime declarations: types, morphisms, state\n\
         //! - [`enforcement`] — Sealed types and the principal-path entry surface\n\
         //! - [`pipeline`] — `pipeline::run::<T, P>` and the resolver dispatch\n\
         //!\n\
         //! # Enforcement layer\n\
         //!\n\
         //! [`enforcement`] provides the sealed types that v0.2.2 forbids downstream\n\
         //! from constructing directly:\n\
         //!\n\
         //! **Layer 1 — Opaque witnesses.** [`enforcement::Datum`],\n\
         //! [`enforcement::Validated`], [`enforcement::Derivation`],\n\
         //! [`enforcement::FreeRank`], [`enforcement::Grounded`],\n\
         //! [`enforcement::Certified`], [`enforcement::Triad`]: sealed types with\n\
         //! private fields. Only the foundation's pipeline / resolver paths produce them.\n\
         //!\n\
         //! **Layer 2 — Declarative builders.** [`enforcement::CompileUnitBuilder`]\n\
         //! and 8 others collect declarations and emit `Validated<T, Phase>` on\n\
         //! success or [`enforcement::ShapeViolation`] with a machine-readable IRI.\n\
         //!\n\
         //! **Layer 3 — Term AST.** [`enforcement::Term`] and\n\
         //! [`enforcement::TermArena`]: stack-resident, `#![no_std]`-safe expression\n\
         //! trees. The `Term` enum's struct-variant constructors are the canonical\n\
         //! builder API — there is no DSL macro in v0.2.2.\n\
         //!\n\
         //! # Resolvers (v0.2.2 W12)\n\
         //!\n\
         //! Verdict-producing resolvers are free functions in module-per-resolver\n\
         //! organization. Each function returns a `Result<Certified<Cert>, Witness>`:\n\
         //!\n\
         //! - `enforcement::resolver::inhabitance::certify(input)` — inhabitance verdict\n\
         //! - `enforcement::resolver::tower_completeness::certify(input)` — tower completeness\n\
         //! - `enforcement::resolver::incremental_completeness::certify(input)` — incremental\n\
         //! - `enforcement::resolver::grounding_aware::certify(unit)` — grounding-aware\n\
         //!\n\
         //! # Migration from v0.2.1\n\
         //!\n\
         //! - `uor_ground!` macro                  → deleted; use `pipeline::run::<T, P>`\n\
         //! - `#[derive(ConstrainedType)]`         → deleted; declare `const _VALIDATED: Validated<…, CompileTime>`\n\
         //! - `#[uor_grounded(level = …)]`         → deleted; use phantom-typed `Mul::<W32>::apply(…)` etc.\n\
         //! - `Primitives` trait                   → use `HostTypes` + `DefaultHostTypes`\n\
         //! - 4 cert shim types                    → use `Certified<C>` parametric carrier\n\
         //! - `Resolver::new().certify(…)` structs → `enforcement::resolver::name::certify(…)` functions\n\
         //! - `run_pipeline(&datum, level)`        → `pipeline::run::<T, P>(validated_compile_unit)`",
        ontology.version,
    ));

    f.line("#![no_std]");
    f.blank();
    f.line("pub mod bridge;");
    f.line("pub mod enforcement;");
    f.line("pub mod enums;");
    f.line("pub mod kernel;");
    f.line("pub mod pipeline;");
    f.line("pub mod user;");
    f.blank();
    f.line("pub use enums::*;");
    f.blank();

    // Primitives trait (v0.2.1 surface — retained for backwards compatibility).
    f.doc_comment("XSD primitive type family.");
    f.doc_comment("");
    f.doc_comment("Implementors choose concrete representations for each XSD type.");
    f.doc_comment("PRISM might use `u64` for integers at Q0, `u128` at higher quantum");
    f.doc_comment("levels, or a bignum library. The foundation does not constrain this choice.");
    f.doc_comment("");
    f.doc_comment("**v0.2.2 deprecation notice:** `Primitives` will be removed in a future");
    f.doc_comment("version in favor of [`HostTypes`], which narrows the trait to the four");
    f.doc_comment("slots that genuinely vary across host environments (`Decimal`, `DateTime`,");
    f.doc_comment("`HostString`, `WitnessBytes`) and lets the foundation own the integer /");
    f.doc_comment("boolean / IRI representation derived from `WittLevel`.");
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

    // v0.2.2 W10: HostTypes trait + DefaultHostTypes canonical impl.
    f.doc_comment(
        "v0.2.2 W10: narrow host-types trait that lets downstream choose representations",
    );
    f.doc_comment(
        "only for the slots that genuinely vary across host environments. Foundation-owned",
    );
    f.doc_comment(
        "types (Witt-level integers, booleans, IRIs, canonicalBytes) are derived from the",
    );
    f.doc_comment("`WittLevel` family and not exposed here.");
    f.doc_comment("");
    f.doc_comment(
        "MSRV 1.70 forbids `associated_type_defaults`, so v0.2.2 ships [`DefaultHostTypes`]",
    );
    f.doc_comment("as the canonical default impl. Downstream can either use `DefaultHostTypes`");
    f.doc_comment("directly or implement `HostTypes` on their own marker struct, optionally");
    f.doc_comment("overriding individual associated types.");
    f.line("pub trait HostTypes {");
    f.indented_doc_comment(
        "Real-number representation for kernel observables (entropies, amplitudes, rates).\n\
         `DefaultHostTypes` selects `f64`. Override with `f128`, arbitrary-precision\n\
         rational, or interval arithmetic as needed.",
    );
    f.line("    type Decimal;");
    f.blank();
    f.indented_doc_comment(
        "Host event timestamp.\n\
         `DefaultHostTypes` selects `i64` interpreted as Unix nanoseconds.\n\
         Override with `i128` for wider range, or a domain-specific timestamp type.",
    );
    f.line("    type DateTime;");
    f.blank();
    f.indented_doc_comment(
        "Host-supplied opaque string (NOT a foundation IRI).\n\
         `DefaultHostTypes` selects `str`. Override with owned `String`, `Cow<'_, str>`, etc.",
    );
    f.line("    type HostString: ?Sized;");
    f.blank();
    f.indented_doc_comment(
        "Host-supplied opaque byte sequence (NOT a foundation `canonicalBytes` constant).\n\
         `DefaultHostTypes` selects `[u8]`. Override with owned `Vec<u8>`, `Bytes`, etc.",
    );
    f.line("    type WitnessBytes: ?Sized;");
    f.line("}");
    f.blank();

    f.doc_comment(
        "v0.2.2 W10: canonical default impl of [`HostTypes`]. Selects `f64`/`i64`/`str`/`[u8]`.",
    );
    f.doc_comment(
        "Use as `type H = uor_foundation::DefaultHostTypes;` to inherit the defaults; replace",
    );
    f.doc_comment("with a downstream marker struct if any slot needs an override.");
    f.line("#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct DefaultHostTypes;");
    f.blank();
    f.line("impl HostTypes for DefaultHostTypes {");
    f.line("    type Decimal = f64;");
    f.line("    type DateTime = i64;");
    f.line("    type HostString = str;");
    f.line("    type WitnessBytes = [u8];");
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
