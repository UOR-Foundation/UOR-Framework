# SHACL Conformance Standards

## Overview

The UOR conformance suite validates 252 OWL instance graphs against SHACL NodeShapes
defined in `conformance/shapes/uor-shapes.ttl`. One NodeShape is defined per
ontology class (396 total).

## Shape File

`conformance/shapes/uor-shapes.ttl` contains:
- 396 `sh:NodeShape` declarations (one per class)
- `sh:targetClass` targeting each OWL class
- Cardinality constraints (`sh:minCount`, `sh:maxCount`) on required properties
- Type constraints (`sh:class`, `sh:datatype`) on property values

## The 252 Instance Tests

| Test | File | Validates |
|------|------|-----------|
| test1_ring | `tests/fixtures/test1_ring.rs` | `schema:Ring` with all required properties |
| test2_primitives | `tests/fixtures/test2_primitives.rs` | All 10 `op:*` individuals with correct types |
| test3_term_graph | `tests/fixtures/test3_term_graph.rs` | `schema:Application` + `schema:Literal` + `schema:Datum` |
| test4_state_lifecycle | `tests/fixtures/test4_state_lifecycle.rs` | `state:Context/Binding/Frame/Transition` |
| test5_partition | `tests/fixtures/test5_partition.rs` | `partition:Partition` with 4 component sets |
| test6_critical_identity | `tests/fixtures/test6_critical_identity.rs` | `op:criticalIdentity` + `proof:CriticalIdentityProof` |
| test7_end_to_end | `tests/fixtures/test7_end_to_end.rs` | Full cycle across 8 namespaces |
| test8_fiber_budget | `tests/fixtures/test8_fiber_budget.rs` | `partition:FiberBudget` + `FiberCoordinate` with isClosed |
| test9_constraint_algebra | `tests/fixtures/test9_constraint_algebra.rs` | `type:ResidueConstraint` + `CompositeConstraint` + `MetricAxis` |
| test10_iterative_resolution | `tests/fixtures/test10_iterative_resolution.rs` | `resolver:ResolutionState` + `RefinementSuggestion` + `derivation:RefinementStep` |
| test11_composition | `tests/fixtures/test11_composition.rs` | `morphism:Composition` + `CompositionLaw` + `Identity` |
| test12_factorization | `tests/fixtures/test12_factorization.rs` | Full PRISM pipeline: Query → Resolver → Partition + FiberBudget → Cert → Trace |
| test13_canonical_form | `tests/fixtures/test13_canonical_form.rs` | `CanonicalFormResolver` → `Derivation` with `RewriteStep` chain |
| test14_content_addressing | `tests/fixtures/test14_content_addressing.rs` | `u:Address` → Observable taxonomy → `InvolutionCertificate` |
| test15_boolean_sat | `tests/fixtures/test15_boolean_sat.rs` | `EvaluationResolver` → State lifecycle → Certificate → Trace |
| test16_algebraic_identities | `tests/fixtures/test16_algebraic_identities.rs` | `op:Identity` individuals with lhs/rhs/forAll |
| test17_inter_algebra_maps | `tests/fixtures/test17_inter_algebra_maps.rs` | `op:Identity` phi-pipeline individuals |
| test18_analytical_completeness | `tests/fixtures/test18_analytical_completeness.rs` | `observable:Jacobian`, `observable:BettiNumber`, `observable:SpectralGap` |
| test19_homological_pipeline | `tests/fixtures/test19_homological_pipeline.rs` | `homology:Simplex` → `ChainComplex` → `HomologyGroup` pipeline |
| test20_sheaf_consistency | `tests/fixtures/test20_sheaf_consistency.rs` | `cohomology:Sheaf` → `Stalk` → `Section` → `GluingObstruction` |
| test21_topological_delta | `tests/fixtures/test21_topological_delta.rs` | `morphism:TopologicalDelta` with Betti/Euler/nerve before+after |
| test22_index_bridge | `tests/fixtures/test22_index_bridge.rs` | Full φ+ψ pipeline (6 phi_ + 6 psi_ individuals) |
| test23_identity_grounding | `tests/fixtures/test23_identity_grounding.rs` | `verificationDomain`/`verificationPathNote` spot-check |
| test24_verification_domain | `tests/fixtures/test24_verification_domain.rs` | `VerificationDomain` vocabulary + typed identity grounding |
| test25_geometric_character | `tests/fixtures/test25_geometric_character.rs` | `GeometricCharacter` vocabulary + typed operation links |
| test26_complexity_class | `tests/fixtures/test26_complexity_class.rs` | `ComplexityClass` vocabulary + typed resolver links |
| test27_rewrite_rule | `tests/fixtures/test27_rewrite_rule.rs` | `RewriteRule` vocabulary + `groundedIn` cross-reference |
| test28_measurement_unit | `tests/fixtures/test28_measurement_unit.rs` | `MeasurementUnit` vocabulary + typed observable links |
| test29_coordinate_kind | `tests/fixtures/test29_coordinate_kind.rs` | `CoordinateKind` vocabulary + typed coordinate queries |
| test30_proof_coverage | `tests/fixtures/test30_proof_coverage.rs` | `proof:ComputationCertificate`/`AxiomaticDerivation`/`CriticalIdentityProof` + `provesIdentity`/`atQuantumLevel`/`universalScope` |
| test31_quantum_level | `tests/fixtures/test31_quantum_level.rs` | `schema:QuantumLevel` individuals Q0–Q3 with `quantumIndex`/`bitsWidth`/`cycleSize`/`nextLevel` |
| test32_arc_grounding | `tests/fixtures/test32_arc_grounding.rs` | `morphism:GroundingMap` + `ProjectionMap` + `GroundingCertificate` |
| test33_graph_gaps | `tests/fixtures/test33_graph_gaps.rs` | `type:CompleteType` + `cert:CompletenessCertificate` + thermodynamic observables |
| test34_completeness_candidate | `tests/fixtures/test34_completeness_candidate.rs` | `type:CompletenessCandidate` + `CompletenessWitness` + `resolver:CompletenessResolver` |
| test35_completeness_certificate | `tests/fixtures/test35_completeness_certificate.rs` | `cert:CompletenessAuditTrail` + completeness certification pipeline |
| test36_q1_ring | `tests/fixtures/test36_q1_ring.rs` | `schema:Q1Ring` + `op:QuantumLevelBinding` + `resolver:QuantumLevelResolver` |
| test37_quantum_level_binding | `tests/fixtures/test37_quantum_level_binding.rs` | Quantum level binding with QL_ identities |
| test38_session_lifecycle | `tests/fixtures/test38_session_lifecycle.rs` | `state:Session` + `BindingAccumulator` + `SessionBoundary` lifecycle |
| test39_session_boundary | `tests/fixtures/test39_session_boundary.rs` | `state:SessionBoundaryType` vocabulary + `resolver:SessionResolver` |
| test40_type_synthesis_goal | `tests/fixtures/test40_type_synthesis_goal.rs` | `type:TypeSynthesisGoal` + `TypeSynthesisResult` + `SynthesizedType` (Amendment 28) |
| test41_synthesis_result | `tests/fixtures/test41_synthesis_result.rs` | `resolver:TypeSynthesisResolver` + `ConstraintSearchState` + `derivation:SynthesisStep` (Amendment 28) |
| test42_quantum_lift | `tests/fixtures/test42_quantum_lift.rs` | `type:QuantumLift` + `LiftObstruction` + `resolver:IncrementalCompletenessResolver` (Amendment 29) |
| test43_spectral_sequence | `tests/fixtures/test43_spectral_sequence.rs` | `observable:SpectralSequencePage` + `LiftObstructionClass` (Amendment 29) |
| test44_monodromy_flat | `tests/fixtures/test44_monodromy_flat.rs` | `type:FlatType` + trivial `HolonomyGroup` + `Monodromy` + `DihedralElement` (Amendment 30) |
| test45_monodromy_twisted | `tests/fixtures/test45_monodromy_twisted.rs` | `type:TwistedType` + non-trivial `HolonomyGroup` + `Monodromy` + `LiftObstruction` (Amendment 30) |
| test46_monodromy_pipeline | `tests/fixtures/test46_monodromy_pipeline.rs` | `resolver:MonodromyResolver` end-to-end pipeline: ConstrainedType → HolonomyGroup → MonodromyClass → TwistedType (Amendment 30) |
| test47_thermo_pipeline | `tests/fixtures/test47_thermo_pipeline.rs` | `observable:ThermoObservable` + `hardnessEstimate` + `trace:residualEntropy` (Amendment 31) |
| test48_phase_diagram | `tests/fixtures/test48_phase_diagram.rs` | `observable:CatastropheObservable` + `phaseN`/`phaseG` + `PhaseBoundaryType` + `onResonanceLine` (Amendment 31) |
| test49_reversible_resolution | `tests/fixtures/test49_reversible_resolution.rs` | `partition:FiberBudget` + `ancillaFiber` + `reversibleStrategy` (Amendment 31) |
| test50_jacobian_resolver | `tests/fixtures/test50_jacobian_resolver.rs` | `resolver:JacobianGuidedResolver` + `ResolutionState` + `guidingJacobian` (Amendment 31) |
| test51_product_type_pipeline | `tests/fixtures/test51_product_type_pipeline.rs` | `type:ProductType` + component assertions + `FiberBudget` (Amendment 31) |
| test52_sum_type_variant | `tests/fixtures/test52_sum_type_variant.rs` | `type:SumType` + variant assertions (Amendment 31) |
| test53_superposed_fiber | `tests/fixtures/test53_superposed_fiber.rs` | `type:SuperposedFiberState` + `amplitude` + `resolver:SuperpositionResolver` (Amendment 32) |
| test54_saturated_context | `tests/fixtures/test54_saturated_context.rs` | `state:SaturatedContext` + `saturationDegree` + `contextTemperature` + `isSaturated` (Amendment 33) |
| test55_saturation_witness | `tests/fixtures/test55_saturation_witness.rs` | `state:SaturationWitness` + `witnessBinding` + `witnessStep` + `residualFreeCount` (Amendment 33) |
| test56_domain_saturation_record | `tests/fixtures/test56_domain_saturation_record.rs` | `state:DomainSaturationRecord` + `saturatedDomain` + `domainFreeCount` (Amendment 33) |
| test57_saturation_phase | `tests/fixtures/test57_saturation_phase.rs` | `state:SaturationPhase` vocabulary: Unsaturated, PartialSaturation, FullSaturation (Amendment 33) |
| test58_saturation_certificate | `tests/fixtures/test58_saturation_certificate.rs` | `cert:SaturationCertificate` + `certifiedSaturation` + `saturationWitness` (Amendment 33) |
| test59_saturation_aware_resolver | `tests/fixtures/test59_saturation_aware_resolver.rs` | `resolver:SaturationAwareResolver` + `usedSaturation` (Amendment 33) |
| test60_impossibility_witness | `tests/fixtures/test60_impossibility_witness.rs` | `proof:ImpossibilityWitness` + `forbidsSignature` + `impossibilityReason` (Amendment 34) |
| test61_morphospace_record | `tests/fixtures/test61_morphospace_record.rs` | `proof:MorphospaceRecord` + `achievabilityStatus` + `verifiedAtLevel` (Amendment 34) |
| test62_morphospace_boundary | `tests/fixtures/test62_morphospace_boundary.rs` | `proof:MorphospaceBoundary` + `boundaryType` (Amendment 34) |
| test63_forbidden_signature | `tests/fixtures/test63_forbidden_signature.rs` | `type:ForbiddenSignature` + `targetForbidden` (Amendment 34) |
| test64_achievability_status | `tests/fixtures/test64_achievability_status.rs` | `observable:AchievabilityStatus` vocabulary: Achievable, Forbidden (Amendment 34) |
| test65_geodesic_trace | `tests/fixtures/test65_geodesic_trace.rs` | `trace:GeodesicTrace` + `isGeodesic` + `geodesicCertificate` + `stepEntropyCost` (Amendment 35) |
| test66_geodesic_certificate | `tests/fixtures/test66_geodesic_certificate.rs` | `cert:GeodesicCertificate` + `certifiedGeodesic` + `geodesicTrace` (Amendment 35) |
| test67_geodesic_violation | `tests/fixtures/test67_geodesic_violation.rs` | `trace:GeodesicViolation` + `violationReason` (Amendment 35) |
| test68_geodesic_validator | `tests/fixtures/test68_geodesic_validator.rs` | `resolver:GeodesicValidator` + `validateGeodesic` (Amendment 35) |
| test69_geodesic_ordered | `tests/fixtures/test69_geodesic_ordered.rs` | `trace:GeodesicTrace` + `adiabaticallyOrdered` + `jacobianAtStep` (Amendment 35) |
| test70_measurement_resolver | `tests/fixtures/test70_measurement_resolver.rs` | `resolver:MeasurementResolver` + `collapseAmplitude` + `collapsedFiber` (Amendment 36) |
| test71_measurement_event | `tests/fixtures/test71_measurement_event.rs` | `trace:MeasurementEvent` + `preCollapseEntropy` + `postCollapseLandauerCost` (Amendment 36) |
| test72_measurement_certificate | `tests/fixtures/test72_measurement_certificate.rs` | `cert:MeasurementCertificate` + `certifiedMeasurement` + `vonNeumannEntropy` + `landauerCost` (Amendment 36) |
| test73_collapsed_fiber_state | `tests/fixtures/test73_collapsed_fiber_state.rs` | `type:CollapsedFiberState` + `collapsedFrom` + `survivingAmplitude` (Amendment 36) |
| test74_quantum_thermodynamic | `tests/fixtures/test74_quantum_thermodynamic.rs` | `op:QuantumThermodynamicDomain` + QuantumThermodynamic verification domain (Amendment 36) |
| test75_partition_product | `tests/fixtures/test75_partition_product.rs` | `partition:PartitionProduct` + `leftFactor` + `rightFactor` (Amendment 37) |
| test76_partition_coproduct | `tests/fixtures/test76_partition_coproduct.rs` | `partition:PartitionCoproduct` + `leftSummand` + `rightSummand` (Amendment 37) |
| test77_geodesic_evidence | `tests/fixtures/test77_geodesic_evidence.rs` | `cert:GeodesicEvidenceBundle` + `evidenceBundle` (Amendment 37) |
| test78_born_rule | `tests/fixtures/test78_born_rule.rs` | `cert:BornRuleVerification` + `bornRuleVerified` (Amendment 37) |
| test79_measurement_outcome | `tests/fixtures/test79_measurement_outcome.rs` | `trace:MeasurementOutcome` + `outcomeValue` + `outcomeProbability` (Amendment 37) |
| test80_partition_exhaustive | `tests/fixtures/test80_partition_exhaustive.rs` | `partition:Partition` + `isExhaustive` (Amendment 37) |
| test81_dihedral_algebra | `tests/fixtures/test81_dihedral_algebra.rs` | `observable:DihedralElement` + `rotationExponent` + `reflectionBit` (Amendment 37) |
| test82_level_successor | `tests/fixtures/test82_level_successor.rs` | `schema:QuantumLevel` + `levelSuccessor` (Amendment 37) |
| test83_amplitude_normalization | `tests/fixtures/test83_amplitude_normalization.rs` | `type:SuperposedFiberState` + `normalizationVerified` (Amendment 37) |
| test84_enum_variant | `tests/fixtures/test84_enum_variant.rs` | `op:VerificationDomain` + `enumVariant` (Amendment 37) |
| test85_q1_ring_grounding | `tests/fixtures/test85_q1_ring_grounding.rs` | Q1Ring individual grounding at quantum level Q1 (Amendment 39) |
| test86_quantum_lift_trivial | `tests/fixtures/test86_quantum_lift_trivial.rs` | QuantumLift with trivial LiftObstruction — Q1 lift (Amendment 39) |
| test87_spectral_convergence | `tests/fixtures/test87_spectral_convergence.rs` | SpectralSequencePage convergence at E2 — Q1 scale (Amendment 39) |
| test88_lift_obstruction_nontrivial | `tests/fixtures/test88_lift_obstruction_nontrivial.rs` | Non-trivial LiftObstruction with TwistedType and holonomyClassified (Amendment 39) |
| test89_lift_refinement_suggestion | `tests/fixtures/test89_lift_refinement_suggestion.rs` | LiftRefinementSuggestion with obstructionClass — Q1 failure path (Amendment 39) |
| test90_resolved_lift | `tests/fixtures/test90_resolved_lift.rs` | Resolved lift — SynthesizedType at Q1 with basisSize increment (Amendment 39) |
| test91_synthesis_goal_q1 | `tests/fixtures/test91_synthesis_goal_q1.rs` | TypeSynthesisGoal with Q1 target signature (Amendment 39) |
| test92_synthesis_checkpoint | `tests/fixtures/test92_synthesis_checkpoint.rs` | SynthesisCheckpoint with checkpointStep and checkpointState (Amendment 39) |
| test93_synthesis_signature | `tests/fixtures/test93_synthesis_signature.rs` | SynthesisSignature with realisedEuler/Betti/achievabilityStatus (Amendment 39) |
| test94_synthesized_type | `tests/fixtures/test94_synthesized_type.rs` | SynthesizedType with TypeSynthesisResult and MinimalConstraintBasis (Amendment 39) |
| test95_unreachable_signature | `tests/fixtures/test95_unreachable_signature.rs` | Unreachable signature rejection — TypeSynthesisGoal with invalid target (Amendment 39) |
| test96_geodesic_trace_q1 | `tests/fixtures/test96_geodesic_trace_q1.rs` | GeodesicTrace at Q1 ring scale with GeodesicCertificate (Amendment 40) |
| test97_evidence_bundle_ar1 | `tests/fixtures/test97_evidence_bundle_ar1.rs` | GeodesicEvidenceBundle with isAR1Ordered (Amendment 40) |
| test98_evidence_bundle_dc10 | `tests/fixtures/test98_evidence_bundle_dc10.rs` | GeodesicEvidenceBundle with isDC10Selected (Amendment 40) |
| test99_measurement_born_q1 | `tests/fixtures/test99_measurement_born_q1.rs` | MeasurementCertificate with BornRuleVerification at Q1 (Amendment 40) |
| test100_normative_chain | `tests/fixtures/test100_normative_chain.rs` | Full normative chain — Trace → GeodesicTrace → Certificate → EvidenceBundle (Amendment 40) |
| test101_lift_chain_flat | `tests/fixtures/test101_lift_chain_flat.rs` | Flat LiftChain Q0→Q3 with trivial holonomy (Amendment 41) |
| test102_lift_chain_twisted | `tests/fixtures/test102_lift_chain_twisted.rs` | Twisted LiftChain Q0→Q2 with non-trivial obstruction (Amendment 41) |
| test103_obstruction_chain_empty | `tests/fixtures/test103_obstruction_chain_empty.rs` | Empty ObstructionChain — vacuously satisfied (Amendment 41) |
| test104_obstruction_chain_nontrivial | `tests/fixtures/test104_obstruction_chain_nontrivial.rs` | Non-trivial ObstructionChain with LiftObstructionClass (Amendment 41) |
| test105_lift_chain_certificate | `tests/fixtures/test105_lift_chain_certificate.rs` | LiftChainCertificate certifying flat lift chain (Amendment 41) |
| test106_chain_audit_trail | `tests/fixtures/test106_chain_audit_trail.rs` | ChainAuditTrail recording resolution steps across levels (Amendment 41) |
| test107_tower_resolver | `tests/fixtures/test107_tower_resolver.rs` | TowerCompletenessResolver across Q0→Q3 tower (Amendment 41) |
| test108_inductive_proof | `tests/fixtures/test108_inductive_proof.rs` | InductiveProof with derivation witness (Amendment 41) |
| test109_validity_scope | `tests/fixtures/test109_validity_scope.rs` | Identity with validityKind annotation (Amendment 41) |
| test110_tower_roundtrip | `tests/fixtures/test110_tower_roundtrip.rs` | Full tower round-trip Q0→Q3 end-to-end integrity (Amendment 41) |
| test111_address_crypto_pinning | `tests/fixtures/test111_address_crypto_pinning.rs` | Address with digestAlgorithm + canonicalBytes (Amendment 43) |
| test112_address_canonical_bytes | `tests/fixtures/test112_address_canonical_bytes.rs` | Address at Q1 with 6-byte canonical form (Amendment 43) |
| test113_carry_constraint_pinning | `tests/fixtures/test113_carry_constraint_pinning.rs` | Carry constraint fiber-pinning map identities (Amendment 44) |
| test114_joint_satisfiability | `tests/fixtures/test114_joint_satisfiability.rs` | Nerve joint satisfiability decision procedure (Amendment 44) |
| test115_dihedral_inverse_order | `tests/fixtures/test115_dihedral_inverse_order.rs` | Dihedral inverse and order identities D_8, D_9 (Amendment 44) |
| test116_constraint_expressiveness | `tests/fixtures/test116_constraint_expressiveness.rs` | Constraint language expressiveness boundary EXP_1–3 (Amendment 44) |
| test117_sumtype_topology | `tests/fixtures/test117_sumtype_topology.rs` | SumType topological identity algebra ST_3–5 (Amendment 44) |
| test118_synthesis_reachability | `tests/fixtures/test118_synthesis_reachability.rs` | TypeSynthesis reachability domain completeness TS_8–10 (Amendment 44) |
| test119_obstruction_termination | `tests/fixtures/test119_obstruction_termination.rs` | ObstructionChain termination guarantee QT_8, QT_9 (Amendment 44) |
| test120_coefficient_ring | `tests/fixtures/test120_coefficient_ring.rs` | Sheaf coefficient ring grounding COEFF_1 (Amendment 44) |
| test121_gluing_feedback | `tests/fixtures/test121_gluing_feedback.rs` | GluingObstruction resolver feedback GO_1 (Amendment 44) |
| test122_session_saturation | `tests/fixtures/test122_session_saturation.rs` | Session saturation lifecycle bridge SR_6, SR_7 (Amendment 44) |
| test123_amplitude_index | `tests/fixtures/test123_amplitude_index.rs` | SuperposedFiberState amplitude index set QM_6 (Amendment 44) |
| test124_glyph | `tests/fixtures/test124_glyph.rs` | `u:Glyph` basic fixture (Amendment 46) |
| test125_dihedral_group | `tests/fixtures/test125_dihedral_group.rs` | `op:DihedralGroup` basic fixture (Amendment 46) |
| test126_validity_scope_kind | `tests/fixtures/test126_validity_scope_kind.rs` | `op:ValidityScopeKind` enum class fixture (Amendment 46) |
| test127_quantum_level_resolver | `tests/fixtures/test127_quantum_level_resolver.rs` | `resolver:QuantumLevelResolver` fixture (Amendment 46) |
| test128_stratum_observable | `tests/fixtures/test128_stratum_observable.rs` | `observable:StratumObservable` fixture (Amendment 46) |
| test129_metric_observable | `tests/fixtures/test129_metric_observable.rs` | `observable:MetricObservable` fixture (Amendment 46) |
| test130_path_observable | `tests/fixtures/test130_path_observable.rs` | `observable:PathObservable` fixture (Amendment 46) |
| test131_cascade_observable | `tests/fixtures/test131_cascade_observable.rs` | `observable:CascadeObservable` fixture (Amendment 46) |
| test132_holonomy_observable | `tests/fixtures/test132_holonomy_observable.rs` | `observable:HolonomyObservable` fixture (Amendment 46) |
| test133_incompatibility_metric | `tests/fixtures/test133_incompatibility_metric.rs` | `observable:IncompatibilityMetric` fixture (Amendment 46) |
| test134_stratum_value | `tests/fixtures/test134_stratum_value.rs` | `observable:StratumValue` fixture (Amendment 46) |
| test135_stratum_delta | `tests/fixtures/test135_stratum_delta.rs` | `observable:StratumDelta` fixture (Amendment 46) |
| test136_stratum_trajectory | `tests/fixtures/test136_stratum_trajectory.rs` | `observable:StratumTrajectory` fixture (Amendment 46) |
| test137_path_length | `tests/fixtures/test137_path_length.rs` | `observable:PathLength` fixture (Amendment 46) |
| test138_total_variation | `tests/fixtures/test138_total_variation.rs` | `observable:TotalVariation` fixture (Amendment 46) |
| test139_winding_number | `tests/fixtures/test139_winding_number.rs` | `observable:WindingNumber` fixture (Amendment 46) |
| test140_cascade_length | `tests/fixtures/test140_cascade_length.rs` | `observable:CascadeLength` fixture (Amendment 46) |
| test141_cascade_count | `tests/fixtures/test141_cascade_count.rs` | `observable:CascadeCount` fixture (Amendment 46) |
| test142_catastrophe_threshold | `tests/fixtures/test142_catastrophe_threshold.rs` | `observable:CatastropheThreshold` fixture (Amendment 46) |
| test143_catastrophe_count | `tests/fixtures/test143_catastrophe_count.rs` | `observable:CatastropheCount` fixture (Amendment 46) |
| test144_commutator | `tests/fixtures/test144_commutator.rs` | `observable:Commutator` fixture (Amendment 46) |
| test145_curvature_flux | `tests/fixtures/test145_curvature_flux.rs` | `observable:CurvatureFlux` fixture (Amendment 46) |
| test146_parallel_transport | `tests/fixtures/test146_parallel_transport.rs` | `observable:ParallelTransport` fixture (Amendment 47) |
| test147_cascade_entropy | `tests/fixtures/test147_cascade_entropy.rs` | `observable:CascadeEntropy` fixture (Amendment 47) |
| test148_phase_boundary_type | `tests/fixtures/test148_phase_boundary_type.rs` | `observable:PhaseBoundaryType` enum class fixture (Amendment 47) |
| test149_face_map | `tests/fixtures/test149_face_map.rs` | `homology:FaceMap` fixture (Amendment 47) |
| test150_nerve_functor | `tests/fixtures/test150_nerve_functor.rs` | `homology:NerveFunctor` fixture (Amendment 47) |
| test151_chain_functor | `tests/fixtures/test151_chain_functor.rs` | `homology:ChainFunctor` fixture (Amendment 47) |
| test152_restriction_map | `tests/fixtures/test152_restriction_map.rs` | `cohomology:RestrictionMap` fixture (Amendment 47) |
| test153_coherence_proof | `tests/fixtures/test153_coherence_proof.rs` | `proof:CoherenceProof` fixture (Amendment 47) |
| test154_derivation_step | `tests/fixtures/test154_derivation_step.rs` | `derivation:DerivationStep` fixture (Amendment 47) |
| test155_computation_step | `tests/fixtures/test155_computation_step.rs` | `trace:ComputationStep` fixture (Amendment 47) |
| test156_trace_metrics | `tests/fixtures/test156_trace_metrics.rs` | `trace:TraceMetrics` fixture (Amendment 47) |
| test157_isometry_certificate | `tests/fixtures/test157_isometry_certificate.rs` | `cert:IsometryCertificate` fixture (Amendment 47) |
| test158_lift_chain_certificate | `tests/fixtures/test158_lift_chain_certificate.rs` | `cert:LiftChainCertificate` fixture (Amendment 47) |
| test159_chain_audit_trail | `tests/fixtures/test159_chain_audit_trail.rs` | `cert:ChainAuditTrail` fixture (Amendment 47) |
| test160_shared_context | `tests/fixtures/test160_shared_context.rs` | `state:SharedContext` + `ContextLease` (Amendment 48) |
| test161_execution_policy | `tests/fixtures/test161_execution_policy.rs` | `resolver:ExecutionPolicy` + `ExecutionPolicyKind` (Amendment 48) |
| test162_session_composition | `tests/fixtures/test162_session_composition.rs` | `state:SessionComposition` (Amendment 48) |
| test163_distributed_saturation | `tests/fixtures/test163_distributed_saturation.rs` | Distributed saturation MC_6/MC_7 (Amendment 48) |
| test164_embedding | `tests/fixtures/test164_embedding.rs` | `morphism:Embedding` — quantum-level injective transform (Amendment 49) |
| test165_action | `tests/fixtures/test165_action.rs` | `morphism:Action` — group action on type space (Amendment 49) |
| test166_session_boundary_type | `tests/fixtures/test166_session_boundary_type.rs` | `state:SessionBoundaryType` vocabulary (Amendment 49) |
| test167_metric_axis | `tests/fixtures/test167_metric_axis.rs` | `type:MetricAxis` vocabulary — three metric axes (Amendment 49) |
| test168_witt_carry | `tests/fixtures/test168_witt_carry.rs` | `op:WC_1` — Witt coordinate identification identity (Amendment 53) |
| test169_arithmetic_valuation | `tests/fixtures/test169_arithmetic_valuation.rs` | `op:ArithmeticValuation` verification domain (Amendment 53) |
| test170_kan_complex | `tests/fixtures/test170_kan_complex.rs` | `homology:KanComplex` + `HornFiller` (Amendment 54) |
| test171_postnikov_truncation | `tests/fixtures/test171_postnikov_truncation.rs` | `homology:PostnikovTruncation` + `KInvariant` (Amendment 54) |
| test172_homotopy_group | `tests/fixtures/test172_homotopy_group.rs` | `observable:HomotopyGroup` + homotopy observables (Amendment 54) |
| test173_homotopy_end_to_end | `tests/fixtures/test173_homotopy_end_to_end.rs` | Homotopy end-to-end: KanComplex → PostnikovTruncation → HomotopyGroup (Amendment 54) |
| test174_homotopy_resolver | `tests/fixtures/test174_homotopy_resolver.rs` | `resolver:HomotopyResolver` + homotopyTarget/homotopyResult (Amendment 55) |
| test175_homotopy_pipeline | `tests/fixtures/test175_homotopy_pipeline.rs` | Extended ψ-pipeline: ConstraintNerve → KanComplex → PostnikovTruncation (Amendment 55) |
| test176_moduli_space | `tests/fixtures/test176_moduli_space.rs` | `type:ModuliSpace` + moduliQuantumLevel + moduliDimension (Amendment 56) |
| test177_deformation_complex | `tests/fixtures/test177_deformation_complex.rs` | `homology:DeformationComplex` + tangentDimension + obstructionDimension (Amendment 56) |
| test178_holonomy_stratum | `tests/fixtures/test178_holonomy_stratum.rs` | `type:HolonomyStratum` + stratumCodimension (Amendment 56) |
| test179_moduli_end_to_end | `tests/fixtures/test179_moduli_end_to_end.rs` | Moduli end-to-end: ModuliSpace → DeformationComplex → HolonomyStratum (Amendment 56) |
| test180_moduli_resolver | `tests/fixtures/test180_moduli_resolver.rs` | `resolver:ModuliResolver` + moduliTarget + moduliDeformation (Amendment 57) |
| test181_stratification_record | `tests/fixtures/test181_stratification_record.rs` | `observable:StratificationRecord` + stratificationLevel (Amendment 57) |
| test182_whitehead_product | `tests/fixtures/test182_whitehead_product.rs` | `observable:WhiteheadProduct` + whiteheadTrivial (Amendment 54 gap closure) |
| test183_deformation_family | `tests/fixtures/test183_deformation_family.rs` | `type:DeformationFamily` + familyParameter + familyPreservesCompleteness (Amendment 56 gap closure) |
| test184_versal_deformation | `tests/fixtures/test184_versal_deformation.rs` | `type:VersalDeformation` + versalBase + versalDimension (Amendment 56 gap closure) |

## Structural Validation

Since a full SHACL engine is not included as a runtime dependency, the conformance
suite performs structural validation of each instance graph:

1. **Syntax check**: The Turtle source is non-empty and contains `@prefix` declarations.
2. **Required term check**: Each test fixture must contain the required class and property IRIs.
3. **Type check**: Named individuals must have type assertions referencing known classes.

Full SHACL engine validation (e.g., using Apache Jena's `shacl validate`) can be
run externally against the generated ontology and test fixtures.

## Writing New Test Fixtures

New instance graphs should:
1. Be placed in `conformance/src/tests/fixtures/` as `test<n>_<name>.rs`
2. Declare all required namespaces via `@prefix`
3. Use full IRI constants from `conformance/shapes/uor-shapes.ttl`
4. Include at least one `owl:NamedIndividual` with a `sh:targetClass`-covered type

## References

- [SHACL W3C Specification](https://www.w3.org/TR/shacl/)
- [SHACL Core Constraints](https://www.w3.org/TR/shacl/#core-components)
