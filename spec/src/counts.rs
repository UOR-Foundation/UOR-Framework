//! Authoritative ontology inventory counts.
//!
//! **This is the single file to update when ontology terms change.**
//! All crates import from here. The spec crate's tests verify that
//! [`Ontology::full()`](crate::Ontology::full) produces exactly these counts.

/// Number of namespace modules.
pub const NAMESPACES: usize = 33;

/// Total OWL classes across all namespaces.
///
/// v0.2.1 Phase 1: +13 (5 verdict classes for Inhabitance — InhabitanceCertificate,
/// InhabitanceImpossibilityWitness, InhabitanceSearchTrace, InhabitanceStep,
/// InhabitanceCheckpoint; 4 resolver subclasses — InhabitanceResolver,
/// TwoSatDecider, HornSatDecider, ResidualVerdictResolver; 1 schema:ValueTuple;
/// 1 reduction:FailureField; 1 resolver:CertifyMapping; 1 conformance:PreludeExport).
/// v0.2.1 Phase 7a: +3 (reduction:SatBound, reduction:TimingBound,
/// type:ConstraintDefaults — parametric metadata for codegen).
/// v0.2.2 Phase A: +1 (observable:LandauerBudget — sealed carrier for accumulated
/// Landauer cost; backs the Rust enforcement::LandauerBudget newtype that holds
/// one of the two clocks of UorTime).
/// v0.2.2 Phase C.4: +2 (cert:MultiplicationCertificate, resolver:MultiplicationResolver).
/// v0.2.2 Phase D (Q4): net 0 (-7 enumerated Constraint subclasses deleted:
/// ResidueConstraint, CarryConstraint, DepthConstraint, CompositeConstraint,
/// HammingConstraint, SiteConstraint, AffineConstraint; +3 parametric
/// classes: BoundConstraint, BoundShape, Conjunction; +4 observable
/// subclasses: observable:ValueModObservable, derivation:DerivationDepthObservable,
/// carry:CarryDepthObservable, partition:FreeRankObservable).
/// v0.2.2 Phase E: +5 (cert:PartitionCertificate, partition:PartitionComponent,
/// observable:GroundingSigma, observable:JacobianObservable,
/// derivation:DerivationTrace).
/// v0.2.2 T1.2 (cleanup): +1 (conformance:InteractionShape — backing class
/// for InteractionDeclarationBuilder, originally planned in Phase E).
pub const CLASSES: usize = 466;

/// Total properties including the global `uor:space` annotation.
///
/// v0.2.1 Phase 1: +31.
/// v0.2.1 Phase 7a: +7 (3 SatBound: maxVarCount/maxClauseCount/maxLiteralsPerClause;
/// 2 TimingBound: preflightBudgetNs/runtimeBudgetNs; 1 type:defaultValue;
/// 1 op:isRingOp — op:arity already existed as xsd:nonNegativeInteger).
/// v0.2.2 W8: +4 (schema:triadStratum, schema:triadSpectrum, schema:triadAddress,
/// state:groundedTriad — Triad bundling).
/// v0.2.2 Phase A: +1 (observable:landauerNats — accumulated Landauer cost on
/// LandauerBudget, unit observable:Nats).
/// v0.2.2 Phase C.4: +4 (cert:splittingFactor, cert:subMultiplicationCount,
/// cert:landauerCostNats, linear:stackBudgetBytes).
/// v0.2.2 Phase D (Q4): +4 (type:boundObservable, type:boundShape,
/// type:boundArguments, type:conjuncts).
/// v0.2.2 Phase E: +1 (derivation:traceEventCount).
pub const PROPERTIES: usize = 942;

/// Namespace-level properties only (excludes global annotation).
pub const NAMESPACE_PROPERTIES: usize = 941;

