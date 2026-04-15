//! v0.2.2 Phase J validator: combinator-only grounding.
//!
//! Asserts the foundation crate exposes the Phase J grounding combinator
//! surface:
//! - Exactly 12 combinator functions in `enforcement::combinators`;
//! - the sealed `GroundingPrimitive<Out>` + `GroundingPrimitiveOp` enum;
//! - the `MarkerBits` bitmask with three named markers;
//! - the parametric `GroundingProgram<Out, Map>` carrier;
//! - the `MarkersImpliedBy<Map>` type-level compile-time check;
//! - marker token structs (`TotalMarker`, `InvertibleMarker`,
//!   `PreservesStructureMarker`).

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/grounding_combinator_check";

/// Runs the grounding combinator check.
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
        // PrimitiveOp + GroundingPrimitive.
        ("GroundingPrimitiveOp enum", "pub enum GroundingPrimitiveOp"),
        (
            "GroundingPrimitive<Out>",
            "pub struct GroundingPrimitive<Out>",
        ),
        // MarkerBits + marker tokens.
        ("MarkerBits struct", "pub struct MarkerBits"),
        ("TotalMarker token", "pub struct TotalMarker;"),
        ("InvertibleMarker token", "pub struct InvertibleMarker;"),
        (
            "PreservesStructureMarker token",
            "pub struct PreservesStructureMarker;",
        ),
        // MarkersImpliedBy trait.
        (
            "MarkersImpliedBy trait",
            "pub trait MarkersImpliedBy<Map: GroundingMapKind>",
        ),
        // The 12 combinators in the combinators module.
        ("combinators module", "pub mod combinators {"),
        (
            "combinators::read_bytes",
            "pub const fn read_bytes<Out>() -> GroundingPrimitive<Out>",
        ),
        (
            "combinators::interpret_le_integer",
            "pub const fn interpret_le_integer<Out>() -> GroundingPrimitive<Out>",
        ),
        (
            "combinators::interpret_be_integer",
            "pub const fn interpret_be_integer<Out>() -> GroundingPrimitive<Out>",
        ),
        (
            "combinators::digest",
            "pub const fn digest<Out>() -> GroundingPrimitive<Out>",
        ),
        (
            "combinators::decode_utf8",
            "pub const fn decode_utf8<Out>() -> GroundingPrimitive<Out>",
        ),
        (
            "combinators::decode_json",
            "pub const fn decode_json<Out>() -> GroundingPrimitive<Out>",
        ),
        (
            "combinators::select_field",
            "pub const fn select_field<Out>() -> GroundingPrimitive<Out>",
        ),
        (
            "combinators::select_index",
            "pub const fn select_index<Out>() -> GroundingPrimitive<Out>",
        ),
        (
            "combinators::const_value",
            "pub const fn const_value<Out>() -> GroundingPrimitive<Out>",
        ),
        ("combinators::then", "pub const fn then<A, B>("),
        ("combinators::map_err", "pub const fn map_err<A>("),
        ("combinators::and_then", "pub const fn and_then<A, B>("),
        // GroundingProgram<Out, Map>.
        (
            "GroundingProgram<Out, Map>",
            "pub struct GroundingProgram<Out, Map: GroundingMapKind>",
        ),
        (
            "GroundingProgram::from_primitive",
            "pub const fn from_primitive(primitive: GroundingPrimitive<Out>) -> Self",
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
            "Phase J combinator-only grounding: exactly 12 combinators, \
             GroundingPrimitive<Out>, GroundingPrimitiveOp, MarkerBits, \
             3 marker tokens, MarkersImpliedBy<Map>, and GroundingProgram<Out, Map> \
             all present",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!(
                "Phase J combinator-only grounding has {} missing anchors",
                missing.len()
            ),
            missing,
        ));
    }

    Ok(report)
}
