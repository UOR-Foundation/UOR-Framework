//! SHACL validator.
//!
//! Validates the 100 OWL instance test graphs against the UOR SHACL shapes.
//! Each test graph is defined as a Turtle string in `tests/fixtures/`.
//! Validation checks structural constraints without a full SHACL engine:
//! - Required properties are present
//! - Type assertions are correct
//! - Cardinality minimums are met

use uor_ontology::counts;

use crate::report::{ConformanceReport, TestResult};
use crate::tests;

/// Runs all 110 SHACL instance conformance tests.
pub fn validate() -> ConformanceReport {
    let mut report = ConformanceReport::new();
    let before_tests = report.results.len();

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
    run_test(
        "test30_proof_coverage",
        tests::fixtures::TEST30_PROOF_COVERAGE,
        &mut report,
    );
    run_test(
        "test31_quantum_level",
        tests::fixtures::TEST31_QUANTUM_LEVEL,
        &mut report,
    );
    run_test(
        "test32_arc_grounding",
        tests::fixtures::TEST32_ARC_GROUNDING,
        &mut report,
    );
    run_test(
        "test33_graph_gaps",
        tests::fixtures::TEST33_GRAPH_GAPS,
        &mut report,
    );
    run_test(
        "test34_completeness_candidate",
        tests::fixtures::TEST34_COMPLETENESS_CANDIDATE,
        &mut report,
    );
    run_test(
        "test35_completeness_certificate",
        tests::fixtures::TEST35_COMPLETENESS_CERTIFICATE,
        &mut report,
    );
    run_test(
        "test36_q1_ring",
        tests::fixtures::TEST36_Q1_RING,
        &mut report,
    );
    run_test(
        "test37_quantum_level_binding",
        tests::fixtures::TEST37_QUANTUM_LEVEL_BINDING,
        &mut report,
    );
    run_test(
        "test38_session_lifecycle",
        tests::fixtures::TEST38_SESSION_LIFECYCLE,
        &mut report,
    );
    run_test(
        "test39_session_boundary",
        tests::fixtures::TEST39_SESSION_BOUNDARY,
        &mut report,
    );
    run_test(
        "test40_type_synthesis_goal",
        tests::fixtures::TEST40_TYPE_SYNTHESIS_GOAL,
        &mut report,
    );
    run_test(
        "test41_synthesis_result",
        tests::fixtures::TEST41_SYNTHESIS_RESULT,
        &mut report,
    );
    run_test(
        "test42_quantum_lift",
        tests::fixtures::TEST42_QUANTUM_LIFT,
        &mut report,
    );
    run_test(
        "test43_spectral_sequence",
        tests::fixtures::TEST43_SPECTRAL_SEQUENCE,
        &mut report,
    );
    run_test(
        "test44_monodromy_flat",
        tests::fixtures::TEST44_MONODROMY_FLAT,
        &mut report,
    );
    run_test(
        "test45_monodromy_twisted",
        tests::fixtures::TEST45_MONODROMY_TWISTED,
        &mut report,
    );
    run_test(
        "test46_monodromy_pipeline",
        tests::fixtures::TEST46_MONODROMY_PIPELINE,
        &mut report,
    );
    run_test(
        "test47_thermo_pipeline",
        tests::fixtures::TEST47_THERMO_PIPELINE,
        &mut report,
    );
    run_test(
        "test48_phase_diagram",
        tests::fixtures::TEST48_PHASE_DIAGRAM,
        &mut report,
    );
    run_test(
        "test49_reversible_resolution",
        tests::fixtures::TEST49_REVERSIBLE_RESOLUTION,
        &mut report,
    );
    run_test(
        "test50_jacobian_resolver",
        tests::fixtures::TEST50_JACOBIAN_RESOLVER,
        &mut report,
    );
    run_test(
        "test51_product_type_pipeline",
        tests::fixtures::TEST51_PRODUCT_TYPE_PIPELINE,
        &mut report,
    );
    run_test(
        "test52_sum_type_variant",
        tests::fixtures::TEST52_SUM_TYPE_VARIANT,
        &mut report,
    );
    run_test(
        "test53_superposed_fiber",
        tests::fixtures::TEST53_SUPERPOSED_FIBER,
        &mut report,
    );
    run_test(
        "test54_saturated_context",
        tests::fixtures::TEST54_SATURATED_CONTEXT,
        &mut report,
    );
    run_test(
        "test55_saturation_witness",
        tests::fixtures::TEST55_SATURATION_WITNESS,
        &mut report,
    );
    run_test(
        "test56_domain_saturation_record",
        tests::fixtures::TEST56_DOMAIN_SATURATION_RECORD,
        &mut report,
    );
    run_test(
        "test57_saturation_phase",
        tests::fixtures::TEST57_SATURATION_PHASE,
        &mut report,
    );
    run_test(
        "test58_saturation_certificate",
        tests::fixtures::TEST58_SATURATION_CERTIFICATE,
        &mut report,
    );
    run_test(
        "test59_saturation_aware_resolver",
        tests::fixtures::TEST59_SATURATION_AWARE_RESOLVER,
        &mut report,
    );
    run_test(
        "test60_impossibility_witness",
        tests::fixtures::TEST60_IMPOSSIBILITY_WITNESS,
        &mut report,
    );
    run_test(
        "test61_morphospace_record",
        tests::fixtures::TEST61_MORPHOSPACE_RECORD,
        &mut report,
    );
    run_test(
        "test62_morphospace_boundary",
        tests::fixtures::TEST62_MORPHOSPACE_BOUNDARY,
        &mut report,
    );
    run_test(
        "test63_forbidden_signature",
        tests::fixtures::TEST63_FORBIDDEN_SIGNATURE,
        &mut report,
    );
    run_test(
        "test64_achievability_status",
        tests::fixtures::TEST64_ACHIEVABILITY_STATUS,
        &mut report,
    );
    run_test(
        "test65_geodesic_trace",
        tests::fixtures::TEST65_GEODESIC_TRACE,
        &mut report,
    );
    run_test(
        "test66_geodesic_certificate",
        tests::fixtures::TEST66_GEODESIC_CERTIFICATE,
        &mut report,
    );
    run_test(
        "test67_geodesic_violation",
        tests::fixtures::TEST67_GEODESIC_VIOLATION,
        &mut report,
    );
    run_test(
        "test68_geodesic_validator",
        tests::fixtures::TEST68_GEODESIC_VALIDATOR,
        &mut report,
    );
    run_test(
        "test69_geodesic_ordered",
        tests::fixtures::TEST69_GEODESIC_ORDERED,
        &mut report,
    );
    run_test(
        "test70_measurement_resolver",
        tests::fixtures::TEST70_MEASUREMENT_RESOLVER,
        &mut report,
    );
    run_test(
        "test71_measurement_event",
        tests::fixtures::TEST71_MEASUREMENT_EVENT,
        &mut report,
    );
    run_test(
        "test72_measurement_certificate",
        tests::fixtures::TEST72_MEASUREMENT_CERTIFICATE,
        &mut report,
    );
    run_test(
        "test73_collapsed_fiber_state",
        tests::fixtures::TEST73_COLLAPSED_FIBER_STATE,
        &mut report,
    );
    run_test(
        "test74_quantum_thermodynamic",
        tests::fixtures::TEST74_QUANTUM_THERMODYNAMIC,
        &mut report,
    );
    // Amendment 37: Gap Closure tests
    run_test(
        "test75_partition_product",
        tests::fixtures::TEST75_PARTITION_PRODUCT,
        &mut report,
    );
    run_test(
        "test76_partition_coproduct",
        tests::fixtures::TEST76_PARTITION_COPRODUCT,
        &mut report,
    );
    run_test(
        "test77_geodesic_evidence",
        tests::fixtures::TEST77_GEODESIC_EVIDENCE,
        &mut report,
    );
    run_test(
        "test78_born_rule",
        tests::fixtures::TEST78_BORN_RULE,
        &mut report,
    );
    run_test(
        "test79_measurement_outcome",
        tests::fixtures::TEST79_MEASUREMENT_OUTCOME,
        &mut report,
    );
    run_test(
        "test80_partition_exhaustive",
        tests::fixtures::TEST80_PARTITION_EXHAUSTIVE,
        &mut report,
    );
    run_test(
        "test81_dihedral_algebra",
        tests::fixtures::TEST81_DIHEDRAL_ALGEBRA,
        &mut report,
    );
    run_test(
        "test82_level_successor",
        tests::fixtures::TEST82_LEVEL_SUCCESSOR,
        &mut report,
    );
    run_test(
        "test83_amplitude_normalization",
        tests::fixtures::TEST83_AMPLITUDE_NORMALIZATION,
        &mut report,
    );
    run_test(
        "test84_enum_variant",
        tests::fixtures::TEST84_ENUM_VARIANT,
        &mut report,
    );
    // Amendment 39: Q1 Lift fixtures
    run_test(
        "test85_q1_ring_grounding",
        tests::fixtures::TEST85_Q1_RING_GROUNDING,
        &mut report,
    );
    run_test(
        "test86_quantum_lift_trivial",
        tests::fixtures::TEST86_QUANTUM_LIFT_TRIVIAL,
        &mut report,
    );
    run_test(
        "test87_spectral_convergence",
        tests::fixtures::TEST87_SPECTRAL_CONVERGENCE,
        &mut report,
    );
    run_test(
        "test88_lift_obstruction_nontrivial",
        tests::fixtures::TEST88_LIFT_OBSTRUCTION_NONTRIVIAL,
        &mut report,
    );
    run_test(
        "test89_lift_refinement_suggestion",
        tests::fixtures::TEST89_LIFT_REFINEMENT_SUGGESTION,
        &mut report,
    );
    run_test(
        "test90_resolved_lift",
        tests::fixtures::TEST90_RESOLVED_LIFT,
        &mut report,
    );
    // Amendment 39: Q1 Inverse Pipeline fixtures
    run_test(
        "test91_synthesis_goal_q1",
        tests::fixtures::TEST91_SYNTHESIS_GOAL_Q1,
        &mut report,
    );
    run_test(
        "test92_synthesis_checkpoint",
        tests::fixtures::TEST92_SYNTHESIS_CHECKPOINT,
        &mut report,
    );
    run_test(
        "test93_synthesis_signature",
        tests::fixtures::TEST93_SYNTHESIS_SIGNATURE,
        &mut report,
    );
    run_test(
        "test94_synthesized_type",
        tests::fixtures::TEST94_SYNTHESIZED_TYPE,
        &mut report,
    );
    run_test(
        "test95_unreachable_signature",
        tests::fixtures::TEST95_UNREACHABLE_SIGNATURE,
        &mut report,
    );
    // Amendment 40: Q1 Normative Certification fixtures
    run_test(
        "test96_geodesic_trace_q1",
        tests::fixtures::TEST96_GEODESIC_TRACE_Q1,
        &mut report,
    );
    run_test(
        "test97_evidence_bundle_ar1",
        tests::fixtures::TEST97_EVIDENCE_BUNDLE_AR1,
        &mut report,
    );
    run_test(
        "test98_evidence_bundle_dc10",
        tests::fixtures::TEST98_EVIDENCE_BUNDLE_DC10,
        &mut report,
    );
    run_test(
        "test99_measurement_born_q1",
        tests::fixtures::TEST99_MEASUREMENT_BORN_Q1,
        &mut report,
    );
    run_test(
        "test100_normative_chain",
        tests::fixtures::TEST100_NORMATIVE_CHAIN,
        &mut report,
    );
    // Amendment 41: Tower chain test fixtures
    run_test(
        "test101_lift_chain_flat",
        tests::fixtures::TEST101_LIFT_CHAIN_FLAT,
        &mut report,
    );
    run_test(
        "test102_lift_chain_twisted",
        tests::fixtures::TEST102_LIFT_CHAIN_TWISTED,
        &mut report,
    );
    run_test(
        "test103_obstruction_chain_empty",
        tests::fixtures::TEST103_OBSTRUCTION_CHAIN_EMPTY,
        &mut report,
    );
    run_test(
        "test104_obstruction_chain_nontrivial",
        tests::fixtures::TEST104_OBSTRUCTION_CHAIN_NONTRIVIAL,
        &mut report,
    );
    run_test(
        "test105_lift_chain_certificate",
        tests::fixtures::TEST105_LIFT_CHAIN_CERTIFICATE,
        &mut report,
    );
    run_test(
        "test106_chain_audit_trail",
        tests::fixtures::TEST106_CHAIN_AUDIT_TRAIL,
        &mut report,
    );
    run_test(
        "test107_tower_resolver",
        tests::fixtures::TEST107_TOWER_RESOLVER,
        &mut report,
    );
    run_test(
        "test108_inductive_proof",
        tests::fixtures::TEST108_INDUCTIVE_PROOF,
        &mut report,
    );
    run_test(
        "test109_validity_scope",
        tests::fixtures::TEST109_VALIDITY_SCOPE,
        &mut report,
    );
    run_test(
        "test110_tower_roundtrip",
        tests::fixtures::TEST110_TOWER_ROUNDTRIP,
        &mut report,
    );
    // Amendment 43: Cryptographic Primitive Pinning
    run_test(
        "test111_address_crypto_pinning",
        tests::fixtures::TEST111_ADDRESS_CRYPTO_PINNING,
        &mut report,
    );
    run_test(
        "test112_address_canonical_bytes",
        tests::fixtures::TEST112_ADDRESS_CANONICAL_BYTES,
        &mut report,
    );

    // Verify test fixture count matches expected
    let test_count = report.results.len() - before_tests;
    if test_count == counts::SHACL_TESTS {
        report.push(TestResult::pass(
            "ontology/shacl/test_count",
            format!("SHACL test count matches expected: {}", counts::SHACL_TESTS),
        ));
    } else {
        report.push(TestResult::fail(
            "ontology/shacl/test_count",
            format!(
                "Expected {} SHACL tests but ran {}",
                counts::SHACL_TESTS,
                test_count
            ),
        ));
    }

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
        "test30_proof_coverage" => validate_proof_coverage_shacl(turtle_src),
        "test31_quantum_level" => validate_quantum_level_shacl(turtle_src),
        "test32_arc_grounding" => validate_arc_grounding_shacl(turtle_src),
        "test33_graph_gaps" => validate_graph_gaps_shacl(turtle_src),
        "test34_completeness_candidate" => validate_completeness_candidate_shacl(turtle_src),
        "test35_completeness_certificate" => validate_completeness_certificate_shacl(turtle_src),
        "test36_q1_ring" => validate_q1_ring_shacl(turtle_src),
        "test37_quantum_level_binding" => validate_quantum_level_binding_shacl(turtle_src),
        "test38_session_lifecycle" => validate_session_lifecycle_shacl(turtle_src),
        "test39_session_boundary" => validate_session_boundary_shacl(turtle_src),
        "test40_type_synthesis_goal" => validate_type_synthesis_goal_shacl(turtle_src),
        "test41_synthesis_result" => validate_synthesis_result_shacl(turtle_src),
        "test42_quantum_lift" => validate_quantum_lift_shacl(turtle_src),
        "test43_spectral_sequence" => validate_spectral_sequence_shacl(turtle_src),
        "test44_monodromy_flat" => validate_monodromy_flat_shacl(turtle_src),
        "test45_monodromy_twisted" => validate_monodromy_twisted_shacl(turtle_src),
        "test46_monodromy_pipeline" => validate_monodromy_pipeline_shacl(turtle_src),
        "test47_thermo_pipeline" => validate_thermo_pipeline_shacl(turtle_src),
        "test48_phase_diagram" => validate_phase_diagram_shacl(turtle_src),
        "test49_reversible_resolution" => validate_reversible_resolution_shacl(turtle_src),
        "test50_jacobian_resolver" => validate_jacobian_resolver_shacl(turtle_src),
        "test51_product_type_pipeline" => validate_product_type_pipeline_shacl(turtle_src),
        "test52_sum_type_variant" => validate_sum_type_variant_shacl(turtle_src),
        "test53_superposed_fiber" => validate_superposed_fiber_shacl(turtle_src),
        "test54_saturated_context" => validate_saturated_context_shacl(turtle_src),
        "test55_saturation_witness" => validate_saturation_witness_shacl(turtle_src),
        "test56_domain_saturation_record" => validate_domain_saturation_record_shacl(turtle_src),
        "test57_saturation_phase" => validate_saturation_phase_shacl(turtle_src),
        "test58_saturation_certificate" => validate_saturation_certificate_shacl(turtle_src),
        "test59_saturation_aware_resolver" => validate_saturation_aware_resolver_shacl(turtle_src),
        "test60_impossibility_witness" => validate_impossibility_witness_shacl(turtle_src),
        "test61_morphospace_record" => validate_morphospace_record_shacl(turtle_src),
        "test62_morphospace_boundary" => validate_morphospace_boundary_shacl(turtle_src),
        "test63_forbidden_signature" => validate_forbidden_signature_shacl(turtle_src),
        "test64_achievability_status" => validate_achievability_status_shacl(turtle_src),
        "test65_geodesic_trace" => validate_geodesic_trace_shacl(turtle_src),
        "test66_geodesic_certificate" => validate_geodesic_certificate_shacl(turtle_src),
        "test67_geodesic_violation" => validate_geodesic_violation_shacl(turtle_src),
        "test68_geodesic_validator" => validate_geodesic_validator_shacl(turtle_src),
        "test69_geodesic_ordered" => validate_geodesic_ordered_shacl(turtle_src),
        "test70_measurement_resolver" => validate_measurement_resolver_shacl(turtle_src),
        "test71_measurement_event" => validate_measurement_event_shacl(turtle_src),
        "test72_measurement_certificate" => validate_measurement_certificate_shacl(turtle_src),
        "test73_collapsed_fiber_state" => validate_collapsed_fiber_state_shacl(turtle_src),
        "test74_quantum_thermodynamic" => validate_quantum_thermodynamic_shacl(turtle_src),
        "test75_partition_product" => validate_basic_turtle(turtle_src),
        "test76_partition_coproduct" => validate_basic_turtle(turtle_src),
        "test77_geodesic_evidence" => validate_basic_turtle(turtle_src),
        "test78_born_rule" => validate_basic_turtle(turtle_src),
        "test79_measurement_outcome" => validate_basic_turtle(turtle_src),
        "test80_partition_exhaustive" => validate_basic_turtle(turtle_src),
        "test81_dihedral_algebra" => validate_basic_turtle(turtle_src),
        "test82_level_successor" => validate_basic_turtle(turtle_src),
        "test83_amplitude_normalization" => validate_basic_turtle(turtle_src),
        "test84_enum_variant" => validate_basic_turtle(turtle_src),
        "test85_q1_ring_grounding" => validate_basic_turtle(turtle_src),
        "test86_quantum_lift_trivial" => validate_basic_turtle(turtle_src),
        "test87_spectral_convergence" => validate_basic_turtle(turtle_src),
        "test88_lift_obstruction_nontrivial" => validate_basic_turtle(turtle_src),
        "test89_lift_refinement_suggestion" => validate_basic_turtle(turtle_src),
        "test90_resolved_lift" => validate_basic_turtle(turtle_src),
        "test91_synthesis_goal_q1" => validate_basic_turtle(turtle_src),
        "test92_synthesis_checkpoint" => validate_basic_turtle(turtle_src),
        "test93_synthesis_signature" => validate_basic_turtle(turtle_src),
        "test94_synthesized_type" => validate_basic_turtle(turtle_src),
        "test95_unreachable_signature" => validate_basic_turtle(turtle_src),
        "test96_geodesic_trace_q1" => validate_basic_turtle(turtle_src),
        "test97_evidence_bundle_ar1" => validate_basic_turtle(turtle_src),
        "test98_evidence_bundle_dc10" => validate_basic_turtle(turtle_src),
        "test99_measurement_born_q1" => validate_basic_turtle(turtle_src),
        "test100_normative_chain" => validate_basic_turtle(turtle_src),
        "test101_lift_chain_flat" => validate_basic_turtle(turtle_src),
        "test102_lift_chain_twisted" => validate_basic_turtle(turtle_src),
        "test103_obstruction_chain_empty" => validate_basic_turtle(turtle_src),
        "test104_obstruction_chain_nontrivial" => validate_basic_turtle(turtle_src),
        "test105_lift_chain_certificate" => validate_basic_turtle(turtle_src),
        "test106_chain_audit_trail" => validate_basic_turtle(turtle_src),
        "test107_tower_resolver" => validate_basic_turtle(turtle_src),
        "test108_inductive_proof" => validate_basic_turtle(turtle_src),
        "test109_validity_scope" => validate_basic_turtle(turtle_src),
        "test110_tower_roundtrip" => validate_basic_turtle(turtle_src),
        "test111_address_crypto_pinning" => validate_basic_turtle(turtle_src),
        "test112_address_canonical_bytes" => validate_basic_turtle(turtle_src),
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
    check_contains(src, "op:verificationDomain", "Missing verificationDomain")?;
    check_contains(src, "op:Enumerative", "Missing Enumerative individual")?;
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

fn validate_proof_coverage_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "proof:ComputationCertificate",
        "Missing ComputationCertificate type",
    )?;
    check_contains(
        src,
        "proof:AxiomaticDerivation",
        "Missing AxiomaticDerivation type",
    )?;
    check_contains(
        src,
        "proof:CriticalIdentityProof",
        "Missing CriticalIdentityProof type",
    )?;
    check_contains(
        src,
        "proof:provesIdentity",
        "Missing provesIdentity property",
    )?;
    check_contains(
        src,
        "proof:atQuantumLevel",
        "Missing atQuantumLevel property",
    )?;
    check_contains(
        src,
        "proof:universalScope",
        "Missing universalScope property",
    )?;
    check_contains(src, "proof:verified", "Missing verified property")?;
    check_contains(
        src,
        "op:MN_1",
        "Missing MN_1 identity reference (Amendment 30 proof coverage)",
    )?;
    check_contains(
        src,
        "op:MN_7",
        "Missing MN_7 identity reference (Amendment 30 proof coverage)",
    )?;
    Ok(())
}

