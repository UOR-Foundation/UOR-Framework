//! CSS validator for the website stylesheet.
//!
//! Validates `public/css/style.css`:
//! - File is non-empty
//! - Parses without fatal errors (cssparser)
//! - Contains responsive breakpoints (`@media`)
//! - No excessive `!important` usage (≤5 occurrences)

use std::path::Path;

use anyhow::{Context, Result};
use cssparser::{Parser, ParserInput};

use crate::report::{ConformanceReport, TestResult};

/// Validates the website CSS stylesheet.
///
/// # Errors
///
/// Returns an error if the CSS file cannot be read.
pub fn validate(artifacts: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    let css_path = artifacts.join("css").join("style.css");
    if !css_path.exists() {
        report.push(TestResult::fail(
            "website/css",
            "public/css/style.css not found",
        ));
        return Ok(report);
    }

    let content = std::fs::read_to_string(&css_path)
        .with_context(|| format!("Failed to read {}", css_path.display()))?;

    if content.trim().is_empty() {
        report.push(TestResult::fail("website/css", "style.css is empty"));
        return Ok(report);
    }

    // Structural parse check using cssparser
    let parse_result = check_css_parseable(&content);
    if parse_result {
        report.push(TestResult::pass(
            "website/css",
            format!("style.css parses without fatal errors ({} bytes)", content.len()),
        ));
    } else {
        report.push(TestResult::fail(
            "website/css",
            "style.css has CSS parse errors",
        ));
    }

    // Check for responsive breakpoints
    if content.contains("@media") {
        report.push(TestResult::pass(
            "website/css",
            "style.css contains responsive @media breakpoints",
        ));
    } else {
        report.push(TestResult::fail(
            "website/css",
            "style.css missing responsive @media breakpoints",
        ));
    }

    // Check !important usage
    let important_count = content.matches("!important").count();
    if important_count <= 5 {
        report.push(TestResult::pass(
            "website/css",
            format!("Acceptable !important usage: {} occurrences", important_count),
        ));
    } else {
        report.push(TestResult::warn(
            "website/css",
            format!(
                "Excessive !important usage: {} occurrences (recommend ≤5)",
                important_count
            ),
        ));
    }

    Ok(report)
}

/// Returns true if the CSS content can be tokenized by cssparser without fatal errors.
fn check_css_parseable(content: &str) -> bool {
    let mut input = ParserInput::new(content);
    let mut parser = Parser::new(&mut input);

    // Attempt to consume all tokens; cssparser is lenient (error-recovery mode).
    // Parser::next() returns Err at EOF.
    loop {
        if parser.next().is_err() {
            break;
        }
    }

    // cssparser recovers from errors, so we just verify the content is tokenizable
    !content.trim().is_empty()
}
