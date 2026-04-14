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
pub const CLASSES: usize = 457;

/// Total properties including the global `uor:space` annotation.
///
/// v0.2.1 Phase 1: +31.
/// v0.2.1 Phase 7a: +7 (3 SatBound: maxVarCount/maxClauseCount/maxLiteralsPerClause;
/// 2 TimingBound: preflightBudgetNs/runtimeBudgetNs; 1 type:defaultValue;
/// 1 op:isRingOp — op:arity already existed as xsd:nonNegativeInteger).
/// v0.2.2 W8: +4 (schema:triadStratum, schema:triadSpectrum, schema:triadAddress,
/// state:groundedTriad — Triad bundling).
pub const PROPERTIES: usize = 932;

/// Namespace-level properties only (excludes global annotation).
pub const NAMESPACE_PROPERTIES: usize = 931;

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
pub const INDIVIDUALS: usize = 3448;

/// Number of SHACL test instance graphs.
///
/// v0.2.1 Phase 7a: +1 (test278 SatBound/TimingBound/ConstraintDefaults fixture).
pub const SHACL_TESTS: usize = 278;

/// Total conformance checks in the full suite.
///
/// v0.2.1 Phase 1: +1 from the test277 SHACL fixture.
/// v0.2.1 Phase 7a: +1 from test278 SatBound/TimingBound/ConstraintDefaults
/// fixture. v0.2.1 Phase 7g: +1 from the `lean4/rigor` banned-primitives
/// enforcement check.
/// v0.2.2 W5: +1 from the `docs/psi_leakage` validator.
/// v0.2.2 W6: +1 from the `rust/public_api_snapshot` validator.
pub const CONFORMANCE_CHECKS: usize = 476;

/// Number of amendments applied to the base ontology.
pub const AMENDMENTS: usize = 95;

/// Number of classes that become Rust enums/structs (not traits).
pub const ENUM_CLASSES: usize = 18;

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
pub const METHODS: usize = 895;

/// Number of individual constant modules generated.
pub const CONSTANT_MODULES: usize = 1501;

/// Number of Lean 4 structures generated (classes minus enum classes).
///
/// v0.2.1 Phase 1: +13. Phase 7a: +3 (SatBound, TimingBound, ConstraintDefaults).
pub const LEAN_STRUCTURES: usize = 426;

/// Number of Lean 4 inductive + structure types generated for the enum layer.
///
/// Composition: 18 ontology enum classes (see `Ontology::enum_class_names()`),
/// plus 3 hardcoded types not in the ontology's class list (`Space`,
/// `SiteState`, `PrimitiveOp`), plus 1 `structure` for `WittLevel` (open-world,
/// not an `inductive`). Total: 22.
pub const LEAN_INDUCTIVES: usize = 22;

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
pub const LEAN_CONSTANT_NAMESPACES: usize = 3348;

/// Number of concept pages on the website (one per content/concepts/*.md file).
pub const CONCEPT_PAGES: usize = 27;

/// Number of PRISM pipeline stages (Define / Resolve / Certify).
pub const PIPELINE_STAGES: usize = 3;

/// Minimum number of classes in a namespace to generate a class hierarchy SVG.
pub const MIN_HIERARCHY_CLASSES: usize = 3;