fn validate_quantum_level_shacl(src: &str) -> Result<(), String> {
    check_contains(src, "schema:QuantumLevel", "Missing QuantumLevel type")?;
    check_contains(src, "schema:quantumIndex", "Missing quantumIndex property")?;
    check_contains(src, "schema:bitsWidth", "Missing bitsWidth property")?;
    check_contains(src, "schema:cycleSize", "Missing cycleSize property")?;
    check_contains(src, "schema:nextLevel", "Missing nextLevel property")?;
    check_contains(src, "schema:Q0", "Missing Q0 individual")?;
    check_contains(src, "schema:Q1", "Missing Q1 individual")?;
    check_contains(src, "schema:Q2", "Missing Q2 individual")?;
    check_contains(src, "schema:Q3", "Missing Q3 individual")?;
    // Q3 terminal condition: must NOT have nextLevel
    if let Some(q3_start) = src.find("schema:Q3") {
        let q3_block = &src[q3_start..];
        let q3_end = q3_block.find("\n\n").unwrap_or(q3_block.len());
        let q3_section = &q3_block[..q3_end];
        if q3_section.contains("nextLevel") {
            return Err("Q3 must not have nextLevel (terminal condition)".to_string());
        }
    }
    Ok(())
}

fn validate_arc_grounding_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "morphism:GroundingMap",
        "Missing GroundingMap type assertion",
    )?;
    check_contains(
        src,
        "morphism:groundedAddress",
        "Missing groundedAddress property",
    )?;
    check_contains(
        src,
        "morphism:symbolConstraints",
        "Missing symbolConstraints property",
    )?;
    check_contains(
        src,
        "morphism:ProjectionMap",
        "Missing ProjectionMap type assertion",
    )?;
    check_contains(
        src,
        "morphism:projectionFrame",
        "Missing projectionFrame property",
    )?;
    check_contains(
        src,
        "morphism:projectionSource",
        "Missing projectionSource property",
    )?;
    check_contains(
        src,
        "morphism:roundTripCoherence",
        "Missing roundTripCoherence property",
    )?;
    check_contains(
        src,
        "query:RelationQuery",
        "Missing RelationQuery type assertion",
    )?;
    check_contains(src, "query:sourceAddress", "Missing sourceAddress property")?;
    check_contains(src, "query:relationType", "Missing relationType property")?;
    check_contains(src, "query:targetFiber", "Missing targetFiber property")?;
    check_contains(src, "query:groundingMap", "Missing groundingMap property")?;
    check_contains(src, "query:projectionMap", "Missing projectionMap property")?;
    Ok(())
}

