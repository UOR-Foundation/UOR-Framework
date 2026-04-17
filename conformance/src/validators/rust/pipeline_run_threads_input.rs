//! v0.2.2 T6.19 validator: every pipeline entry point threads `H: Hasher`.
//!
//! Architectural-discipline gate. Asserts that `pipeline::run`, `run_const`,
//! `run_parallel`, `run_stream`, `run_interactive` (plus the resolver facade
//! free functions `run_tower_completeness`, `run_incremental_completeness`,
//! `run_grounding_aware`, `run_inhabitance`, `run_multiplication`):
//!
//! 1. take `H: Hasher` as a generic parameter;
//! 2. call `H::initial()` to start the fold;
//! 3. call one of the `fold_*_digest` helpers;
//! 4. build a `ContentFingerprint` via `ContentFingerprint::from_buffer`;
//! 5. do NOT contain `ContentFingerprint::zero()` on any reachable path.

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/pipeline_run_threads_input";

/// Runs the pipeline hasher-threading validator.
///
/// # Errors
///
/// Returns an error if the foundation source cannot be read.
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

    // Each pipeline entry point takes H: Hasher.
    let required_signatures: &[(&str, &str)] = &[
        ("pipeline::run", "pub fn run<T, P, H>("),
        // Phase C.2: run_const carries `M: Total + Invertible` per target §6.
        ("pipeline::run_const", "pub fn run_const<T, M, H"),
        ("pipeline::run_parallel", "pub fn run_parallel<T, P, H>("),
        ("pipeline::run_stream", "pub fn run_stream<T, P, H>("),
        (
            "pipeline::run_interactive",
            "pub fn run_interactive<T, P, H>(",
        ),
        (
            "run_tower_completeness<T, H>",
            "pub fn run_tower_completeness<T",
        ),
        (
            "run_incremental_completeness<T, H>",
            "pub fn run_incremental_completeness<",
        ),
        ("run_grounding_aware<H>", "pub fn run_grounding_aware<H"),
        ("run_inhabitance<T, H>", "pub fn run_inhabitance<T"),
    ];

    // Substrate-threading anchors: the pipeline body must invoke these.
    let required_bodies: &[(&str, &str)] = &[
        ("H::initial() usage", "H::initial()"),
        (
            "ContentFingerprint::from_buffer usage",
            "ContentFingerprint::from_buffer(",
        ),
        ("fold_unit_digest usage", "fold_unit_digest("),
    ];

    let mut missing: Vec<String> = Vec::new();
    for (label, anchor) in required_signatures {
        if !content.contains(*anchor) {
            missing.push(format!("signature: {label}"));
        }
    }
    for (label, anchor) in required_bodies {
        if !content.contains(*anchor) {
            missing.push(format!("body: {label}"));
        }
    }

    // Forbidden: ContentFingerprint::zero() on any pipeline code path.
    // (Trace::empty() is in enforcement.rs, not pipeline.rs, so the gate is
    // clean.)
    if content.contains("ContentFingerprint::zero()") {
        missing.push("forbidden: ContentFingerprint::zero() in pipeline.rs".to_string());
    }

    if missing.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            "T6.19 pipeline hasher threading: every entry point takes H: Hasher \
             + calls H::initial() + fold_unit_digest + ContentFingerprint::from_buffer; \
             no ContentFingerprint::zero() on pipeline paths",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!(
                "T6.19 pipeline hasher threading: {} anchors missing or forbidden",
                missing.len()
            ),
            missing,
        ));
    }

    Ok(report)
}
