//! `derivation/` namespace — Computation witnesses via term rewriting.
//!
//! Derivations record the step-by-step rewriting of terms to their canonical
//! forms. They serve as verifiable computation witnesses.
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
            imports: &[NS_SCHEMA, NS_OP],
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
        Class {
            id: "https://uor.foundation/derivation/RewriteStep",
            label: "RewriteStep",
            comment: "A single rewrite step in a derivation: the application of \
                      one rewrite rule to transform a term.",
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
    ]
}
