//! Target-doc cross-ref A.1: every sealed-type row in
//! `external/uor-foundation-target-v2.md` §2 appears in
//! `rust/escape_hatch_lint`'s `SEALED_TYPES`.
//!
//! The lint cannot attest its own coverage; this validator parses §2's
//! markdown table and asserts each named Rust type appears in the lint
//! source. Fails with the list of types the lint is missing.

use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/target_doc/sealed_type_coverage";

/// Runs the §2 sealed-type cross-reference check.
///
/// # Errors
///
/// Returns an error if the target document or the lint source cannot be read.
pub fn validate(workspace: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();
    let target_path = workspace.join("external/uor-foundation-target-v2.md");
    let lint_path = workspace.join("conformance/src/validators/rust/escape_hatch_lint.rs");

    let target_doc = match fs::read_to_string(&target_path) {
        Ok(s) => s,
        Err(e) => {
            report.push(TestResult::fail(
                VALIDATOR,
                format!("failed to read {}: {e}", target_path.display()),
            ));
            return Ok(report);
        }
    };
    let lint_src = match fs::read_to_string(&lint_path) {
        Ok(s) => s,
        Err(e) => {
            report.push(TestResult::fail(
                VALIDATOR,
                format!("failed to read {}: {e}", lint_path.display()),
            ));
            return Ok(report);
        }
    };

    let expected = extract_section2_types(&target_doc);
    let missing: Vec<String> = expected
        .iter()
        .filter(|ty| !lint_mentions_type(&lint_src, ty))
        .cloned()
        .collect();

    if missing.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            format!(
                "target §2 sealed-type table: all {} entries covered by escape_hatch_lint::SEALED_TYPES",
                expected.len()
            ),
        ));
    } else {
        report.push(TestResult::fail_with_details(
            VALIDATOR,
            format!(
                "target §2 sealed-type table: {} entries missing from escape_hatch_lint::SEALED_TYPES",
                missing.len()
            ),
            missing,
        ));
    }
    Ok(report)
}

/// Parse §2's markdown table. Each row starts with `|`; the leftmost
/// cell carries the sealed-type name possibly wrapped in backticks, with
/// generic parameters. We extract just the bare identifier (before any
/// `<` or space).
fn extract_section2_types(doc: &str) -> Vec<String> {
    let mut in_section2 = false;
    let mut table_started = false;
    let mut out = Vec::new();
    for line in doc.lines() {
        if line.starts_with("## 2.") || line.starts_with("## 2 ") {
            in_section2 = true;
            continue;
        }
        if in_section2 && (line.starts_with("## 3.") || line.starts_with("## 3 ")) {
            break;
        }
        if !in_section2 {
            continue;
        }
        let trimmed = line.trim_start();
        if !trimmed.starts_with('|') {
            continue;
        }
        // Skip the trait table (labeled "Sealed trait"); we only want the
        // sealed-type table (labeled "Sealed type"). Header contains
        // "Sealed type" or "Sealed trait"; separator row is the `---`
        // line. Pick the first table.
        if trimmed.contains("Sealed type") {
            table_started = true;
            continue;
        }
        if trimmed.contains("Sealed trait") {
            // Entered the traits table — stop collecting types.
            break;
        }
        if !table_started {
            continue;
        }
        // Separator row: e.g. `|---|---|---|`
        if trimmed
            .chars()
            .all(|c| c == '|' || c == '-' || c == ' ' || c == ':')
        {
            continue;
        }
        // Extract first cell.
        if let Some(first_cell) = trimmed[1..].split('|').next() {
            let raw = first_cell.trim().trim_matches('`');
            // Skip "individuals" markers — rows like
            // "operad:OperadComposition individuals" are ontology-level;
            // we only want Rust types.
            if raw.contains(" individuals") || raw.contains(':') {
                continue;
            }
            // Strip generic params and trailing annotations.
            let name: String = raw
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if !name.is_empty() && name.chars().next().is_some_and(|c| c.is_uppercase()) {
                out.push(name);
            }
        }
    }
    // De-dup (shouldn't be needed; defensive).
    out.sort();
    out.dedup();
    out
}

/// Does the escape-hatch lint source mention the type name as a quoted
/// string entry (inside `SEALED_TYPES`)? Scan for the literal `"<Name>"`.
fn lint_mentions_type(lint_src: &str, type_name: &str) -> bool {
    let quoted = format!("\"{type_name}\"");
    lint_src.contains(&quoted)
}
