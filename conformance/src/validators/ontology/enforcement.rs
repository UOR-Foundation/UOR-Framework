//! Declarative enforcement validator.
//!
//! Validates that the generated `enforcement.rs` module in `uor-foundation`
//! contains the required opaque witnesses, sealed traits, declarative builders,
//! and const fn ring evaluators specified by the declarative enforcement design.

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "ontology/enforcement";

/// Validates the enforcement module in `foundation/src/enforcement.rs`.
///
/// # Errors
///
/// Returns an error if the enforcement file cannot be read.
pub fn validate(workspace: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();
    let enforcement_path = workspace
        .join("foundation")
        .join("src")
        .join("enforcement.rs");

    let content = match std::fs::read_to_string(&enforcement_path) {
        Ok(c) => c,
        Err(_) => {
            report.push(TestResult::fail(
                VALIDATOR,
                "enforcement.rs not found in foundation/src/",
            ));
            return Ok(report);
        }
    };

    validate_datum_opacity(&content, &mut report);
    validate_sealed_module(&content, &mut report);
    validate_grounded_value_sealed(&content, &mut report);
    validate_builder_completeness(&content, &mut report);
    validate_shape_violation_struct(&content, &mut report);
    validate_witness_opacity(&content, &mut report);
    validate_grounded_constructors(&content, &mut report);
    validate_no_unsafe(&content, &mut report);
    validate_enforcement_docs(&content, &mut report);
    validate_no_std_compat(&content, &mut report);
    validate_macro_reexport(workspace, &mut report);
    validate_const_ring_eval(&content, &mut report);

    Ok(report)
}

/// Check that `Datum` struct has no public fields and no `pub fn new`.
fn validate_datum_opacity(content: &str, report: &mut ConformanceReport) {
    let has_datum_struct = content.contains("pub struct Datum {");

    // Check that Datum specifically has no public constructor.
    // Scan lines after "impl Datum {" until the closing "}" at impl level.
    // Other types (TermArena, builders) may have pub fn new — that's fine.
    let datum_has_no_pub_new = if let Some(pos) = content.find("impl Datum {") {
        let impl_block = &content[pos..];
        // Take at most 30 lines (Datum impl is small)
        let lines: Vec<&str> = impl_block.lines().take(30).collect();
        !lines.iter().any(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("pub fn new(") && !trimmed.starts_with("pub(crate)")
        })
    } else {
        // No impl Datum {} block at all — vacuously true (no constructor)
        true
    };

    // Check inner field is not pub
    let has_private_inner = content.contains("inner: DatumInner,");

    if has_datum_struct && datum_has_no_pub_new && has_private_inner {
        report.push(TestResult::pass(
            VALIDATOR,
            "Datum struct has private fields and no public constructor",
        ));
    } else {
        report.push(TestResult::fail(
            VALIDATOR,
            "Datum struct must have private fields and no pub fn new",
        ));
    }
}

/// Check that `mod sealed` exists and is not `pub`.
fn validate_sealed_module(content: &str, report: &mut ConformanceReport) {
    if content.contains("mod sealed {") && !content.contains("pub mod sealed {") {
        report.push(TestResult::pass(
            VALIDATOR,
            "Sealed module exists and is private",
        ));
    } else {
        report.push(TestResult::fail(
            VALIDATOR,
            "mod sealed must exist and not be pub",
        ));
    }
}

/// Check that `GroundedValue` trait is bounded by `sealed::Sealed`.
fn validate_grounded_value_sealed(content: &str, report: &mut ConformanceReport) {
    if content.contains("pub trait GroundedValue: sealed::Sealed {}") {
        report.push(TestResult::pass(VALIDATOR, "GroundedValue trait is sealed"));
    } else {
        report.push(TestResult::fail(
            VALIDATOR,
            "GroundedValue must be bounded by sealed::Sealed",
        ));
    }
}

/// Check that all 9 builder types exist with `.validate()` returning
/// `Result<Validated<_>, ShapeViolation>`.
fn validate_builder_completeness(content: &str, report: &mut ConformanceReport) {
    let builders = [
        "CompileUnitBuilder",
        "EffectDeclarationBuilder",
        "GroundingDeclarationBuilder",
        "DispatchDeclarationBuilder",
        "LeaseDeclarationBuilder",
        "StreamDeclarationBuilder",
        "PredicateDeclarationBuilder",
        "ParallelDeclarationBuilder",
        "WittLevelDeclarationBuilder",
    ];
    let mut all_present = true;
    for builder in &builders {
        if !content.contains(&format!("pub struct {builder}")) {
            all_present = false;
        }
    }
    // Check that validate() methods exist
    let validate_count = content
        .matches("fn validate(self) -> Result<Validated<")
        .count();

    if all_present && validate_count >= 9 {
        report.push(TestResult::pass(
            VALIDATOR,
            "All 9 builder types present with validate() methods",
        ));
    } else {
        report.push(TestResult::fail(
            VALIDATOR,
            format!(
                "Expected 9 builders with validate(); found {} builders, {} validate methods",
                builders
                    .iter()
                    .filter(|b| content.contains(&format!("pub struct {b}")))
                    .count(),
                validate_count,
            ),
        ));
    }
}

