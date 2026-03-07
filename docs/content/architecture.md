# Architecture

## Workspace Layout

The UOR Framework is a Rust workspace with seven member crates:

| Crate | Type | Role |
|-------|------|------|
| `uor-ontology` | Library | Ontology as typed Rust data + serializers (internal, not published) |
| `uor-codegen` | Library | Code generator: ontology → Rust traits (internal) |
| `uor-foundation` | Library | **Generated** Rust trait crate (published to crates.io) |
| `uor-conformance` | Library | Workspace-wide conformance validators |
| `uor-docs` | Library | Documentation generator |
| `uor-website` | Library | Static site generator |
| `uor-clients` | Binaries | CLI tools: build, conformance, docs, website, crate |

## Dependency Order

```
uor-ontology (spec/)
  ↓ Ontology::full()
  ├── uor-build      → public/uor.foundation.{json,ttl,nt}   (RDF/OWL export)
  ├── uor-docs       → public/docs/                           (documentation export)
  ├── uor-website    → public/                                 (website export)
  ├── uor-crate      → foundation/src/                         (Rust export)
  └── uor-conformance → validates all of the above
```

## spec/ Library (uor-ontology)

`uor-ontology` encodes the complete ontology as typed Rust static data:

- **`model.rs`**: Core types — `Namespace`, `Class`, `Property`, `Individual`, `Ontology`
- **`namespaces/*.rs`**: 16 modules, one per namespace (all {@count:amendments} amendments applied)
- **`serializer/jsonld.rs`**: Serializes to JSON-LD 1.1
- **`serializer/turtle.rs`**: Serializes to Turtle 1.1
- **`serializer/ntriples.rs`**: Serializes to N-Triples

The entry point `Ontology::full()` uses `OnceLock` for thread-safe lazy initialization.

This crate is the single source of truth. It is internal (`publish = false`).

## codegen/ Library (uor-codegen)

`uor-codegen` generates the published Rust trait crate from the ontology:

- **`mapping.rs`**: Namespace → module, XSD → Rust type, class IRI → path tables
- **`traits.rs`**: Class → trait, property → method generation
- **`enums.rs`**: Enum detection (PrimitiveOp, MetricAxis, Space, FiberState, GeometricCharacter, VerificationDomain, ComplexityClass, RewriteRule, MeasurementUnit, CoordinateKind, SessionBoundaryType, PhaseBoundaryType, SaturationPhase, AchievabilityStatus, ProofModality) and QuantumLevel newtype struct generation
- **`individuals.rs`**: Named individual → const module / PrimitiveOp impl generation
- **`emit.rs`**: Rust source code writer

## foundation/ Library (uor-foundation)

`uor-foundation` is the **generated** published crate. Every file in `foundation/src/`
is produced by `uor-crate` — never hand-edited.

- **{@count:traits} traits** (one per OWL class, generic over `Primitives`)
- **365 methods** (one per property with a domain)
- **15 enums** (Space, PrimitiveOp, MetricAxis, FiberState, GeometricCharacter, VerificationDomain, ComplexityClass, RewriteRule, MeasurementUnit, CoordinateKind, SessionBoundaryType, PhaseBoundaryType, SaturationPhase, AchievabilityStatus, ProofModality) **+ 1 struct** (QuantumLevel)
- **699 constant modules** (for named individuals)
- **Zero mandatory dependencies** — pure traits

Module structure: `kernel/` (3 namespaces), `bridge/` (10 namespaces), `user/` (3 namespaces).

## conformance/ Library

`uor-conformance` is the workspace quality gate. It validates:

1. **Rust source**: API documentation, style conventions
2. **Ontology artifacts**: JSON-LD 1.1, OWL 2 DL constraints, RDF 1.1, Turtle 1.1, inventory counts
3. **SHACL instances**: {@count:shacl_tests} test graphs validated against {@count:shapes} NodeShapes
4. **Generated crate**: Trait completeness, method completeness, individual completeness
5. **Documentation**: Diataxis structure, completeness, accuracy, links
6. **Website**: HTML5, WCAG 2.1 AA, CSS, coverage, links

The conformance suite is the **single gate** — all components must pass before a release.

## docs/ Library

`uor-docs` generates documentation with enforced accuracy:

- Namespace reference pages are **100% auto-generated** from `uor_ontology::Ontology::full()`
- Prose pages use `{@class}`, `{@prop}`, `{@ind}` DSL, validated at build time
- Every spec term must appear in at least one page (completeness check)

## website/ Library

`uor-website` generates the static site at `https://uor.foundation/`:

- Templates use the Tera template engine
- Namespace pages are auto-generated (no hand-written HTML for spec terms)
- Search index is generated from all 200 classes, 387 properties, 712 individuals
- No external dependencies (no CDN, no tracking, no third-party scripts)

## Build Pipeline