/// Total named individuals across all namespaces.
/// Includes 1870 AST term individuals (LiteralExpression / ForAllDeclaration)
/// generated from identity lhs/rhs/forAll string values.
///
/// v0.2.1 Phase 1: +76.
/// v0.2.1 Phase 7a: +5 (TwoSatBound, HornSatBound, PreflightTimingBound,
/// RuntimeTimingBound, ResidueDefaultModulus).
/// v0.2.2 W4+W14: +5 (morphism:DigestGroundingMap, morphism:BinaryGroundingMap,
/// reduction:ShapeMismatch, two reduction:FailureField individuals for
/// ShapeMismatch's `expected` and `got` fields).
/// v0.2.2 Phase C.1: +4 (schema:W40, schema:W48, schema:W56, schema:W64 —
/// dense u64-backed Witt levels).
/// v0.2.2 Phase C.2: +8 (schema:W72, W80, W88, W96, W104, W112, W120, W128 —
/// dense u128-backed Witt levels).
/// v0.2.2 Phase C.3: +16 (schema:W160, W192, W224, W256, W384, W448, W512,
/// W520, W528, W1024, W2048, W4096, W8192, W12288, W16384, W32768 —
/// Limbs<N>-backed Witt levels covering semantically-meaningful intermediates
/// and powers-of-two above native).
/// v0.2.2 Phase C.4: +1 (resolver:multiplicationCertifyMapping).
/// v0.2.2 Phase D (Q4): +12 (6 BoundShape individuals: EqualBound, LessEqBound,
/// GreaterEqBound, RangeContainBound, ResidueClassBound, AffineEqualBound;
/// 6 BoundConstraint kind individuals: residue/hamming/depth/carry/site/affine
/// ConstraintKind).
/// v0.2.2 Phase E: +4 (partition:PartitionComponent individuals:
/// Irreducible, Reducible, Units, Exterior).
pub const INDIVIDUALS: usize = 3493;

/// Number of SHACL test instance graphs.
///
/// v0.2.1 Phase 7a: +1 (test278 SatBound/TimingBound/ConstraintDefaults fixture).
/// v0.2.2 Phase C.4: +1 (test279 MultiplicationCertificate +
/// MultiplicationResolver + linear:stackBudgetBytes fixture).
/// v0.2.2 Phase E: +1 (test280 Phase E bridge namespace completion fixture).
pub const SHACL_TESTS: usize = 280;

/// Total conformance checks in the full suite.
///
/// v0.2.1 Phase 1: +1 from the test277 SHACL fixture.
/// v0.2.1 Phase 7a: +1 from test278 SatBound/TimingBound/ConstraintDefaults
/// fixture. v0.2.1 Phase 7g: +1 from the `lean4/rigor` banned-primitives
/// enforcement check.
/// v0.2.2 W5: +1 from the `docs/psi_leakage` validator.
/// v0.2.2 W6: +1 from the `rust/public_api_snapshot` validator.
/// v0.2.2 Phase A: +1 from the `rust/uor_time_surface` validator.
/// v0.2.2 Phase B: +1 from the `rust/phantom_tag` validator.
/// v0.2.2 Phase C.4: +1 from the `test279` MultiplicationCertificate fixture.
/// v0.2.2 Phase C verifiers: +1 from `rust/witt_tower_completeness`, +1 from
/// `rust/multiplication_resolver`.
/// v0.2.2 Phase D verifier: +1 from `rust/parametric_constraints`.
/// v0.2.2 Phase E: +1 from `rust/bridge_namespace_completion`, +1 from
/// `test280_bridge_completion` SHACL fixture.
/// v0.2.2 Phase F: +1 from `rust/driver_shape`.
/// v0.2.2 Phase G: +1 from `rust/const_fn_frontier`.
/// v0.2.2 Phase J: +1 from `rust/grounding_combinator_check`.
/// v0.2.2 Phase H: +6 from `rust/feature_flag_layout`,
/// `rust/escape_hatch_lint`, `rust/no_std_build_check`,
/// `rust/alloc_build_check`, `rust/all_features_build_check`,
/// `rust/uor_foundation_verify_build`.
/// v0.2.2 T1.5 (cleanup): +1 from `docs/concept_pages_count`.
/// v0.2.2 T2.0 (cleanup): +2 from `rust/public_api_functional`
/// (foundation_e2e + verify_round_trip).
/// v0.2.2 T2.3 (cleanup): +1 from `rust/ebnf_constraint_decl`.
pub const CONFORMANCE_CHECKS: usize = 497;

/// Number of amendments applied to the base ontology.
pub const AMENDMENTS: usize = 95;

/// Number of classes that become Rust enums/structs (not traits).
pub const ENUM_CLASSES: usize = 19;

/// Number of `op:Identity` individuals (and corresponding proofs).
pub const IDENTITY_COUNT: usize = 624;

/// Kernel-space namespace count.
pub const KERNEL_NAMESPACES: usize = 17;

/// Bridge-space namespace count.
pub const BRIDGE_NAMESPACES: usize = 13;

/// User-space namespace count.
pub const USER_NAMESPACES: usize = 3;

