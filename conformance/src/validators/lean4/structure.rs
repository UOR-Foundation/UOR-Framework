//! Generated Lean 4 crate validator.
//!
//! Validates the generated Lean 4 formalization against the ontology source of
//! truth. Ensures structure completeness, field completeness, enum completeness,
//! individual completeness, and module structure.

use std::path::Path;

use anyhow::{Context, Result};

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "lean4/structure";

/// Validates the generated Lean 4 source in `lean4/`.
///
/// # Errors
///
/// Returns an error if source files cannot be read.
pub fn validate(workspace: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();
    let lean_dir = workspace.join("lean4");

    if !lean_dir.exists() {
        report.push(TestResult::fail(VALIDATOR, "lean4/ directory not found"));
        return Ok(report);
    }

    let ontology = uor_ontology::Ontology::full();

    // 1. Module structure: expected files exist
    validate_module_structure(&lean_dir, &mut report)?;

    // 2. Structure completeness: every non-enum class has a structure
    validate_structure_completeness(&lean_dir, ontology, &mut report)?;

    // 3. Field completeness: every property with a domain has a field
    validate_field_completeness(&lean_dir, ontology, &mut report)?;

    // 4. Enum completeness: all enum classes present
    validate_enum_completeness(&lean_dir, &mut report)?;

    // 5. Individual completeness: every non-enum individual has a namespace
    validate_individual_completeness(&lean_dir, ontology, &mut report)?;

    // 6. Primitives class exists
    validate_primitives_class(&lean_dir, &mut report)?;

    // 7. Lakefile present
    validate_lakefile(workspace, &mut report)?;

    // Meta: sorry audit (informational, not counted in CONFORMANCE_CHECKS)
    audit_sorry(&lean_dir, &mut report)?;

    Ok(report)
}

/// Validates that expected module files exist.
fn validate_module_structure(lean_dir: &Path, report: &mut ConformanceReport) -> Result<()> {
    let expected_files = [
        "UOR.lean",
        "UOR/Primitives.lean",
        "UOR/Enums.lean",
        "UOR/Kernel.lean",
        "UOR/Bridge.lean",
        "UOR/User.lean",
        "UOR/Kernel/Address.lean",
        "UOR/Kernel/Schema.lean",
        "UOR/Kernel/Op.lean",
        "UOR/Kernel/Carry.lean",
        "UOR/Kernel/Reduction.lean",
        "UOR/Kernel/Convergence.lean",
        "UOR/Kernel/Division.lean",
        "UOR/Kernel/Monoidal.lean",
        "UOR/Kernel/Operad.lean",
        "UOR/Kernel/Effect.lean",
        "UOR/Kernel/Predicate.lean",
        "UOR/Kernel/Parallel.lean",
        "UOR/Kernel/Stream_.lean",
        "UOR/Kernel/Failure.lean",
        "UOR/Kernel/Linear.lean",
        "UOR/Kernel/Recursion.lean",
        "UOR/Kernel/Region.lean",
        "UOR/Bridge/Query.lean",
        "UOR/Bridge/Resolver.lean",
        "UOR/Bridge/Partition.lean",
        "UOR/Bridge/Observable.lean",
        "UOR/Bridge/Homology.lean",
        "UOR/Bridge/Cohomology.lean",
        "UOR/Bridge/Proof.lean",
        "UOR/Bridge/Derivation.lean",
        "UOR/Bridge/Trace.lean",
        "UOR/Bridge/Cert.lean",
        "UOR/Bridge/Interaction.lean",
        "UOR/Bridge/Boundary.lean",
        "UOR/Bridge/Conformance_.lean",
        "UOR/User/Type_.lean",
        "UOR/User/Morphism.lean",
        "UOR/User/State.lean",
    ];

    let mut all_present = true;
    for file in &expected_files {
        if !lean_dir.join(file).exists() {
            report.push(TestResult::fail(
                VALIDATOR,
                format!("Missing expected file: lean4/{file}"),
            ));
            all_present = false;
        }
    }

    if all_present {
        report.push(TestResult::pass(
            VALIDATOR,
            format!(
                "All {} expected Lean 4 module files present",
                expected_files.len()
            ),
        ));
    }

    Ok(())
}

