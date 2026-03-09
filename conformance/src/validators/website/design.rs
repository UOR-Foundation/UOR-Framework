//! Website design system validator.
//!
//! Checks that the CSS design system is correctly implemented with custom properties,
//! kind badges, and a print stylesheet.

use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

/// Validates the website design system.
///
/// # Errors
///
/// Returns an error if artifact files cannot be read.
pub fn validate(artifacts: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    check_css_custom_properties(artifacts, &mut report)?;
    check_kind_badges(artifacts, &mut report)?;
    check_print_stylesheet(artifacts, &mut report)?;

    Ok(report)
}

/// CSS must define custom properties for all three space colors and the cert color.
fn check_css_custom_properties(artifacts: &Path, report: &mut ConformanceReport) -> Result<()> {
    let css_path = artifacts.join("css").join("style.css");
    if !css_path.exists() {
        report.push(TestResult::fail(
            "website/design/css-custom-properties",
            "css/style.css not found in generated website",
        ));
        return Ok(());
    }

    let css = std::fs::read_to_string(&css_path)?;
    let required = [
        "--color-kernel",
        "--color-bridge",
        "--color-user",
        "--color-cert",
    ];
    let missing: Vec<String> = required
        .iter()
        .filter(|&&prop| !css.contains(prop))
        .map(|&s| s.to_string())
        .collect();

    if missing.is_empty() {
        report.push(TestResult::pass(
            "website/design/css-custom-properties",
            "css/style.css defines all required CSS custom properties (kernel/bridge/user/cert)",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            "website/design/css-custom-properties",
            "css/style.css missing required CSS custom properties",
            missing,
        ));
    }

    Ok(())
}

/// search.html must contain badge class names for all ontology term kinds.
fn check_kind_badges(artifacts: &Path, report: &mut ConformanceReport) -> Result<()> {
    let search_path = artifacts.join("search.html");
    if !search_path.exists() {
        report.push(TestResult::fail(
            "website/design/kind-badges",
            "search.html not found in generated website",
        ));
        return Ok(());
    }

    let html = std::fs::read_to_string(&search_path)?;
    let required = ["badge-class", "badge-property", "badge-individual"];
    let missing: Vec<String> = required
        .iter()
        .filter(|&&badge| !html.contains(badge))
        .map(|&s| s.to_string())
        .collect();

    if missing.is_empty() {
        report.push(TestResult::pass(
            "website/design/kind-badges",
            "search.html contains kind badge class names (badge-class/property/individual)",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            "website/design/kind-badges",
            "search.html missing kind badge class names",
            missing,
        ));
    }

    Ok(())
}

/// CSS must contain an @media print block.
fn check_print_stylesheet(artifacts: &Path, report: &mut ConformanceReport) -> Result<()> {
    let css_path = artifacts.join("css").join("style.css");
    if !css_path.exists() {
        report.push(TestResult::fail(
            "website/design/print-stylesheet",
            "css/style.css not found in generated website",
        ));
        return Ok(());
    }

    let css = std::fs::read_to_string(&css_path)?;

    if css.contains("@media print") {
        report.push(TestResult::pass(
            "website/design/print-stylesheet",
            "css/style.css contains @media print block",
        ));
    } else {
        report.push(TestResult::fail(
            "website/design/print-stylesheet",
            "css/style.css missing @media print block",
        ));
    }

    Ok(())
}