/// Number of trait methods generated (properties with domains,
/// excluding enum-class-domain and cross-namespace-domain properties).
///
/// v0.2.1 Phase 1: +31. Phase 7a: +7 from new parametric metadata properties.
/// v0.2.2 W8: +4 (triadStratum, triadSpectrum, triadAddress on schema:Triad;
/// groundedTriad on state:GroundedContext).
/// v0.2.2 Phase A: +1 (landauerNats on observable:LandauerBudget).
/// v0.2.2 Phase C.4: +4 (splittingFactor, subMultiplicationCount,
/// landauerCostNats on MultiplicationCertificate; stackBudgetBytes on LinearBudget).
/// v0.2.2 Phase D (Q4): +4 (boundObservable, boundShape, boundArguments on
/// BoundConstraint; conjuncts on Conjunction). The 11 properties previously
/// on the 7 deleted constraint subclasses are retained under new domains
/// (BoundConstraint or Conjunction), so no net loss.
/// v0.2.2 Phase E: +1 (derivation:traceEventCount on DerivationTrace).
pub const METHODS: usize = 905;

/// Number of individual constant modules generated.
pub const CONSTANT_MODULES: usize = 1501;

/// Number of Lean 4 structures generated (classes minus enum classes).
///
/// v0.2.1 Phase 1: +13. Phase 7a: +3 (SatBound, TimingBound, ConstraintDefaults).
/// v0.2.2 Phase C.4: +2 (MultiplicationCertificate, MultiplicationResolver).
/// v0.2.2 Phase E: +4 (PartitionCertificate, GroundingSigma, JacobianObservable,
/// DerivationTrace; PartitionComponent is an enum class, not a structure).
/// v0.2.2 T1.2 (cleanup): +1 (InteractionShape — regular structure).
pub const LEAN_STRUCTURES: usize = 433;

/// Number of Lean 4 inductive + structure types generated for the enum layer.
///
/// Composition: 18 ontology enum classes (see `Ontology::enum_class_names()`),
/// plus 3 hardcoded types not in the ontology's class list (`Space`,
/// `SiteState`, `PrimitiveOp`), plus 1 `structure` for `WittLevel` (open-world,
/// not an `inductive`). Total: 22.
pub const LEAN_INDUCTIVES: usize = 23;

/// Number of Lean 4 individual constant namespaces generated.
///
/// One `namespace <name> ... end <name>` block is emitted per non-enum
/// named individual in the ontology. This is distinct from
/// `CONSTANT_MODULES`, which counts the per-namespace-module constant
/// files produced by the Rust codegen — those are container modules,
/// not per-individual namespace blocks.
///
/// v0.2.2 W4+W14: +5 (DigestGroundingMap, BinaryGroundingMap, ShapeMismatch,
/// shapeMismatch_expected_field, shapeMismatch_got_field).
/// v0.2.2 Phase C.4: +1 (multiplicationCertifyMapping — a resolver individual,
/// not a WittLevel, so it gets a namespace block like the other CertifyMappings).
///
/// **Note**: WittLevel individuals (Phase C.1+) are NOT counted here. WittLevel
/// is in `enum_class_names()` and its individuals are emitted as `def Wn` in
/// `lean4/UOR/Enums.lean`, not as `namespace ... end` blocks. They contribute
/// to the WittLevel def list (visible in `Enums.lean`) but not to the
/// per-individual constant namespace count.
pub const LEAN_CONSTANT_NAMESPACES: usize = 3361;

/// Number of concept pages on the website (one per content/concepts/*.md file).
/// Number of concept pages on the website (one per `website/content/concepts/*.md`,
/// excluding `prism.md` which is merged into the pipeline page).
///
/// v0.2.2 T1.5 (cleanup): corrected 27 → 12. The previous value (27) did not
/// match either `website/content/concepts/` (12 files after excluding
/// `prism.md`) or `docs/content/concepts/` (33 files). The discrepancy slipped
/// through because no validator enforced the constant. The new
/// `docs/concept_pages_count` validator walks `website/content/concepts/`
/// (the authoritative site source) and asserts the count matches this
/// constant.
pub const CONCEPT_PAGES: usize = 12;

/// Number of PRISM pipeline stages (Define / Resolve / Certify).
pub const PIPELINE_STAGES: usize = 3;

/// Minimum number of classes in a namespace to generate a class hierarchy SVG.
pub const MIN_HIERARCHY_CLASSES: usize = 3;
