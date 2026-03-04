//! Ontology inventory validator.
//!
//! Verifies that the built ontology artifact contains the correct counts:
//! - 16 namespaces (3 Kernel / 10 Bridge / 3 User)
//! - 130 classes
//! - 233 namespace-level properties + 1 global annotation = 234 via property_count()
//! - 304 named individuals (each with required property assertions)

use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;
use uor_ontology::model::Space;

use crate::report::{ConformanceReport, TestResult};

/// Expected inventory counts for the UOR Foundation ontology.
const EXPECTED_NAMESPACES: usize = 16;
const EXPECTED_CLASSES: usize = 130;
const EXPECTED_PROPERTIES: usize = 234;
const EXPECTED_INDIVIDUALS: usize = 304;

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

    // Hardening: identity grounding (all op:Identity individuals have typed verification properties)
    validate_identity_grounding(&mut report);

    // Amendment 23: vocabulary individual validators
    validate_verification_domain_individuals(&mut report);
    validate_geometric_character_individuals(&mut report);
    validate_complexity_class_individuals(&mut report);
    validate_rewrite_rule_individuals(&mut report);
    validate_measurement_unit_individuals(&mut report);
    validate_coordinate_kind_individuals(&mut report);

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

/// Validates that namespace space annotations follow the 3/10/3 classification.
///
/// Expected: 3 Kernel (u, schema, op), 10 Bridge, 3 User (type, state, morphism).
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

    if bridge.len() == 10 {
        report.push(TestResult::pass(
            validator,
            format!("Correct bridge-space count: 10 ({:?})", bridge),
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            format!(
                "Wrong bridge-space count: expected 10, got {} ({:?})",
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
        // nerveFunctorN: type assertion only
        ("https://uor.foundation/homology/nerveFunctorN", &[]),
        // chainFunctorC: type assertion only
        ("https://uor.foundation/homology/chainFunctorC", &[]),
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
        "AA_", "AM_", "TH_", "AR_", "PD_", "RC_", "DC_", "HA_", "IT_", "phi_", "psi_",
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

/// Validates that all `op:Identity` individuals are grounded with typed
/// verification properties: `hasVerificationStatus` (IriRef to op:Verifiable
/// or op:Derivable), at least one `verificationDomain` (IriRef to a domain
/// individual), and optionally `verificationPathNote`.
fn validate_identity_grounding(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/identity_grounding";

    let identity_type = "https://uor.foundation/op/Identity";
    let status_prop = "https://uor.foundation/op/hasVerificationStatus";
    let domain_prop = "https://uor.foundation/op/verificationDomain";
    let valid_statuses = [
        "https://uor.foundation/op/Verifiable",
        "https://uor.foundation/op/Derivable",
    ];
    let valid_domains = [
        "https://uor.foundation/op/Enumerative",
        "https://uor.foundation/op/Algebraic",
        "https://uor.foundation/op/Geometric",
        "https://uor.foundation/op/Analytical",
        "https://uor.foundation/op/Thermodynamic",
        "https://uor.foundation/op/Topological",
        "https://uor.foundation/op/Pipeline",
        "https://uor.foundation/op/IndexTheoretic",
    ];

    let mut total = 0usize;
    let mut verifiable = 0usize;
    let mut derivable = 0usize;
    let mut all_valid = true;

    for module in &ontology.namespaces {
        for ind in &module.individuals {
            if ind.type_ != identity_type {
                continue;
            }
            total += 1;

            // Check hasVerificationStatus
            let status = ind
                .properties
                .iter()
                .find(|(k, _)| *k == status_prop)
                .map(|(_, v)| v);

            match status {
                Some(uor_ontology::IndividualValue::IriRef(iri))
                    if valid_statuses.contains(iri) =>
                {
                    if *iri == "https://uor.foundation/op/Verifiable" {
                        verifiable += 1;
                    } else {
                        derivable += 1;
                    }
                }
                Some(_) => {
                    report.push(TestResult::fail(
                        validator,
                        format!(
                            "Identity {} has invalid hasVerificationStatus (must be IriRef to op:Verifiable or op:Derivable)",
                            ind.id
                        ),
                    ));
                    all_valid = false;
                }
                None => {
                    report.push(TestResult::fail(
                        validator,
                        format!("Identity {} missing hasVerificationStatus", ind.id),
                    ));
                    all_valid = false;
                }
            }

            // Check verificationDomain (at least one, each from closed set)
            let domains: Vec<_> = ind
                .properties
                .iter()
                .filter(|(k, _)| *k == domain_prop)
                .collect();

            if domains.is_empty() {
                report.push(TestResult::fail(
                    validator,
                    format!("Identity {} missing verificationDomain", ind.id),
                ));
                all_valid = false;
            } else {
                for (_, v) in &domains {
                    if let uor_ontology::IndividualValue::IriRef(iri) = v {
                        if !valid_domains.contains(iri) {
                            report.push(TestResult::fail(
                                validator,
                                format!(
                                    "Identity {} has invalid verificationDomain: {}",
                                    ind.id, iri
                                ),
                            ));
                            all_valid = false;
                        }
                    } else {
                        report.push(TestResult::fail(
                            validator,
                            format!("Identity {} verificationDomain is not an IriRef", ind.id),
                        ));
                        all_valid = false;
                    }
                }
            }
        }
    }

    if all_valid && total > 0 {
        report.push(TestResult::pass(
            validator,
            format!(
                "All {} identities grounded ({} verifiable, {} derivable)",
                total, verifiable, derivable
            ),
        ));
    }
}

/// Validates the 8 VerificationDomain and 2 VerificationStatus vocabulary individuals.
fn validate_verification_domain_individuals(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/verification_domain";

    let domain_iris = [
        "https://uor.foundation/op/Enumerative",
        "https://uor.foundation/op/Algebraic",
        "https://uor.foundation/op/Geometric",
        "https://uor.foundation/op/Analytical",
        "https://uor.foundation/op/Thermodynamic",
        "https://uor.foundation/op/Topological",
        "https://uor.foundation/op/Pipeline",
        "https://uor.foundation/op/IndexTheoretic",
    ];
    let status_iris = [
        "https://uor.foundation/op/Verifiable",
        "https://uor.foundation/op/Derivable",
    ];

    let mut all_found = true;
    for iri in &domain_iris {
        if ontology.find_individual(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("VerificationDomain individual {} not found", iri),
            ));
            all_found = false;
        }
    }
    for iri in &status_iris {
        if ontology.find_individual(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("VerificationStatus individual {} not found", iri),
            ));
            all_found = false;
        }
    }

    if all_found {
        report.push(TestResult::pass(
            validator,
            "All 8 VerificationDomain + 2 VerificationStatus individuals present",
        ));
    }
}

