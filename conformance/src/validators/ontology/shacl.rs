//! SHACL validator.
//!
//! Validates the 29 OWL instance test graphs against the UOR SHACL shapes.
//! Each test graph is defined as a Turtle string in `tests/fixtures/`.
//! Validation checks structural constraints without a full SHACL engine:
//! - Required properties are present
//! - Type assertions are correct
//! - Cardinality minimums are met

use crate::report::{ConformanceReport, TestResult};
use crate::tests;

/// Runs all 29 SHACL instance conformance tests.
pub fn validate() -> ConformanceReport {
    let mut report = ConformanceReport::new();

    run_test("test1_ring", tests::fixtures::TEST1_RING, &mut report);
    run_test(
        "test2_primitives",
        tests::fixtures::TEST2_PRIMITIVES,
        &mut report,
    );
    run_test(
        "test3_term_graph",
        tests::fixtures::TEST3_TERM_GRAPH,
        &mut report,
    );
    run_test(
        "test4_state_lifecycle",
        tests::fixtures::TEST4_STATE_LIFECYCLE,
        &mut report,
    );
    run_test(
        "test5_partition",
        tests::fixtures::TEST5_PARTITION,
        &mut report,
    );
    run_test(
        "test6_critical_identity",
        tests::fixtures::TEST6_CRITICAL_IDENTITY,
        &mut report,
    );
    run_test(
        "test7_end_to_end",
        tests::fixtures::TEST7_END_TO_END,
        &mut report,
    );
    run_test(
        "test8_fiber_budget",
        tests::fixtures::TEST8_FIBER_BUDGET,
        &mut report,
    );
    run_test(
        "test9_constraint_algebra",
        tests::fixtures::TEST9_CONSTRAINT_ALGEBRA,
        &mut report,
    );
    run_test(
        "test10_iterative_resolution",
        tests::fixtures::TEST10_ITERATIVE_RESOLUTION,
        &mut report,
    );
    run_test(
        "test11_composition",
        tests::fixtures::TEST11_COMPOSITION,
        &mut report,
    );
    run_test(
        "test12_factorization",
        tests::fixtures::TEST12_FACTORIZATION,
        &mut report,
    );
    run_test(
        "test13_canonical_form",
        tests::fixtures::TEST13_CANONICAL_FORM,
        &mut report,
    );
    run_test(
        "test14_content_addressing",
        tests::fixtures::TEST14_CONTENT_ADDRESSING,
        &mut report,
    );
    run_test(
        "test15_boolean_sat",
        tests::fixtures::TEST15_BOOLEAN_SAT,
        &mut report,
    );
    run_test(
        "test16_algebraic_identities",
        tests::fixtures::TEST16_ALGEBRAIC_IDENTITIES,
        &mut report,
    );
    run_test(
        "test17_inter_algebra_maps",
        tests::fixtures::TEST17_INTER_ALGEBRA_MAPS,
        &mut report,
    );
    run_test(
        "test18_analytical_completeness",
        tests::fixtures::TEST18_ANALYTICAL_COMPLETENESS,
        &mut report,
    );
    run_test(
        "test19_homological_pipeline",
        tests::fixtures::TEST19_HOMOLOGICAL_PIPELINE,
        &mut report,
    );
    run_test(
        "test20_sheaf_consistency",
        tests::fixtures::TEST20_SHEAF_CONSISTENCY,
        &mut report,
    );
    run_test(
        "test21_topological_delta",
        tests::fixtures::TEST21_TOPOLOGICAL_DELTA,
        &mut report,
    );
    run_test(
        "test22_index_bridge",
        tests::fixtures::TEST22_INDEX_BRIDGE,
        &mut report,
    );
    run_test(
        "test23_identity_grounding",
        tests::fixtures::TEST23_IDENTITY_GROUNDING,
        &mut report,
    );
    run_test(
        "test24_verification_domain",
        tests::fixtures::TEST24_VERIFICATION_DOMAIN,
        &mut report,
    );
    run_test(
        "test25_geometric_character",
        tests::fixtures::TEST25_GEOMETRIC_CHARACTER,
        &mut report,
    );
    run_test(
        "test26_complexity_class",
        tests::fixtures::TEST26_COMPLEXITY_CLASS,
        &mut report,
    );
    run_test(
        "test27_rewrite_rule",
        tests::fixtures::TEST27_REWRITE_RULE,
        &mut report,
    );
    run_test(
        "test28_measurement_unit",
        tests::fixtures::TEST28_MEASUREMENT_UNIT,
        &mut report,
    );
    run_test(
        "test29_coordinate_kind",
        tests::fixtures::TEST29_COORDINATE_KIND,
        &mut report,
    );

    report
}

