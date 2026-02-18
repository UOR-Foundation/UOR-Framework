//! Ontology inventory validator.
//!
//! Verifies that the built ontology artifact contains the correct counts:
//! - 14 namespaces
//! - 82 classes
//! - 119 namespace-level properties + 1 global annotation = 120 via property_count()
//! - 14 named individuals

use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::report::{ConformanceReport, TestResult};

/// Expected inventory counts for the UOR Foundation ontology.
const EXPECTED_NAMESPACES: usize = 14;
const EXPECTED_CLASSES: usize = 82;
const EXPECTED_PROPERTIES: usize = 120;
const EXPECTED_INDIVIDUALS: usize = 14;

/// Validates the ontology inventory counts in the built JSON-LD artifact.
///
/// # Errors
///
/// Returns an error if the artifact file cannot be read or parsed.
pub fn validate(artifacts: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    // Also validate against the live spec (no file I/O needed)
    validate_spec_counts(&mut report);

    // Validate the built JSON-LD artifact
    let json_path = artifacts.join("uor.foundation.json");
    if !json_path.exists() {
        report.push(TestResult::fail(
            "ontology/inventory",
            "uor.foundation.json not found in artifacts directory",
        ));
        return Ok(report);
    }

    let content = std::fs::read_to_string(&json_path)
        .with_context(|| format!("Failed to read {}", json_path.display()))?;

    let value: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {} as JSON", json_path.display()))?;

    validate_json_inventory(&value, &mut report);

    Ok(report)
}

/// Validates inventory counts directly from the spec (no file I/O).
fn validate_spec_counts(report: &mut ConformanceReport) {
    let ontology = uor_spec::Ontology::full();

    let ns_count = ontology.namespaces.len();
    let class_count = ontology.class_count();
    let property_count = ontology.property_count();
    let individual_count = ontology.individual_count();

    check_count(
        report,
        "namespaces",
        ns_count,
        EXPECTED_NAMESPACES,
        "ontology/inventory",
    );
    check_count(
        report,
        "classes",
        class_count,
        EXPECTED_CLASSES,
        "ontology/inventory",
    );
    check_count(
        report,
        "properties",
        property_count,
        EXPECTED_PROPERTIES,
        "ontology/inventory",
    );
    check_count(
        report,
        "individuals",
        individual_count,
        EXPECTED_INDIVIDUALS,
        "ontology/inventory",
    );
}

/// Checks a count matches the expected value.
fn check_count(
    report: &mut ConformanceReport,
    label: &str,
    actual: usize,
    expected: usize,
    validator: &str,
) {
    if actual == expected {
        report.push(TestResult::pass(
            validator,
            format!("Correct {} count: {}", label, actual),
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            format!(
                "Wrong {} count: expected {}, got {}",
                label, expected, actual
            ),
        ));
    }
}

/// Returns true if a JSON-LD node's `@type` field contains the given value.
///
/// Handles both string and array forms of `@type`.
fn node_has_type(node: &Value, target: &str) -> bool {
    match node.get("@type") {
        Some(Value::String(t)) => t == target,
        Some(Value::Array(types)) => types.iter().any(|t| t.as_str() == Some(target)),
        _ => false,
    }
}

/// Validates inventory counts from the JSON-LD graph.
fn validate_json_inventory(value: &Value, report: &mut ConformanceReport) {
    let graph = match value.get("@graph").and_then(|g| g.as_array()) {
        Some(g) => g,
        None => {
            report.push(TestResult::fail(
                "ontology/inventory",
                "JSON-LD artifact missing @graph array",
            ));
            return;
        }
    };

    // Count classes (type owl:Class)
    let class_count = graph
        .iter()
        .filter(|node| node_has_type(node, "owl:Class"))
        .count();

    // Count properties (owl:DatatypeProperty | owl:ObjectProperty | owl:AnnotationProperty)
    let property_count = graph
        .iter()
        .filter(|node| {
            node_has_type(node, "owl:DatatypeProperty")
                || node_has_type(node, "owl:ObjectProperty")
                || node_has_type(node, "owl:AnnotationProperty")
        })
        .count();

    // Count named individuals (owl:NamedIndividual)
    let individual_count = graph
        .iter()
        .filter(|node| node_has_type(node, "owl:NamedIndividual"))
        .count();

    check_count(
        report,
        "classes (JSON-LD)",
        class_count,
        EXPECTED_CLASSES,
        "ontology/inventory",
    );
    check_count(
        report,
        "properties (JSON-LD)",
        property_count,
        EXPECTED_PROPERTIES,
        "ontology/inventory",
    );
    check_count(
        report,
        "individuals (JSON-LD)",
        individual_count,
        EXPECTED_INDIVIDUALS,
        "ontology/inventory",
    );
}