fn validate_graph_gaps_shacl(src: &str) -> Result<(), String> {
    // Gap B
    check_contains(
        src,
        "resolver:nerveEulerCharacteristic",
        "Missing nerveEulerCharacteristic property",
    )?;
    check_contains(
        src,
        "cohomology:addressesSuggestion",
        "Missing addressesSuggestion property",
    )?;
    // Gap D
    check_contains(
        src,
        "type:CompleteType",
        "Missing CompleteType type assertion",
    )?;
    check_contains(
        src,
        "cert:CompletenessCertificate",
        "Missing CompletenessCertificate type assertion",
    )?;
    check_contains(src, "cert:certifiedType", "Missing certifiedType property")?;
    // Gap E
    check_contains(
        src,
        "observable:ThermoObservable",
        "Missing ThermoObservable type assertion",
    )?;
    check_contains(
        src,
        "observable:ResidualEntropy",
        "Missing ResidualEntropy type assertion",
    )?;
    check_contains(src, "observable:Nats", "Missing Nats individual")?;
    // Gap G
    check_contains(
        src,
        "morphism:GroundingCertificate",
        "Missing GroundingCertificate type assertion",
    )?;
    check_contains(
        src,
        "morphism:groundingCertMap",
        "Missing groundingCertMap property",
    )?;
    Ok(())
}