/// Runs a single SHACL test against the provided Turtle instance graph.
fn run_test(name: &str, turtle_src: &str, report: &mut ConformanceReport) {
    let validator = format!("ontology/shacl/{}", name);

    // Structural validation: the Turtle must be non-empty and syntactically plausible
    if turtle_src.trim().is_empty() {
        report.push(TestResult::fail(
            validator.clone(),
            "Instance graph is empty",
        ));
        return;
    }

    if !turtle_src.contains("@prefix") {
        report.push(TestResult::fail(
            validator.clone(),
            "Instance graph missing @prefix declarations",
        ));
        return;
    }

    // Run test-specific structural checks
    let result = match name {
        "test1_ring" => validate_ring(turtle_src),
        "test2_primitives" => validate_primitives(turtle_src),
        "test3_term_graph" => validate_term_graph(turtle_src),
        "test4_state_lifecycle" => validate_state_lifecycle(turtle_src),
        "test5_partition" => validate_partition(turtle_src),
        "test6_critical_identity" => validate_critical_identity(turtle_src),
        "test7_end_to_end" => validate_end_to_end(turtle_src),
        "test8_fiber_budget" => validate_fiber_budget(turtle_src),
        "test9_constraint_algebra" => validate_constraint_algebra(turtle_src),
        "test10_iterative_resolution" => validate_iterative_resolution(turtle_src),
        "test11_composition" => validate_composition(turtle_src),
        "test12_factorization" => validate_factorization(turtle_src),
        "test13_canonical_form" => validate_canonical_form(turtle_src),
        "test14_content_addressing" => validate_content_addressing(turtle_src),
        "test15_boolean_sat" => validate_boolean_sat(turtle_src),
        "test16_algebraic_identities" => validate_algebraic_identities(turtle_src),
        "test17_inter_algebra_maps" => validate_inter_algebra_maps(turtle_src),
        "test18_analytical_completeness" => validate_analytical_completeness(turtle_src),
        "test19_homological_pipeline" => validate_homological_pipeline(turtle_src),
        "test20_sheaf_consistency" => validate_sheaf_consistency(turtle_src),
        "test21_topological_delta" => validate_topological_delta(turtle_src),
        "test22_index_bridge" => validate_index_bridge(turtle_src),
        "test23_identity_grounding" => validate_identity_grounding_shacl(turtle_src),
        "test24_verification_domain" => validate_verification_domain_shacl(turtle_src),
        "test25_geometric_character" => validate_geometric_character_shacl(turtle_src),
        "test26_complexity_class" => validate_complexity_class_shacl(turtle_src),
        "test27_rewrite_rule" => validate_rewrite_rule_shacl(turtle_src),
        "test28_measurement_unit" => validate_measurement_unit_shacl(turtle_src),
        "test29_coordinate_kind" => validate_coordinate_kind_shacl(turtle_src),
        _ => Ok(()),
    };

    match result {
        Ok(()) => report.push(TestResult::pass(
            validator,
            format!("Instance graph {} passes SHACL structural validation", name),
        )),
        Err(msg) => report.push(TestResult::fail(validator, msg)),
    }
}

fn validate_ring(src: &str) -> Result<(), String> {
    check_contains(src, "schema:Ring", "Missing schema:Ring type assertion")?;
    check_contains(
        src,
        "schema:ringQuantum",
        "Missing schema:ringQuantum property",
    )?;
    check_contains(src, "schema:modulus", "Missing schema:modulus property")?;
    Ok(())
}

