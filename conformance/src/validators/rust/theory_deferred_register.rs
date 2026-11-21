//! Phase 6 + Phase 7d conformance check: bijection between
//! `Path4TheoryDeferred` classifications and rows in
//! `docs/theory_deferred.md`, AND every Path-4 class has a generated
//! Null stub in `foundation/src/**/*.rs` carrying the exact
//! `#[doc(hidden)]` + THEORY-DEFERRED banner combination.
//!
//! Fails when:
//! - A Path-4 class has no register row (missing row).
//! - A register row names a class that's not Path-4 (dangling row).
//! - A register row has an empty research-question column.
//! - A Path-4 class has no `pub struct Null{Name}<H: HostTypes>` in
//!   the generated source (Phase 7d regression).
//! - A Path-4 Null stub is missing `#[doc(hidden)]` or the
//!   THEORY-DEFERRED marker in the 400-char window before the
//!   struct declaration (banner drift).

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/theory_deferred_register";

const BANNER_MARKER: &str =
    "THEORY-DEFERRED \\u{2014} not a valid implementation; see [docs/theory_deferred.md].";

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

    // Phase 7d banner check: bind each Path-4 class to its own
    // namespace source file, not to a concatenation of the repository.
    // Local names collide: reduction:Epoch is Path 1 and correctly lacks
    // the banner required for stream:Epoch (Path 4).
    let mut missing_stubs: Vec<String> = Vec::new();
    let mut missing_banners: Vec<String> = Vec::new();
    for qualified in &path4 {
        let Some((namespace, name)) = qualified.split_once(':') else {
            missing_stubs.push(qualified.clone());
            continue;
        };
        let Some(source) = load_foundation_namespace_source(workspace, namespace) else {
            missing_stubs.push(qualified.clone());
            continue;
        };
        let decl = format!("pub struct Null{name}<H: HostTypes>");
        match source.find(&decl) {
            None => missing_stubs.push(qualified.clone()),
            Some(pos) => {
                let start = pos.saturating_sub(400);
                let window = &source[start..pos];
                if !window.contains("#[doc(hidden)]") || !window.contains(BANNER_MARKER) {
                    missing_banners.push(qualified.clone());
                }
            }
        }
    }

    if missing_rows.is_empty()
        && dangling_rows.is_empty()
        && empty_rq.is_empty()
        && missing_stubs.is_empty()
        && missing_banners.is_empty()
    {
        report.push(TestResult::pass(
            VALIDATOR,
            format!(
                "Theory-deferred register parity: {} Path-4 classes match {} register rows \
                 (all stubs carry `#[doc(hidden)]` + THEORY-DEFERRED banner)",
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
    if !missing_stubs.is_empty() {
        msg.push_str(&format!(
            "\n  Path-4 classes with no `pub struct Null{{Name}}<H: HostTypes>` \
             (Phase 7d regression, {}):",
            missing_stubs.len()
        ));
        for k in missing_stubs.iter().take(10) {
            msg.push_str(&format!("\n    - {k}"));
        }
    }
    if !missing_banners.is_empty() {
        msg.push_str(&format!(
            "\n  Path-4 Null stubs missing `#[doc(hidden)]` / THEORY-DEFERRED banner ({}):",
            missing_banners.len()
        ));
        for k in missing_banners.iter().take(10) {
            msg.push_str(&format!("\n    - {k}"));
        }
    }
    report.push(TestResult::fail(VALIDATOR, msg));
    Ok(report)
}

/// Loads the generated source for exactly one ontology namespace.
///
/// Names are not globally unique. Searching all generated files together
/// can find a Path-1 Null stub before the corresponding Path-4 stub.
fn load_foundation_namespace_source(workspace: &Path, namespace: &str) -> Option<String> {
    let root = workspace.join("foundation/src");
    for subdir in ["bridge", "kernel", "user"] {
        let candidate = root.join(subdir).join(format!("{namespace}.rs"));
        if candidate.is_file() {
            return std::fs::read_to_string(candidate).ok();
        }
    }
    None
}
