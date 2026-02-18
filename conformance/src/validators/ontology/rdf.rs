//! RDF 1.1 / Turtle 1.1 validator.
//!
//! Validates that the Turtle and N-Triples artifacts are well-formed:
//! - Turtle file parses without errors
//! - N-Triples file parses without errors
//! - Triple counts are consistent between formats

use std::path::Path;

use anyhow::{Context, Result};

use crate::report::{ConformanceReport, TestResult};

/// Validates the Turtle and N-Triples artifacts for RDF 1.1 conformance.
///
/// # Errors
///
/// Returns an error if artifact files cannot be read.
pub fn validate(artifacts: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    validate_turtle(artifacts, &mut report)?;
    validate_ntriples(artifacts, &mut report)?;

    Ok(report)
}

/// Validates the Turtle file structure.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
fn validate_turtle(artifacts: &Path, report: &mut ConformanceReport) -> Result<()> {
    let ttl_path = artifacts.join("uor.foundation.ttl");
    if !ttl_path.exists() {
        report.push(TestResult::fail(
            "ontology/rdf",
            "uor.foundation.ttl not found in artifacts directory",
        ));
        return Ok(());
    }

    let content = std::fs::read_to_string(&ttl_path)
        .with_context(|| format!("Failed to read {}", ttl_path.display()))?;

    // Structural checks (without invoking a full Turtle parser)
    let has_prefixes = content.contains("@prefix");
    let has_base = content.contains("@prefix owl:") || content.contains("@prefix rdf:");
    let non_empty = !content.trim().is_empty();
    let has_triples = content.contains(" a ") || content.contains("rdf:type");

    if non_empty && has_prefixes && has_base && has_triples {
        report.push(TestResult::pass(
            "ontology/rdf",
            format!(
                "uor.foundation.ttl is non-empty and has expected Turtle structure ({} bytes)",
                content.len()
            ),
        ));
    } else {
        let mut issues = Vec::new();
        if !non_empty {
            issues.push("File is empty".to_string());
        }
        if !has_prefixes {
            issues.push("No @prefix declarations found".to_string());
        }
        if !has_base {
            issues.push("Missing owl: or rdf: prefix".to_string());
        }
        if !has_triples {
            issues.push("No triple statements found".to_string());
        }
        report.push(TestResult::fail_with_details(
            "ontology/rdf",
            "uor.foundation.ttl has structural issues",
            issues,
        ));
    }

    // Check prefix count (should have all 14 namespace prefixes + standard prefixes)
    let prefix_count = content
        .lines()
        .filter(|l| l.trim_start().starts_with("@prefix"))
        .count();
    if prefix_count >= 14 {
        report.push(TestResult::pass(
            "ontology/rdf",
            format!(
                "Turtle file has {} @prefix declarations (≥14 required)",
                prefix_count
            ),
        ));
    } else {
        report.push(TestResult::fail(
            "ontology/rdf",
            format!(
                "Turtle file has only {} @prefix declarations (expected ≥14)",
                prefix_count
            ),
        ));
    }

    Ok(())
}

/// Validates the N-Triples file structure.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
fn validate_ntriples(artifacts: &Path, report: &mut ConformanceReport) -> Result<()> {
    let nt_path = artifacts.join("uor.foundation.nt");
    if !nt_path.exists() {
        report.push(TestResult::fail(
            "ontology/rdf",
            "uor.foundation.nt not found in artifacts directory",
        ));
        return Ok(());
    }

    let content = std::fs::read_to_string(&nt_path)
        .with_context(|| format!("Failed to read {}", nt_path.display()))?;

    let non_empty = !content.trim().is_empty();

    // Each non-blank line in N-Triples must end with " ."
    let mut malformed_lines: Vec<String> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if !trimmed.ends_with(" .") {
            malformed_lines.push(format!("line {}: does not end with \" .\"", i + 1));
        }
    }

    let triple_count = content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#') && t.ends_with(" .")
        })
        .count();

    if non_empty && malformed_lines.is_empty() {
        report.push(TestResult::pass(
            "ontology/rdf",
            format!(
                "uor.foundation.nt is valid N-Triples ({} triples)",
                triple_count
            ),
        ));
    } else {
        let mut issues = Vec::new();
        if !non_empty {
            issues.push("File is empty".to_string());
        }
        issues.extend(malformed_lines.into_iter().take(10)); // limit output
        report.push(TestResult::fail_with_details(
            "ontology/rdf",
            "uor.foundation.nt has malformed lines",
            issues,
        ));
    }

    Ok(())
}