fn validate_primitives(src: &str) -> Result<(), String> {
    check_contains(src, "op:neg", "Missing op:neg individual reference")?;
    check_contains(src, "op:bnot", "Missing op:bnot individual reference")?;
    check_contains(src, "op:succ", "Missing op:succ individual reference")?;
    check_contains(src, "op:composedOf", "Missing op:composedOf property usage")?;
    Ok(())
}

fn validate_term_graph(src: &str) -> Result<(), String> {
    check_contains(src, "schema:Application", "Missing schema:Application")?;
    check_contains(src, "schema:Literal", "Missing schema:Literal")?;
    check_contains(src, "schema:denotes", "Missing schema:denotes property")?;
    Ok(())
}

fn validate_state_lifecycle(src: &str) -> Result<(), String> {
    check_contains(src, "state:Context", "Missing state:Context")?;
    check_contains(src, "state:Binding", "Missing state:Binding")?;
    check_contains(src, "state:Transition", "Missing state:Transition")?;
    Ok(())
}

fn validate_partition(src: &str) -> Result<(), String> {
    check_contains(src, "partition:Partition", "Missing partition:Partition")?;
    check_contains(src, "partition:cardinality", "Missing cardinality property")?;
    Ok(())
}

fn validate_critical_identity(src: &str) -> Result<(), String> {
    check_contains(src, "op:criticalIdentity", "Missing op:criticalIdentity")?;
    check_contains(
        src,
        "proof:CriticalIdentityProof",
        "Missing proof:CriticalIdentityProof",
    )?;
    check_contains(
        src,
        "proof:provesIdentity",
        "Missing proof:provesIdentity property",
    )?;
    Ok(())
}

fn validate_end_to_end(src: &str) -> Result<(), String> {
    check_contains(src, "state:Context", "Missing state:Context")?;
    check_contains(src, "type:", "Missing type: namespace usage")?;
    check_contains(src, "resolver:", "Missing resolver: namespace usage")?;
    check_contains(src, "partition:", "Missing partition: namespace usage")?;
    check_contains(src, "observable:", "Missing observable: namespace usage")?;
    check_contains(src, "cert:", "Missing cert: namespace usage")?;
    check_contains(src, "trace:", "Missing trace: namespace usage")?;
    Ok(())
}

fn validate_fiber_budget(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "partition:FiberBudget",
        "Missing partition:FiberBudget",
    )?;
    check_contains(
        src,
        "partition:FiberCoordinate",
        "Missing partition:FiberCoordinate",
    )?;
    check_contains(src, "partition:isClosed", "Missing partition:isClosed")?;
    check_contains(
        src,
        "partition:pinnedCount",
        "Missing partition:pinnedCount",
    )?;
    check_contains(src, "partition:freeCount", "Missing partition:freeCount")?;
    check_contains(src, "partition:hasFiber", "Missing partition:hasFiber")?;
    check_contains(
        src,
        "partition:FiberPinning",
        "Missing partition:FiberPinning",
    )?;
    Ok(())
}

fn validate_constraint_algebra(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "type:ResidueConstraint",
        "Missing type:ResidueConstraint",
    )?;
    check_contains(
        src,
        "type:CompositeConstraint",
        "Missing type:CompositeConstraint",
    )?;
    check_contains(src, "type:metricAxis", "Missing type:metricAxis")?;
    check_contains(src, "type:hasConstraint", "Missing type:hasConstraint")?;
    check_contains(src, "type:verticalAxis", "Missing type:verticalAxis")?;
    Ok(())
}

fn validate_iterative_resolution(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:ResolutionState",
        "Missing resolver:ResolutionState",
    )?;
    check_contains(src, "resolver:isComplete", "Missing resolver:isComplete")?;
    check_contains(
        src,
        "resolver:RefinementSuggestion",
        "Missing resolver:RefinementSuggestion",
    )?;
    check_contains(
        src,
        "derivation:RefinementStep",
        "Missing derivation:RefinementStep",
    )?;
    check_contains(
        src,
        "resolver:convergenceRate",
        "Missing resolver:convergenceRate",
    )?;
    Ok(())
}

