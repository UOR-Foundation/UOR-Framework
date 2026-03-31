//! `predicate/` namespace — Predicates and dispatch.
//!
//! The `predicate/` namespace formalizes boolean-valued functions on kernel
//! objects: resolver dispatch, cascade guard evaluation, and conditional
//! resolution paths. Every predicate is total (evaluation terminates for
//! all inputs) and pure (no side effects).
//!
//! - **Amendment 72**: 9 classes, 15 properties, 0 individuals (identities in op/)
//!
//! **Space classification:** `kernel` — immutable algebra.

use crate::model::iris::*;
use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};

/// Returns the `predicate/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "predicate",
            iri: NS_PREDICATE,
            label: "UOR Predicates and Dispatch",
            comment: "Boolean-valued functions on kernel objects. Formalizes \
                      resolver dispatch, cascade guard evaluation, and \
                      conditional resolution paths.",
            space: Space::Kernel,
            imports: &[NS_OP, NS_SCHEMA, NS_TYPE, NS_STATE, NS_EFFECT],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/predicate/Predicate",
            label: "Predicate",
            comment: "A total, pure, boolean-valued function on a kernel \
                      object. Evaluation terminates for all inputs and \
                      produces no side effects.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/predicate/TypePredicate",
            label: "TypePredicate",
            comment: "A predicate over type:TypeDefinition. Used for \
                      resolver dispatch.",
            subclass_of: &["https://uor.foundation/predicate/Predicate"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/predicate/StatePredicate",
            label: "StatePredicate",
            comment: "A predicate over state:Context or \
                      cascade:CascadeState. Used for cascade stage guards.",
            subclass_of: &["https://uor.foundation/predicate/Predicate"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/predicate/FiberPredicate",
            label: "FiberPredicate",
            comment: "A predicate over partition:FiberCoordinate. Used for \
                      fiber-level selection in geodesic resolution.",
            subclass_of: &["https://uor.foundation/predicate/Predicate"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/predicate/DispatchRule",
            label: "DispatchRule",
            comment: "A pair (Predicate, Target) where Target is a \
                      resolver:Resolver class. The kernel evaluates the \
                      predicate; if true, the target resolver is selected.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/predicate/DispatchTable",
            label: "DispatchTable",
            comment: "An ordered set of DispatchRules for a single dispatch \
                      point. Must satisfy exhaustiveness and mutual exclusion.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/predicate/GuardedTransition",
            label: "GuardedTransition",
            comment: "A triple (StatePredicate, effect:Effect, \
                      cascade:CascadeStage). The guard is a StatePredicate; \
                      if true, the effect is applied and the cascade advances \
                      to the target stage.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/predicate/MatchArm",
            label: "MatchArm",
            comment: "A single case in a pattern match: a Predicate and a \
                      result Term. The match evaluates predicates in order \
                      and returns the result of the first matching arm.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/predicate/MatchExpression",
            label: "MatchExpression",
            comment: "A term formed by evaluating a sequence of MatchArms. \
                      Extends the term language with deterministic conditional \
                      evaluation.",
            subclass_of: &["https://uor.foundation/schema/Term"],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        // Object properties
        Property {
            id: "https://uor.foundation/predicate/evaluatesOver",
            label: "evaluatesOver",
            comment: "The OWL class of objects this predicate accepts as input.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/predicate/Predicate"),
            range: OWL_CLASS,
        },
        Property {
            id: "https://uor.foundation/predicate/dispatchPredicate",
            label: "dispatchPredicate",
            comment: "The predicate that triggers this dispatch rule.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/predicate/DispatchRule"),
            range: "https://uor.foundation/predicate/Predicate",
        },
        Property {
            id: "https://uor.foundation/predicate/dispatchTarget",
            label: "dispatchTarget",
            comment: "The resolver selected when the predicate is satisfied.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/predicate/DispatchRule"),
            range: "https://uor.foundation/resolver/Resolver",
        },
        Property {
            id: "https://uor.foundation/predicate/dispatchRules",
            label: "dispatchRules",
            comment: "The ordered set of rules in this table.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/predicate/DispatchTable"),
            range: "https://uor.foundation/predicate/DispatchRule",
        },
        Property {
            id: "https://uor.foundation/predicate/guardPredicate",
            label: "guardPredicate",
            comment: "The guard predicate for this transition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/predicate/GuardedTransition"),
            range: "https://uor.foundation/predicate/StatePredicate",
        },
        Property {
            id: "https://uor.foundation/predicate/guardEffect",
            label: "guardEffect",
            comment: "The effect applied when the guard is satisfied.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/predicate/GuardedTransition"),
            range: "https://uor.foundation/effect/Effect",
        },
        Property {
            id: "https://uor.foundation/predicate/guardTarget",
            label: "guardTarget",
            comment: "The cascade stage to advance to.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/predicate/GuardedTransition"),
            // Full IRI string: predicate/ cannot import cascade/
            // because cascade/ will import predicate/ in Phase 3
            range: "https://uor.foundation/cascade/CascadeStage",
        },
        Property {
            id: "https://uor.foundation/predicate/matchArms",
            label: "matchArms",
            comment: "The ordered arms of this match expression.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/predicate/MatchExpression"),
            range: "https://uor.foundation/predicate/MatchArm",
        },
        Property {
            id: "https://uor.foundation/predicate/armPredicate",
            label: "armPredicate",
            comment: "The predicate guarding this arm.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/predicate/MatchArm"),
            range: "https://uor.foundation/predicate/Predicate",
        },
        Property {
            id: "https://uor.foundation/predicate/armResult",
            label: "armResult",
            comment: "The result term if this arm matches.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/predicate/MatchArm"),
            range: "https://uor.foundation/schema/Term",
        },
        Property {
            id: "https://uor.foundation/predicate/boundedEvaluator",
            label: "boundedEvaluator",
            comment: "A termination witness for user-declared predicates. \
                      Kernel predicates are total by construction; \
                      user-declared predicates must carry a descent measure \
                      certifying termination.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/predicate/Predicate"),
            range: "https://uor.foundation/recursion/DescentMeasure",
        },
        // Datatype properties
        Property {
            id: "https://uor.foundation/predicate/dispatchIndex",
            label: "dispatchIndex",
            comment: "Position in the dispatch table (evaluation order).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/predicate/DispatchRule"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/predicate/isExhaustive",
            label: "isExhaustive",
            comment: "True iff the disjunction of all dispatch predicates is \
                      a tautology over the input class.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/predicate/DispatchTable"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/predicate/isMutuallyExclusive",
            label: "isMutuallyExclusive",
            comment: "True iff no two dispatch predicates can be \
                      simultaneously true for any input.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/predicate/DispatchTable"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/predicate/armIndex",
            label: "armIndex",
            comment: "Position in the match expression (evaluation order).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/predicate/MatchArm"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
    ]
}
