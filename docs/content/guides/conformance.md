# Conformance Guide

## Overview

The UOR conformance suite validates all workspace artifacts against professional
standards. Run it with:

```sh
cargo run --bin uor-conformance
```

## What Is Validated

### Ontology Conformance

| Artifact | Standard | Validator |
|----------|----------|-----------|
| `public/uor.foundation.json` | JSON-LD 1.1 | `validators/ontology/jsonld.rs` |
| `public/uor.foundation.json` | OWL 2 DL | `validators/ontology/owl.rs` |
| Inventory counts | 16/205/408/740 | `validators/ontology/inventory.rs` |
| `public/uor.foundation.ttl` | RDF 1.1 / Turtle 1.1 | `validators/ontology/rdf.rs` |
| {@count:shacl_tests} test instance graphs | SHACL | `validators/ontology/shacl.rs` |

### Documentation Conformance

| Check | Validator |
|-------|-----------|
| All 205 classes documented | `validators/docs/completeness.rs` |
| Namespace pages accurate | `validators/docs/accuracy.rs` |
| Diataxis structure present | `validators/docs/structure.rs` |
| No broken internal links | `validators/docs/links.rs` |

### Website Conformance

| Check | Standard | Validator |
|-------|----------|-----------|
| HTML5 structure | HTML5 | `validators/website/html.rs` |
| Accessibility | WCAG 2.1 AA | `validators/website/accessibility.rs` |
| Namespace page coverage | — | `validators/website/coverage.rs` |
| CSS validity | CSS | `validators/website/css.rs` |
| Internal links | — | `validators/website/links.rs` |

### SHACL Tests 34–53 (v3.4.0–v4.0.0)

| Test | What It Validates |
|------|-------------------|
| test34 | CompletenessCandidate, CompletenessWitness, CompletenessResolver (Amendment 25) |
| test35 | CompletenessCertificate, CompletenessAuditTrail, witnessCount (Amendment 25) |
| test36 | Q1Ring, Q1bitWidth, Q1capacity, nextLevel chain (Amendment 26) |
| test37 | QuantumLevelBinding, universallyValid, verifiedAtLevel (Amendment 26) |
| test38 | Session, BindingAccumulator, SessionResolver, SessionQuery (Amendment 27) |
| test39 | SessionBoundary, SessionBoundaryType vocabulary individuals (Amendment 27) |
| test40 | TypeSynthesisGoal, TypeSynthesisResolver, synthesisGoal (Amendment 28) |
| test41 | Full synthesis round-trip: Goal→Resolver→Result→SynthesizedType→MinimalConstraintBasis→SynthesisSignature→SynthesisStep (Amendment 28) |
| test42 | QuantumLift, LiftObstruction (obstructionTrivial=true), IncrementalCompletenessResolver (Amendment 29) |
| test43 | SpectralSequencePage: page 1 (differentialIsZero=false) → page 2 (convergedAt=2) (Amendment 29) |
| test44 | FlatType + HolonomyGroup (order=1) + Monodromy (isTrivialMonodromy=true) + ClosedConstraintPath (Amendment 30) |
| test45 | TwistedType + non-trivial HolonomyGroup + LiftObstruction (obstructionTrivial=false) + LiftObstructionClass + DihedralElement (Amendment 30) |
| test46 | MonodromyResolver end-to-end pipeline: ConstrainedType → HolonomyGroup → MonodromyClass → TwistedType (Amendment 30) |
| test47 | ThermoObservable + hardnessEstimate + ComputationTrace + residualEntropy (Amendment 31) |
| test48 | CatastropheObservable + phaseN/phaseG + PhaseBoundaryType + onResonanceLine (Amendment 31) |
| test49 | FiberBudget + FiberCoordinate + ancillaFiber + reversibleStrategy (Amendment 31) |
| test50 | JacobianGuidedResolver + ResolutionState + guidingJacobian (Amendment 31) |
| test51 | ProductType + component assertions + FiberBudget (Amendment 31) |
| test52 | SumType + variant assertions (Amendment 31) |
| test53 | SuperposedFiberState + amplitude + SuperpositionResolver (Amendment 32) |

### SHACL Tests 54–74 (v4.1.0)

| Test | What It Validates |
|------|-------------------|
| test54 | SaturatedContext + saturationDegree + contextTemperature + isSaturated (Amendment 33) |
| test55 | SaturationWitness + witnessBinding + witnessStep + residualFreeCount (Amendment 33) |
| test56 | DomainSaturationRecord + saturatedDomain + domainFreeCount (Amendment 33) |
| test57 | SaturationPhase vocabulary: Unsaturated, PartialSaturation, FullSaturation (Amendment 33) |
| test58 | SaturationCertificate + certifiedSaturation + saturationWitness (Amendment 33) |
| test59 | SaturationAwareResolver + usedSaturation (Amendment 33) |
| test60 | ImpossibilityWitness + forbidsSignature + impossibilityReason (Amendment 34) |
| test61 | MorphospaceRecord + achievabilityStatus + verifiedAtLevel (Amendment 34) |
| test62 | MorphospaceBoundary + boundaryType (Amendment 34) |
| test63 | ForbiddenSignature + targetForbidden (Amendment 34) |
| test64 | AchievabilityStatus vocabulary: Achievable, Forbidden (Amendment 34) |
| test65 | GeodesicTrace + isGeodesic + geodesicCertificate + stepEntropyCost (Amendment 35) |
| test66 | GeodesicCertificate + certifiedGeodesic + geodesicTrace (Amendment 35) |
| test67 | GeodesicViolation + violationReason (Amendment 35) |
| test68 | GeodesicValidator + validateGeodesic (Amendment 35) |
| test69 | GeodesicTrace + adiabaticallyOrdered + jacobianAtStep (Amendment 35) |
| test70 | MeasurementResolver + collapseAmplitude + collapsedFiber (Amendment 36) |
| test71 | MeasurementEvent + preCollapseEntropy + postCollapseLandauerCost (Amendment 36) |
| test72 | MeasurementCertificate + certifiedMeasurement + vonNeumannEntropy + landauerCost (Amendment 36) |
| test73 | CollapsedFiberState + collapsedFrom + survivingAmplitude (Amendment 36) |
| test74 | QuantumThermodynamicDomain + QuantumThermodynamic verification domain (Amendment 36) |

## Adding a New SHACL Test

1. Create `conformance/src/tests/fixtures/test<n>_<name>.rs`
2. Define a `pub const TEST<N>_<NAME>: &str = r#"..."#;` with Turtle source
3. Export it from `conformance/src/tests/fixtures/mod.rs`
4. Register it in `conformance/src/validators/ontology/shacl.rs`
5. Add a check function `validate_<name>(src: &str) -> Result<(), String>`

## Running Individual Validators

The conformance library is structured so each validator can be called independently:

```rust
use uor_conformance::validators::ontology::owl;

let report = owl::validate();
assert!(report.all_passed());
```

## CI Integration

The CI workflow runs full conformance as the last step:

```yaml
- run: cargo run --bin uor-conformance  # exits non-zero on failure
```
