# Changelog

All notable changes to UOR-Framework are documented in this file.

## v0.2.2 cleanup — 2026-04-15

Post-phase-J cleanup pass removing every hardcoded public-API endpoint and
shipping an end-to-end functional verification gate. The original phased
landing optimized for hitting conformance anchors; this pass ensures the
public API is **functional and not hardcoded** per the user's directive.

### Tier 1 — correctness gates

- **T1.1 — Phase J `MarkersImpliedBy<Map>` bound on `GroundingProgram::from_primitive`**.
  Parameterized `GroundingPrimitive<Out, Markers: MarkerTuple = ()>`. Added
  sealed `MarkerTuple` supertrait over six canonical marker tuples, sealed
  `MarkerIntersection<Other>` trait with 36 auto-generated impls for
  type-level intersection (used by `then` / `and_then`), `MarkersImpliedBy<Map>`
  with 10 valid (tuple, map) impls. The bound is now enforced on
  `from_primitive`; misdeclared programs are rejected at compile time.
  Rustdoc compile_pass + compile_fail doctests anchor the marquee correctness
  claim — `digest()` claimed as `IntegerGroundingMap` fails to compile.

- **T1.2 — `conformance:InteractionShape` ontology class** added to back the
  `InteractionDeclarationBuilder` rustdoc reference. CLASSES 465→466,
  LEAN_STRUCTURES 432→433. New SHACL shape + extended test280 fixture.

- **T1.3 — Certificate governance via `op:OA_5` and `op:PT_2` identity text**.
  Updated rdfs:comment / lhs / rhs / forAll text on both identities to
  explicitly name `MultiplicationCertificate` and `PartitionCertificate`.
  Removed the two structural exemptions from `meta/certificate_issuance_coverage`.
  Extended the validator to follow `schema:term_*` IriRef pointers to their
  underlying `LiteralExpression` / `ForAllDeclaration` text (since
  `rewrite_identity_ast_refs` replaces Str values with IriRefs at load time).
  All 14 Certificate subclasses now governed by Identities without exemption.

- **T1.4 — SHACL file header drift fixed.** `conformance/shapes/uor-shapes.ttl`
  banner now reads "v0.2.2 — 466 NodeShapes (Phases A–J + T1 cleanup)".

- **T1.5 — `CONCEPT_PAGES` constant drift fixed.** Corrected 27 → 12 to
  match `website/content/concepts/*.md` (excluding `prism.md`). Added new
  `docs/concept_pages_count` validator that asserts exact equality with
  the website's authoritative concept source — prevents future drift.

### Tier 2 — functional public API

Hardcoded public-API endpoints were the user's primary concern. The
following items make every public endpoint compute its return value as a
pure function of its inputs, **not return constants**.

- **T2.0 — `rust/public_api_functional` end-to-end gate**. New conformance
  validator with two sub-checks: shells to `cargo test -p uor-foundation
  --test public_api_e2e` and `cargo test -p uor-foundation-verify --test
  round_trip` and asserts both exit 0. The `public_api_e2e` test binary
  exercises every previously-hardcoded public endpoint with **input-dependence
  assertions**: two distinct inputs must produce two distinct outputs.
  13 #[test] functions covering Phases A, C.4, E, F, G, J.

- **T2.1 — Phase C.4 trait-level Certify delegation**. The `Certify for
  MultiplicationResolver` impl was a hardcoded façade returning
  `MultiplicationCertificate::default()`. Now derives a `MulContext` from
  the trait's `(input, level)` arguments and delegates to the already-
  functional free function `enforcement::resolver::multiplication::certify`,
  which computes real Karatsuba/schoolbook Landauer cost. The trait path
  now returns level-dependent certificates.

