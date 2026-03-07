//! Ontology inventory validator.
//!
//! Verifies that the built ontology artifact contains the correct counts:
//! - 16 namespaces (3 Kernel / 10 Bridge / 3 User)
//! - 180 classes
//! - 337 namespace-level properties + 1 global annotation = 338 via property_count()
//! - 652 named individuals (each with required property assertions)

use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;
use uor_ontology::model::Space;

use crate::report::{ConformanceReport, TestResult};

/// Expected inventory counts for the UOR Foundation ontology.
const EXPECTED_NAMESPACES: usize = 16;
const EXPECTED_CLASSES: usize = 200;
const EXPECTED_PROPERTIES: usize = 387;
const EXPECTED_INDIVIDUALS: usize = 712;

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

    // v3.2: proof coverage and quantum scope
    validate_proof_coverage(&mut report);
    validate_quantum_scope(&mut report);

    // Amendment 23: vocabulary individual validators
    validate_verification_domain_individuals(&mut report);
    validate_geometric_character_individuals(&mut report);
    validate_complexity_class_individuals(&mut report);
    validate_rewrite_rule_individuals(&mut report);
    validate_measurement_unit_individuals(&mut report);
    validate_coordinate_kind_individuals(&mut report);
    validate_surface_symmetry_identity(&mut report);
    validate_saturation_phase_individuals(&mut report);
    validate_achievability_status_individuals(&mut report);

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
pub(crate) fn check_count(
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
        // Amendment 25: Completeness Certification
        "CC_",  // Amendment 26: Quantum Level Scaling
        "QL_",  // Amendment 27: Session-Scoped Resolution
        "SR_",  // Amendment 28: Type Synthesis
        "TS_",  // Amendment 29: Quantum Level Spectral Sequence
        "QLS_", // Amendment 30: Monodromy Observables
        "MN_",  // Amendment 31: Product/Sum Type identities
        "PT_",  // Amendment 31: Sum Type identities
        "ST_",  // Amendment 33: Saturated Context Limit
        "SC_",  // Amendment 34: Morphospace Achievability
        "MS_",  // Amendment 35: Computational Geodesic
        "GD_",  // Amendment 36: Measurement Boundary
        "QM_",
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

/// Validates that all `op:Identity` individuals are grounded with at least one
/// `verificationDomain` (IriRef to a domain individual) and optionally
/// `verificationPathNote`.
fn validate_identity_grounding(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/identity_grounding";

    let identity_type = "https://uor.foundation/op/Identity";
    let domain_prop = "https://uor.foundation/op/verificationDomain";
    let valid_domains = [
        "https://uor.foundation/op/Enumerative",
        "https://uor.foundation/op/Algebraic",
        "https://uor.foundation/op/Geometric",
        "https://uor.foundation/op/Analytical",
        "https://uor.foundation/op/Thermodynamic",
        "https://uor.foundation/op/Topological",
        "https://uor.foundation/op/Pipeline",
        "https://uor.foundation/op/IndexTheoretic",
        "https://uor.foundation/op/SuperpositionDomain",
        "https://uor.foundation/op/QuantumThermodynamic",
    ];

    let mut total = 0usize;
    let mut all_valid = true;

    for module in &ontology.namespaces {
        for ind in &module.individuals {
            if ind.type_ != identity_type {
                continue;
            }
            total += 1;

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
            format!("All {} identities grounded with verificationDomain", total),
        ));
    }
}

