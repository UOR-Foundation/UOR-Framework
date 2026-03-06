# SHACL Conformance Standards

## Overview

The UOR conformance suite validates 53 OWL instance graphs against SHACL NodeShapes
defined in `conformance/shapes/uor-shapes.ttl`. One NodeShape is defined per
ontology class (180 total).

## Shape File

`conformance/shapes/uor-shapes.ttl` contains:
- 180 `sh:NodeShape` declarations (one per class)
- `sh:targetClass` targeting each OWL class
- Cardinality constraints (`sh:minCount`, `sh:maxCount`) on required properties
- Type constraints (`sh:class`, `sh:datatype`) on property values

## The 53 Instance Tests

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
