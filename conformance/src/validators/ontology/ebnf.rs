//! EBNF grammar artifact validator (Amendment 42).
//!
//! Validates that the generated `uor.term.ebnf` file is well-formed and
//! contains the expected ontology-derived content.

use std::path::Path;

use anyhow::{Context, Result};

use crate::report::{ConformanceReport, TestResult};

/// Validates the EBNF grammar artifact for structural correctness.
///
/// Checks that `uor.term.ebnf` exists, is non-empty, contains balanced
/// EBNF comments, and includes all expected operations, quantum levels,
/// rewrite rules, and the specification version string.
///
/// # Errors
///
/// Returns an error if the artifact file cannot be read.
pub fn validate(artifacts: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();
    let validator = "ontology/ebnf";

    let ebnf_path = artifacts.join("uor.term.ebnf");
    if !ebnf_path.exists() {
        report.push(TestResult::fail(
            validator,
            "uor.term.ebnf not found in artifacts directory",
        ));
        return Ok(report);
    }

    let content = std::fs::read_to_string(&ebnf_path)
        .with_context(|| format!("Failed to read {}", ebnf_path.display()))?;

    let mut issues: Vec<String> = Vec::new();

    // Structure
    if content.trim().is_empty() {
        issues.push("File is empty".to_string());
    }
    if !content.contains("::=") {
        issues.push("No ::= definitions found".to_string());
    }

    // Balanced comments
    let opens = content.matches("(*").count();
    let closes = content.matches("*)").count();
    if opens != closes {
        issues.push(format!(
            "Unbalanced comments: {} opens vs {} closes",
            opens, closes
        ));
    }

    // All 10 PrimitiveOp operations
    for op in &[
        "neg", "bnot", "succ", "pred", "add", "sub", "mul", "xor", "and", "or",
    ] {
        let terminal = format!("\"{}\"", op);
        if !content.contains(&terminal) {
            issues.push(format!("Missing operation terminal: {}", op));
        }
    }

    // All 4 quantum levels
    for level in &["Q0", "Q1", "Q2", "Q3"] {
        let terminal = format!("\"{}\"", level);
        if !content.contains(&terminal) {
            issues.push(format!("Missing quantum level: {}", level));
        }
    }

    // Version string from live spec
    let ontology = uor_ontology::Ontology::full();
    if !content.contains(ontology.version) {
        issues.push(format!("Missing version string: {}", ontology.version));
    }

    // All 6 rewrite rules
    for rule in &[
        "CriticalIdentity",
        "Involution",
        "Associativity",
        "Commutativity",
        "IdentityElement",
        "Normalization",
    ] {
        if !content.contains(rule) {
            issues.push(format!("Missing rewrite rule: {}", rule));
        }
    }

    if issues.is_empty() {
        report.push(TestResult::pass(
            validator,
            format!(
                "uor.term.ebnf is well-formed ({} bytes, {} definitions, {} comments)",
                content.len(),
                content.matches("::=").count(),
                opens
            ),
        ));
    } else {
        report.push(TestResult::fail_with_details(
            validator,
            "uor.term.ebnf has structural issues",
            issues,
        ));
    }

    Ok(report)
}
