# Changelog

All notable changes to UOR-Framework are documented in this file.

## v0.2.2 — 2026-04-14

v0.2.2 closes the five v0.2.1 enforcement escape hatches, ships the three
residual ontology items, addresses four deeper correctness items, and lands
five cross-cutting items. **18 work items total.** Backwards compatibility is
not a constraint; the release criterion is *no second path*.

### BREAKING — surface deletions

- **W1**: deleted `uor_ground!` macro (entire `uor-foundation-macros` crate).
- **W2**: deleted `#[derive(ConstrainedType)]` and `#[derive(CompileUnit)]`.
- **W3**: deleted `#[uor_grounded(level = "...")]` attribute.
- **W15**: **deleted the entire `uor-foundation-macros` crate** from the
  workspace. Removed from `Cargo.toml` workspace members. The pipeline now
  uses direct `pub(crate)` constructors. The contract is enforced at the
  type and visibility level, not at the macro level.
- **W2 cascade**: deleted `__macro_internals::GroundedShapeSealed` back-door,
  `MacroProvenance`, `__uor_macro_mint_validated`, `__uor_macro_mint_grounded`.

### Ontology additions

- **W7** (`spec/src/namespaces/op.rs`): corrected `op:Pipeline` and
  `op:Topological` rdfs:comments to reflect the actual ψ_1..6 / ψ_7..9
  inter-algebra map split. The earlier ψ_1..6 chain (constraint nerve →
  simplicial homology) is established under `op:Topological`; the later
  ψ_7..9 tower (Postnikov truncation, homotopy group extraction,
  k-invariant computation) is established under `op:Pipeline`.
- **W8** (`spec/src/namespaces/schema.rs`, `query.rs`, `state.rs`):
  `schema:Triad` gains three functional projection properties
  (`triadStratum`, `triadSpectrum`, `triadAddress`) bundling the canonical
  observable triple of a Datum at grounding time. `query:RingElement`
  renamed to `query:Address`. `state:groundedTriad` added on
  `state:GroundedContext`.
- **W4** (`spec/src/namespaces/morphism.rs`): two new GroundingMap individuals
  — `morphism:DigestGroundingMap` (one-way hash; total but not invertible;
  no structure preservation) and `morphism:BinaryGroundingMap` (raw byte
  ingestion; total and invertible; no structure beyond bit identity).
- **W14** (`spec/src/namespaces/reduction.rs`): added
  `reduction:ShapeMismatch` PipelineFailureReason individual with two
  FailureField individuals for the `expected` and `got` shape IRIs. The
  parametric `PipelineFailure` enum codegen picks it up automatically.

### Ontology counts (`spec/src/counts.rs`)

- `PROPERTIES`: 928 → **932** (+4 W8 properties)
- `NAMESPACE_PROPERTIES`: 927 → **931**
- `INDIVIDUALS`: 3443 → **3448** (+5: 2 GroundingMap + 1 ShapeMismatch + 2 FailureField)
- `METHODS`: 891 → **895**
- `LEAN_CONSTANT_NAMESPACES`: 3343 → **3348**
- `CONFORMANCE_CHECKS`: 474 → **476** (+2 new validators)

### Rust enforcement surface (additions)

- **W11** `enforcement::Certificate` sealed trait + `Certified<C>` parametric
  carrier. Replaces the v0.2.1 per-class shim duplication. All 10
  `cert:Certificate` subclasses now have a sealed Rust kind that implements
  `Certificate` with an `IRI` constant and `Evidence` associated type.
  Six previously-unshimmed classes (`TransformCertificate`,
  `IsometryCertificate`, `InvolutionCertificate`, `GeodesicCertificate`,
  `MeasurementCertificate`, `BornRuleVerification`) gain Rust visibility.
  Supporting evidence types (`CompletenessAuditTrail`, `ChainAuditTrail`,
  `GeodesicEvidenceBundle`) are exposed as concrete public structs.
- **W12** `enforcement::resolver::*::certify` free functions replace the
  v0.2.1 unit-struct façades:
  - `enforcement::resolver::inhabitance::certify(input)`
  - `enforcement::resolver::tower_completeness::certify(input)`
  - `enforcement::resolver::incremental_completeness::certify(input)`
  - `enforcement::resolver::grounding_aware::certify(unit)`

  Each returns `Result<Certified<Cert>, Witness>`. The v0.2.1 unit structs
  remain alongside the new free functions for the v0.2.2 release cycle.
