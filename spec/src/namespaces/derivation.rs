//! `derivation/` namespace — Computation witnesses via term rewriting.
//!
//! Derivations record the step-by-step rewriting of terms to their canonical
//! forms. They serve as verifiable computation witnesses.
//!
//! Amendment 11 adds `DerivationStep` as an abstract parent for `RewriteStep`
//! (term-level) and `RefinementStep` (type-level), plus properties for tracking
//! type refinement through the iterative resolution loop.
//!
//! **Space classification:** `bridge` — kernel-produced, user-consumed.

use crate::model::iris::*;
use crate::model::{
    Class, Individual, IndividualValue, Namespace, NamespaceModule, Property, PropertyKind, Space,
};

/// Returns the `derivation/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "derivation",
            iri: NS_DERIVATION,
            label: "UOR Derivations",
            comment: "Computation witnesses recording term rewriting sequences from \
                      original terms to their canonical forms.",
            space: Space::Bridge,
            imports: &[NS_OBSERVABLE, NS_OP, NS_RESOLVER, NS_SCHEMA, NS_TYPE],
        },
        classes: classes(),
        properties: properties(),
        individuals: individuals(),
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/derivation/Derivation",
            label: "Derivation",
            comment: "A complete term rewriting witness: the full sequence of \
                      rewrite steps transforming an original term into its canonical \
                      form.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 11: DerivationStep abstract parent
        Class {
            id: "https://uor.foundation/derivation/DerivationStep",
            label: "DerivationStep",
            comment: "An abstract step in a derivation. Concrete subclasses are \
                      RewriteStep (term-level rewriting) and RefinementStep \
                      (type-level refinement).",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/derivation/RewriteStep",
            label: "RewriteStep",
            comment: "A single rewrite step in a derivation: the application of \
                      one rewrite rule to transform a term.",
            subclass_of: &["https://uor.foundation/derivation/DerivationStep"],
            disjoint_with: &[],
        },
        // Amendment 11: RefinementStep
        Class {
            id: "https://uor.foundation/derivation/RefinementStep",
            label: "RefinementStep",
            comment: "A type-level refinement step: the application of a constraint \
                      to narrow a type, pinning additional site coordinates. \
                      Complements RewriteStep (term-level) in the derivation \
                      hierarchy.",
            subclass_of: &["https://uor.foundation/derivation/DerivationStep"],
            disjoint_with: &[],
        },
        // Amendment 23: Typed controlled vocabulary class
        Class {
            id: "https://uor.foundation/derivation/RewriteRule",
            label: "RewriteRule",
            comment: "A named rewrite rule that can be applied in a derivation step. \
                      Each RewriteRule individual represents a specific algebraic law \
                      or normalization strategy used during term rewriting.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/derivation/TermMetrics",
            label: "TermMetrics",
            comment: "Metrics describing the size and complexity of a term.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 28: Type synthesis step
        Class {
            id: "https://uor.foundation/derivation/SynthesisStep",
            label: "SynthesisStep",
            comment: "A single step in the construction of a SynthesizedType: one constraint \
                      added to the synthesis candidate and the resulting change in the constraint \
                      nerve's topological signature. Ordered by derivation:stepIndex. Analogous \
                      to derivation:RewriteStep in the forward pipeline.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 38: Synthesis checkpoint for resumable Q1+ synthesis
        Class {
            id: "https://uor.foundation/derivation/SynthesisCheckpoint",
            label: "SynthesisCheckpoint",
            comment: "A persistent snapshot of a ConstraintSearchState at a \
                      specific SynthesisStep, allowing a TypeSynthesisResolver \
                      to resume exploration after interruption. Essential at \
                      Q1+ scale where exhaustive synthesis is computationally \
                      significant.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/derivation/originalTerm",
            label: "originalTerm",
            comment: "The term at the start of the derivation, before any rewriting.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/Derivation"),
            range: "https://uor.foundation/schema/Term",
        },
        Property {
            id: "https://uor.foundation/derivation/canonicalTerm",
            label: "canonicalTerm",
            comment: "The canonical form produced at the end of the derivation.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/Derivation"),
            range: "https://uor.foundation/schema/Term",
        },
        Property {
            id: "https://uor.foundation/derivation/result",
            label: "result",
            comment: "The datum value obtained by evaluating the canonical term.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/Derivation"),
            range: "https://uor.foundation/schema/Datum",
        },
        Property {
            id: "https://uor.foundation/derivation/step",
            label: "step",
            comment: "A rewrite step in this derivation.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/derivation/Derivation"),
            range: "https://uor.foundation/derivation/RewriteStep",
        },
        Property {
            id: "https://uor.foundation/derivation/termMetrics",
            label: "termMetrics",
            comment: "Metrics for the canonical term produced by this derivation.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/Derivation"),
            range: "https://uor.foundation/derivation/TermMetrics",
        },
        Property {
            id: "https://uor.foundation/derivation/from",
            label: "from",
            comment: "The term before this rewrite step.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/RewriteStep"),
            range: "https://uor.foundation/schema/Term",
        },
        Property {
            id: "https://uor.foundation/derivation/to",
            label: "to",
            comment: "The term after this rewrite step.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/RewriteStep"),
            range: "https://uor.foundation/schema/Term",
        },
        // derivation:rule property removed by Amendment 23 (replaced by hasRewriteRule)
        // Amendment 23: Typed controlled vocabulary properties
        Property {
            id: "https://uor.foundation/derivation/hasRewriteRule",
            label: "hasRewriteRule",
            comment: "The typed rewrite rule applied in this step. Provides a \
                      structured reference to a named RewriteRule individual, \
                      complementing the string-valued derivation:rule property.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/RewriteStep"),
            range: "https://uor.foundation/derivation/RewriteRule",
        },
        Property {
            id: "https://uor.foundation/derivation/groundedIn",
            label: "groundedIn",
            comment: "The algebraic identity that grounds this rewrite rule. \
                      Links a RewriteRule to the op:Identity individual that \
                      justifies its application.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/RewriteRule"),
            range: "https://uor.foundation/op/Identity",
        },
        Property {
            id: "https://uor.foundation/derivation/stepCount",
            label: "stepCount",
            comment: "The total number of rewrite steps in this derivation.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/derivation/TermMetrics"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/derivation/termSize",
            label: "termSize",
            comment: "The number of nodes in the canonical term's syntax tree.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/derivation/TermMetrics"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // Amendment 11: RefinementStep properties
        Property {
            id: "https://uor.foundation/derivation/previousType",
            label: "previousType",
            comment: "The type before this refinement step was applied.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/RefinementStep"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/derivation/appliedConstraint",
            label: "appliedConstraint",
            comment: "The constraint that was applied in this refinement step.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/RefinementStep"),
            range: "https://uor.foundation/type/Constraint",
        },
        Property {
            id: "https://uor.foundation/derivation/refinedType",
            label: "refinedType",
            comment: "The type after this refinement step was applied.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/RefinementStep"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/derivation/sitesClosed",
            label: "sitesClosed",
            comment: "The number of site coordinates pinned by this refinement step.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/derivation/RefinementStep"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // Amendment 28: SynthesisStep properties
        Property {
            id: "https://uor.foundation/derivation/stepIndex",
            label: "stepIndex",
            comment: "Zero-based sequential index of this step within the synthesis derivation.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/derivation/SynthesisStep"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/derivation/addedConstraint",
            label: "addedConstraint",
            comment: "The constraint added in this synthesis step.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/SynthesisStep"),
            range: "https://uor.foundation/type/Constraint",
        },
        Property {
            id: "https://uor.foundation/derivation/signatureBefore",
            label: "signatureBefore",
            comment: "The constraint nerve signature before this synthesis step.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/SynthesisStep"),
            range: "https://uor.foundation/observable/SynthesisSignature",
        },
        Property {
            id: "https://uor.foundation/derivation/signatureAfter",
            label: "signatureAfter",
            comment: "The constraint nerve signature after this synthesis step.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/SynthesisStep"),
            range: "https://uor.foundation/observable/SynthesisSignature",
        },
        // Amendment 38: SynthesisCheckpoint properties
        Property {
            id: "https://uor.foundation/derivation/checkpointStep",
            label: "checkpointStep",
            comment: "The SynthesisStep at which this checkpoint was taken.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/SynthesisCheckpoint"),
            range: "https://uor.foundation/derivation/SynthesisStep",
        },
        Property {
            id: "https://uor.foundation/derivation/checkpointState",
            label: "checkpointState",
            comment: "The ConstraintSearchState snapshot captured by this \
                      checkpoint.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/derivation/SynthesisCheckpoint"),
            range: "https://uor.foundation/resolver/ConstraintSearchState",
        },
        // Amendment 41: Bridge property — domain is resolver:TowerCompletenessResolver
        Property {
            id: "https://uor.foundation/derivation/towerCheckpoint",
            label: "towerCheckpoint",
            comment: "Links a TowerCompletenessResolver to a SynthesisCheckpoint \
                      issued at each completed step. Cross-namespace bridge \
                      property: domain is resolver:TowerCompletenessResolver.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/resolver/TowerCompletenessResolver"),
            range: "https://uor.foundation/derivation/SynthesisCheckpoint",
        },
    ]
}