fn validate_completeness_candidate_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "type:CompletenessCandidate",
        "Missing CompletenessCandidate type assertion",
    )?;
    check_contains(
        src,
        "type:completenessCandidate",
        "Missing completenessCandidate property",
    )?;
    check_contains(
        src,
        "type:candidateNerve",
        "Missing candidateNerve property",
    )?;
    check_contains(
        src,
        "resolver:CompletenessResolver",
        "Missing CompletenessResolver type assertion",
    )?;
    check_contains(
        src,
        "resolver:completenessTarget",
        "Missing completenessTarget property",
    )?;
    check_contains(
        src,
        "type:CompletenessWitness",
        "Missing CompletenessWitness type assertion",
    )?;
    check_contains(
        src,
        "type:witnessConstraint",
        "Missing witnessConstraint property",
    )?;
    check_contains(src, "type:fibersClosed", "Missing fibersClosed property")?;
    Ok(())
}

fn validate_completeness_certificate_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "cert:CompletenessCertificate",
        "Missing CompletenessCertificate type assertion",
    )?;
    check_contains(src, "cert:auditTrail", "Missing auditTrail property")?;
    check_contains(
        src,
        "cert:CompletenessAuditTrail",
        "Missing CompletenessAuditTrail type assertion",
    )?;
    check_contains(src, "cert:witnessCount", "Missing witnessCount property")?;
    check_contains(
        src,
        "type:CompleteType",
        "Missing CompleteType type assertion",
    )?;
    check_contains(src, "cert:certifiedType", "Missing certifiedType property")?;
    Ok(())
}

