//! Phase 6 conformance check: bijection between `Path4TheoryDeferred`
//! classifications and rows in `docs/theory_deferred.md`.
//!
//! Fails when:
//! - A Path-4 class has no register row (missing row).
//! - A register row names a class that's not Path-4 (dangling row).
//! - A register row has an empty research-question column.

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/theory_deferred_register";

/// Runs the Phase 6 theory-deferred-register validation.
///
/// # Errors
///
/// Returns an error if the ontology cannot be read.
pub fn validate(workspace: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    // Collect Path-4 classifications.
    let ontology = uor_ontology::Ontology::full();
    let entries = uor_codegen::classification::classify_all(ontology);
    let mut path4: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for e in &entries {
        if matches!(
            e.path_kind,
            uor_codegen::classification::PathKind::Path4TheoryDeferred
        ) {
            // The register uses the `{prefix}:{LocalName}` canonical form to
            // stay independent of any single namespace IRI scheme.
            path4.insert(format!("{}:{}", e.namespace, e.class_local));
        }
    }

    // Parse docs/theory_deferred.md rows.
    let doc_path = workspace.join("docs/theory_deferred.md");
    let source = match std::fs::read_to_string(&doc_path) {
        Ok(s) => s,
        Err(e) => {
            report.push(TestResult::fail(
                VALIDATOR,
                format!("cannot read {}: {e}", doc_path.display()),
            ));
            return Ok(report);
        }
    };
    let mut rows: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let mut in_table = false;
    let mut header_passed = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if !in_table {
            if trimmed.starts_with("| Class IRI ") {
                in_table = true;
            }
            continue;
        }
        if trimmed.is_empty() || !trimmed.starts_with('|') {
            // End of table.
            break;
        }
        if trimmed.starts_with("|---") {
            header_passed = true;
            continue;
        }
        if !header_passed {
            continue;
        }
        // Expected: | `foo:Bar` | `foo` | research question |
        let cells: Vec<&str> = trimmed
            .trim_start_matches('|')
            .trim_end_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        if cells.len() < 3 {
            continue;
        }
        let iri_cell = cells[0].trim_matches('`').to_string();
        let rq = cells[2].to_string();
        rows.insert(iri_cell, rq);
    }

    let mut missing_rows: Vec<&String> = path4.iter().filter(|k| !rows.contains_key(*k)).collect();
    missing_rows.sort();
    let mut dangling_rows: Vec<&String> = rows.keys().filter(|k| !path4.contains(*k)).collect();
    dangling_rows.sort();
    let mut empty_rq: Vec<&String> = rows
        .iter()
        .filter(|(_, rq)| rq.is_empty())
        .map(|(k, _)| k)
        .collect();
    empty_rq.sort();

    if missing_rows.is_empty() && dangling_rows.is_empty() && empty_rq.is_empty() {
        report.push(TestResult::pass(
            VALIDATOR,
            format!(
                "Theory-deferred register parity: {} Path-4 classes match {} register rows",
                path4.len(),
                rows.len()
            ),
        ));
        return Ok(report);
    }

    let mut msg = String::from("Theory-deferred register drift:");
    if !missing_rows.is_empty() {
        msg.push_str(&format!(
            "\n  missing register rows ({}):",
            missing_rows.len()
        ));
        for k in missing_rows.iter().take(10) {
            msg.push_str(&format!("\n    - {k}"));
        }
    }
    if !dangling_rows.is_empty() {
        msg.push_str(&format!(
            "\n  dangling register rows ({}):",
            dangling_rows.len()
        ));
        for k in dangling_rows.iter().take(10) {
            msg.push_str(&format!("\n    - {k}"));
        }
    }
    if !empty_rq.is_empty() {
        msg.push_str(&format!(
            "\n  empty research-question columns ({}):",
            empty_rq.len()
        ));
        for k in empty_rq.iter().take(10) {
            msg.push_str(&format!("\n    - {k}"));
        }
    }
    report.push(TestResult::fail(VALIDATOR, msg));
    Ok(report)
}
