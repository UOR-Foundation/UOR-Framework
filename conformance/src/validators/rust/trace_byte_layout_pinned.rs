//! v0.2.2 T6.21 validator: pin the Trace / digest byte layout.
//!
//! Asserts the byte-level interop contracts that `verify_trace` relies on
//! are stable:
//!
//! 1. `primitive_op_discriminant(PrimitiveOp) -> u8` matches the 0..=9 range;
//! 2. `certificate_kind_discriminant(CertificateKind) -> u8` matches 1..=21
//!    (5 Phase C kinds + 16 Phase D per-resolver kinds);
//! 3. `TRACE_REPLAY_FORMAT_VERSION = 2` (bumped from 1 when the Phase D
//!    per-resolver `CertificateKind` variants landed — the 1..=5 discriminant
//!    bytes are preserved, so older traces still decode correctly);
//! 4. the six byte-layout helpers exist: `fold_unit_digest`,
//!    `fold_parallel_digest`, `fold_stream_digest`, `fold_interaction_digest`,
//!    `fold_constraint_ref`, `fold_stream_step_digest`, `fold_interaction_step_digest`.

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/trace_byte_layout_pinned";

/// Runs the trace-byte-layout pin check.
///
/// # Errors
///
/// Returns an error if the foundation enforcement source cannot be read.
pub fn validate(workspace: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();
    let enforcement_path = workspace.join("foundation/src/enforcement.rs");
    let content = match std::fs::read_to_string(&enforcement_path) {
        Ok(c) => c,
        Err(e) => {
            report.push(TestResult::fail(
                VALIDATOR,
                format!("failed to read {}: {e}", enforcement_path.display()),
            ));
            return Ok(report);
        }
    };

    let required: &[(&str, &str)] = &[
        (
            "primitive_op_discriminant signature",
            "pub const fn primitive_op_discriminant(op: crate::PrimitiveOp) -> u8",
        ),
        (
            "certificate_kind_discriminant signature",
            "pub const fn certificate_kind_discriminant(kind: CertificateKind) -> u8",
        ),
        (
            "TRACE_REPLAY_FORMAT_VERSION = 2",
            "pub const TRACE_REPLAY_FORMAT_VERSION: u16 = 2",
        ),
        (
            "fold_unit_digest helper",
            "pub fn fold_unit_digest<H: Hasher>",
        ),
        (
            "fold_parallel_digest helper",
            "pub fn fold_parallel_digest<H: Hasher>",
        ),
        (
            "fold_stream_digest helper",
            "pub fn fold_stream_digest<H: Hasher>",
        ),
        (
            "fold_interaction_digest helper",
            "pub fn fold_interaction_digest<H: Hasher>",
        ),
        (
            "fold_constraint_ref helper",
            "pub fn fold_constraint_ref<H: Hasher>",
        ),
        (
            "fold_stream_step_digest helper",
            "pub fn fold_stream_step_digest<H: Hasher>",
        ),
        (
            "fold_interaction_step_digest helper",
            "pub fn fold_interaction_step_digest<H: Hasher>",
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
            "T6.21 trace byte layout: primitive/certificate discriminants, \
             TRACE_REPLAY_FORMAT_VERSION = 2, and 7 fold_*_digest helpers pinned",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!("T6.21 trace byte layout: {} anchors missing", missing.len()),
            missing,
        ));
    }

    Ok(report)
}
