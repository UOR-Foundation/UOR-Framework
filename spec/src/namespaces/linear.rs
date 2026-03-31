//! `linear/` namespace — Linear resources.
//!
//! The `linear/` namespace formalizes linear discipline on fiber consumption.
//! Each fiber in a complete resolution path is targeted by exactly one effect.
//!
//! - **Amendment 77**: 6 classes, 8 properties, 0 individuals (identities in op/)
//!
//! **Space classification:** `kernel` — immutable algebra.

use crate::model::iris::*;
use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};

/// Returns the `linear/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "linear",
            iri: NS_LINEAR,
            label: "UOR Linear Resources",
            comment: "Linear discipline on fiber consumption. Formalizes \
                      that each fiber in a complete resolution path is \
                      targeted by exactly one effect.",
            space: Space::Kernel,
            imports: &[NS_OP, NS_EFFECT, NS_PARTITION, NS_TYPE, NS_STATE, NS_TRACE],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/linear/LinearFiber",
            label: "LinearFiber",
            comment: "A fiber coordinate annotated with a linearity \
                      constraint: must be pinned exactly once in any \
                      complete resolution path.",
            subclass_of: &["https://uor.foundation/partition/FiberCoordinate"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/linear/LinearEffect",
            label: "LinearEffect",
            comment: "A PinningEffect that consumes its target LinearFiber. \
                      After application, the fiber is no longer available \
                      for pinning by any subsequent effect.",
            subclass_of: &["https://uor.foundation/effect/PinningEffect"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/linear/LinearTrace",
            label: "LinearTrace",
            comment: "A computation trace where every fiber in the budget is \
                      targeted by exactly one LinearEffect.",
            subclass_of: &["https://uor.foundation/trace/ComputationTrace"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/linear/LinearBudget",
            label: "LinearBudget",
            comment: "The multiset of LinearFibers available at a given \
                      point in resolution. Starts as the full fiber budget; \
                      each LinearEffect removes exactly one element.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/linear/LeaseAllocation",
            label: "LeaseAllocation",
            comment: "A binding between a state:ContextLease and a subset \
                      of LinearFibers. Formalizes what resources a lease \
                      claims.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/linear/AffineFiber",
            label: "AffineFiber",
            comment: "A fiber that may be pinned at most once (but need not \
                      be pinned). Relaxation of LinearFiber for incomplete \
                      resolution paths.",
            subclass_of: &["https://uor.foundation/partition/FiberCoordinate"],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        // Object properties
        Property {
            id: "https://uor.foundation/linear/linearTarget",
            label: "linearTarget",
            comment: "The single fiber consumed by this effect.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/linear/LinearEffect"),
            range: "https://uor.foundation/linear/LinearFiber",
        },
        Property {
            id: "https://uor.foundation/linear/budgetFibers",
            label: "budgetFibers",
            comment: "The fibers remaining in the budget.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/linear/LinearBudget"),
            range: "https://uor.foundation/linear/LinearFiber",
        },
        Property {
            id: "https://uor.foundation/linear/budgetContext",
            label: "budgetContext",
            comment: "The context associated with this budget state.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/linear/LinearBudget"),
            range: "https://uor.foundation/state/Context",
        },
        Property {
            id: "https://uor.foundation/linear/leaseTarget",
            label: "leaseTarget",
            comment: "The fibers claimed by this lease.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/linear/LeaseAllocation"),
            range: "https://uor.foundation/linear/LinearFiber",
        },
        Property {
            id: "https://uor.foundation/linear/leaseSource",
            label: "leaseSource",
            comment: "The ContextLease individual that owns this allocation.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/linear/LeaseAllocation"),
            range: "https://uor.foundation/state/ContextLease",
        },
        Property {
            id: "https://uor.foundation/linear/leaseAllocation",
            label: "leaseAllocation",
            comment: "Links a state:ContextLease to its LinearFiber \
                      allocation.",
            kind: PropertyKind::Object,
            functional: true,
            // Cross-namespace domain: state:ContextLease
            // This property will NOT generate a trait method (cross-NS domain)
            domain: Some("https://uor.foundation/state/ContextLease"),
            range: "https://uor.foundation/linear/LeaseAllocation",
        },
        // Datatype properties
        Property {
            id: "https://uor.foundation/linear/remainingCount",
            label: "remainingCount",
            comment: "Number of unconsumed fibers. Equals freeCount on the \
                      associated context.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/linear/LinearBudget"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/linear/leaseCardinality",
            label: "leaseCardinality",
            comment: "Number of fibers claimed by this lease.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/linear/LeaseAllocation"),
            range: XSD_POSITIVE_INTEGER,
        },
    ]
}