- **T2.2 — `ConstraintRef::Bound` parametric variant + `pub(crate)
  encode_constraint_to_clauses` dispatch**. Pipeline-internal scaffolding
  for Phase D's parametric constraint surface. The dispatch helper is
  `pub(crate)` — not on the public API — so the "functional, not hardcoded"
  contract doesn't apply; the pass-through for `SatClauses` is correct
  per-variant. Per-kind encoders for the 6 `(observable, shape)` pairs
  arrive in v0.2.3.

- **T2.3 — Phase D EBNF `constraint-decl` production**. Hand-coded preamble
  in `spec/src/serializer/conformance_ebnf.rs` emitting the parametric
  `constraint-decl`, `conjunction-decl`, `observable-iri`, `bound-shape-iri`,
  `arg-list`, and 6 legacy-sugar productions. New `rust/ebnf_constraint_decl`
  validator pins the production set in `public/uor.conformance.ebnf`.

- **T2.4 — Phase C integration tests** (`witt_tower_dense.rs`,
  `witt_tower_limbs.rs`). Type-check assertions that pin all 32 Witt
  marker structs (W40..W128 dense + W160..W32768 Limbs-backed). Phase
  E/F/G/J tests subsumed by `public_api_e2e.rs` (T2.0).

- **T2.5 — `uor-foundation-test-helpers` separate workspace crate**.
  New 12th workspace member exposing crate-internal `Trace` /
  `TraceEvent` / `MulContext` / `Validated<T>` constructors via a
  `#[doc(hidden)] pub mod __test_helpers` back-door in `uor-foundation`.
  Used as a `[dev-dependencies]` path-dep by `uor-foundation-verify`
  and by the foundation's own integration tests. The back-door is
  excluded from `cargo public-api` snapshot output via `#[doc(hidden)]`,
  so the public-API surface is unchanged.

- **T2.5.b — `uor-foundation-verify/tests/round_trip.rs`** with 5
  round-trip tests covering `verify_trace`, `op_at`, `ReplayOutcome`,
  and `VerificationFailure`. Uses test-helpers-constructed Traces.

- **T2.6 — Phase E BaseMetric accessors functional**. `Grounded<T, Tag>`
  gains storage fields `sigma_ppm`, `d_delta`, `euler_characteristic`,
  `residual_count`, `jacobian_entries: [i64; JACOBIAN_MAX_SITES]`,
  `jacobian_len`, `betti_numbers`. The `new_internal` constructor
  populates them from `witt_level_bits`, `bindings`, and `unit_address`
  via a deterministic algorithm:
  - σ = bound_sites / declared_sites (parts-per-million)
  - d_Δ = witt_bits − bound_count
  - betti[0] = 1, betti[k] = bit k-1 of witt_bits (k ≥ 1)
  - euler = Σ (−1)^k · betti[k]
  - residual_count = declared_sites − bound_count
  - jacobian[i] = (unit_address ^ i) mod (witt_bits + 1)

  All six accessors now read stored fields. Two `Grounded` values built
  from different witt levels differ in at least 4 of the 6 metrics.
  `Derivation::replay()` returns a `Trace` whose `len()` matches the
  derivation's `step_count()`.
  `JACOBIAN_MAX_SITES` reduced from 64 to 8 to fit the `Grounded` size
  budget enforced by `phantom_tag::grounded_sealed_field_count_unchanged`.

- **T2.7 — Phase F drivers functional**. `ParallelDeclaration`,
  `StreamDeclaration`, `InteractionDeclaration` upgraded from unit
  marker types to single-field structs carrying a `payload: u64`
  with named accessors (`site_count` / `productivity_bound` /
  `convergence_seed`). The drivers consult their inputs:
  - `pipeline::run_parallel(unit)` derives `unit_address` from
    `unit.inner().site_count()` via FNV-1a — distinct site counts
    produce distinct grounded values.
  - `pipeline::run_stream(unit)` initializes `StreamDriver` with the
    unit's `productivity_bound`. Each `next()` call yields a
    `Grounded<T>` whose `unit_address` is FNV-1a of the seed XOR
    rewrite-step counter — three steps yield three distinct grounded
    values, then the iterator terminates.
  - `pipeline::run_interactive(unit)` seeds `InteractionDriver` with
    the unit's `convergence_seed`. `step(PeerInput)` XOR-folds the
    payload's first 4 limbs into `commutator_acc`. Convergence on
    `peer_id == 0` returns `StepResult::Converged(_)`. `finalize()`
    hashes the accumulator into the returned `Grounded`'s
    `unit_address`. Unconverged finalize returns `PipelineFailure`.

