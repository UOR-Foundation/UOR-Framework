//! Ontology inventory validator.
//!
//! Verifies that the built ontology artifact contains the correct counts
//! as defined in [`uor_ontology::counts`].

use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;
use uor_ontology::counts;
use uor_ontology::model::{IndividualValue, Space};

use crate::report::{ConformanceReport, TestResult};

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
    // Amendment 37: Gap closure validators
    validate_superposition_identity_coverage(&mut report);
    validate_partition_tensor_product(&mut report);
    validate_geodesic_decomposition(&mut report);
    validate_born_rule_coverage(&mut report);
    validate_enum_variant_alignment(&mut report);
    // Amendment 38-40: Q1 Conformance Coverage validators
    validate_q1_ring_grounding(&mut report);
    validate_lift_obstruction_paths(&mut report);
    validate_synthesis_q1_coverage(&mut report);
    validate_evidence_bundle_properties(&mut report);
    validate_normative_chain_integrity(&mut report);
    // Amendment 41: Tower chain vocabulary validators
    validate_validity_scope_individuals(&mut report);
    validate_tower_chain_vocabulary(&mut report);
    // Amendment 42: EBNF grammar alignment
    validate_ebnf_grammar_alignment(&mut report);
    // Amendment 43: Cryptographic Primitive Pinning
    validate_crypto_pinning_vocabulary(&mut report);
    // Amendment 44: Structural Gap Closures
    validate_carry_constraint_pinning(&mut report);
    validate_joint_satisfiability_coverage(&mut report);
    validate_dihedral_algebra_completeness(&mut report);
    validate_constraint_expressiveness(&mut report);
    validate_sumtype_topology(&mut report);
    validate_synthesis_reachability(&mut report);
    validate_obstruction_termination(&mut report);
    validate_coefficient_ring_grounding(&mut report);
    validate_gluing_feedback(&mut report);
    validate_session_saturation_bridge(&mut report);
    validate_amplitude_index_characterization(&mut report);

    // Amendment 45: Self-Auditing Meta-Validators
    validate_certificate_issuance_coverage(&mut report);
    validate_identity_proof_bijection(&mut report);
    validate_shacl_fixture_coverage(&mut report);

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
        counts::NAMESPACES,
        "ontology/inventory",
    );
    check_count(
        report,
        "classes",
        class_count,
        counts::CLASSES,
        "ontology/inventory",
    );
    check_count(
        report,
        "properties",
        property_count,
        counts::PROPERTIES,
        "ontology/inventory",
    );
    check_count(
        report,
        "individuals",
        individual_count,
        counts::INDIVIDUALS,
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

    if kernel.len() == counts::KERNEL_NAMESPACES {
        report.push(TestResult::pass(
            validator,
            format!(
                "Correct kernel-space count: {} ({:?})",
                counts::KERNEL_NAMESPACES,
                kernel
            ),
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            format!(
                "Wrong kernel-space count: expected {}, got {} ({:?})",
                counts::KERNEL_NAMESPACES,
                kernel.len(),
                kernel
            ),
        ));
    }

    if bridge.len() == counts::BRIDGE_NAMESPACES {
        report.push(TestResult::pass(
            validator,
            format!(
                "Correct bridge-space count: {} ({:?})",
                counts::BRIDGE_NAMESPACES,
                bridge
            ),
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            format!(
                "Wrong bridge-space count: expected {}, got {} ({:?})",
                counts::BRIDGE_NAMESPACES,
                bridge.len(),
                bridge
            ),
        ));
    }

    if user.len() == counts::USER_NAMESPACES {
        report.push(TestResult::pass(
            validator,
            format!(
                "Correct user-space count: {} ({:?})",
                counts::USER_NAMESPACES,
                user
            ),
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            format!(
                "Wrong user-space count: expected {}, got {} ({:?})",
                counts::USER_NAMESPACES,
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
        "QM_",  // Amendment 37: SuperpositionResolver identities
        "SP_",  // Amendment 41: Tower identities
        "QT_",  // Amendment 44: Structural Gap Closures
        "jsat_", "EXP_", "GO_", "COEFF_",
        // Amendment 46: Certificate Issuance Coverage
        "CIC_", "GC_",
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
    let mut proved_iris: HashSet<&str> = HashSet::new();
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
    let inductive_type = "https://uor.foundation/proof/InductiveProof";
    let at_ql_prop = "https://uor.foundation/proof/atQuantumLevel";
    let univ_prop = "https://uor.foundation/proof/universalScope";

    let mut all_valid = true;
    let mut cert_count = 0usize;
    let mut axiomatic_count = 0usize;
    let mut inductive_count = 0usize;

    if let Some(proof_module) = ontology.find_namespace("proof") {
        for ind in &proof_module.individuals {
            let is_cert = ind.type_ == cert_type || ind.type_ == crit_type;
            let is_axiomatic = ind.type_ == axiomatic_type;
            let is_inductive = ind.type_ == inductive_type;

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

            if is_inductive {
                inductive_count += 1;
                let has_univ = ind.properties.iter().any(|(k, _)| *k == univ_prop);
                let has_ql = ind.properties.iter().any(|(k, _)| *k == at_ql_prop);
                if !has_univ {
                    report.push(TestResult::fail(
                        validator,
                        format!("InductiveProof {} missing universalScope", ind.id),
                    ));
                    all_valid = false;
                }
                if has_ql {
                    report.push(TestResult::fail(
                        validator,
                        format!("InductiveProof {} should not have atQuantumLevel", ind.id),
                    ));
                    all_valid = false;
                }
            }
        }
    }

    if all_valid && (cert_count + axiomatic_count + inductive_count) > 0 {
        report.push(TestResult::pass(
            validator,
            format!(
                "Quantum scope valid: {} computation certificates, \
                 {} axiomatic derivations, {} inductive proofs",
                cert_count, axiomatic_count, inductive_count
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
        counts::CLASSES,
        "ontology/inventory",
    );
    check_count(
        report,
        "properties (JSON-LD)",
        property_count,
        counts::PROPERTIES,
        "ontology/inventory",
    );
    check_count(
        report,
        "individuals (JSON-LD)",
        individual_count,
        counts::INDIVIDUALS,
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

fn validate_superposition_identity_coverage(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/superposition_identity";
    let sp_ids = [
        "https://uor.foundation/op/SP_1",
        "https://uor.foundation/op/SP_2",
        "https://uor.foundation/op/SP_3",
        "https://uor.foundation/op/SP_4",
    ];
    let mut all_found = true;
    for iri in &sp_ids {
        if ontology.find_individual(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("SP identity {} not found", iri),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(validator, "All 4 SP_ identities present"));
    }
}

fn validate_partition_tensor_product(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/partition_tensor";
    let class_iris = [
        "https://uor.foundation/partition/PartitionProduct",
        "https://uor.foundation/partition/PartitionCoproduct",
    ];
    let mut all_found = true;
    for iri in &class_iris {
        if ontology.find_class(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("Class {} not found", iri),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "PartitionProduct and PartitionCoproduct classes present",
        ));
    }
}

fn validate_geodesic_decomposition(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/geodesic_decomposition";
    if ontology
        .find_individual("https://uor.foundation/op/GD_6")
        .is_some()
    {
        report.push(TestResult::pass(
            validator,
            "GD_6 geodesic decomposition identity present",
        ));
    } else {
        report.push(TestResult::fail(validator, "GD_6 identity not found"));
    }
}

fn validate_born_rule_coverage(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/born_rule";
    let mut all_found = true;
    if ontology
        .find_individual("https://uor.foundation/op/QM_5")
        .is_none()
    {
        report.push(TestResult::fail(validator, "QM_5 identity not found"));
        all_found = false;
    }
    if ontology
        .find_class("https://uor.foundation/cert/BornRuleVerification")
        .is_none()
    {
        report.push(TestResult::fail(
            validator,
            "BornRuleVerification class not found",
        ));
        all_found = false;
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "QM_5 identity and BornRuleVerification class present",
        ));
    }
}

fn validate_enum_variant_alignment(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/enum_variant";
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
    let ev_prop = "https://uor.foundation/op/enumVariant";
    let mut all_valid = true;
    for iri in &domain_iris {
        if let Some(ind) = ontology.find_individual(iri) {
            let has_ev = ind.properties.iter().any(|(k, _)| *k == ev_prop);
            if !has_ev {
                report.push(TestResult::fail(
                    validator,
                    format!("VerificationDomain {} missing enumVariant", iri),
                ));
                all_valid = false;
            }
        }
    }
    if all_valid {
        report.push(TestResult::pass(
            validator,
            "All 10 VerificationDomain individuals have enumVariant",
        ));
    }
}

/// Validates Q1Ring class exists (Amendment 39).
fn validate_q1_ring_grounding(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/q1_ring_grounding";
    let q1ring = "https://uor.foundation/schema/Q1Ring";
    if ontology.find_class(q1ring).is_some() {
        report.push(TestResult::pass(validator, "Q1Ring class present"));
    } else {
        report.push(TestResult::fail(validator, "Q1Ring class not found"));
    }
}

/// Validates QuantumLift and LiftObstruction classes exist (Amendment 39).
fn validate_lift_obstruction_paths(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/lift_obstruction_paths";
    let class_iris = [
        "https://uor.foundation/type/QuantumLift",
        "https://uor.foundation/type/LiftObstruction",
    ];
    let mut all_found = true;
    for iri in &class_iris {
        if ontology.find_class(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("Class {} not found", iri),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "QuantumLift and LiftObstruction classes present",
        ));
    }
}

/// Validates SynthesisCheckpoint class and properties exist (Amendment 38).
fn validate_synthesis_q1_coverage(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/synthesis_q1_coverage";
    let class_iri = "https://uor.foundation/derivation/SynthesisCheckpoint";
    let prop_iris = [
        "https://uor.foundation/derivation/checkpointStep",
        "https://uor.foundation/derivation/checkpointState",
    ];
    let class_found = ontology.find_class(class_iri).is_some();
    let props_found = prop_iris
        .iter()
        .all(|iri| ontology.find_property(iri).is_some());
    if class_found && props_found {
        report.push(TestResult::pass(
            validator,
            "SynthesisCheckpoint class and properties present",
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            "SynthesisCheckpoint class or properties missing",
        ));
    }
}

/// Validates isAR1Ordered and isDC10Selected properties exist (Amendment 38).
fn validate_evidence_bundle_properties(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/evidence_bundle_properties";
    let prop_iris = [
        "https://uor.foundation/cert/isAR1Ordered",
        "https://uor.foundation/cert/isDC10Selected",
    ];
    let mut all_found = true;
    for iri in &prop_iris {
        if ontology.find_property(iri).is_none() {
            report.push(TestResult::fail(
                validator,
                format!("Property {} not found", iri),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "isAR1Ordered and isDC10Selected properties present",
        ));
    }
}

/// Validates the normative cert chain is connected (Amendment 40).
fn validate_normative_chain_integrity(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/normative_chain_integrity";
    let required = [
        "https://uor.foundation/cert/certifiedGeodesic",
        "https://uor.foundation/cert/evidenceBundle",
        "https://uor.foundation/cert/isAR1Ordered",
        "https://uor.foundation/cert/isDC10Selected",
    ];
    let all_found = required
        .iter()
        .all(|iri| ontology.find_property(iri).is_some());
    if all_found {
        report.push(TestResult::pass(
            validator,
            "Normative chain properties all present",
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            "Normative chain incomplete: missing cert/ properties",
        ));
    }
}

/// Validates workspace-level inventory invariants (requires workspace root path).
///
/// Currently checks that the SHACL shapes file has exactly one NodeShape per class.
///
/// Validates that the 4 ValidityScopeKind individuals exist (Amendment 41).
fn validate_validity_scope_individuals(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/validity_scope_individuals";
    let scope_type = "https://uor.foundation/op/ValidityScopeKind";
    let expected = [
        "Universal",
        "ParametricLower",
        "ParametricRange",
        "LevelSpecific",
    ];

    if let Some(op_module) = ontology.find_namespace("op") {
        let scope_inds: Vec<&str> = op_module
            .individuals
            .iter()
            .filter(|i| i.type_ == scope_type)
            .map(|i| i.label)
            .collect();
        let mut all_found = true;
        for name in &expected {
            if !scope_inds.contains(name) {
                report.push(TestResult::fail(
                    validator,
                    format!("Missing ValidityScopeKind individual: {}", name),
                ));
                all_found = false;
            }
        }
        if all_found {
            report.push(TestResult::pass(
                validator,
                format!(
                    "All {} ValidityScopeKind individuals present",
                    expected.len()
                ),
            ));
        }
    } else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
    }
}

/// Validates Amendment 41 tower chain vocabulary exists.
fn validate_tower_chain_vocabulary(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/tower_chain_vocabulary";

    let expected_classes = [
        "https://uor.foundation/type/LiftChain",
        "https://uor.foundation/type/ObstructionChain",
        "https://uor.foundation/cert/LiftChainCertificate",
        "https://uor.foundation/cert/ChainAuditTrail",
        "https://uor.foundation/resolver/TowerCompletenessResolver",
        "https://uor.foundation/proof/InductiveProof",
        "https://uor.foundation/op/ValidityScopeKind",
    ];

    let all_classes: Vec<&str> = ontology
        .namespaces
        .iter()
        .flat_map(|m| m.classes.iter().map(|c| c.id))
        .collect();

    let mut all_found = true;
    for cls in &expected_classes {
        if !all_classes.contains(cls) {
            report.push(TestResult::fail(
                validator,
                format!("Missing Amendment 41 class: {}", cls),
            ));
            all_found = false;
        }
    }

    // Check QT_1 through QT_7 identities exist
    if let Some(op_module) = ontology.find_namespace("op") {
        for i in 1..=7 {
            let label = format!("QT_{}", i);
            let has = op_module
                .individuals
                .iter()
                .any(|ind| ind.label == label.as_str());
            if !has {
                report.push(TestResult::fail(
                    validator,
                    format!("Missing tower identity: {}", label),
                ));
                all_found = false;
            }
        }
    }

    if all_found {
        report.push(TestResult::pass(
            validator,
            "All Amendment 41 tower chain vocabulary present (7 classes, 7 QT_ identities)",
        ));
    }
}

/// Validates that the EBNF grammar operations align with the ontology's
/// PrimitiveOp individuals and QuantumLevel individuals.
fn validate_ebnf_grammar_alignment(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/ebnf_grammar_alignment";

    // The EBNF grammar references 10 PrimitiveOp operations and 4 quantum levels.
    let expected_ops = [
        "neg", "bnot", "succ", "pred", "add", "sub", "mul", "xor", "and", "or",
    ];
    let expected_levels = ["Q0", "Q1", "Q2", "Q3"];

    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let schema_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "schema");

    let mut all_found = true;

    if let Some(op_mod) = op_ns {
        for op_name in &expected_ops {
            let iri = format!("https://uor.foundation/op/{}", op_name);
            if !op_mod.individuals.iter().any(|ind| ind.id == iri.as_str()) {
                report.push(TestResult::fail(
                    validator,
                    format!(
                        "EBNF grammar references op:{} but individual not found",
                        op_name
                    ),
                ));
                all_found = false;
            }
        }
    } else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    }

    if let Some(schema_mod) = schema_ns {
        for level in &expected_levels {
            let iri = format!("https://uor.foundation/schema/{}", level);
            if !schema_mod
                .individuals
                .iter()
                .any(|ind| ind.id == iri.as_str())
            {
                report.push(TestResult::fail(
                    validator,
                    format!(
                        "EBNF grammar references schema:{} but individual not found",
                        level
                    ),
                ));
                all_found = false;
            }
        }
    } else {
        report.push(TestResult::fail(validator, "schema/ namespace not found"));
        return;
    }

    if all_found {
        report.push(TestResult::pass(
            validator,
            "EBNF grammar operations and quantum levels align with ontology (10 ops, 4 levels)",
        ));
    }
}

/// Validates that Amendment 43 cryptographic primitive pinning properties
/// exist with correct domain, range, and functionality.
fn validate_crypto_pinning_vocabulary(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/crypto_pinning_vocabulary";

    let u_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "u");

    let Some(u_mod) = u_ns else {
        report.push(TestResult::fail(validator, "u/ namespace not found"));
        return;
    };

    let mut all_valid = true;

    // Check u:digestAlgorithm
    let digest_alg = u_mod
        .properties
        .iter()
        .find(|p| p.id == "https://uor.foundation/u/digestAlgorithm");
    if let Some(prop) = digest_alg {
        if prop.domain != Some("https://uor.foundation/u/Address") {
            report.push(TestResult::fail(
                validator,
                "u:digestAlgorithm has incorrect domain",
            ));
            all_valid = false;
        }
        if !prop.functional {
            report.push(TestResult::fail(
                validator,
                "u:digestAlgorithm must be functional",
            ));
            all_valid = false;
        }
    } else {
        report.push(TestResult::fail(
            validator,
            "u:digestAlgorithm property not found",
        ));
        all_valid = false;
    }

    // Check u:canonicalBytes
    let canonical = u_mod
        .properties
        .iter()
        .find(|p| p.id == "https://uor.foundation/u/canonicalBytes");
    if let Some(prop) = canonical {
        if prop.domain != Some("https://uor.foundation/u/Address") {
            report.push(TestResult::fail(
                validator,
                "u:canonicalBytes has incorrect domain",
            ));
            all_valid = false;
        }
        if prop.range != "http://www.w3.org/2001/XMLSchema#hexBinary" {
            report.push(TestResult::fail(
                validator,
                "u:canonicalBytes must have range xsd:hexBinary",
            ));
            all_valid = false;
        }
        if !prop.functional {
            report.push(TestResult::fail(
                validator,
                "u:canonicalBytes must be functional",
            ));
            all_valid = false;
        }
    } else {
        report.push(TestResult::fail(
            validator,
            "u:canonicalBytes property not found",
        ));
        all_valid = false;
    }

    if all_valid {
        report.push(TestResult::pass(
            validator,
            "Amendment 43 crypto pinning vocabulary present (digestAlgorithm, canonicalBytes)",
        ));
    }
}

/// Validates carry constraint fiber-pinning identities (Amendment 44, G7).
fn validate_carry_constraint_pinning(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/carry_constraint_pinning";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let expected = ["CC_PINS", "CC_COST_FIBER"];
    let mut all_found = true;
    for label in &expected {
        let has = op_mod.individuals.iter().any(|ind| ind.label == *label);
        if !has {
            report.push(TestResult::fail(
                validator,
                format!("Missing carry constraint pinning identity: {}", label),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 carry constraint pinning identities present (CC_PINS, CC_COST_FIBER)",
        ));
    }
}

/// Validates nerve joint satisfiability identities (Amendment 44, G1).
fn validate_joint_satisfiability_coverage(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/joint_satisfiability_coverage";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let expected = ["jsat_RR", "jsat_CR", "jsat_CC"];
    let mut all_found = true;
    for label in &expected {
        let has = op_mod.individuals.iter().any(|ind| ind.label == *label);
        if !has {
            report.push(TestResult::fail(
                validator,
                format!("Missing joint satisfiability identity: {}", label),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 joint satisfiability identities present (jsat_RR, jsat_CR, jsat_CC)",
        ));
    }
}

/// Validates dihedral algebra completeness (Amendment 44, G2).
fn validate_dihedral_algebra_completeness(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/dihedral_algebra_completeness";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let expected = ["D_7", "D_8", "D_9"];
    let mut all_found = true;
    for label in &expected {
        let has = op_mod.individuals.iter().any(|ind| ind.label == *label);
        if !has {
            report.push(TestResult::fail(
                validator,
                format!("Missing dihedral algebra identity: {}", label),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 dihedral algebra identities present (D_7, D_8, D_9)",
        ));
    }
}

/// Validates constraint expressiveness boundary identities (Amendment 44, G5).
fn validate_constraint_expressiveness(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/constraint_expressiveness";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let expected = ["EXP_1", "EXP_2", "EXP_3"];
    let mut all_found = true;
    for label in &expected {
        let has = op_mod.individuals.iter().any(|ind| ind.label == *label);
        if !has {
            report.push(TestResult::fail(
                validator,
                format!("Missing constraint expressiveness identity: {}", label),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 constraint expressiveness identities present (EXP_1, EXP_2, EXP_3)",
        ));
    }
}

/// Validates SumType topological identity algebra (Amendment 44, G4).
fn validate_sumtype_topology(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/sumtype_topology";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let expected = ["ST_3", "ST_4", "ST_5"];
    let mut all_found = true;
    for label in &expected {
        let has = op_mod.individuals.iter().any(|ind| ind.label == *label);
        if !has {
            report.push(TestResult::fail(
                validator,
                format!("Missing SumType topology identity: {}", label),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 SumType topology identities present (ST_3, ST_4, ST_5)",
        ));
    }
}

/// Validates TypeSynthesis reachability domain completeness (Amendment 44, G3).
fn validate_synthesis_reachability(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/synthesis_reachability";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let expected = ["TS_8", "TS_9", "TS_10"];
    let mut all_found = true;
    for label in &expected {
        let has = op_mod.individuals.iter().any(|ind| ind.label == *label);
        if !has {
            report.push(TestResult::fail(
                validator,
                format!("Missing synthesis reachability identity: {}", label),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 synthesis reachability identities present (TS_8, TS_9, TS_10)",
        ));
    }
}

/// Validates ObstructionChain termination guarantee (Amendment 44, G6).
fn validate_obstruction_termination(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/obstruction_termination";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let expected = ["QT_8", "QT_9"];
    let mut all_found = true;
    for label in &expected {
        let has = op_mod.individuals.iter().any(|ind| ind.label == *label);
        if !has {
            report.push(TestResult::fail(
                validator,
                format!("Missing obstruction termination identity: {}", label),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 obstruction termination identities present (QT_8, QT_9)",
        ));
    }
}

/// Validates sheaf coefficient ring grounding (Amendment 44, G11).
fn validate_coefficient_ring_grounding(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/coefficient_ring_grounding";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let has = op_mod.individuals.iter().any(|ind| ind.label == "COEFF_1");
    if has {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 coefficient ring identity present (COEFF_1)",
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            "Missing coefficient ring identity: COEFF_1",
        ));
    }
}

/// Validates GluingObstruction resolver feedback (Amendment 44, G9).
fn validate_gluing_feedback(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/gluing_feedback";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let has = op_mod.individuals.iter().any(|ind| ind.label == "GO_1");
    if has {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 gluing feedback identity present (GO_1)",
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            "Missing gluing feedback identity: GO_1",
        ));
    }
}

/// Validates session saturation lifecycle bridge (Amendment 44, G8).
fn validate_session_saturation_bridge(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/session_saturation_bridge";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let expected = ["SR_6", "SR_7"];
    let mut all_found = true;
    for label in &expected {
        let has = op_mod.individuals.iter().any(|ind| ind.label == *label);
        if !has {
            report.push(TestResult::fail(
                validator,
                format!("Missing session saturation identity: {}", label),
            ));
            all_found = false;
        }
    }
    if all_found {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 session saturation identities present (SR_6, SR_7)",
        ));
    }
}

/// Validates SuperposedFiberState amplitude index characterization (Amendment 44, G10).
fn validate_amplitude_index_characterization(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "ontology/inventory/amplitude_index_characterization";
    let op_ns = ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == "op");
    let Some(op_mod) = op_ns else {
        report.push(TestResult::fail(validator, "op/ namespace not found"));
        return;
    };
    let has = op_mod.individuals.iter().any(|ind| ind.label == "QM_6");
    if has {
        report.push(TestResult::pass(
            validator,
            "Amendment 44 amplitude index identity present (QM_6)",
        ));
    } else {
        report.push(TestResult::fail(
            validator,
            "Missing amplitude index identity: QM_6",
        ));
    }
}

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

/// Amendment 45, Rule 1: Every direct `cert:Certificate` subclass must have
/// a governing `op:Identity` individual that references it.
fn validate_certificate_issuance_coverage(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "meta/certificate_issuance_coverage";
    let cert_root = "https://uor.foundation/cert/Certificate";

    // Collect ALL direct cert:Certificate subclasses across all namespaces
    let cert_subclasses: Vec<_> = ontology
        .namespaces
        .iter()
        .flat_map(|m| m.classes.iter())
        .filter(|c| c.subclass_of.contains(&cert_root))
        .collect();

    // Collect all op:Identity individuals
    let identity_type = "https://uor.foundation/op/Identity";
    let identities: Vec<_> = ontology
        .namespaces
        .iter()
        .flat_map(|m| m.individuals.iter())
        .filter(|i| i.type_ == identity_type)
        .collect();

    let mut all_governed = true;
    for cert_class in &cert_subclasses {
        let governed = identities.iter().any(|id| {
            id.properties.iter().any(|(_, v)| match v {
                IndividualValue::Str(s) => {
                    s.contains(cert_class.label) || s.contains(cert_class.id)
                }
                IndividualValue::IriRef(iri) => iri.contains(cert_class.id),
                IndividualValue::List(iris) => iris.iter().any(|iri| iri.contains(cert_class.id)),
                _ => false,
            })
        });

        if !governed {
            report.push_meta(TestResult::fail(
                validator,
                format!(
                    "cert:Certificate subclass {} has no governing Identity",
                    cert_class.label
                ),
            ));
            all_governed = false;
        }
    }

    if all_governed {
        report.push_meta(TestResult::pass(
            validator,
            format!(
                "All {} Certificate subclasses governed by Identities",
                cert_subclasses.len()
            ),
        ));
    }
}

/// Amendment 45, Rule 2: Strict bijection between `op:Identity` individuals
/// and proof individuals via `provesIdentity`.
fn validate_identity_proof_bijection(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "meta/identity_proof_bijection";

    let identity_type = "https://uor.foundation/op/Identity";
    let proves_prop = "https://uor.foundation/proof/provesIdentity";

    // All op:Identity IRIs
    let identity_iris: HashSet<&str> = ontology
        .namespaces
        .iter()
        .flat_map(|m| m.individuals.iter())
        .filter(|i| i.type_ == identity_type)
        .map(|i| i.id)
        .collect();

    // All provesIdentity targets from proof individuals
    let proved_iris: HashSet<&str> = ontology
        .namespaces
        .iter()
        .flat_map(|m| m.individuals.iter())
        .flat_map(|i| i.properties.iter())
        .filter(|(k, _)| *k == proves_prop)
        .filter_map(|(_, v)| {
            if let IndividualValue::IriRef(iri) = v {
                Some(*iri)
            } else {
                None
            }
        })
        .collect();

    let mut all_ok = true;

    // Direction 1: every Identity must be proved
    for iri in &identity_iris {
        if !proved_iris.contains(iri) {
            report.push_meta(TestResult::fail(
                validator,
                format!("Identity {} has no proof individual", iri),
            ));
            all_ok = false;
        }
    }

    // Direction 2: every proof target must be a valid Identity
    for iri in &proved_iris {
        if !identity_iris.contains(iri) {
            report.push_meta(TestResult::fail(
                validator,
                format!("Proof targets nonexistent Identity {}", iri),
            ));
            all_ok = false;
        }
    }

    if all_ok {
        report.push_meta(TestResult::pass(
            validator,
            format!(
                "Identity-proof bijection holds: {} identities, {} proofs",
                identity_iris.len(),
                proved_iris.len()
            ),
        ));
    }
}

/// Amendment 45, Rule 3: Every kernel/bridge class must appear in at least
/// one SHACL fixture.
fn validate_shacl_fixture_coverage(report: &mut ConformanceReport) {
    let ontology = uor_ontology::Ontology::full();
    let validator = "meta/shacl_fixture_coverage";

    // Classes in kernel and bridge spaces only
    let required_classes: Vec<_> = ontology
        .namespaces
        .iter()
        .filter(|m| matches!(m.namespace.space, Space::Kernel | Space::Bridge))
        .flat_map(|m| {
            let prefix = m.namespace.prefix;
            m.classes.iter().map(move |c| (prefix, c))
        })
        .collect();

    let all_fixtures = crate::tests::fixtures::all_fixture_sources();

    let mut all_covered = true;
    for (prefix, class) in &required_classes {
        // Extract local name from IRI (everything after last '/')
        let local_name = class.id.rsplit('/').next().unwrap_or("");
        let prefixed_form = format!("{}:{}", prefix, local_name);

        let covered = all_fixtures.iter().any(|src| {
            src.contains(&prefixed_form) || src.contains(class.id) || src.contains(class.label)
        });

        if !covered {
            report.push_meta(TestResult::fail(
                validator,
                format!(
                    "Kernel/bridge class {} has no SHACL fixture coverage",
                    class.label
                ),
            ));
            all_covered = false;
        }
    }

    if all_covered {
        report.push_meta(TestResult::pass(
            validator,
            format!(
                "All {} kernel/bridge classes have SHACL fixture coverage",
                required_classes.len()
            ),
        ));
    }
}