- **W4** `enforcement::Grounding` trait gains `type Map: GroundingMapKind`
  associated type. Sealed marker traits `GroundingMapKind`,
  `PreservesMetric`, `PreservesStructure`, `Total`, `Invertible` partition
  the kinds by structural property. Foundation operations requiring
  structure preservation gate on `<G as Grounding>::Map: PreservesStructure`
  and reject digest-style impls at the call site. Five sealed kind structs
  (`IntegerGroundingMap`, `Utf8GroundingMap`, `JsonGroundingMap`,
  `DigestGroundingMap`, `BinaryGroundingMap`) implement the marker-trait
  table from the v0.2.2 plan.
- **W3** Unary phantom-typed ring ops (`Neg<L>`, `BNot<L>`, `Succ<L>`)
  next to the existing binary `Add/Sub/Mul/And/Or/Xor<L>`. New `UnaryRingOp`
  trait. New `Embed<From, To>` sealed level promotion (canonical injection
  ι : R_n → R_n′ for n ≤ n′), gated by the sealed `ValidLevelEmbedding`
  trait. Downward coercion is intentionally not supplied — projection is
  lossy and goes through `morphism:ProjectionMap` instances.
- **W13** `enforcement::Validated<T, Phase: ValidationPhase = Runtime>`
  parametric phase. New sealed `ValidationPhase` trait with `CompileTime`
  and `Runtime` markers. `From<Validated<T, CompileTime>> for
  Validated<T, Runtime>` impl provides the subsumption: a compile-time
  witness is usable wherever a runtime witness is required. The default
  phase is `Runtime` so v0.2.1 call sites that wrote `Validated<T>`
  continue to compile unchanged.
- **W14** `pipeline::run<T, P>` typed entry point: consumes
  `Validated<CompileUnit, P>`, returns `Result<Grounded<T>, PipelineFailure>`
  for an explicit `T: GroundedShape` and `P: ValidationPhase`. New
  `CompileUnit::witt_level()` and `CompileUnit::thermodynamic_budget()`
  accessors. New `PipelineFailure::ShapeMismatch { expected, got }` variant
  emitted automatically by the parametric `PipelineFailure` codegen from
  the W14 ontology addition.
- **W8** `enforcement::Triad<L>` struct: bundles the (stratum, spectrum,
  address) projection of a Datum at grounding time. Phantom-typed at level
  `L`, no public constructor — built only by foundation code. Field access
  via `stratum()`, `spectrum()`, `address()` accessors.
- **W10** `HostTypes` trait + `DefaultHostTypes` canonical impl. Narrows
  the v0.2.1 six-slot `Primitives` trait to the four slots that genuinely
  vary across host environments (`Decimal`, `DateTime`, `HostString`,
  `WitnessBytes`). Foundation-owned types (Witt-level integers, booleans,
  IRIs, canonical bytes) are derived from `WittLevel` and not exposed.
  `Primitives` remains as a deprecated alias for v0.2.1 backwards
  compatibility.

### Conformance suite

- **W5** new validator `docs/psi_leakage`: scans the consumer-facing crate
  surface (`README.md`, `foundation/README.md`, `foundation/docs/`) for
  unauthorized ψ vocabulary references. Mathematically correct internal use
  in `proof/`, `op/`, `homology/`, `cohomology/`, `derivation/` is excluded.
- **W6** new validator `rust/public_api_snapshot`: pins the exact set of
  `pub` items in `uor-foundation`'s enforcement, lib, and pipeline modules
  to a snapshot file at `foundation/tests/public-api.snapshot`. Drift
  requires explicit snapshot update review. Initial baseline: **129
  pinned symbols**.
- v0.2.2 release artifact `public/uor.conformance.ebnf` joins
  `public/uor.term.ebnf` as a complete release artifact emitted by
  `cargo run --bin uor-build`. The conformance EBNF grammar is published
  alongside the primary Term-language grammar.

### Tests (W17)

New test files under `foundation/tests/`:

- `grounding_map_kind_markers.rs` — exact marker-trait coverage per W4 plan
  table; one test per kind asserts which markers it implements.
- `host_types_surface.rs` — pins the exact `HostTypes` shape, asserts
  `DefaultHostTypes` selects `f64`/`i64`/`str`/`[u8]`, demonstrates an
  embedded-host override.
- `validated_phases.rs` — asserts `ValidationPhase` is implemented by
  `CompileTime` and `Runtime`, that the default phase resolves to `Runtime`,
  and that the `From<Validated<_, CompileTime>>` subsumption compiles.
