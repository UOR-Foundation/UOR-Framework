//! v0.2.2 Phase E validator: bridge namespace completion.
//!
//! Asserts that the foundation crate exposes the sealed bridge-namespace
//! surface introduced by Phase E: the Query/Coordinate/BindingQuery/Partition/
//! PartitionComponent/Trace/TraceEvent types, the six BaseMetric accessors on
//! `Grounded<T, Tag>`, the `MAX_BETTI_DIMENSION` / `JACOBIAN_MAX_SITES`
//! constants, the `SigmaValue` and `JacobianMetric<L>` sealed carriers, the
//! `HomologyClass<N>` / `CohomologyClass<N>` parametric classes, the
//! `Derivation::replay` accessor, and the `InteractionDeclarationBuilder`
//! entry point.

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/bridge_namespace_completion";

/// Runs the bridge namespace completion check.
///
/// # Errors
///
/// Returns an error if the foundation source file cannot be read.
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
        // Constants.
        (
            "MAX_BETTI_DIMENSION constant",
            "pub const MAX_BETTI_DIMENSION: usize = 8;",
        ),
        (
            "JACOBIAN_MAX_SITES constant",
            "pub const JACOBIAN_MAX_SITES: usize = 8;",
        ),
        ("TRACE_MAX_EVENTS constant", "pub const TRACE_MAX_EVENTS"),
        // Sealed BaseMetric carriers.
        ("SigmaValue sealed type", "pub struct SigmaValue"),
        (
            "JacobianMetric<L> sealed type",
            "pub struct JacobianMetric<L>",
        ),
        ("PartitionComponent enum", "pub enum PartitionComponent"),
        // Bridge surface.
        ("Query sealed type", "pub struct Query"),
        ("Coordinate<L> sealed type", "pub struct Coordinate<L>"),
        ("BindingQuery sealed type", "pub struct BindingQuery"),
        ("Partition sealed type", "pub struct Partition"),
        ("TraceEvent sealed type", "pub struct TraceEvent"),
        ("Trace sealed type", "pub struct Trace"),
        (
            "HomologyClass<N>",
            "pub struct HomologyClass<const N: usize>",
        ),
        (
            "CohomologyClass<N>",
            "pub struct CohomologyClass<const N: usize>",
        ),
        (
            "InteractionDeclarationBuilder",
            "pub struct InteractionDeclarationBuilder",
        ),
        // Phase A.4: sealed BaseMetric newtype carriers.
        ("DDeltaMetric sealed type", "pub struct DDeltaMetric"),
        ("EulerMetric sealed type", "pub struct EulerMetric"),
        ("ResidualMetric sealed type", "pub struct ResidualMetric"),
        ("BettiMetric sealed type", "pub struct BettiMetric"),
        // Phase A.3: sealed stratum newtype.
        ("Stratum<L> sealed type", "pub struct Stratum<L>"),
        // Six BaseMetric accessors on Grounded now return sealed newtypes.
        (
            "Grounded::d_delta accessor",
            "pub const fn d_delta(&self) -> DDeltaMetric",
        ),
        (
            "Grounded::sigma accessor",
            "pub fn sigma(&self) -> SigmaValue",
        ),
        // v0.2.2 T2.6 (cleanup): BaseMetric field storage anchors.
        ("Grounded::sigma_ppm field", "sigma_ppm: u32"),
        (
            "Grounded::jacobian_entries field",
            "jacobian_entries: [i64; JACOBIAN_MAX_SITES]",
        ),
        (
            "Grounded::betti_numbers field",
            "betti_numbers: [u32; MAX_BETTI_DIMENSION]",
        ),
        (
            "Grounded::jacobian accessor",
            "pub fn jacobian(&self) -> JacobianMetric<T>",
        ),
        (
            "Grounded::betti accessor",
            "pub const fn betti(&self) -> BettiMetric",
        ),
        (
            "Grounded::euler accessor",
            "pub const fn euler(&self) -> EulerMetric",
        ),
        (
            "Grounded::residual accessor",
            "pub const fn residual(&self) -> ResidualMetric",
        ),
        // Phase A.1: uor_time accessors on Grounded and Certified.
        (
            "Grounded::uor_time accessor",
            "pub const fn uor_time(&self) -> UorTime",
        ),
        (
            "Grounded::triad accessor",
            "pub const fn triad(&self) -> Triad<T>",
        ),
        // Derivation::replay accessor (v0.2.2 T6.12: no longer const fn;
        // seeds from `content_fingerprint` bytes at runtime).
        (
            "Derivation::replay accessor",
            "pub fn replay(&self) -> Trace",
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
            "Phase E bridge namespace completion: MAX_BETTI_DIMENSION, \
             SigmaValue, JacobianMetric<L>, Query/Coordinate/BindingQuery/\
             Partition/Trace/TraceEvent/HomologyClass/CohomologyClass, six \
             BaseMetric accessors on Grounded, Derivation::replay, and \
             InteractionDeclarationBuilder all present",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!(
                "Phase E bridge namespace completion has {} missing anchors",
                missing.len()
            ),
            missing,
        ));
    }

    Ok(report)
}
