//! Phase 11c conformance gate: `foundation/src/blanket_impls.rs` exists,
//! starts with the `// @codegen-exempt` banner, and contains every
//! Path-3-allow-listed blanket impl plus the required supertrait
//! closures (Observable / ThermoObservable on `Validated<T, Phase>`).

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/blanket_impls_exempt";

/// Phase 11a Path-3 traits whose blanket impls must appear in
/// `foundation/src/blanket_impls.rs`. Mirrors
/// `uor_codegen::classification::PATH3_ALLOW_LIST` (5 entries) plus
/// the two supertraits (`Observable`, `ThermoObservable`).
const REQUIRED_BLANKET_IMPLS: &[&str] = &[
    "impl<T, Phase, H> Observable<H> for Validated<T, Phase>",
    "impl<T, Phase, H> ThermoObservable<H> for Validated<T, Phase>",
    "impl<T, Phase, H> LandauerBudget<H> for Validated<T, Phase>",
    "impl<T, Phase, H> JacobianObservable<H> for Validated<T, Phase>",
    "impl<T, Phase, H> CarryDepthObservable<H> for Validated<T, Phase>",
    "impl<T, Phase, H> DerivationDepthObservable<H> for Validated<T, Phase>",
    "impl<T, Phase, H> FreeRankObservable<H> for Validated<T, Phase>",
];

/// Runs the Phase 11c blanket_impls_exempt validator.
///
/// # Errors
///
/// Returns an error if `foundation/src/blanket_impls.rs` cannot be read.
pub fn validate(workspace: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    let path = workspace.join("foundation/src/blanket_impls.rs");
    let body = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            report.push(TestResult::fail(
                VALIDATOR,
                format!(
                    "cannot read foundation/src/blanket_impls.rs: {e} \
                     (Phase 11e requires the hand-written file)"
                ),
            ));
            return Ok(report);
        }
    };

    let mut failures: Vec<String> = Vec::new();

    // Banner — first non-blank line must be `// @codegen-exempt`.
    let banner_ok = body
        .lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim().starts_with("// @codegen-exempt"))
        .unwrap_or(false);
    if !banner_ok {
        failures.push(
            "first non-blank line must be `// @codegen-exempt` so codegen \
             preserves the file across `uor-crate` runs"
                .to_string(),
        );
    }

    for needle in REQUIRED_BLANKET_IMPLS {
        if !body.contains(needle) {
            failures.push(format!("missing blanket impl: `{needle}`"));
        }
    }

    if failures.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            format!(
                "Phase 11 blanket impls: @codegen-exempt banner present; \
                 all {} required impls (Observable/ThermoObservable + 5 \
                 Path-3 leaf traits) emit on `Validated<T, Phase>`",
                REQUIRED_BLANKET_IMPLS.len()
            ),
        ));
    } else {
        let mut summary = format!(
            "Phase 11 blanket_impls.rs drift: {} issue(s):",
            failures.len()
        );
        for f in &failures {
            summary.push_str("\n       - ");
            summary.push_str(f);
        }
        report.push(TestResult::fail(VALIDATOR, summary));
    }

    Ok(report)
}