fn validate_q1_ring_shacl(src: &str) -> Result<(), String> {
    check_contains(src, "schema:Q1Ring", "Missing Q1Ring type assertion")?;
    check_contains(src, "schema:Q1bitWidth", "Missing Q1bitWidth property")?;
    check_contains(src, "schema:Q1capacity", "Missing Q1capacity property")?;
    check_contains(src, "schema:nextLevel", "Missing nextLevel property")?;
    Ok(())
}

fn validate_quantum_level_binding_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "op:QuantumLevelBinding",
        "Missing QuantumLevelBinding type assertion",
    )?;
    check_contains(
        src,
        "op:universallyValid",
        "Missing universallyValid property",
    )?;
    check_contains(
        src,
        "op:verifiedAtLevel",
        "Missing verifiedAtLevel property",
    )?;
    check_contains(src, "op:bindingLevel", "Missing bindingLevel property")?;
    Ok(())
}

fn validate_session_lifecycle_shacl(src: &str) -> Result<(), String> {
    check_contains(src, "state:Session", "Missing Session type assertion")?;
    check_contains(
        src,
        "state:sessionBindings",
        "Missing sessionBindings property",
    )?;
    check_contains(
        src,
        "state:sessionQueries",
        "Missing sessionQueries property",
    )?;
    check_contains(
        src,
        "state:BindingAccumulator",
        "Missing BindingAccumulator type assertion",
    )?;
    check_contains(
        src,
        "state:accumulatedBindings",
        "Missing accumulatedBindings property",
    )?;
    check_contains(
        src,
        "resolver:SessionResolver",
        "Missing SessionResolver type assertion",
    )?;
    check_contains(
        src,
        "resolver:sessionAccumulator",
        "Missing sessionAccumulator property",
    )?;
    check_contains(
        src,
        "query:SessionQuery",
        "Missing SessionQuery type assertion",
    )?;
    check_contains(
        src,
        "query:sessionMembership",
        "Missing sessionMembership property",
    )?;
    Ok(())
}

fn validate_session_boundary_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "state:SessionBoundary",
        "Missing SessionBoundary type assertion",
    )?;
    check_contains(src, "state:boundaryType", "Missing boundaryType property")?;
    check_contains(
        src,
        "state:ContradictionBoundary",
        "Missing ContradictionBoundary individual",
    )?;
    check_contains(
        src,
        "state:boundaryReason",
        "Missing boundaryReason property",
    )?;
    check_contains(src, "state:priorContext", "Missing priorContext property")?;
    check_contains(src, "state:freshContext", "Missing freshContext property")?;
    Ok(())
}

fn validate_type_synthesis_goal_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "type:TypeSynthesisGoal",
        "Missing TypeSynthesisGoal type assertion",
    )?;
    check_contains(
        src,
        "type:targetEulerCharacteristic",
        "Missing targetEulerCharacteristic property",
    )?;
    check_contains(
        src,
        "resolver:TypeSynthesisResolver",
        "Missing TypeSynthesisResolver type assertion",
    )?;
    check_contains(
        src,
        "resolver:synthesisGoal",
        "Missing synthesisGoal property",
    )?;
    Ok(())
}

fn validate_synthesis_result_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "type:TypeSynthesisResult",
        "Missing TypeSynthesisResult type assertion",
    )?;
    check_contains(
        src,
        "type:SynthesizedType",
        "Missing SynthesizedType type assertion",
    )?;
    check_contains(
        src,
        "type:MinimalConstraintBasis",
        "Missing MinimalConstraintBasis type assertion",
    )?;
    check_contains(src, "type:basisSize", "Missing basisSize property")?;
    check_contains(
        src,
        "observable:SynthesisSignature",
        "Missing SynthesisSignature type assertion",
    )?;
    check_contains(
        src,
        "observable:realisedEuler",
        "Missing realisedEuler property",
    )?;
    check_contains(
        src,
        "derivation:SynthesisStep",
        "Missing SynthesisStep type assertion",
    )?;
    check_contains(src, "derivation:stepIndex", "Missing stepIndex property")?;
    Ok(())
}

