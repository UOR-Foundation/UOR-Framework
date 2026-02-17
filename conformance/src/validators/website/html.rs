//! HTML5 structural validator for the generated website.
//!
//! Checks structural requirements on all `.html` files in `public/`:
//! - `<title>` element present on every page
//! - Semantic elements present: `<nav>`, `<main>`, `<footer>`
//! - `lang` attribute on `<html>` element

use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

use crate::report::{ConformanceReport, TestResult};

/// Validates HTML5 structure of all website pages.
///
/// # Errors
///
/// Returns an error if HTML files cannot be read.
pub fn validate(artifacts: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    if !artifacts.exists() {
        report.push(TestResult::warn(
            "website/html",
            "Artifacts directory not found — skipping HTML validation",
        ));
        return Ok(report);
    }

    let mut issues: Vec<String> = Vec::new();
    let mut pages_checked = 0u32;

    for entry in WalkDir::new(artifacts)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|x| x == "html")
                .unwrap_or(false)
                && !e.path().to_string_lossy().contains("target")
        })
    {
        let path = entry.path();
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                issues.push(format!("Cannot read {}: {}", path.display(), e));
                continue;
            }
        };

        let rel_path = path
            .strip_prefix(artifacts)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        let file_issues = check_html_structure(&rel_path, &content);
        issues.extend(file_issues);
        pages_checked += 1;
    }

    if pages_checked == 0 {
        report.push(TestResult::warn(
            "website/html",
            "No HTML files found in artifacts directory",
        ));
        return Ok(report);
    }

    if issues.is_empty() {
        report.push(TestResult::pass(
            "website/html",
            format!("All {} HTML pages pass structural validation", pages_checked),
        ));
    } else {
        report.push(TestResult::fail_with_details(
            "website/html",
            format!("HTML structure issues across {} pages checked", pages_checked),
            issues,
        ));
    }

    Ok(report)
}

/// Checks a single HTML file for structural issues using string-based heuristics.
fn check_html_structure(path: &str, content: &str) -> Vec<String> {
    let lower = content.to_lowercase();
    let mut issues = Vec::new();

    // Must have a <title> element
    if !lower.contains("<title") {
        issues.push(format!("{}: missing <title> element", path));
    }

    // Must have <main> element
    if !lower.contains("<main") {
        issues.push(format!("{}: missing <main> element", path));
    }

    // Must have <nav> element
    if !lower.contains("<nav") {
        issues.push(format!("{}: missing <nav> element", path));
    }

    // Must have <footer> element
    if !lower.contains("<footer") {
        issues.push(format!("{}: missing <footer> element", path));
    }

    // Must have lang attribute on html element
    if !lower.contains("lang=") {
        issues.push(format!("{}: <html> missing lang attribute", path));
    }

    issues
}