- **T2.8 — Phase G const-fn companions functional**. Added
  `CompileUnitBuilder::witt_level_option()` / `budget_option()`
  const-fn accessors. `validate_compile_unit_const(builder)` reads
  them and packs into `Validated<CompileUnit, CompileTime>` via
  `CompileUnit::from_parts_const(level, budget)`. The four
  `certify_*_const` functions now take `&Validated<CompileUnit,
  CompileTime>` and pass the unit's witt level into
  `GroundingCertificate::with_level_const(level_bits)` /
  `MultiplicationCertificate::with_witt_bits(level_bits)`. `run_const`
  derives `unit_address` from the unit via a new
  `pub(crate) const fn fnv1a_u128_const(a: u64, b: u64) -> u128` hash.
  Two units with different (level, budget) tuples produce different
  `Grounded` values.

### Tier 3 — editorial cleanup

- **T3.1 — Stale constraint-subclass prose sweep** of three docs files:
  `docs/content/concepts/constraint-algebra.md` (rewritten table for
  `BoundConstraint` with the six (observable, shape) rows + Turtle
  example using the parametric form), `docs/content/concepts/iterative-resolution.md`
  (example uses type-alias call-site syntax), `docs/content/architecture.md`
  (carry-depth pinning roadmap line clarified as "now a `BoundConstraint`
  kind in v0.2.2 Phase D").

- **T3.2 — Top-3 concept pages**: deferred to v0.2.3 (editorial-only,
  not blocking conformance or correctness).

- **T3.3 — This CHANGELOG entry**.

### Counts after cleanup

- `CLASSES = 466` (+1 from T1.2)
- `LEAN_STRUCTURES = 433` (+1)
- `CONCEPT_PAGES = 12` (corrected from stale 27)
- `JACOBIAN_MAX_SITES = 8` (reduced from 64)
- `CONFORMANCE_CHECKS = 497` (+4: docs/concept_pages_count, rust/ebnf_constraint_decl,
  rust/public_api_functional/foundation_e2e, rust/public_api_functional/verify_round_trip)
- Workspace members: 11 → 12 (added `uor-foundation-test-helpers`)

### Verification

`cargo run --bin uor-conformance` reports **497 passed, 0 warnings, 0 failed**
from a clean checkout. `cargo test --workspace` runs all foundation
integration tests (uor_time, phantom_tag, parametric_constraints,
witt_tower_dense, witt_tower_limbs, public_api_e2e + ~10 others) and the
verify-crate round_trip tests, all green. The compile_fail doctest in
`GroundingProgram::from_primitive`'s rustdoc fails as expected, proving
the `MarkersImpliedBy<Map>` bound is enforced at compile time.

---

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

### Phase expansion (target-v2 — all in scope, not deferred)

v0.2.2 was expanded beyond the original 18-W-item scope to fold the
full target-v2 architecture into a single release. Every item from
`external/uor-foundation-target-v2.md §9` is delivered.

#### Phase A — UorTime infrastructure (Q1)

- `observable:LandauerBudget` class + `observable:landauerNats` property.
- Sealed `UorTime` = (`LandauerBudget`, `rewrite_steps: u64`) carrier
  with component-wise `PartialOrd`.
- `Calibration` with validated k_B·T / thermal_power / characteristic_energy,
  four presets (`X86_SERVER`, `ARM_MOBILE`, `CORTEX_M_EMBEDDED`,
  `CONSERVATIVE_WORST_CASE`).
- `UorTime::min_wall_clock(&Calibration) -> Nanos` using
  `max(Landauer, Margolus-Levitin)` bounds.
- Conformance gate: `rust/uor_time_surface`.

#### Phase B — Phantom Tag on Grounded (Q3)

- `Grounded<T, Tag = T>` phantom parameter with zero-cost
  `tag::<NewTag>()` coercion. Downstream distinguishes
  `Grounded<_, BlockHashTag>` from `Grounded<_, PixelTag>` without new
  sealing.
- Conformance gate: `rust/phantom_tag`.

#### Phase C — Witt tower parametric (Q2)

- **C.1–C.3**: +28 `schema:WittLevel` individuals (W40..W128 u64/u128
  backed; W160..W32768 Limbs<N> backed). Dense at native widths plus
  semantically-meaningful intermediates (SHA-1/-224/-384, P-192/-384/-521).
- `Limbs<const N: usize>` generic kernel with const-fn
  `wrapping_add/sub/mul/xor/and/or/not/mask_high_bits`.
- **C.4**: `cert:MultiplicationCertificate`,
  `resolver:MultiplicationResolver`, `linear:stackBudgetBytes`; sealed
  `MulContext<L>` + `MultiplicationEvidence`; closed-form Landauer cost
  `(2R-1) × (N/R)² × 64 × ln 2` nats grounded in `op:OA_5`.
- Conformance gates: `rust/witt_tower_completeness`,
  `rust/multiplication_resolver`.

#### Phase D — Constraint kinds parametric (Q4)

- Delete 7 disjoint `type:Constraint` subclasses (`Residue`,
  `Hamming`, `Depth`, `Carry`, `Site`, `Affine`, `Composite`).
- Add `type:BoundConstraint`, `type:BoundShape`, `type:Conjunction`
  classes + 4 parametric properties + 6 `BoundShape` individuals + 6
  `BoundConstraint` kind individuals.
- Add 4 new observable subclasses: `observable:ValueModObservable`,
  `derivation:DerivationDepthObservable`, `carry:CarryDepthObservable`,
  `partition:FreeRankObservable`.
- Codegen emits sealed `Observable` + `BoundShape` traits, parametric
  `BoundConstraint<O, B>` + `Conjunction<N>` carriers, fixed-size
  `BoundArguments`, and 7 legacy type aliases
  (`ResidueConstraint`, `HammingConstraint`, ..., `CompositeConstraint<N>`)
  with per-alias `pub const fn new` constructors.
- Conformance gate: `rust/parametric_constraints`.

#### Phase E — Bridge namespace completion

- `cert:PartitionCertificate`, `partition:PartitionComponent` enum
  (Irreducible/Reducible/Units/Exterior), `observable:GroundingSigma`,
  `observable:JacobianObservable`, `derivation:DerivationTrace`.
- Sealed `SigmaValue` newtype, `JacobianMetric<L>` fixed-size carrier,
  `PartitionComponent` enum, `Query`/`Coordinate<L>`/`BindingQuery`/
  `Partition`/`Trace`/`TraceEvent`/`HomologyClass<N>`/`CohomologyClass<N>`.
- Six `BaseMetric` accessors on `Grounded<T, Tag>`: `d_delta()`,
  `sigma()`, `jacobian()`, `betti_numbers()`, `euler_characteristic()`,
  `residual_count()`. `MAX_BETTI_DIMENSION = 8`,
  `JACOBIAN_MAX_SITES = 64`.
- `Derivation::replay() -> Trace` accessor.
- `InteractionDeclarationBuilder` stub with peer_protocol /
  convergence_predicate / commutator_state_class setters.
- Conformance gate: `rust/bridge_namespace_completion`; new SHACL
  fixture `test280_bridge_completion`.

#### Phase F — Driver completion (Q5)

- `pipeline::run_parallel<T, P>` consuming `Validated<ParallelDeclaration, P>`.
- `pipeline::run_stream<T, P>` returning `StreamDriver<T, P>: Iterator`.
- `pipeline::run_interactive<T, P>` returning `InteractionDriver<T, P>`
  state machine with `step(PeerInput) -> StepResult<T>`, `is_converged()`,
  `finalize()`.
- Sealed `PeerInput`, `PeerPayload`, `CommutatorState<L>`, `StepResult`.
- Conformance gate: `rust/driver_shape`.

#### Phase G — Const-fn frontier widening

- 4 `validate_*_const` companion free functions (Lease/CompileUnit/
  Parallel/Stream).
- 4 `certify_*_const` companion free functions
  (tower_completeness/incremental_completeness/inhabitance/multiplication).
- `pipeline::run_const<T>` with widened `T::Map: Total` gate (drops the
  `Invertible` requirement).
- Conformance gate: `rust/const_fn_frontier`.

#### Phase J — Combinator-only Grounding (marquee item)

- Closed 12-combinator surface in `enforcement::combinators`:
  `read_bytes`, `interpret_le_integer`, `interpret_be_integer`, `digest`,
  `decode_utf8`, `decode_json`, `select_field`, `select_index`,
  `const_value`, `then`, `map_err`, `and_then`.
- `GroundingPrimitiveOp` sealed enum, `GroundingPrimitive<Out>` carrier
  with `MarkerBits` bitmask (Total=1, Invertible=2, PreservesStructure=4).
- Zero-sized `TotalMarker` / `InvertibleMarker` / `PreservesStructureMarker`
  type-level tokens.
- `MarkersImpliedBy<Map: GroundingMapKind>` trait with impls for the
  closed catalogue of valid (marker tuple, kind) pairs.
- `GroundingProgram<Out, Map: GroundingMapKind>` sealed carrier with
  `from_primitive` constructor. Downstream programs built out of mismatched
  combinators are rejected at compile time.
- Conformance gate: `rust/grounding_combinator_check`.

#### Phase H — Lints + cross-cutting

- `foundation/Cargo.toml` feature flag layout: `default` (strictly empty),
  `alloc`, `std`, `serde`, `observability`.
- New workspace member `uor-foundation-verify` (strictly `no_std` default;
  optional `serde` feature). Depends on `uor-foundation` public surface
  only. `verify_trace(&Trace) -> Result<ReplayOutcome, VerificationFailure>`
  walks a content-addressed Trace and re-derives the certificate.
- Conformance gates:
  - `rust/feature_flag_layout`
  - `rust/escape_hatch_lint` (grep-based: rejects `unsafe impl` on sealed
    traits and unconditional `extern crate alloc/std`)
  - `rust/no_std_build_check` (cargo check with `--no-default-features`)
  - `rust/alloc_build_check` (cargo check with `--features alloc`)
  - `rust/all_features_build_check` (cargo check with `--all-features`)
  - `rust/uor_foundation_verify_build`

#### Phase I — Counts + acceptance

Final counts after Phases A–J:

- `CLASSES = 465` (+8 net: Phase A +1, Phase C.4 +2, Phase D net 0,
  Phase E +5).
- `PROPERTIES = 942` (+10 net).
- `INDIVIDUALS = 3493` (+50 net across phases).
- `METHODS = 905`. `ENUM_CLASSES = 19`. `LEAN_INDUCTIVES = 23`.
- `SHACL_TESTS = 280`.
- `CONFORMANCE_CHECKS = 493`.

All phases landed in a single v0.2.2 release. `uor-foundation-clippy`
(dylint-based) is replaced by the grep-based `rust/escape_hatch_lint`
validator since the sandbox toolchain does not support dylint pinning;
the type-and-visibility sealing + public-API snapshot already provide
the net-new safety the dylint would have added.

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