/// Validates the 9 VerificationDomain vocabulary individuals.
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
        "https://uor.foundation/op/SuperpositionDomain",
        "https://uor.foundation/op/QuantumThermodynamic",
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

    if all_found {
        report.push(TestResult::pass(
            validator,
            "All 10 VerificationDomain individuals present",
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

/// Validates the 4 MeasurementUnit vocabulary individuals.
fn validate_measurement_unit_individuals(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/measurement_unit";

    let mu_iris = [
        "https://uor.foundation/observable/Bits",
        "https://uor.foundation/observable/RingSteps",
        "https://uor.foundation/observable/Dimensionless",
        "https://uor.foundation/observable/Nats",
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
            "All 4 MeasurementUnit individuals present",
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

/// Validates that every `op:Identity` individual has a corresponding proof individual
/// with `proof:provesIdentity` linking to it.
fn validate_proof_coverage(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/proof_coverage";

    let identity_type = "https://uor.foundation/op/Identity";
    let proves_prop = "https://uor.foundation/proof/provesIdentity";

    // Collect all identity IRIs across all namespaces
    let mut identity_iris: Vec<&str> = Vec::new();
    for module in &ontology.namespaces {
        for ind in &module.individuals {
            if ind.type_ == identity_type {
                identity_iris.push(ind.id);
            }
        }
    }

    // Collect all IRIs that are targets of proof:provesIdentity
    let mut proved_iris: std::collections::HashSet<&str> = std::collections::HashSet::new();
    if let Some(proof_module) = ontology.find_namespace("proof") {
        for ind in &proof_module.individuals {
            for (k, v) in ind.properties {
                if *k == proves_prop {
                    if let uor_ontology::IndividualValue::IriRef(iri) = v {
                        proved_iris.insert(iri);
                    }
                }
            }
        }
    }

    let uncovered: Vec<_> = identity_iris
        .iter()
        .filter(|iri| !proved_iris.contains(**iri))
        .collect();

    if uncovered.is_empty() {
        report.push(TestResult::pass(
            validator,
            format!(
                "All {} identities covered by proof individuals",
                identity_iris.len()
            ),
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            format!(
                "{} identities not covered by proof individuals: {:?}",
                uncovered.len(),
                &uncovered[..uncovered.len().min(5)]
            ),
        ));
    }
}

/// Validates quantum scope consistency: `ComputationCertificate` individuals
/// must have `atQuantumLevel` and must NOT have `universalScope`;
/// `AxiomaticDerivation` individuals must have `universalScope` and must NOT
/// have `atQuantumLevel`.
fn validate_quantum_scope(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/quantum_scope";

    let cert_type = "https://uor.foundation/proof/ComputationCertificate";
    let crit_type = "https://uor.foundation/proof/CriticalIdentityProof";
    let axiomatic_type = "https://uor.foundation/proof/AxiomaticDerivation";
    let at_ql_prop = "https://uor.foundation/proof/atQuantumLevel";
    let univ_prop = "https://uor.foundation/proof/universalScope";

    let mut all_valid = true;
    let mut cert_count = 0usize;
    let mut axiomatic_count = 0usize;

    if let Some(proof_module) = ontology.find_namespace("proof") {
        for ind in &proof_module.individuals {
            let is_cert = ind.type_ == cert_type || ind.type_ == crit_type;
            let is_axiomatic = ind.type_ == axiomatic_type;

            if is_cert {
                cert_count += 1;
                let has_ql = ind.properties.iter().any(|(k, _)| *k == at_ql_prop);
                let has_univ = ind.properties.iter().any(|(k, _)| *k == univ_prop);
                if !has_ql {
                    report.push(TestResult::fail(
                        validator,
                        format!("ComputationCertificate {} missing atQuantumLevel", ind.id),
                    ));
                    all_valid = false;
                }
                if has_univ {
                    report.push(TestResult::fail(
                        validator,
                        format!(
                            "ComputationCertificate {} should not have universalScope",
                            ind.id
                        ),
                    ));
                    all_valid = false;
                }
            }

            if is_axiomatic {
                axiomatic_count += 1;
                let has_univ = ind.properties.iter().any(|(k, _)| *k == univ_prop);
                let has_ql = ind.properties.iter().any(|(k, _)| *k == at_ql_prop);
                if !has_univ {
                    report.push(TestResult::fail(
                        validator,
                        format!("AxiomaticDerivation {} missing universalScope", ind.id),
                    ));
                    all_valid = false;
                }
                if has_ql {
                    report.push(TestResult::fail(
                        validator,
                        format!(
                            "AxiomaticDerivation {} should not have atQuantumLevel",
                            ind.id
                        ),
                    ));
                    all_valid = false;
                }
            }
        }
    }

    if all_valid && (cert_count + axiomatic_count) > 0 {
        report.push(TestResult::pass(
            validator,
            format!(
                "Quantum scope valid: {} computation certificates, {} axiomatic derivations",
                cert_count, axiomatic_count
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

/// Validates the Surface Symmetry identity and its proof individual.
fn validate_surface_symmetry_identity(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/surface_symmetry";

    let identity_iri = "https://uor.foundation/op/surfaceSymmetry";
    let proof_iri = "https://uor.foundation/proof/prf_surfaceSymmetry";

    let mut all_found = true;

    if ontology.find_individual(identity_iri).is_none() {
        report.push(TestResult::fail(
            validator,
            "op:surfaceSymmetry individual not found",
        ));
        all_found = false;
    }

    if ontology.find_individual(proof_iri).is_none() {
        report.push(TestResult::fail(
            validator,
            "proof:prf_surfaceSymmetry individual not found",
        ));
        all_found = false;
    }

    if all_found {
        report.push(TestResult::pass(
            validator,
            "op:surfaceSymmetry and proof:prf_surfaceSymmetry present",
        ));
    }
}

/// Validates the 3 SaturationPhase vocabulary individuals.
fn validate_saturation_phase_individuals(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/saturation_phase";

    let sp_iris = [
        "https://uor.foundation/state/Unsaturated",
        "https://uor.foundation/state/PartialSaturation",
        "https://uor.foundation/state/FullSaturation",
    ];

    let mut all_found = true;
    for iri in &sp_iris {
        if ontology.find_individual(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("SaturationPhase individual {} not found", iri),
            ));
            all_found = false;
        }
    }

    if all_found {
        report.push(TestResult::pass(
            validator,
            "All 3 SaturationPhase individuals present",
        ));
    }
}

/// Validates the 2 AchievabilityStatus vocabulary individuals.
fn validate_achievability_status_individuals(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/achievability_status";

    let as_iris = [
        "https://uor.foundation/observable/Achievable",
        "https://uor.foundation/observable/Forbidden",
    ];

    let mut all_found = true;
    for iri in &as_iris {
        if ontology.find_individual(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("AchievabilityStatus individual {} not found", iri),
            ));
            all_found = false;
        }
    }

    if all_found {
        report.push(TestResult::pass(
            validator,
            "All 2 AchievabilityStatus individuals present",
        ));
    }
}

/// Validates workspace-level inventory invariants (requires workspace root path).
///
/// Currently checks that the SHACL shapes file has exactly one NodeShape per class.
///
/// # Errors
///
/// Returns an error if the shapes file cannot be read.
pub fn validate_workspace(workspace: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    let shapes_path = workspace.join("conformance/shapes/uor-shapes.ttl");
    let content = std::fs::read_to_string(&shapes_path)
        .with_context(|| format!("Failed to read {}", shapes_path.display()))?;

    let shape_count = content
        .lines()
        .filter(|l| l.contains("sh:NodeShape"))
        .count();
    let expected = uor_ontology::Ontology::full().class_count();

    check_count(
        &mut report,
        "SHACL shapes (NodeShape declarations)",
        shape_count,
        expected,
        "ontology/inventory/shapes_count",
    );

    Ok(report)
}
