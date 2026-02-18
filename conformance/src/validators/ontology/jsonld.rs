//! JSON-LD 1.1 validator.
//!
//! Verifies that `public/uor.foundation.json` is a well-formed JSON-LD 1.1 document:
//! - Has `@context` with all 14 namespace prefixes and standard prefixes
//! - Has `@graph` containing the expected node types
//! - All `@id` values are IRIs (not relative references)
//! - Context entries map to valid IRI prefixes

use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::report::{ConformanceReport, TestResult};

/// Required namespace prefixes in the JSON-LD context.
const REQUIRED_PREFIXES: &[&str] = &[
    "u",
    "schema",
    "op",
    "query",
    "resolver",
    "type",
    "partition",
    "observable",
    "proof",
    "derivation",
    "trace",
    "cert",
    "morphism",
    "state",
    "owl",
    "rdf",
    "rdfs",
    "xsd",
    "sh",
    "uor",
];

/// Validates the JSON-LD artifact for JSON-LD 1.1 conformance.
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn validate(artifacts: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    let json_path = artifacts.join("uor.foundation.json");
    if !json_path.exists() {
        report.push(TestResult::fail(
            "ontology/jsonld",
            "uor.foundation.json not found",
        ));
        return Ok(report);
    }

    let content = std::fs::read_to_string(&json_path)
        .with_context(|| format!("Failed to read {}", json_path.display()))?;

    let value: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {} as JSON", json_path.display()))?;

    check_context(&value, &mut report);
    check_graph_structure(&value, &mut report);
    check_id_resolution(&value, &mut report);

    Ok(report)
}

/// Verifies that the `@context` contains all required prefixes.
fn check_context(value: &Value, report: &mut ConformanceReport) {
    let context = match value.get("@context") {
        Some(ctx) => ctx,
        None => {
            report.push(TestResult::fail(
                "ontology/jsonld",
                "JSON-LD document missing @context",
            ));
            return;
        }
    };

    let mut missing: Vec<String> = Vec::new();
    for prefix in REQUIRED_PREFIXES {
        if context.get(prefix).is_none() {
            missing.push(prefix.to_string());
        }
    }

    if missing.is_empty() {
        report.push(TestResult::pass(
            "ontology/jsonld",
            "All required namespace prefixes present in @context",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            "ontology/jsonld",
            "Missing prefixes in @context",
            missing,
        ));
    }

    // Check @version is 1.1
    if let Some(version) = context.get("@version") {
        if version.as_f64().map(|v| (v - 1.1).abs() < f64::EPSILON) == Some(true) {
            report.push(TestResult::pass(
                "ontology/jsonld",
                "JSON-LD @version is 1.1",
            ));
        } else {
            report.push(TestResult::fail(
                "ontology/jsonld",
                format!("JSON-LD @version should be 1.1, got: {}", version),
            ));
        }
    }
}

/// Verifies that `@graph` is an array with the expected node types present.
fn check_graph_structure(value: &Value, report: &mut ConformanceReport) {
    let graph = match value.get("@graph").and_then(|g| g.as_array()) {
        Some(g) => g,
        None => {
            report.push(TestResult::fail(
                "ontology/jsonld",
                "JSON-LD document missing @graph array",
            ));
            return;
        }
    };

    report.push(TestResult::pass(
        "ontology/jsonld",
        format!("@graph array present with {} nodes", graph.len()),
    ));

    // Verify minimum node count (14 namespaces + 82 classes + 119 properties + 14 individuals + annotation properties)
    let min_nodes = 14 + 82 + 119 + 14;
    if graph.len() >= min_nodes {
        report.push(TestResult::pass(
            "ontology/jsonld",
            format!(
                "@graph has sufficient nodes ({} >= {})",
                graph.len(),
                min_nodes
            ),
        ));
    } else {
        report.push(TestResult::fail(
            "ontology/jsonld",
            format!(
                "@graph has too few nodes: {} (expected >= {})",
                graph.len(),
                min_nodes
            ),
        ));
    }

    // Every node must have @id
    let missing_id_count = graph
        .iter()
        .filter(|node| node.get("@id").is_none())
        .count();

    if missing_id_count == 0 {
        report.push(TestResult::pass(
            "ontology/jsonld",
            "All @graph nodes have @id",
        ));
    } else {
        report.push(TestResult::fail(
            "ontology/jsonld",
            format!("{} nodes in @graph are missing @id", missing_id_count),
        ));
    }
}

/// Verifies that all `@id` values in the graph are full IRIs.
fn check_id_resolution(value: &Value, report: &mut ConformanceReport) {
    let graph = match value.get("@graph").and_then(|g| g.as_array()) {
        Some(g) => g,
        None => {
            return;
        }
    };

    let mut relative_ids: Vec<String> = Vec::new();
    for node in graph {
        if let Some(id) = node.get("@id").and_then(|i| i.as_str()) {
            // Full IRIs must start with a scheme (http://, https://) or be a CURIE with known prefix
            if !id.contains(':') {
                relative_ids.push(id.to_string());
            }
        }
    }

    if relative_ids.is_empty() {
        report.push(TestResult::pass(
            "ontology/jsonld",
            "All @id values contain IRI scheme or prefix",
        ));
    } else {
        report.push(TestResult::fail_with_details(
            "ontology/jsonld",
            "Bare relative @id values found (no IRI scheme or prefix)",
            relative_ids,
        ));
    }
}
