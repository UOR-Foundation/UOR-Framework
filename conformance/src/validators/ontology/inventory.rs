//! Ontology inventory validator.
//!
//! Verifies that the built ontology artifact contains the correct counts:
//! - 14 namespaces (3 Kernel / 8 Bridge / 3 User)
//! - 103 classes
//! - 176 namespace-level properties + 1 global annotation = 177 via property_count()
//! - 255 named individuals (each with required property assertions)

use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;
use uor_ontology::model::Space;

use crate::report::{ConformanceReport, TestResult};

/// Expected inventory counts for the UOR Foundation ontology.
const EXPECTED_NAMESPACES: usize = 14;
const EXPECTED_CLASSES: usize = 103;
const EXPECTED_PROPERTIES: usize = 177;
const EXPECTED_INDIVIDUALS: usize = 255;

/// Validates the ontology inventory counts in the built JSON-LD artifact.
///
/// # Errors
///
/// Returns an error if the artifact file cannot be read or parsed.
pub fn validate(artifacts: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    // Also validate against the live spec (no file I/O needed)
    validate_spec_counts(&mut report);

    // Hardening: three-space classification
    validate_space_classification(&mut report);

    // Hardening: individual completeness
    validate_individual_completeness(&mut report);

    // Hardening: identity completeness (all op:Identity individuals have lhs/rhs/forAll)
    validate_identity_completeness(&mut report);

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
    let ontology = uor_ontology::Ontology::full();

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

/// Validates that namespace space annotations follow the 3/8/3 classification.
///
/// Expected: 3 Kernel (u, schema, op), 8 Bridge, 3 User (type, state, morphism).
fn validate_space_classification(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/space_classification";

    let kernel: Vec<_> = ontology
        .namespaces
        .iter()
        .filter(|m| m.namespace.space == Space::Kernel)
        .map(|m| m.namespace.prefix)
        .collect();
    let bridge: Vec<_> = ontology
        .namespaces
        .iter()
        .filter(|m| m.namespace.space == Space::Bridge)
        .map(|m| m.namespace.prefix)
        .collect();
    let user: Vec<_> = ontology
        .namespaces
        .iter()
        .filter(|m| m.namespace.space == Space::User)
        .map(|m| m.namespace.prefix)
        .collect();

    if kernel.len() == 3 {
        report.push(TestResult::pass(
            validator,
            format!("Correct kernel-space count: 3 ({:?})", kernel),
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            format!(
                "Wrong kernel-space count: expected 3, got {} ({:?})",
                kernel.len(),
                kernel
            ),
        ));
    }

    if bridge.len() == 8 {
        report.push(TestResult::pass(
            validator,
            format!("Correct bridge-space count: 8 ({:?})", bridge),
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            format!(
                "Wrong bridge-space count: expected 8, got {} ({:?})",
                bridge.len(),
                bridge
            ),
        ));
    }

    if user.len() == 3 {
        report.push(TestResult::pass(
            validator,
            format!("Correct user-space count: 3 ({:?})", user),
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            format!(
                "Wrong user-space count: expected 3, got {} ({:?})",
                user.len(),
                user
            ),
        ));
    }
}

/// Validates that every named individual has the minimum required property assertions.
fn validate_individual_completeness(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/individual_completeness";

    // Define minimum required properties per individual
    let requirements: &[(&str, &[&str])] = &[
        // 10 operations: all require arity
        ("https://uor.foundation/op/neg", &["op/arity"]),
        ("https://uor.foundation/op/bnot", &["op/arity"]),
        ("https://uor.foundation/op/succ", &["op/arity"]),
        ("https://uor.foundation/op/pred", &["op/arity"]),
        ("https://uor.foundation/op/add", &["op/arity"]),
        ("https://uor.foundation/op/sub", &["op/arity"]),
        ("https://uor.foundation/op/mul", &["op/arity"]),
        ("https://uor.foundation/op/xor", &["op/arity"]),
        ("https://uor.foundation/op/and", &["op/arity"]),
        ("https://uor.foundation/op/or", &["op/arity"]),
        // criticalIdentity: lhs, rhs, forAll
        (
            "https://uor.foundation/op/criticalIdentity",
            &["op/lhs", "op/rhs", "op/forAll"],
        ),
        // D2n: generatedBy, presentation
        (
            "https://uor.foundation/op/D2n",
            &["op/generatedBy", "op/presentation"],
        ),
        // pi1, zero: value
        ("https://uor.foundation/schema/pi1", &["schema/value"]),
        ("https://uor.foundation/schema/zero", &["schema/value"]),
        // MetricAxis individuals: type assertion only (no required properties)
        ("https://uor.foundation/type/verticalAxis", &[]),
        ("https://uor.foundation/type/horizontalAxis", &[]),
        ("https://uor.foundation/type/diagonalAxis", &[]),
        // criticalComposition: lawComponents, lawResult
        (
            "https://uor.foundation/morphism/criticalComposition",
            &["morphism/lawComponents", "morphism/lawResult"],
        ),
        // AD_1: addressing bijection — lhs, rhs, forAll
        (
            "https://uor.foundation/op/AD_1",
            &["op/lhs", "op/rhs", "op/forAll"],
        ),
        // AD_2: embedding coherence — lhs, rhs, forAll
        (
            "https://uor.foundation/op/AD_2",
            &["op/lhs", "op/rhs", "op/forAll"],
        ),
    ];

    let mut all_found = true;

    for (iri, required_props) in requirements {
        match ontology.find_individual(iri) {
            Some(individual) => {
                for prop_suffix in *required_props {
                    let full_prop = format!("https://uor.foundation/{prop_suffix}");
                    let has_prop = individual.properties.iter().any(|(k, _)| *k == full_prop);
                    if !has_prop {
                        report.push(TestResult::fail(
                            validator,
                            format!("Individual {} missing required property {}", iri, full_prop),
                        ));
                        all_found = false;
                    }
                }
            }
            None => {
                report.push(TestResult::fail(
                    validator,
                    format!("Named individual {} not found in ontology", iri),
                ));
                all_found = false;
            }
        }
    }

    if all_found {
        report.push(TestResult::pass(
            validator,
            format!(
                "All {} named individuals have required property assertions",
                requirements.len()
            ),
        ));
    }
}

/// Validates that all `op:Identity` individuals have lhs, rhs, and forAll properties,
/// and that every expected algebra prefix group is represented.
fn validate_identity_completeness(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/identity_completeness";

    let op_module = match ontology.find_namespace("op") {
        Some(m) => m,
        None => {
            report.push(TestResult::fail(validator, "op namespace not found"));
            return;
        }
    };

    let identity_type = "https://uor.foundation/op/Identity";
    let identities: Vec<_> = op_module
        .individuals
        .iter()
        .filter(|i| i.type_ == identity_type)
        .collect();

    let mut all_valid = true;
    for ind in &identities {
        let has_lhs = ind
            .properties
            .iter()
            .any(|(k, _)| *k == "https://uor.foundation/op/lhs");
        let has_rhs = ind
            .properties
            .iter()
            .any(|(k, _)| *k == "https://uor.foundation/op/rhs");
        let has_forall = ind
            .properties
            .iter()
            .any(|(k, _)| *k == "https://uor.foundation/op/forAll");
        if !has_lhs || !has_rhs || !has_forall {
            report.push(TestResult::fail(
                validator,
                format!("Identity {} missing lhs/rhs/forAll", ind.id),
            ));
            all_valid = false;
        }
    }

    // Verify expected algebra prefix groups are all present
    let expected_prefixes = [
        "R_A", "R_M", "B_", "X_", "D_", "U_", "AG_", "CA_", "C_", "CDI", "CL_", "CM_", "CR_", "F_",
        "FL_", "FPM_", "FS_", "RE_", "IR_", "SF_", "RD_", "SE_", "OO_", "CB_", "OB_M", "OB_C",
        "OB_H", "OB_P", "CT_", "CF_", "HG_", "T_C", "T_I", "T_E", "T_A", "AU_", "EF_", "AD_",
        "AA_", "AM_", "TH_", "AR_", "PD_", "RC_", "DC_", "HA_", "IT_", "phi_",
    ];
    for prefix in &expected_prefixes {
        let has = identities.iter().any(|i| i.label.starts_with(prefix));
        if !has {
            report.push(TestResult::fail(
                validator,
                format!("No identity with prefix {} found", prefix),
            ));
            all_valid = false;
        }
    }

    if all_valid {
        report.push(TestResult::pass(
            validator,
            format!("{} identity individuals validated", identities.len()),
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