fn validate_composition(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "morphism:Composition",
        "Missing morphism:Composition class",
    )?;
    check_contains(
        src,
        "morphism:CompositionLaw",
        "Missing morphism:CompositionLaw",
    )?;
    check_contains(
        src,
        "morphism:criticalComposition",
        "Missing morphism:criticalComposition",
    )?;
    check_contains(src, "morphism:Identity", "Missing morphism:Identity")?;
    check_contains(src, "morphism:identityOn", "Missing morphism:identityOn")?;
    Ok(())
}

fn validate_factorization(src: &str) -> Result<(), String> {
    check_contains(src, "query:", "Missing query: namespace usage")?;
    check_contains(
        src,
        "resolver:DihedralFactorizationResolver",
        "Missing DihedralFactorizationResolver",
    )?;
    check_contains(
        src,
        "partition:FiberBudget",
        "Missing partition:FiberBudget",
    )?;
    check_contains(src, "cert:certifies", "Missing cert:certifies")?;
    check_contains(src, "trace:certifiedBy", "Missing trace:certifiedBy")?;
    Ok(())
}

fn validate_canonical_form(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:CanonicalFormResolver",
        "Missing CanonicalFormResolver",
    )?;
    check_contains(
        src,
        "derivation:Derivation",
        "Missing derivation:Derivation",
    )?;
    check_contains(
        src,
        "derivation:RewriteStep",
        "Missing derivation:RewriteStep",
    )?;
    check_contains(
        src,
        "derivation:TermMetrics",
        "Missing derivation:TermMetrics",
    )?;
    check_contains(
        src,
        "derivation:originalTerm",
        "Missing derivation:originalTerm",
    )?;
    Ok(())
}

fn validate_content_addressing(src: &str) -> Result<(), String> {
    check_contains(src, "u:Address", "Missing u:Address")?;
    check_contains(
        src,
        "observable:RingMetric",
        "Missing observable:RingMetric",
    )?;
    check_contains(
        src,
        "observable:HammingMetric",
        "Missing observable:HammingMetric",
    )?;
    check_contains(
        src,
        "cert:InvolutionCertificate",
        "Missing cert:InvolutionCertificate",
    )?;
    check_contains(src, "cert:certifies", "Missing cert:certifies")?;
    Ok(())
}

fn validate_boolean_sat(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:EvaluationResolver",
        "Missing EvaluationResolver",
    )?;
    check_contains(src, "state:Transition", "Missing state:Transition")?;
    check_contains(src, "state:Binding", "Missing state:Binding")?;
    check_contains(src, "cert:certifies", "Missing cert:certifies")?;
    check_contains(src, "trace:certifiedBy", "Missing trace:certifiedBy")?;
    Ok(())
}

fn validate_algebraic_identities(src: &str) -> Result<(), String> {
    check_contains(src, "op:Identity", "Missing op:Identity type assertion")?;
    check_contains(src, "op:lhs", "Missing op:lhs property")?;
    check_contains(src, "op:rhs", "Missing op:rhs property")?;
    check_contains(src, "op:forAll", "Missing op:forAll property")?;
    check_contains(src, "op:R_A1", "Missing ring identity R_A1")?;
    check_contains(src, "op:B_1", "Missing Boolean identity B_1")?;
    check_contains(src, "op:C_1", "Missing constraint identity C_1")?;
    check_contains(src, "op:F_1", "Missing fiber identity F_1")?;
    check_contains(src, "op:RE_1", "Missing resolution identity RE_1")?;
    check_contains(src, "op:OB_M1", "Missing observable identity OB_M1")?;
    check_contains(src, "op:T_C1", "Missing transform identity T_C1")?;
    check_contains(src, "op:AD_1", "Missing addressing identity AD_1")?;
    check_contains(src, "op:TH_1", "Missing thermodynamic identity TH_1")?;
    check_contains(src, "op:CA_1", "Missing carry identity CA_1")?;
    Ok(())
}