- `unary_ring_ops.rs` — exercises `Neg<W8>`, `BNot<W8>`, `Succ<W8>`,
  `Neg<W32>`, plus `Embed<W8, W16>` and `Embed<W8, W32>` widening. Verifies
  the critical-composition law `Succ = Neg ∘ BNot` directly.

### Documentation (W18)

- Crate-level `//!` rustdoc rewritten as a v0.2.2 principal-data-path
  tutorial with an ASCII diagram showing the
  `host bytes → Grounding<Map> → Datum → Validated<T, Phase> → pipeline::run::<T, P> → Grounded<T> → Triad<L>` flow.
- Migration table from v0.2.1 to v0.2.2 (each deleted symbol mapped to its
  v0.2.2 replacement) embedded in the crate-root rustdoc.
- `enforcement::prelude` re-exports the full v0.2.2 surface
  (`Certified`, `Triad`, `Certificate`, `GroundingMapKind`, marker traits,
  cert kind structs, Validation phases, unary ring ops, Embed) alongside
  the v0.2.1 carry-over symbols.

### Deferred to v0.2.3+

- **`uor-foundation-clippy` dylint crate**: the v0.2.2 plan §W6 envisioned
  a dylint-based custom lint group for catching escape-hatch construction
  attempts in downstream code. The contract is already enforced at the
  type and visibility level (`pub(crate)` constructors, sealed traits,
  the public-API snapshot validator), so the dylint adds defense-in-depth
  rather than net-new safety. Deferred to v0.2.3 to avoid coupling the
  toolchain to a specific Clippy/HIR pin.
- **`Grounding`-combinator-only verification** (W4 "honest limit"): making
  `ground()` implementable only via foundation-supplied combinators so the
  foundation can verify (not just tag) that a `DigestGroundingMap` impl is
  actually deterministic and total. This is the 1.0.0 stability prerequisite.
- **Embedded developer cookbook** (W18 expansion): a `cookbook` module
  with 10 doc-only recipes for common principal-path patterns. Deferred
  to v0.2.3 alongside the rewritten consumer-facing concept docs.
- **Website docs editorial sweep**: removing ψ from
  `docs/content/concepts/` markdown pages. The v0.2.2 ψ-leakage gate is
  scoped to the consumer-facing crate surface; the website sweep is a
  separate editorial undertaking in v0.2.3.

## v0.2.1 — 2026-04-13

v0.2.1 bundles the **Inhabitance Verdict Instantiation** ontology release with
the **Zero-Overhead Ergonomics Surface** Rust/Lean 4 additions. Every item is
strictly extensional with respect to v0.2.0 — no public signatures removed,
no breaking API changes.

### Ontology (strictly extensional)

- **New classes** (13): `cert:InhabitanceCertificate`, `proof:InhabitanceImpossibilityWitness`,
  `trace:InhabitanceSearchTrace`, `derivation:InhabitanceStep`, `derivation:InhabitanceCheckpoint`,
  `resolver:InhabitanceResolver`, `resolver:TwoSatDecider`, `resolver:HornSatDecider`,
  `resolver:ResidualVerdictResolver`, `resolver:CertifyMapping`, `schema:ValueTuple`,
  `reduction:FailureField`, `conformance:PreludeExport`.
- **New properties** (31) across `cert/`, `proof/`, `trace/`, `derivation/`,
  `predicate/`, `resolver/`, `conformance/`, `reduction/`, `parallel/`, `stream/`,
  `state/` — including the parametric metadata (`resolver:forResolver`,
  `conformance:surfaceForm`, `reduction:fieldName`, etc.) that drives the
  ontology-first code-generation pattern.
- **New individuals** (80+): `predicate:InhabitanceDispatchTable` plus 3 rules,
  4 `op:Identity` individuals (IH_1, IH_2a, IH_2b, IH_3) with full proof
  coverage, 4 `resolver:CertifyMapping` facts, 11 `reduction:FailureField`
  individuals, 16 `conformance:PreludeExport` individuals, 6 new
  `conformance:Shape` instances with 17 `PropertyConstraint` decompositions.

### Rust ergonomics surface (`uor-foundation` + `uor-foundation-macros`)

- **Sealed wrappers**: `Validated<T: OntologyTarget>` now auto-derefs to `T`;
  `Grounded<T: GroundedShape>` wraps the compile-time ground-state witness
  with O(1) binding lookup (`op:GS_5`).
