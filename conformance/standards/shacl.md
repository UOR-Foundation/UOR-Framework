# SHACL Conformance Standards

## Overview

The UOR conformance suite validates 15 OWL instance graphs against SHACL NodeShapes
defined in `conformance/shapes/uor-shapes.ttl`. One NodeShape is defined per
ontology class (98 total).

## Shape File

`conformance/shapes/uor-shapes.ttl` contains:
- 98 `sh:NodeShape` declarations (one per class)
- `sh:targetClass` targeting each OWL class
- Cardinality constraints (`sh:minCount`, `sh:maxCount`) on required properties
- Type constraints (`sh:class`, `sh:datatype`) on property values

## The 15 Instance Tests

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