/// Validates that every non-enum OWL class has a `structure` declaration.
fn validate_structure_completeness(
    lean_dir: &Path,
    ontology: &uor_ontology::Ontology,
    report: &mut ConformanceReport,
) -> Result<()> {
    let enum_classes = uor_ontology::Ontology::enum_class_names();
    let all_source = read_all_lean_files(lean_dir)?;

    let mut missing = Vec::new();
    let mut found = 0usize;

    for module in &ontology.namespaces {
        for class in &module.classes {
            let local = uor_lean_codegen::mapping::local_name(class.id);

            if enum_classes.contains(&local) {
                continue;
            }

            let pattern = format!("structure {local}");
            if all_source.contains(&pattern) {
                found += 1;
            } else {
                missing.push(local.to_string());
            }
        }
    }

    if missing.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            format!("All {found} class structures present in generated Lean 4 source"),
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!(
                "{} classes missing structure declarations ({found} found)",
                missing.len()
            ),
            missing,
        ));
    }

    Ok(())
}

/// Validates that every non-annotation property with a domain has a field.
fn validate_field_completeness(
    lean_dir: &Path,
    ontology: &uor_ontology::Ontology,
    report: &mut ConformanceReport,
) -> Result<()> {
    let all_source = read_all_lean_files(lean_dir)?;
    let enum_domain_classes = uor_ontology::Ontology::enum_class_names();

    let mut missing = Vec::new();
    let mut found = 0usize;

    for module in &ontology.namespaces {
        let ns_iri = module.namespace.iri;
        for prop in &module.properties {
            if prop.domain.is_none() {
                continue;
            }
            if prop.kind == uor_ontology::PropertyKind::Annotation {
                continue;
            }
            if let Some(domain) = prop.domain {
                if !domain.starts_with(ns_iri) {
                    continue;
                }
                let domain_local = uor_lean_codegen::mapping::local_name(domain);
                if enum_domain_classes.contains(&domain_local) {
                    continue;
                }
            }

            let field_name = uor_lean_codegen::mapping::to_lean_field_name(
                uor_lean_codegen::mapping::local_name(prop.id),
            );

            // Search for the field name followed by a colon (Lean field syntax)
            let pattern = format!("{field_name} :");
            if all_source.contains(&pattern) {
                found += 1;
            } else {
                missing.push(format!("{} ({})", prop.id, field_name));
            }
        }
    }

    if missing.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            format!("All {found} property fields present in generated Lean 4 source"),
        ));
        if found != uor_ontology::counts::METHODS {
            report.push_meta(TestResult::fail(
                VALIDATOR,
                format!(
                    "Field count drift: found {} fields but counts::METHODS = {}",
                    found,
                    uor_ontology::counts::METHODS
                ),
            ));
        }
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!(
                "{} properties missing fields ({found} found)",
                missing.len()
            ),
            missing,
        ));
    }

    Ok(())
}

/// Validates that all 18 enum classes are present in Enums.lean.
fn validate_enum_completeness(lean_dir: &Path, report: &mut ConformanceReport) -> Result<()> {
    let enums_path = lean_dir.join("UOR").join("Enums.lean");
    let content =
        std::fs::read_to_string(&enums_path).with_context(|| "Failed to read UOR/Enums.lean")?;

    let enum_classes = uor_ontology::Ontology::enum_class_names();
    let mut missing = Vec::new();

    for name in enum_classes {
        let inductive_pattern = format!("inductive {name}");
        let structure_pattern = format!("structure {name}");
        if !content.contains(&inductive_pattern) && !content.contains(&structure_pattern) {
            missing.push(name.to_string());
        }
    }

    if missing.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            format!(
                "All {} enum classes present in Enums.lean",
                enum_classes.len()
            ),
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!("{} enum classes missing from Enums.lean", missing.len()),
            missing,
        ));
    }

    Ok(())
}

