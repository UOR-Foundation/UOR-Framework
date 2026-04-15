//! v0.2.2 Phase G validator: widened const-fn frontier.
//!
//! Asserts that the pipeline crate exposes the Phase G const-fn surface:
//! - 4 `validate_*_const` companion functions for the Lease/CompileUnit/
//!   Parallel/Stream builders;
//! - 4 `certify_*_const` companion functions for the tower_completeness,
//!   incremental_completeness, inhabitance, and multiplication resolvers;
//! - `pipeline::run_const` with the widened `T::Map: Total` gate.

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/const_fn_frontier";

/// Runs the const-fn frontier check.
///
/// # Errors
///
/// Returns an error if the pipeline source file cannot be read.
pub fn validate(workspace: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();
    let pipeline_path = workspace.join("foundation/src/pipeline.rs");
    let enforcement_path = workspace.join("foundation/src/enforcement.rs");
    let pipeline_content = match std::fs::read_to_string(&pipeline_path) {
        Ok(c) => c,
        Err(e) => {
            report.push(TestResult::fail(
                VALIDATOR,
                format!("failed to read {}: {e}", pipeline_path.display()),
            ));
            return Ok(report);
        }
    };
    let enforcement_content = match std::fs::read_to_string(&enforcement_path) {
        Ok(c) => c,
        Err(e) => {
            report.push(TestResult::fail(
                VALIDATOR,
                format!("failed to read {}: {e}", enforcement_path.display()),
            ));
            return Ok(report);
        }
    };
    // Concatenate for substring search across both files.
    let content = format!("{pipeline_content}\n{enforcement_content}");

    let required: &[(&str, &str)] = &[
        // validate_*_const family (4).
        ("validate_lease_const", "pub const fn validate_lease_const("),
        (
            "validate_compile_unit_const",
            "pub const fn validate_compile_unit_const(",
        ),
        (
            "validate_parallel_const",
            "pub const fn validate_parallel_const(",
        ),
        (
            "validate_stream_const",
            "pub const fn validate_stream_const(",
        ),
        // certify_*_const family (4).
        (
            "certify_tower_completeness_const",
            "pub const fn certify_tower_completeness_const(",
        ),
        (
            "certify_incremental_completeness_const",
            "pub const fn certify_incremental_completeness_const(",
        ),
        (
            "certify_inhabitance_const",
            "pub const fn certify_inhabitance_const(",
        ),
        (
            "certify_multiplication_const",
            "pub const fn certify_multiplication_const(",
        ),
        // Widened const pipeline entry.
        ("pipeline::run_const", "pub const fn run_const<T>("),
        // v0.2.2 T2.8 (cleanup): anchors asserting functional input-dependence.
        (
            "CompileUnit::from_parts_const",
            "pub(crate) const fn from_parts_const(level: WittLevel, budget: u64)",
        ),
        (
            "CompileUnitBuilder::witt_level_option",
            "pub const fn witt_level_option(&self) -> Option<WittLevel>",
        ),
        (
            "CompileUnitBuilder::budget_option",
            "pub const fn budget_option(&self) -> Option<u64>",
        ),
        (
            "GroundingCertificate::with_level_const",
            "pub(crate) const fn with_level_const(witt_bits: u16)",
        ),
        (
            "fnv1a_u128_const helper",
            "pub(crate) const fn fnv1a_u128_const(a: u64, b: u64) -> u128",
        ),
        (
            "validate_compile_unit_const reads builder.witt_level_option",
            "builder.witt_level_option()",
        ),
        (
            "certify_tower_completeness_const reads unit witt level",
            "unit.inner().witt_level().witt_length() as u16",
        ),
        (
            "run_const reads unit and derives unit_address",
            "fnv1a_u128_const(",
        ),
    ];

    let mut missing: Vec<String> = Vec::new();
    for (label, anchor) in required {
        if !content.contains(*anchor) {
            missing.push((*label).to_string());
        }
    }

    if missing.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            "Phase G const-fn frontier: 4 validate_*_const + 4 certify_*_const \
             + pipeline::run_const all present",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!(
                "Phase G const-fn frontier has {} missing anchors",
                missing.len()
            ),
            missing,
        ));
    }

    Ok(report)
}