/// Validates the 9 GeometricCharacter vocabulary individuals and that all
/// 10 operation individuals have `hasGeometricCharacter`.
fn validate_geometric_character_individuals(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/geometric_character";

    let gc_iris = [
        "https://uor.foundation/op/RingReflection",
        "https://uor.foundation/op/HypercubeReflection",
        "https://uor.foundation/op/Rotation",
        "https://uor.foundation/op/RotationInverse",
        "https://uor.foundation/op/Translation",
        "https://uor.foundation/op/Scaling",
        "https://uor.foundation/op/HypercubeTranslation",
        "https://uor.foundation/op/HypercubeProjection",
        "https://uor.foundation/op/HypercubeJoin",
    ];

    let mut all_valid = true;
    for iri in &gc_iris {
        if ontology.find_individual(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("GeometricCharacter individual {} not found", iri),
            ));
            all_valid = false;
        }
    }

    // Check all 10 operations have hasGeometricCharacter
    let gc_prop = "https://uor.foundation/op/hasGeometricCharacter";
    let op_types = ["Involution", "UnaryOp", "BinaryOp"];
    if let Some(op_module) = ontology.find_namespace("op") {
        for ind in &op_module.individuals {
            let type_local = ind.type_.rsplit('/').next().unwrap_or("");
            if op_types.contains(&type_local) {
                let has_gc = ind.properties.iter().any(|(k, _)| *k == gc_prop);
                if !has_gc {
                    report.push(TestResult::fail(
                        validator,
                        format!("Operation {} missing hasGeometricCharacter", ind.id),
                    ));
                    all_valid = false;
                }
            }
        }
    }

    if all_valid {
        report.push(TestResult::pass(
            validator,
            "All 9 GeometricCharacter individuals + 10 operations validated",
        ));
    }
}