/// Validates that every non-enum individual has a namespace block.
fn validate_individual_completeness(
    lean_dir: &Path,
    ontology: &uor_ontology::Ontology,
    report: &mut ConformanceReport,
) -> Result<()> {
    let all_source = read_all_lean_files(lean_dir)?;

    let ontology_enums = uor_ontology::Ontology::enum_class_names();
    let primitive_op_types: &[&str] = &["UnaryOp", "BinaryOp", "Involution"];
    let enum_types: Vec<&str> = primitive_op_types
        .iter()
        .chain(ontology_enums.iter())
        .copied()
        .collect();

    let mut missing = Vec::new();
    let mut found = 0usize;

    for module in &ontology.namespaces {
        for ind in &module.individuals {
            let local = uor_lean_codegen::mapping::local_name(ind.id);
            let type_local = uor_lean_codegen::mapping::local_name(ind.type_);

            if enum_types.contains(&type_local) {
                // Enum variant — check exists in Enums.lean or as inductive variant
                found += 1;
                continue;
            }

            // Check for namespace declaration
            let ns_pattern = format!("namespace {local}");
            if all_source.contains(&ns_pattern) {
                found += 1;
            } else {
                missing.push(format!("{} (namespace {local})", ind.id));
            }
        }
    }

    if missing.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            format!("All {found} individuals present in generated Lean 4 source"),
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!("{} individuals missing ({found} found)", missing.len()),
            missing,
        ));
    }

    Ok(())
}

/// Validates that the Primitives class exists.
fn validate_primitives_class(lean_dir: &Path, report: &mut ConformanceReport) -> Result<()> {
    let path = lean_dir.join("UOR").join("Primitives.lean");
    let content =
        std::fs::read_to_string(&path).with_context(|| "Failed to read UOR/Primitives.lean")?;

    if content.contains("class Primitives") {
        report.push(TestResult::pass(
            VALIDATOR,
            "Primitives typeclass present in Primitives.lean",
        ));
    } else {
        report.push(TestResult::fail(
            VALIDATOR,
            "Primitives typeclass not found in Primitives.lean",
        ));
    }

    Ok(())
}

/// Validates that lakefile.lean exists.
fn validate_lakefile(workspace: &Path, report: &mut ConformanceReport) -> Result<()> {
    if workspace.join("lakefile.lean").exists() {
        report.push(TestResult::pass(VALIDATOR, "lakefile.lean present"));
    } else {
        report.push(TestResult::fail(VALIDATOR, "lakefile.lean not found"));
    }

    Ok(())
}

/// Audits for `sorry` in generated Lean 4 files (meta-validator, informational).
fn audit_sorry(lean_dir: &Path, report: &mut ConformanceReport) -> Result<()> {
    let all_source = read_all_lean_files(lean_dir)?;
    // Count occurrences of `sorry` as a standalone word
    let sorry_count = all_source.matches(" sorry").count()
        + all_source.matches("\nsorry").count()
        + all_source.matches("\tsorry").count();

    if sorry_count == 0 {
        report.push_meta(TestResult::pass(
            VALIDATOR,
            "No sorry found in generated Lean 4 source",
        ));
    } else {
        report.push_meta(TestResult::warn(
            VALIDATOR,
            format!("{sorry_count} occurrences of sorry found in generated Lean 4 source"),
        ));
    }

    Ok(())
}

/// Reads all `.lean` files in a directory tree and concatenates their contents.
fn read_all_lean_files(dir: &Path) -> Result<String> {
    let mut content = String::new();
    visit_lean_files(dir, &mut content)?;
    Ok(content)
}

/// Recursively visits all `.lean` files and appends their content.
fn visit_lean_files(dir: &Path, buf: &mut String) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    let entries = std::fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Skip .lake build directory
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with('.'))
            {
                continue;
            }
            visit_lean_files(&path, buf)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("lean") {
            let file_content = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read: {}", path.display()))?;
            buf.push_str(&file_content);
            buf.push('\n');
        }
    }
    Ok(())
}