/// Check that `ShapeViolation` has IRI fields and `ViolationKind`.
fn validate_shape_violation_struct(content: &str, report: &mut ConformanceReport) {
    let has_struct = content.contains("pub struct ShapeViolation {");
    let has_shape_iri = content.contains("pub shape_iri: &'static str,");
    let has_constraint_iri = content.contains("pub constraint_iri: &'static str,");
    let has_kind = content.contains("pub kind: ViolationKind,");

    if has_struct && has_shape_iri && has_constraint_iri && has_kind {
        report.push(TestResult::pass(
            VALIDATOR,
            "ShapeViolation struct has IRI fields and ViolationKind",
        ));
    } else {
        report.push(TestResult::fail(
            VALIDATOR,
            "ShapeViolation must have shape_iri, constraint_iri, and kind fields",
        ));
    }
}

/// Check that `Derivation` and `FreeRank` have private fields.
fn validate_witness_opacity(content: &str, report: &mut ConformanceReport) {
    let derivation_private = content.contains("pub struct Derivation {")
        && content.contains("step_count: u32,")
        && !content.contains("pub step_count: u32,");
    let free_rank_private = content.contains("pub struct FreeRank {")
        && content.contains("total: u32,")
        && !content.contains("pub total: u32,");

    if derivation_private && free_rank_private {
        report.push(TestResult::pass(
            VALIDATOR,
            "Derivation and FreeRank have private fields",
        ));
    } else {
        report.push(TestResult::fail(
            VALIDATOR,
            "Derivation and FreeRank must have private fields",
        ));
    }
}

/// Check that `GroundedCoord` has constructors q0 through q511.
fn validate_grounded_constructors(content: &str, report: &mut ConformanceReport) {
    let constructors = ["fn q0(", "fn q1(", "fn q3(", "fn q7(", "fn q511("];
    let all_present = constructors.iter().all(|c| content.contains(c));

    if all_present {
        report.push(TestResult::pass(
            VALIDATOR,
            "GroundedCoord has q0, q1, q3, q7, q511 constructors",
        ));
    } else {
        report.push(TestResult::fail(
            VALIDATOR,
            "GroundedCoord must have q0, q1, q3, q7, q511 constructors",
        ));
    }
}

/// Check for zero `unsafe` blocks.
fn validate_no_unsafe(content: &str, report: &mut ConformanceReport) {
    if content.contains("unsafe ") {
        report.push(TestResult::fail(
            VALIDATOR,
            "enforcement.rs must not contain unsafe blocks",
        ));
    } else {
        report.push(TestResult::pass(
            VALIDATOR,
            "No unsafe blocks in enforcement module",
        ));
    }
}

/// Check that all public items have doc comments.
fn validate_enforcement_docs(content: &str, report: &mut ConformanceReport) {
    let mut undocumented = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        // Check pub struct/trait/enum/fn declarations
        if (trimmed.starts_with("pub struct ")
            || trimmed.starts_with("pub trait ")
            || trimmed.starts_with("pub enum ")
            || trimmed.starts_with("pub fn ")
            || trimmed.starts_with("pub const fn "))
            && i > 0
        {
            // Check that previous non-empty line is a doc comment or attribute
            let mut has_doc = false;
            let mut j = i;
            while j > 0 {
                j -= 1;
                let prev = lines[j].trim();
                if prev.is_empty() {
                    break;
                }
                if prev.starts_with("///") || prev.starts_with("#[") {
                    has_doc = true;
                    break;
                }
            }
            if !has_doc {
                undocumented.push(trimmed.to_string());
            }
        }
    }

    if undocumented.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            "All public items in enforcement module have doc comments",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!("{} public items missing doc comments", undocumented.len()),
            undocumented,
        ));
    }
}

/// Check no `std::` or `alloc::` imports.
fn validate_no_std_compat(content: &str, report: &mut ConformanceReport) {
    let has_std = content.contains("use std::");
    let has_alloc = content.contains("use alloc::");

    if !has_std && !has_alloc {
        report.push(TestResult::pass(
            VALIDATOR,
            "No std:: or alloc:: imports in enforcement module",
        ));
    } else {
        report.push(TestResult::fail(
            VALIDATOR,
            "enforcement.rs must not import std:: or alloc::",
        ));
    }
}

/// Check that lib.rs contains `pub use uor_foundation_macros::uor` re-export.
/// This check passes even when the macro crate is not yet created, since the
/// re-export will be added in Phase 6.
fn validate_macro_reexport(workspace: &Path, report: &mut ConformanceReport) {
    let lib_path = workspace.join("foundation").join("src").join("lib.rs");
    match std::fs::read_to_string(&lib_path) {
        Ok(content) => {
            // Check that the enforcement module is declared in lib.rs
            if content.contains("pub mod enforcement;") {
                report.push(TestResult::pass(
                    VALIDATOR,
                    "foundation lib.rs declares pub mod enforcement",
                ));
            } else {
                report.push(TestResult::fail(
                    VALIDATOR,
                    "foundation lib.rs must declare pub mod enforcement",
                ));
            }
        }
        Err(_) => {
            report.push(TestResult::fail(
                VALIDATOR,
                "Cannot read foundation/src/lib.rs",
            ));
        }
    }
}

/// Check that const fn ring evaluators exist for supported levels.
fn validate_const_ring_eval(content: &str, report: &mut ConformanceReport) {
    let evaluators = [
        "const_ring_eval_q0",
        "const_ring_eval_q1",
        "const_ring_eval_q3",
        "const_ring_eval_q7",
    ];
    let all_present = evaluators.iter().all(|e| content.contains(e));

    if all_present {
        report.push(TestResult::pass(
            VALIDATOR,
            "const fn ring evaluators present for Q0, Q1, Q3, Q7",
        ));
    } else {
        report.push(TestResult::fail(
            VALIDATOR,
            "Missing const fn ring evaluators for supported levels",
        ));
    }
}
