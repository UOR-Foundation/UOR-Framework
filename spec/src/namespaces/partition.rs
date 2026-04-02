//! `partition/` namespace — Irreducibility partitions of the ring (Amendment 5).
//!
//! The partition map Π : T_n → Part(R_n) is the central function of the UOR
//! Framework. It maps a type declaration to a four-component partition of the
//! ring, classifying every ring element as irreducible, reducible, a unit,
//! or exterior to the carrier.
//!
//! Amendment 9 adds fiber budget formalization: fiber coordinates, budget
//! accounting, and fiber pinning — the completeness criterion for type
//! declarations.
//!
//! **Space classification:** `bridge` — produced by the kernel, consumed by user-space.

use crate::model::iris::*;
use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};

/// Returns the `partition/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "partition",
            iri: NS_PARTITION,
            label: "UOR Partitions",
            comment: "Irreducibility partitions produced by type resolution. \
                      A partition divides the ring into four disjoint components: \
                      Irreducible, Reducible, Units, and Exterior.",
            space: Space::Bridge,
            imports: &[NS_SCHEMA, NS_TYPE],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/partition/Partition",
            label: "Partition",
            comment: "A four-component partition of R_n produced by resolving a \
                      type declaration. The four components — Irreducible, Reducible, \
                      Units, Exterior — are mutually disjoint and exhaustive over \
                      the carrier.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/partition/Component",
            label: "Component",
            comment: "A single component of a partition: a set of datum values \
                      belonging to one of the four categories.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/partition/FiberCoordinate",
                "https://uor.foundation/partition/FiberBudget",
            ],
        },
        Class {
            id: "https://uor.foundation/partition/IrreducibleSet",
            label: "IrreducibleSet",
            comment: "The set of irreducible elements under the active type: elements \
                      whose only factorizations involve units or themselves. \
                      Analogous to prime elements in a ring.",
            subclass_of: &["https://uor.foundation/partition/Component"],
            disjoint_with: &[
                "https://uor.foundation/partition/ReducibleSet",
                "https://uor.foundation/partition/UnitSet",
                "https://uor.foundation/partition/ExteriorSet",
            ],
        },
        Class {
            id: "https://uor.foundation/partition/ReducibleSet",
            label: "ReducibleSet",
            comment: "The set of reducible non-unit elements: elements that can be \
                      expressed as a product of two or more non-unit elements.",
            subclass_of: &["https://uor.foundation/partition/Component"],
            disjoint_with: &[
                "https://uor.foundation/partition/IrreducibleSet",
                "https://uor.foundation/partition/UnitSet",
                "https://uor.foundation/partition/ExteriorSet",
            ],
        },
        Class {
            id: "https://uor.foundation/partition/UnitSet",
            label: "UnitSet",
            comment: "The set of invertible elements (units) in the carrier: elements \
                      with a multiplicative inverse. In Z/(2^n)Z, the units are the \
                      odd integers.",
            subclass_of: &["https://uor.foundation/partition/Component"],
            disjoint_with: &[
                "https://uor.foundation/partition/IrreducibleSet",
                "https://uor.foundation/partition/ReducibleSet",
                "https://uor.foundation/partition/ExteriorSet",
            ],
        },
        Class {
            id: "https://uor.foundation/partition/ExteriorSet",
            label: "ExteriorSet",
            comment: "Elements of R_n that fall outside the active carrier — i.e., \
                      outside the type's domain. These are ring elements that do \
                      not participate in the current type resolution.",
            subclass_of: &["https://uor.foundation/partition/Component"],
            disjoint_with: &[
                "https://uor.foundation/partition/IrreducibleSet",
                "https://uor.foundation/partition/ReducibleSet",
                "https://uor.foundation/partition/UnitSet",
            ],
        },
        // Amendment 9: Fiber Budget classes
        Class {
            id: "https://uor.foundation/partition/FiberCoordinate",
            label: "FiberCoordinate",
            comment: "A single fiber coordinate in the iterated Z/2Z fibration. \
                      Each fiber represents one binary degree of freedom in the \
                      ring's structure. The total number of fibers equals the \
                      quantum level n.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/partition/FiberBudget",
                "https://uor.foundation/partition/Component",
            ],
        },
        Class {
            id: "https://uor.foundation/partition/FiberBudget",
            label: "FiberBudget",
            comment: "The fiber budget for a partition: an accounting of how many \
                      fibers are pinned (determined by constraints) versus free \
                      (still available for further refinement). A closed budget \
                      means all fibers are pinned and the type is fully resolved.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/partition/FiberCoordinate",
                "https://uor.foundation/partition/Component",
            ],
        },
        Class {
            id: "https://uor.foundation/partition/FiberPinning",
            label: "FiberPinning",
            comment: "A record of a single fiber being pinned by a constraint. \
                      Links a specific fiber coordinate to the constraint that \
                      determined its value.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 37: Partition Tensor Product (Gap 8)
        Class {
            id: "https://uor.foundation/partition/PartitionProduct",
            label: "PartitionProduct",
            comment: "The tensor product of two partitions: partition(A × B) = \
                      partition(A) ⊗ partition(B). The four-component structure \
                      combines component-wise under the product type construction \
                      (PT_2a). Carries leftFactor and rightFactor links to the \
                      operand partitions.",
            subclass_of: &[OWL_THING],
            disjoint_with: &["https://uor.foundation/partition/PartitionCoproduct"],
        },
        Class {
            id: "https://uor.foundation/partition/PartitionCoproduct",
            label: "PartitionCoproduct",
            comment: "The coproduct (disjoint union) of two partitions: \
                      partition(A + B) = partition(A) ⊕ partition(B). The \
                      four-component structure combines via disjoint union under \
                      the sum type construction (PT_2b). Carries leftSummand and \
                      rightSummand links to the operand partitions.",
            subclass_of: &[OWL_THING],
            disjoint_with: &["https://uor.foundation/partition/PartitionProduct"],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/partition/irreducibles",
            label: "irreducibles",
            comment: "The irreducible component of this partition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: "https://uor.foundation/partition/IrreducibleSet",
        },
        Property {
            id: "https://uor.foundation/partition/reducibles",
            label: "reducibles",
            comment: "The reducible component of this partition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: "https://uor.foundation/partition/ReducibleSet",
        },
        Property {
            id: "https://uor.foundation/partition/units",
            label: "units",
            comment: "The units component of this partition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: "https://uor.foundation/partition/UnitSet",
        },
        Property {
            id: "https://uor.foundation/partition/exterior",
            label: "exterior",
            comment: "The exterior component of this partition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: "https://uor.foundation/partition/ExteriorSet",
        },
        Property {
            id: "https://uor.foundation/partition/member",
            label: "member",
            comment: "A datum value belonging to this partition component.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/partition/Component"),
            range: "https://uor.foundation/schema/Datum",
        },
        Property {
            id: "https://uor.foundation/partition/cardinality",
            label: "cardinality",
            comment: "The number of elements in this partition component. \
                      The cardinalities of the four components must sum to 2^n.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/Component"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/density",
            label: "density",
            comment: "The irreducible density of this partition: |Irr| / |A|, \
                      where A is the active carrier.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: XSD_DECIMAL,
        },
        Property {
            id: "https://uor.foundation/partition/sourceType",
            label: "sourceType",
            comment: "The type declaration that was resolved to produce this \
                      partition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/partition/quantum",
            label: "quantum",
            comment: "The quantum level n at which this partition was computed. \
                      The ring has 2^n elements at this level.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: XSD_POSITIVE_INTEGER,
        },
        // Amendment 9: Fiber Budget properties
        Property {
            id: "https://uor.foundation/partition/fiberPosition",
            label: "fiberPosition",
            comment: "The zero-based position of this fiber coordinate within \
                      the iterated fibration. Position 0 is the least significant \
                      bit; position n-1 is the most significant.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FiberCoordinate"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/fiberState",
            label: "fiberState",
            comment: "The current state of this fiber coordinate: 'pinned' if \
                      determined by a constraint, 'free' if still available for \
                      refinement.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FiberCoordinate"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/fiberBudget",
            label: "fiberBudget",
            comment: "The fiber budget associated with this partition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: "https://uor.foundation/partition/FiberBudget",
        },
        Property {
            id: "https://uor.foundation/partition/totalFibers",
            label: "totalFibers",
            comment: "The total number of fiber coordinates in this budget, \
                      equal to the quantum level n.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FiberBudget"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/pinnedCount",
            label: "pinnedCount",
            comment: "The number of fiber coordinates currently pinned by \
                      constraints.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FiberBudget"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/freeCount",
            label: "freeCount",
            comment: "The number of fiber coordinates still free (not yet \
                      pinned). Equals totalFibers - pinnedCount.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FiberBudget"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/isClosed",
            label: "isClosed",
            comment: "Whether all fibers in this budget are pinned. A closed \
                      budget means the type is fully resolved and the partition \
                      is complete.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FiberBudget"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/partition/hasFiber",
            label: "hasFiber",
            comment: "A fiber coordinate belonging to this budget.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/partition/FiberBudget"),
            range: "https://uor.foundation/partition/FiberCoordinate",
        },
        Property {
            id: "https://uor.foundation/partition/pinnedBy",
            label: "pinnedBy",
            comment: "The constraint that pins this fiber coordinate.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/FiberPinning"),
            range: "https://uor.foundation/type/Constraint",
        },
        Property {
            id: "https://uor.foundation/partition/pinsCoordinate",
            label: "pinsCoordinate",
            comment: "The fiber coordinate that this pinning determines.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/FiberPinning"),
            range: "https://uor.foundation/partition/FiberCoordinate",
        },
        Property {
            id: "https://uor.foundation/partition/hasPinning",
            label: "hasPinning",
            comment: "A fiber pinning record in this budget.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/partition/FiberBudget"),
            range: "https://uor.foundation/partition/FiberPinning",
        },
        // Amendment 31: Reversible computation properties (RC_1–RC_4)
        Property {
            id: "https://uor.foundation/partition/ancillaFiber",
            label: "ancillaFiber",
            comment: "An ancilla fiber coordinate paired with this fiber for \
                      reversible computation (RC_1–RC_4 ancilla model).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/FiberCoordinate"),
            range: "https://uor.foundation/partition/FiberCoordinate",
        },
        Property {
            id: "https://uor.foundation/partition/reversibleStrategy",
            label: "reversibleStrategy",
            comment: "True when this fiber budget uses a reversible computation \
                      strategy preserving information through ancilla fibers.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FiberBudget"),
            range: XSD_BOOLEAN,
        },
        // Amendment 37: ExteriorSet formal criteria (Gap 2)
        Property {
            id: "https://uor.foundation/partition/exteriorCriteria",
            label: "exteriorCriteria",
            comment: "The formal membership criterion for this ExteriorSet: \
                      x ∈ Ext(T) iff x ∉ carrier(T). The ExteriorSet is \
                      context-dependent on the active type T (FPM_9).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/ExteriorSet"),
            range: "https://uor.foundation/schema/TermExpression",
        },
        // Amendment 37: Partition exhaustiveness (Gap 3)
        Property {
            id: "https://uor.foundation/partition/isExhaustive",
            label: "isExhaustive",
            comment: "Whether the four components of this partition are exhaustive \
                      over R_n: |Irr| + |Red| + |Unit| + |Ext| = 2^n (FPM_8). \
                      Set by the kernel after verification.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: XSD_BOOLEAN,
        },
        // Amendment 37: Partition tensor product properties (Gap 8)
        Property {
            id: "https://uor.foundation/partition/leftFactor",
            label: "leftFactor",
            comment: "The left operand partition of this tensor product.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/PartitionProduct"),
            range: "https://uor.foundation/partition/Partition",
        },
        Property {
            id: "https://uor.foundation/partition/rightFactor",
            label: "rightFactor",
            comment: "The right operand partition of this tensor product.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/PartitionProduct"),
            range: "https://uor.foundation/partition/Partition",
        },
        Property {
            id: "https://uor.foundation/partition/leftSummand",
            label: "leftSummand",
            comment: "The left operand partition of this coproduct.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/PartitionCoproduct"),
            range: "https://uor.foundation/partition/Partition",
        },
        Property {
            id: "https://uor.foundation/partition/rightSummand",
            label: "rightSummand",
            comment: "The right operand partition of this coproduct.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/PartitionCoproduct"),
            range: "https://uor.foundation/partition/Partition",
        },
    ]
}
