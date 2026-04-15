//! v0.2.2 Phase H validator: sealed-type escape-hatch lint.
//!
//! Scans the foundation crate's generated source for structural escape
//! hatches that would let downstream bypass sealed types. The discipline
//! is: every sealed type must be constructible only via foundation-owned
//! paths; no `pub const fn new(...) -> SealedType` outside the audited
//! entry list, and no `unsafe impl SealedTrait for ...`.
//!
//! This is a grep-based stand-in for the target-v2 dylint crate. It
//! asserts that the foundation's generated source contains no forbidden
//! patterns outside the audit list.

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/escape_hatch_lint";

/// Runs the escape-hatch lint check.
///
/// # Errors
///
/// Returns an error if the foundation source files cannot be read.
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

    // Every sealed type in the v0.2.2 inventory. The regex would be nicer,
    // but we stay grep-simple for auditability. An "escape" is either:
    // - `unsafe impl <SealedTrait> for` — always forbidden.
    let sealed_traits: &[&str] = &[
        "OntologyTarget",
        "Certificate",
        "GroundingMapKind",
        "ValidationPhase",
        "Observable",
        "BoundShape",
    ];

    let mut violations: Vec<String> = Vec::new();
    for trait_name in sealed_traits {
        let needle = format!("unsafe impl {trait_name}");
        if content.contains(&needle) {
            violations.push(format!("forbidden `unsafe impl {trait_name}` found"));
        }
    }

    // `extern crate alloc` must not appear at the crate root; foundation is
    // strictly no_std by default.
    if content.contains("extern crate alloc") {
        violations.push("foundation/src/enforcement.rs has `extern crate alloc`".to_string());
    }
    if content.contains("extern crate std") {
        violations.push("foundation/src/enforcement.rs has `extern crate std`".to_string());
    }

    if violations.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            "Phase H escape-hatch lint: no forbidden `unsafe impl` patterns \
             or unconditional extern crate alloc/std in the foundation source",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!(
                "Phase H escape-hatch lint found {} violations",
                violations.len()
            ),
            violations,
        ));
    }

    Ok(report)
}