```
cargo run --bin uor-build       → public/uor.foundation.{json,ttl,nt}
cargo run --bin uor-crate       → foundation/src/ (generated Rust traits)
cargo run --bin uor-docs        → public/docs/ + README.md
cargo run --bin uor-website     → public/ (HTML, CSS, JS, search-index.json)
cargo run --bin uor-conformance → validates all of the above
```

## Amendment History

The spec crate implements all 36 amendments from the UOR Foundation completion plan:

| Amendment | Namespace | Key Additions |
|-----------|-----------|---------------|
| 1 | op/ | 10 named operation individuals |
| 2 | schema/ | Ring class, 6 properties, pi1/zero individuals |
| 3 | op/, proof/ | Identity class, criticalIdentity individual, provesIdentity property |
| 4 | op/ | Group/DihedralGroup classes, D2n individual |
| 5 | partition/ (NEW) | 6 classes, 9 properties |
| 6 | morphism/ (NEW) | 4 classes, 10 properties |
| 7 | state/ (NEW) | 4 classes, 16 properties |
| 8 | all | uor:space annotation property on all namespaces |
| 9 | partition/ | FiberCoordinate, FiberBudget, FiberPinning; 11 properties |
| 10 | type/ | Constraint hierarchy (6 classes), MetricAxis, 3 axis individuals |
| 11 | resolver/, derivation/ | ResolutionState, RefinementSuggestion, DerivationStep/RefinementStep |
| 12 | morphism/ | Composition, Identity, CompositionLaw; criticalComposition individual |
| 13 | u/, op/ | Content addressing (AD_1/AD_2), AddressHierarchy, 4 properties |
| 14 | op/ | 50 Ring/Boolean/XOR/Dihedral/Universal/Group/Carry identities |
| 15 | op/ | 42 Constraint/Fiber/Resolution identities |
| 16 | op/ | 52 Observable/Cert/Structure identities |
| 17 | op/ | 64 Transform/Automorphism/Embedding/Analytical identities |
| 18 | observable/, resolver/ | Jacobian, TopologicalObservable, BettiNumber, SpectralGap, ConstraintNerve |
| 19 | op/ | 21 Thermodynamic/Analytical/Partition/Resolution identities |
| 20 | op/ | 6 Differential calculus, Homological algebra, Index theorem identities |
| 21 | op/, homology/, cohomology/ | Structural Reasoning namespaces, ψ-pipeline, identity grounding |
| 22 | morphism/, state/ | TopologicalDelta class, topologicalSnapshot property |
| 23 | op/, resolver/, derivation/, observable/, type/, query/ | Typed controlled vocabularies: 7 new classes, 35 individuals, 12 new properties |
| 24 | morphism/, query/, type/, cert/ | Surface Grounding Layer, GroundingMap, CompleteType, graph gap closures |
| 25 | type/, resolver/, cert/ | Completeness Certification: CompletenessCandidate, CompletenessWitness, CompletenessResolver |
| 26 | schema/, op/, resolver/ | Quantum Level Scaling: Q1Ring, QuantumLevelBinding, QuantumLevelResolver |
| 27 | state/, resolver/, query/ | Session-Scoped Resolution: Session, SessionBoundary, SessionBoundaryType enum |
| 28 | type/, resolver/, derivation/, observable/ | ψ-Pipeline Inversion (Type Synthesis): TypeSynthesisGoal, TypeSynthesisResult |
| 29 | type/, observable/, resolver/ | Quantum Level Spectral Sequence: QuantumLift, LiftObstruction, SpectralSequencePage |
| 30 | observable/, type/, resolver/ | Monodromy Observables: HolonomyGroup, Monodromy, FlatType, TwistedType |
| 31 | proof/, observable/, resolver/, partition/, trace/ | Ontology Gap Closures: EmpiricalVerification, PhaseBoundaryType enum, JacobianGuidedResolver, PT\_/ST\_ identities |
| 32 | type/, resolver/, op/ | RC\_5 Superposition Vocabulary: SuperposedFiberState, SuperpositionResolver, SuperpositionDomain |
| 33 | state/, cert/, resolver/ | Saturated Context Limit: SaturatedContext, SaturationWitness, SaturationPhase enum, SC\_1–SC\_7 |
| 34 | proof/, type/, observable/ | Morphospace Achievability: ImpossibilityWitness, MorphospaceRecord, AchievabilityStatus enum, MS\_1–MS\_5 |
| 35 | trace/, cert/, resolver/ | Computational Geodesic: GeodesicTrace, GeodesicViolation, GeodesicCertificate, GD\_1–GD\_5 |
| 36 | resolver/, trace/, cert/, type/, op/ | Measurement Boundary: MeasurementResolver, MeasurementEvent, CollapsedFiberState, QuantumThermodynamicDomain, QM\_1–QM\_4 |