fn validate_quantum_lift_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "type:QuantumLift",
        "Missing QuantumLift type assertion",
    )?;
    check_contains(src, "type:liftBase", "Missing liftBase property")?;
    check_contains(
        src,
        "type:liftTargetLevel",
        "Missing liftTargetLevel property",
    )?;
    check_contains(
        src,
        "type:LiftObstruction",
        "Missing LiftObstruction type assertion",
    )?;
    check_contains(
        src,
        "type:obstructionTrivial",
        "Missing obstructionTrivial property",
    )?;
    check_contains(
        src,
        "resolver:IncrementalCompletenessResolver",
        "Missing IncrementalCompletenessResolver type assertion",
    )?;
    check_contains(src, "resolver:liftTarget", "Missing liftTarget property")?;
    Ok(())
}

fn validate_spectral_sequence_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:SpectralSequencePage",
        "Missing SpectralSequencePage type assertion",
    )?;
    check_contains(src, "observable:pageIndex", "Missing pageIndex property")?;
    check_contains(
        src,
        "observable:differentialIsZero",
        "Missing differentialIsZero property",
    )?;
    check_contains(
        src,
        "observable:convergedAt",
        "Missing convergedAt property",
    )?;
    Ok(())
}

fn validate_monodromy_flat_shacl(src: &str) -> Result<(), String> {
    check_contains(src, "type:FlatType", "Missing FlatType type assertion")?;
    check_contains(src, "type:holonomyGroup", "Missing holonomyGroup property")?;
    check_contains(
        src,
        "type:monodromyClass",
        "Missing monodromyClass property",
    )?;
    check_contains(
        src,
        "observable:HolonomyGroup",
        "Missing HolonomyGroup type assertion",
    )?;
    check_contains(
        src,
        "observable:holonomyGroupOrder",
        "Missing holonomyGroupOrder property",
    )?;
    check_contains(
        src,
        "observable:Monodromy",
        "Missing Monodromy type assertion",
    )?;
    check_contains(
        src,
        "observable:monodromyLoop",
        "Missing monodromyLoop property",
    )?;
    check_contains(
        src,
        "observable:monodromyElement",
        "Missing monodromyElement property",
    )?;
    check_contains(
        src,
        "observable:isTrivialMonodromy",
        "Missing isTrivialMonodromy property",
    )?;
    check_contains(
        src,
        "observable:DihedralElement",
        "Missing DihedralElement type assertion",
    )?;
    check_contains(
        src,
        "observable:ClosedConstraintPath",
        "Missing ClosedConstraintPath type assertion",
    )?;
    check_contains(
        src,
        "observable:MonodromyClass",
        "Missing MonodromyClass type assertion",
    )?;
    Ok(())
}

fn validate_monodromy_twisted_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "type:TwistedType",
        "Missing TwistedType type assertion",
    )?;
    check_contains(src, "type:holonomyGroup", "Missing holonomyGroup property")?;
    check_contains(
        src,
        "type:monodromyClass",
        "Missing monodromyClass property",
    )?;
    check_contains(
        src,
        "observable:HolonomyGroup",
        "Missing HolonomyGroup type assertion",
    )?;
    check_contains(
        src,
        "observable:holonomyGroup",
        "Missing holonomyGroup generator property",
    )?;
    check_contains(
        src,
        "observable:holonomyGroupOrder",
        "Missing holonomyGroupOrder property",
    )?;
    check_contains(
        src,
        "observable:Monodromy",
        "Missing Monodromy type assertion",
    )?;
    check_contains(
        src,
        "observable:monodromyLoop",
        "Missing monodromyLoop property",
    )?;
    check_contains(
        src,
        "observable:monodromyElement",
        "Missing monodromyElement property",
    )?;
    check_contains(
        src,
        "observable:isTrivialMonodromy",
        "Missing isTrivialMonodromy property",
    )?;
    check_contains(
        src,
        "observable:DihedralElement",
        "Missing DihedralElement type assertion",
    )?;
    check_contains(
        src,
        "observable:isIdentityElement",
        "Missing isIdentityElement property",
    )?;
    check_contains(
        src,
        "observable:ClosedConstraintPath",
        "Missing ClosedConstraintPath type assertion",
    )?;
    check_contains(
        src,
        "type:LiftObstruction",
        "Missing LiftObstruction type assertion",
    )?;
    check_contains(
        src,
        "type:obstructionTrivial",
        "Missing obstructionTrivial property",
    )?;
    check_contains(
        src,
        "observable:LiftObstructionClass",
        "Missing LiftObstructionClass type assertion",
    )?;
    check_contains(
        src,
        "observable:MonodromyClass",
        "Missing MonodromyClass type assertion",
    )?;
    Ok(())
}

fn validate_monodromy_pipeline_shacl(src: &str) -> Result<(), String> {
    // Resolver
    check_contains(
        src,
        "resolver:MonodromyResolver",
        "Missing MonodromyResolver type assertion",
    )?;
    check_contains(
        src,
        "resolver:monodromyTarget",
        "Missing monodromyTarget property",
    )?;
    check_contains(
        src,
        "resolver:holonomyResult",
        "Missing holonomyResult property",
    )?;
    // Input type
    check_contains(
        src,
        "type:ConstrainedType",
        "Missing ConstrainedType type assertion",
    )?;
    check_contains(
        src,
        "type:holonomyGroup",
        "Missing holonomyGroup property on ConstrainedType",
    )?;
    check_contains(
        src,
        "type:monodromyClass",
        "Missing monodromyClass property on ConstrainedType",
    )?;
    // Observables
    check_contains(
        src,
        "observable:ClosedConstraintPath",
        "Missing ClosedConstraintPath type assertion",
    )?;
    check_contains(src, "observable:pathLength", "Missing pathLength property")?;
    check_contains(
        src,
        "observable:Monodromy",
        "Missing Monodromy type assertion",
    )?;
    check_contains(
        src,
        "observable:monodromyLoop",
        "Missing monodromyLoop property",
    )?;
    check_contains(
        src,
        "observable:monodromyElement",
        "Missing monodromyElement property",
    )?;
    check_contains(
        src,
        "observable:isTrivialMonodromy",
        "Missing isTrivialMonodromy property",
    )?;
    check_contains(
        src,
        "observable:DihedralElement",
        "Missing DihedralElement type assertion",
    )?;
    check_contains(
        src,
        "observable:isIdentityElement",
        "Missing isIdentityElement property",
    )?;
    check_contains(
        src,
        "observable:elementOrder",
        "Missing elementOrder property",
    )?;
    check_contains(
        src,
        "observable:HolonomyGroup",
        "Missing HolonomyGroup type assertion",
    )?;
    check_contains(
        src,
        "observable:holonomyGroup",
        "Missing holonomyGroup generator property",
    )?;
    check_contains(
        src,
        "observable:holonomyGroupOrder",
        "Missing holonomyGroupOrder property",
    )?;
    check_contains(
        src,
        "observable:MonodromyClass",
        "Missing MonodromyClass type assertion",
    )?;
    // Output classification
    check_contains(
        src,
        "type:TwistedType",
        "Missing TwistedType type assertion",
    )?;
    Ok(())
}