// Amendment 23: Typed controlled vocabulary individuals
fn individuals() -> Vec<Individual> {
    vec![
        Individual {
            id: "https://uor.foundation/derivation/CriticalIdentityRule",
            type_: "https://uor.foundation/derivation/RewriteRule",
            label: "CriticalIdentityRule",
            comment: "The rewrite rule applying the critical identity: \
                      neg(bnot(x)) → succ(x). Grounded in op:criticalIdentity.",
            properties: &[(
                "https://uor.foundation/derivation/groundedIn",
                IndividualValue::IriRef("https://uor.foundation/op/criticalIdentity"),
            )],
        },
        Individual {
            id: "https://uor.foundation/derivation/InvolutionRule",
            type_: "https://uor.foundation/derivation/RewriteRule",
            label: "InvolutionRule",
            comment: "The rewrite rule applying involution cancellation: \
                      f(f(x)) → x for any involution f.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/derivation/AssociativityRule",
            type_: "https://uor.foundation/derivation/RewriteRule",
            label: "AssociativityRule",
            comment: "The rewrite rule applying associativity to re-bracket \
                      nested binary operations.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/derivation/CommutativityRule",
            type_: "https://uor.foundation/derivation/RewriteRule",
            label: "CommutativityRule",
            comment: "The rewrite rule applying commutativity to reorder operands \
                      of commutative operations.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/derivation/IdentityElementRule",
            type_: "https://uor.foundation/derivation/RewriteRule",
            label: "IdentityElementRule",
            comment: "The rewrite rule eliminating identity elements: \
                      add(x, 0) → x, mul(x, 1) → x, xor(x, 0) → x.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/derivation/NormalizationRule",
            type_: "https://uor.foundation/derivation/RewriteRule",
            label: "NormalizationRule",
            comment: "The rewrite rule normalizing compound expressions to \
                      canonical ordering (e.g., sorting operands by address).",
            properties: &[],
        },
    ]
}
