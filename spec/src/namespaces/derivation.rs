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
use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};

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
            imports: &[NS_SCHEMA, NS_OP, NS_TYPE],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
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
                      to narrow a type, pinning additional fiber coordinates. \
                      Complements RewriteStep (term-level) in the derivation \
                      hierarchy.",
            subclass_of: &["https://uor.foundation/derivation/DerivationStep"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/derivation/TermMetrics",
            label: "TermMetrics",
            comment: "Metrics describing the size and complexity of a term.",
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
        Property {
            id: "https://uor.foundation/derivation/rule",
            label: "rule",
            comment: "The rewrite rule applied in this step (e.g., 'critical_identity', \
                      'involution', 'associativity').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/derivation/RewriteStep"),
            range: XSD_STRING,
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
            id: "https://uor.foundation/derivation/fibersClosed",
            label: "fibersClosed",
            comment: "The number of fiber coordinates pinned by this refinement step.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/derivation/RefinementStep"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
    ]
}