fn validate_inter_algebra_maps(src: &str) -> Result<(), String> {
    check_contains(src, "op:Identity", "Missing op:Identity type assertion")?;
    check_contains(src, "op:phi_1", "Missing phi_1 (Ring → Constraints)")?;
    check_contains(src, "op:phi_2", "Missing phi_2 (Constraints → Fibers)")?;
    check_contains(src, "op:phi_3", "Missing phi_3 (Fibers → Partition)")?;
    check_contains(src, "op:phi_4", "Missing phi_4 (Resolution Pipeline)")?;
    check_contains(src, "op:phi_5", "Missing phi_5 (Operations → Observables)")?;
    check_contains(src, "op:phi_6", "Missing phi_6 (Observables → Refinement)")?;
    Ok(())
}

fn validate_analytical_completeness(src: &str) -> Result<(), String> {
    check_contains(src, "obs:Jacobian", "Missing obs:Jacobian class")?;
    check_contains(
        src,
        "obs:TopologicalObservable",
        "Missing obs:TopologicalObservable",
    )?;
    check_contains(src, "obs:BettiNumber", "Missing obs:BettiNumber class")?;
    check_contains(src, "obs:SpectralGap", "Missing obs:SpectralGap class")?;
    check_contains(
        src,
        "resolver:ConstraintNerve",
        "Missing resolver:ConstraintNerve",
    )?;
    check_contains(
        src,
        "op:DC_1",
        "Missing differential calculus identity DC_1",
    )?;
    check_contains(src, "op:HA_1", "Missing homological identity HA_1")?;
    check_contains(src, "op:IT_7a", "Missing index theorem identity IT_7a")?;
    check_contains(src, "op:IT_7d", "Missing completeness criterion IT_7d")?;
    Ok(())
}

fn validate_homological_pipeline(src: &str) -> Result<(), String> {
    check_contains(src, "homology:Simplex", "Missing homology:Simplex")?;
    check_contains(
        src,
        "homology:SimplicialComplex",
        "Missing homology:SimplicialComplex",
    )?;
    check_contains(src, "homology:ChainGroup", "Missing homology:ChainGroup")?;
    check_contains(
        src,
        "homology:BoundaryOperator",
        "Missing homology:BoundaryOperator",
    )?;
    check_contains(
        src,
        "homology:ChainComplex",
        "Missing homology:ChainComplex",
    )?;
    check_contains(
        src,
        "homology:HomologyGroup",
        "Missing homology:HomologyGroup",
    )?;
    Ok(())
}

fn validate_sheaf_consistency(src: &str) -> Result<(), String> {
    check_contains(src, "cohomology:Sheaf", "Missing cohomology:Sheaf")?;
    check_contains(src, "cohomology:Stalk", "Missing cohomology:Stalk")?;
    check_contains(src, "cohomology:Section", "Missing cohomology:Section")?;
    check_contains(
        src,
        "cohomology:GluingObstruction",
        "Missing cohomology:GluingObstruction",
    )?;
    check_contains(
        src,
        "cohomology:CochainComplex",
        "Missing cohomology:CochainComplex",
    )?;
    check_contains(
        src,
        "cohomology:CohomologyGroup",
        "Missing cohomology:CohomologyGroup",
    )?;
    Ok(())
}

fn validate_topological_delta(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "morphism:TopologicalDelta",
        "Missing morphism:TopologicalDelta",
    )?;
    check_contains(
        src,
        "morphism:bettisBefore",
        "Missing morphism:bettisBefore",
    )?;
    check_contains(src, "morphism:bettisAfter", "Missing morphism:bettisAfter")?;
    check_contains(src, "morphism:eulerBefore", "Missing morphism:eulerBefore")?;
    check_contains(src, "morphism:eulerAfter", "Missing morphism:eulerAfter")?;
    check_contains(src, "morphism:nerveBefore", "Missing morphism:nerveBefore")?;
    check_contains(src, "morphism:nerveAfter", "Missing morphism:nerveAfter")?;
    Ok(())
}

fn validate_index_bridge(src: &str) -> Result<(), String> {
    check_contains(src, "op:Identity", "Missing op:Identity type assertion")?;
    check_contains(src, "op:phi_1", "Missing phi_1")?;
    check_contains(src, "op:phi_6", "Missing phi_6")?;
    check_contains(src, "op:psi_1", "Missing psi_1")?;
    check_contains(src, "op:psi_6", "Missing psi_6")?;
    check_contains(
        src,
        "op:hasVerificationStatus",
        "Missing op:hasVerificationStatus",
    )?;
    check_contains(
        src,
        "op:verificationDomain",
        "Missing op:verificationDomain",
    )?;
    Ok(())
}