fn validate_thermo_pipeline_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:ThermoObservable",
        "Missing ThermoObservable",
    )?;
    check_contains(src, "observable:ResidualEntropy", "Missing ResidualEntropy")?;
    check_contains(
        src,
        "observable:hardnessEstimate",
        "Missing hardnessEstimate",
    )?;
    check_contains(src, "trace:ComputationTrace", "Missing ComputationTrace")?;
    check_contains(src, "trace:residualEntropy", "Missing residualEntropy link")?;
    Ok(())
}

fn validate_phase_diagram_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:CatastropheObservable",
        "Missing CatastropheObservable",
    )?;
    check_contains(src, "observable:phaseN", "Missing phaseN coordinate")?;
    check_contains(src, "observable:phaseG", "Missing phaseG coordinate")?;
    check_contains(
        src,
        "observable:phaseBoundaryType",
        "Missing phaseBoundaryType",
    )?;
    check_contains(src, "observable:onResonanceLine", "Missing onResonanceLine")?;
    Ok(())
}

fn validate_reversible_resolution_shacl(src: &str) -> Result<(), String> {
    check_contains(src, "partition:FiberBudget", "Missing FiberBudget")?;
    check_contains(src, "partition:FiberCoordinate", "Missing FiberCoordinate")?;
    check_contains(src, "partition:ancillaFiber", "Missing ancillaFiber")?;
    check_contains(
        src,
        "partition:reversibleStrategy",
        "Missing reversibleStrategy",
    )?;
    Ok(())
}

fn validate_jacobian_resolver_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:JacobianGuidedResolver",
        "Missing JacobianGuidedResolver",
    )?;
    check_contains(src, "resolver:guidingJacobian", "Missing guidingJacobian")?;
    check_contains(src, "observable:Jacobian", "Missing Jacobian observable")?;
    Ok(())
}

fn validate_product_type_pipeline_shacl(src: &str) -> Result<(), String> {
    check_contains(src, "type:ProductType", "Missing ProductType")?;
    check_contains(src, "type:component", "Missing component property")?;
    check_contains(src, "type:TypeDefinition", "Missing TypeDefinition")?;
    Ok(())
}

fn validate_sum_type_variant_shacl(src: &str) -> Result<(), String> {
    check_contains(src, "type:SumType", "Missing SumType")?;
    check_contains(src, "type:component", "Missing component property")?;
    Ok(())
}

fn validate_superposed_fiber_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "type:SuperposedFiberState",
        "Missing SuperposedFiberState",
    )?;
    check_contains(src, "type:amplitude", "Missing amplitude property")?;
    check_contains(
        src,
        "resolver:SuperpositionResolver",
        "Missing SuperpositionResolver",
    )?;
    Ok(())
}

fn validate_saturated_context_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "state:SaturatedContext",
        "Missing SaturatedContext type assertion",
    )?;
    check_contains(
        src,
        "state:saturationDegree",
        "Missing saturationDegree property",
    )?;
    check_contains(
        src,
        "state:contextTemperature",
        "Missing contextTemperature property",
    )?;
    check_contains(src, "state:isSaturated", "Missing isSaturated property")?;
    check_contains(
        src,
        "state:saturationPhase",
        "Missing saturationPhase property",
    )?;
    Ok(())
}

fn validate_saturation_witness_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "state:SaturationWitness",
        "Missing SaturationWitness type assertion",
    )?;
    check_contains(
        src,
        "state:witnessBinding",
        "Missing witnessBinding property",
    )?;
    check_contains(src, "state:witnessStep", "Missing witnessStep property")?;
    check_contains(
        src,
        "state:residualFreeCount",
        "Missing residualFreeCount property",
    )?;
    Ok(())
}

fn validate_domain_saturation_record_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:DomainSaturationRecord",
        "Missing DomainSaturationRecord type assertion",
    )?;
    check_contains(
        src,
        "observable:saturatedContext",
        "Missing saturatedContext property",
    )?;
    check_contains(
        src,
        "observable:saturatedDomain",
        "Missing saturatedDomain property",
    )?;
    check_contains(
        src,
        "observable:domainFreeCount",
        "Missing domainFreeCount property",
    )?;
    Ok(())
}

fn validate_saturation_phase_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:SaturationPhase",
        "Missing SaturationPhase type assertion",
    )?;
    check_contains(
        src,
        "observable:Unsaturated",
        "Missing Unsaturated individual",
    )?;
    check_contains(
        src,
        "observable:PartialSaturation",
        "Missing PartialSaturation individual",
    )?;
    check_contains(
        src,
        "observable:FullSaturation",
        "Missing FullSaturation individual",
    )?;
    Ok(())
}

fn validate_saturation_certificate_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "cert:SaturationCertificate",
        "Missing SaturationCertificate type assertion",
    )?;
    check_contains(
        src,
        "cert:certifiedSaturation",
        "Missing certifiedSaturation property",
    )?;
    check_contains(
        src,
        "cert:saturationWitness",
        "Missing saturationWitness property",
    )?;
    Ok(())
}

fn validate_saturation_aware_resolver_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:SaturationAwareResolver",
        "Missing SaturationAwareResolver type assertion",
    )?;
    check_contains(
        src,
        "resolver:usedSaturation",
        "Missing usedSaturation property",
    )?;
    Ok(())
}

