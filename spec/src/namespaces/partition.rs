//! `partition/` namespace — Irreducibility partitions of the ring (Amendment 5).
//!
//! The partition map Π : T_n → Part(R_n) is the central function of the UOR
//! Framework. It maps a type declaration to a four-component partition of the
//! ring, classifying every ring element as irreducible, reducible, a unit,
//! or exterior to the carrier.
//!
//! Amendment 9 adds site budget formalization: site coordinates, budget
//! accounting, and site pinning — the completeness criterion for type
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
                "https://uor.foundation/partition/SiteIndex",
                "https://uor.foundation/partition/FreeRank",
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
                "https://uor.foundation/partition/UnitGroup",
                "https://uor.foundation/partition/Complement",
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
                "https://uor.foundation/partition/UnitGroup",
                "https://uor.foundation/partition/Complement",
            ],
        },
        Class {
            id: "https://uor.foundation/partition/UnitGroup",
            label: "UnitGroup",
            comment: "The set of invertible elements (units) in the carrier: elements \
                      with a multiplicative inverse. In Z/(2^n)Z, the units are the \
                      odd integers.",
            subclass_of: &["https://uor.foundation/partition/Component"],
            disjoint_with: &[
                "https://uor.foundation/partition/IrreducibleSet",
                "https://uor.foundation/partition/ReducibleSet",
                "https://uor.foundation/partition/Complement",
            ],
        },
        Class {
            id: "https://uor.foundation/partition/Complement",
            label: "Complement",
            comment: "Elements of R_n that fall outside the active carrier — i.e., \
                      outside the type's domain. These are ring elements that do \
                      not participate in the current type resolution.",
            subclass_of: &["https://uor.foundation/partition/Component"],
            disjoint_with: &[
                "https://uor.foundation/partition/IrreducibleSet",
                "https://uor.foundation/partition/ReducibleSet",
                "https://uor.foundation/partition/UnitGroup",
            ],
        },
        // Amendment 9: Free Rank classes
        Class {
            id: "https://uor.foundation/partition/SiteIndex",
            label: "SiteIndex",
            comment: "A single site coordinate in the iterated Z/2Z fibration. \
                      Each site represents one binary degree of freedom in the \
                      ring's structure. The total number of sites equals the \
                      quantum level n.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/partition/FreeRank",
                "https://uor.foundation/partition/Component",
            ],
        },
        Class {
            id: "https://uor.foundation/partition/FreeRank",
            label: "FreeRank",
            comment: "The site budget for a partition: an accounting of how many \
                      sites are pinned (determined by constraints) versus free \
                      (still available for further refinement). A closed budget \
                      means all sites are pinned and the type is fully resolved.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/partition/SiteIndex",
                "https://uor.foundation/partition/Component",
            ],
        },
        Class {
            id: "https://uor.foundation/partition/SiteBinding",
            label: "SiteBinding",
            comment: "A record of a single site being pinned by a constraint. \
                      Links a specific site coordinate to the constraint that \
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
            range: "https://uor.foundation/partition/UnitGroup",
        },
        Property {
            id: "https://uor.foundation/partition/exterior",
            label: "exterior",
            comment: "The exterior component of this partition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: "https://uor.foundation/partition/Complement",
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
            id: "https://uor.foundation/partition/wittLength",
            label: "wittLength",
            comment: "The Witt level n at which this partition was computed. \
                      The ring has 2^n elements at this level.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: XSD_POSITIVE_INTEGER,
        },
        // Amendment 9: Free Rank properties
        Property {
            id: "https://uor.foundation/partition/sitePosition",
            label: "sitePosition",
            comment: "The zero-based position of this site coordinate within \
                      the iterated fibration. Position 0 is the least significant \
                      bit; position n-1 is the most significant.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/SiteIndex"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/siteState",
            label: "siteState",
            comment: "The current state of this site coordinate: 'pinned' if \
                      determined by a constraint, 'free' if still available for \
                      refinement.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/SiteIndex"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/siteBudget",
            label: "siteBudget",
            comment: "The site budget associated with this partition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/Partition"),
            range: "https://uor.foundation/partition/FreeRank",
        },
        Property {
            id: "https://uor.foundation/partition/totalSites",
            label: "totalSites",
            comment: "The total number of site coordinates in this budget, \
                      equal to the quantum level n.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FreeRank"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/pinnedCount",
            label: "pinnedCount",
            comment: "The number of site coordinates currently pinned by \
                      constraints.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FreeRank"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/freeRank",
            label: "freeRank",
            comment: "The number of site coordinates still free (not yet \
                      pinned). Equals totalSites - pinnedCount.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FreeRank"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/partition/isClosed",
            label: "isClosed",
            comment: "Whether all sites in this budget are pinned. A closed \
                      budget means the type is fully resolved and the partition \
                      is complete.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FreeRank"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/partition/hasSite",
            label: "hasSite",
            comment: "A site coordinate belonging to this budget.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/partition/FreeRank"),
            range: "https://uor.foundation/partition/SiteIndex",
        },
        Property {
            id: "https://uor.foundation/partition/pinnedBy",
            label: "pinnedBy",
            comment: "The constraint that pins this site coordinate.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/SiteBinding"),
            range: "https://uor.foundation/type/Constraint",
        },
        Property {
            id: "https://uor.foundation/partition/pinsCoordinate",
            label: "pinsCoordinate",
            comment: "The site coordinate that this pinning determines.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/SiteBinding"),
            range: "https://uor.foundation/partition/SiteIndex",
        },
        Property {
            id: "https://uor.foundation/partition/hasBinding",
            label: "hasBinding",
            comment: "A site pinning record in this budget.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/partition/FreeRank"),
            range: "https://uor.foundation/partition/SiteBinding",
        },
        // Amendment 31: Reversible computation properties (RC_1–RC_4)
        Property {
            id: "https://uor.foundation/partition/ancillaSite",
            label: "ancillaSite",
            comment: "An ancilla site coordinate paired with this site for \
                      reversible computation (RC_1–RC_4 ancilla model).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/SiteIndex"),
            range: "https://uor.foundation/partition/SiteIndex",
        },
        Property {
            id: "https://uor.foundation/partition/reversibleStrategy",
            label: "reversibleStrategy",
            comment: "True when this site budget uses a reversible computation \
                      strategy preserving information through ancilla sites.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/partition/FreeRank"),
            range: XSD_BOOLEAN,
        },
        // Amendment 37: Complement formal criteria (Gap 2)
        Property {
            id: "https://uor.foundation/partition/exteriorCriteria",
            label: "exteriorCriteria",
            comment: "The formal membership criterion for this Complement: \
                      x ∈ Ext(T) iff x ∉ carrier(T). The Complement is \
                      context-dependent on the active type T (FPM_9).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/partition/Complement"),
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