- **`Certify<I>` trait** — generic over the input type so downstream user
  types flow directly through the resolver façades:
  ```rust
  let cert: Validated<LiftChainCertificate> =
      TowerCompletenessResolver::new().certify(&shape)?;
  let level: WittLevel = cert.target_level();
  ```
- **Four resolver façades** (`TowerCompletenessResolver`,
  `IncrementalCompletenessResolver`, `GroundingAwareResolver`,
  `InhabitanceResolver`) emitted parametrically from `resolver:CertifyMapping`
  individuals.
- **`PipelineFailure` enum** with 7 variants emitted from `reduction:FailureField`.
- **Ring-op phantom wrappers** (`Mul<L>`, `Add<L>`, `Sub<L>`, `Xor<L>`, etc.)
  at `W8` and `W16` with `const fn` implementations.
- **Fragment markers** (`Is2SatShape`, `IsHornShape`, `IsResidualFragment`)
  and `INHABITANCE_DISPATCH_TABLE` const.
- **Full reduction pipeline driver** at `uor_foundation::pipeline` — 6 preflight
  checks, 7 reduction stages, Aspvall-Plass-Tarjan 2-SAT decider,
  unit-propagation Horn-SAT decider, fragment classifier, FNV-1a unit-id hasher.
  `#![no_std]`-compatible. Backs every `Certify::certify` call.
- **Macro surface**: `uor!` (existing), `uor_ground!` (new — expands to real
  `Grounded<T>` via the back-door minting API with a trailing `as Grounded<T>`
  type clause), `#[derive(ConstrainedType)]` (emits `GroundedShape` +
  `ConstrainedTypeShape` impls carrying residue/hamming constraints),
  `#[uor_grounded(level = "WN")]` (compile-time Witt-level assertion).
- **`foundation::enforcement::prelude`** — 18-symbol re-export for the
  consumer-facing one-liners.

### Lean 4 parity (`lean4/UOR/`)

- **New modules**: `Enforcement.lean`, `Pipeline.lean`, `Prelude.lean`.
- `Certify (ρ : Type) (I : Type)` class generic over input type (Lean parity
  with the Rust `Certify<I>` trait).
- `UOR.Pipeline.runTowerCompleteness`, `runIncrementalCompleteness`,
  `runGroundingAware`, `runInhabitance` — Lean-side pipeline entry points.
- `ConstraintRef`, `FragmentKind`, `fragmentClassify` — structural parity
  with the Rust types.
- `lake build` compiles cleanly; `lake upload` publishes to Lean Reservoir.

### Tooling

- **New `cargo-uor` binary**:
  - `cargo uor check <path>` — walks a crate tree for `uor_ground!` invocations,
    parses the conformance grammar, and reports per-invocation validity.
  - `cargo uor inspect <class-name>` — reads the bundled ontology and prints
    the class IRI, const accessors (`GS_7_SATURATION_COST_ESTIMATE`,
    `OA_5_LEVEL_CROSSINGS`, `BUDGET_SOLVENCY_MINIMUM`), and the `rdfs:comment`.
  - `cargo uor explain <iri>` — resolves any ontology IRI (prefixed or
    full-URI form) to its `rdfs:comment`.

### Grammar

- `public/uor.conformance.ebnf` — **parametrically emitted** from the
  ontology's `conformance:Shape` and `PropertyConstraint` individuals via
  `spec/src/serializer/conformance_ebnf.rs`. Adding a new declaration shape
  requires only an ontology edit.

### Counts

| Metric | v0.2.0 | v0.2.1 | Delta |
|---|---|---|---|
| Namespaces | 33 | 33 | 0 |
| Classes | 441 | 454 | +13 |
| Properties | 890 | 921 | +31 |
| Individuals | 3358 | 3438 | +80 |
| `op:Identity` | 624 | 628 | +4 |
| Conformance checks | 471 | 472 | +1 |
| SHACL test fixtures | 276 | 277 | +1 |

### Verification

`cargo test --workspace` • `cargo clippy --all-targets -- -D warnings` •
`cargo run --bin uor-conformance` (`Conformance PASSED.`) •
`cargo check -p uor-foundation --no-default-features` •
`cargo test -p uor-foundation --no-default-features --test no_std` •
`cd /workspaces/UOR-Framework && lake build` (`Build completed successfully.`).

## v0.2.0 — 2026-03-10

Baseline release. See `RELEASING.md` for details.