fn validate_impossibility_witness_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "proof:ImpossibilityWitness",
        "Missing ImpossibilityWitness type assertion",
    )?;
    check_contains(
        src,
        "proof:forbidsSignature",
        "Missing forbidsSignature property",
    )?;
    check_contains(
        src,
        "proof:impossibilityReason",
        "Missing impossibilityReason property",
    )?;
    check_contains(
        src,
        "proof:impossibilityDomain",
        "Missing impossibilityDomain property",
    )?;
    Ok(())
}

fn validate_morphospace_record_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:MorphospaceRecord",
        "Missing MorphospaceRecord type assertion",
    )?;
    check_contains(
        src,
        "observable:achievabilityStatus",
        "Missing achievabilityStatus property",
    )?;
    check_contains(
        src,
        "observable:verifiedAtLevel",
        "Missing verifiedAtLevel property",
    )?;
    check_contains(
        src,
        "observable:morphospaceRecord",
        "Missing morphospaceRecord property",
    )?;
    Ok(())
}

fn validate_morphospace_boundary_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:MorphospaceBoundary",
        "Missing MorphospaceBoundary type assertion",
    )?;
    check_contains(
        src,
        "observable:boundaryType",
        "Missing boundaryType property",
    )?;
    Ok(())
}

fn validate_forbidden_signature_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:ForbiddenSignature",
        "Missing ForbiddenSignature type assertion",
    )?;
    check_contains(
        src,
        "observable:targetForbidden",
        "Missing targetForbidden property",
    )?;
    Ok(())
}

fn validate_achievability_status_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:AchievabilityStatus",
        "Missing AchievabilityStatus type assertion",
    )?;
    check_contains(
        src,
        "observable:Achievable",
        "Missing Achievable individual",
    )?;
    check_contains(src, "observable:Forbidden", "Missing Forbidden individual")?;
    Ok(())
}

fn validate_geodesic_trace_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "trace:GeodesicTrace",
        "Missing GeodesicTrace type assertion",
    )?;
    check_contains(src, "trace:isGeodesic", "Missing isGeodesic property")?;
    check_contains(
        src,
        "trace:geodesicCertificate",
        "Missing geodesicCertificate property",
    )?;
    check_contains(
        src,
        "trace:stepEntropyCost",
        "Missing stepEntropyCost property",
    )?;
    check_contains(
        src,
        "trace:cumulativeEntropyCost",
        "Missing cumulativeEntropyCost property",
    )?;
    Ok(())
}

fn validate_geodesic_certificate_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "cert:GeodesicCertificate",
        "Missing GeodesicCertificate type assertion",
    )?;
    check_contains(
        src,
        "cert:certifiedGeodesic",
        "Missing certifiedGeodesic property",
    )?;
    check_contains(src, "cert:geodesicTrace", "Missing geodesicTrace property")?;
    Ok(())
}

fn validate_geodesic_violation_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:GeodesicViolation",
        "Missing GeodesicViolation type assertion",
    )?;
    check_contains(
        src,
        "observable:violationReason",
        "Missing violationReason property",
    )?;
    Ok(())
}

fn validate_geodesic_validator_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:GeodesicValidator",
        "Missing GeodesicValidator type assertion",
    )?;
    check_contains(
        src,
        "resolver:validateGeodesic",
        "Missing validateGeodesic property",
    )?;
    Ok(())
}

fn validate_geodesic_ordered_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "trace:GeodesicTrace",
        "Missing GeodesicTrace type assertion",
    )?;
    check_contains(
        src,
        "trace:adiabaticallyOrdered",
        "Missing adiabaticallyOrdered property",
    )?;
    check_contains(
        src,
        "trace:jacobianAtStep",
        "Missing jacobianAtStep property",
    )?;
    Ok(())
}

fn validate_measurement_resolver_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:MeasurementResolver",
        "Missing MeasurementResolver type assertion",
    )?;
    check_contains(
        src,
        "resolver:collapseAmplitude",
        "Missing collapseAmplitude property",
    )?;
    check_contains(
        src,
        "resolver:collapsedFiber",
        "Missing collapsedFiber property",
    )?;
    check_contains(
        src,
        "resolver:measurementOutcome",
        "Missing measurementOutcome property",
    )?;
    Ok(())
}

fn validate_measurement_event_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:MeasurementEvent",
        "Missing MeasurementEvent type assertion",
    )?;
    check_contains(
        src,
        "observable:measurementEvent",
        "Missing measurementEvent property",
    )?;
    check_contains(
        src,
        "observable:preCollapseEntropy",
        "Missing preCollapseEntropy property",
    )?;
    check_contains(
        src,
        "observable:postCollapseLandauerCost",
        "Missing postCollapseLandauerCost property",
    )?;
    check_contains(
        src,
        "observable:collapseStep",
        "Missing collapseStep property",
    )?;
    Ok(())
}

fn validate_measurement_certificate_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "cert:MeasurementCertificate",
        "Missing MeasurementCertificate type assertion",
    )?;
    check_contains(
        src,
        "cert:certifiedMeasurement",
        "Missing certifiedMeasurement property",
    )?;
    check_contains(
        src,
        "cert:vonNeumannEntropy",
        "Missing vonNeumannEntropy property",
    )?;
    check_contains(src, "cert:landauerCost", "Missing landauerCost property")?;
    Ok(())
}

fn validate_collapsed_fiber_state_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "type:CollapsedFiberState",
        "Missing CollapsedFiberState type assertion",
    )?;
    check_contains(src, "type:collapsedFrom", "Missing collapsedFrom property")?;
    check_contains(
        src,
        "type:survivingAmplitude",
        "Missing survivingAmplitude property",
    )?;
    Ok(())
}

fn validate_quantum_thermodynamic_shacl(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "observable:QuantumThermodynamicDomain",
        "Missing QuantumThermodynamicDomain type assertion",
    )?;
    check_contains(
        src,
        "op:verificationDomain",
        "Missing verificationDomain property",
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

/// Validates that a Turtle test fixture is syntactically non-empty and
/// contains at least one `owl:NamedIndividual` type assertion.
fn validate_basic_turtle(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "owl:NamedIndividual",
        "Missing owl:NamedIndividual type assertion",
    )
}