/// Validates the 4 ComplexityClass vocabulary individuals.
fn validate_complexity_class_individuals(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/complexity_class";

    let cc_iris = [
        "https://uor.foundation/resolver/ConstantTime",
        "https://uor.foundation/resolver/LogarithmicTime",
        "https://uor.foundation/resolver/LinearTime",
        "https://uor.foundation/resolver/ExponentialTime",
    ];

    let mut all_found = true;
    for iri in &cc_iris {
        if ontology.find_individual(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("ComplexityClass individual {} not found", iri),
            ));
            all_found = false;
        }
    }

    if all_found {
        report.push(TestResult::pass(
            validator,
            "All 4 ComplexityClass individuals present",
        ));
    }
}

/// Validates the 6 RewriteRule vocabulary individuals and CriticalIdentityRule
/// `groundedIn` assertion.
fn validate_rewrite_rule_individuals(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/rewrite_rule";

    let rr_iris = [
        "https://uor.foundation/derivation/CriticalIdentityRule",
        "https://uor.foundation/derivation/InvolutionRule",
        "https://uor.foundation/derivation/AssociativityRule",
        "https://uor.foundation/derivation/CommutativityRule",
        "https://uor.foundation/derivation/IdentityElementRule",
        "https://uor.foundation/derivation/NormalizationRule",
    ];

    let mut all_valid = true;
    for iri in &rr_iris {
        if ontology.find_individual(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("RewriteRule individual {} not found", iri),
            ));
            all_valid = false;
        }
    }

    // Check CriticalIdentityRule has groundedIn → criticalIdentity
    if let Some(cir) =
        ontology.find_individual("https://uor.foundation/derivation/CriticalIdentityRule")
    {
        let has_grounded = cir.properties.iter().any(|(k, v)| {
            *k == "https://uor.foundation/derivation/groundedIn"
                && matches!(
                    v,
                    uor_ontology::IndividualValue::IriRef(
                        "https://uor.foundation/op/criticalIdentity"
                    )
                )
        });
        if !has_grounded {
            report.push(TestResult::fail(
                validator,
                "CriticalIdentityRule missing groundedIn → op:criticalIdentity",
            ));
            all_valid = false;
        }
    }

    if all_valid {
        report.push(TestResult::pass(
            validator,
            "All 6 RewriteRule individuals + groundedIn assertion validated",
        ));
    }
}

/// Validates the 3 MeasurementUnit vocabulary individuals.
fn validate_measurement_unit_individuals(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/measurement_unit";

    let mu_iris = [
        "https://uor.foundation/observable/Bits",
        "https://uor.foundation/observable/RingSteps",
        "https://uor.foundation/observable/Dimensionless",
    ];

    let mut all_found = true;
    for iri in &mu_iris {
        if ontology.find_individual(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("MeasurementUnit individual {} not found", iri),
            ));
            all_found = false;
        }
    }

    if all_found {
        report.push(TestResult::pass(
            validator,
            "All 3 MeasurementUnit individuals present",
        ));
    }
}

/// Validates the 3 CoordinateKind vocabulary individuals.
fn validate_coordinate_kind_individuals(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/coordinate_kind";

    let ck_iris = [
        "https://uor.foundation/query/StratumCoordinate",
        "https://uor.foundation/query/SpectrumCoordinate",
        "https://uor.foundation/query/AddressCoordinate",
    ];

    let mut all_found = true;
    for iri in &ck_iris {
        if ontology.find_individual(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("CoordinateKind individual {} not found", iri),
            ));
            all_found = false;
        }
    }

    if all_found {
        report.push(TestResult::pass(
            validator,
            "All 3 CoordinateKind individuals present",
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