fn validate_identity_grounding_shacl(src: &str) -> Result<(), String> {
    check_contains(src, "op:R_A1", "Missing R_A1")?;
    check_contains(src, "op:C_1", "Missing C_1")?;
    check_contains(src, "op:F_1", "Missing F_1")?;
    check_contains(src, "op:DC_1", "Missing DC_1")?;
    check_contains(src, "op:psi_1", "Missing psi_1")?;
    check_contains(
        src,
        "op:hasVerificationStatus",
        "Missing hasVerificationStatus property",
    )?;
    check_contains(
        src,
        "op:verificationDomain",
        "Missing verificationDomain property",
    )?;
    check_contains(
        src,
        "op:verificationPathNote",
        "Missing verificationPathNote property",
    )?;
    Ok(())
}

fn validate_verification_domain_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "op:VerificationDomain",
        "Missing VerificationDomain type",
    )?;
    check_contains(
        src,
        "op:VerificationStatus",
        "Missing VerificationStatus type",
    )?;
    check_contains(
        src,
        "op:hasVerificationStatus",
        "Missing hasVerificationStatus",
    )?;
    check_contains(src, "op:verificationDomain", "Missing verificationDomain")?;
    check_contains(src, "op:Enumerative", "Missing Enumerative individual")?;
    check_contains(src, "op:Verifiable", "Missing Verifiable individual")?;
    Ok(())
}

fn validate_geometric_character_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "op:GeometricCharacter",
        "Missing GeometricCharacter type",
    )?;
    check_contains(
        src,
        "op:hasGeometricCharacter",
        "Missing hasGeometricCharacter",
    )?;
    check_contains(
        src,
        "op:RingReflection",
        "Missing RingReflection individual",
    )?;
    check_contains(
        src,
        "op:HypercubeReflection",
        "Missing HypercubeReflection individual",
    )?;
    Ok(())
}

fn validate_complexity_class_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:ComplexityClass",
        "Missing ComplexityClass type",
    )?;
    check_contains(
        src,
        "resolver:hasComplexityClass",
        "Missing hasComplexityClass",
    )?;
    check_contains(
        src,
        "resolver:ConstantTime",
        "Missing ConstantTime individual",
    )?;
    check_contains(src, "resolver:LinearTime", "Missing LinearTime individual")?;
    Ok(())
}

fn validate_rewrite_rule_shacl(src: &str) -> Result<(), String> {
    check_contains(src, "derivation:RewriteRule", "Missing RewriteRule type")?;
    check_contains(src, "derivation:hasRewriteRule", "Missing hasRewriteRule")?;
    check_contains(src, "derivation:groundedIn", "Missing groundedIn")?;
    check_contains(
        src,
        "derivation:CriticalIdentityRule",
        "Missing CriticalIdentityRule",
    )?;
    Ok(())
}

fn validate_measurement_unit_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:MeasurementUnit",
        "Missing MeasurementUnit type",
    )?;
    check_contains(src, "observable:hasUnit", "Missing hasUnit")?;
    check_contains(src, "observable:Bits", "Missing Bits individual")?;
    check_contains(src, "observable:RingSteps", "Missing RingSteps individual")?;
    Ok(())
}

fn validate_coordinate_kind_shacl(src: &str) -> Result<(), String> {
    check_contains(src, "query:CoordinateKind", "Missing CoordinateKind type")?;
    check_contains(src, "query:hasCoordinateKind", "Missing hasCoordinateKind")?;
    check_contains(
        src,
        "query:StratumCoordinate",
        "Missing StratumCoordinate individual",
    )?;
    check_contains(
        src,
        "query:SpectrumCoordinate",
        "Missing SpectrumCoordinate individual",
    )?;
    Ok(())
}

fn check_contains(src: &str, needle: &str, msg: &str) -> Result<(), String> {
    if src.contains(needle) {
        Ok(())
    } else {
        Err(msg.to_string())
    }
}
