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
    let content = match std::fs::read_to_string(&pipeline_path) {
        Ok(c) => c,
        Err(e) => {
            report.push(TestResult::fail(
                VALIDATOR,
                format!("failed to read {}: {e}", pipeline_path.display()),
            ));
            return Ok(report);
        }
    };

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
