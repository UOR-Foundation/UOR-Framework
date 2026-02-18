//! `partition/` namespace — Irreducibility partitions of the ring (Amendment 5).
//!
//! The partition map Π : T_n → Part(R_n) is the central function of the UOR
//! Framework. It maps a type declaration to a four-component partition of the
//! ring, classifying every ring element as irreducible, reducible, a unit,
//! or exterior to the carrier.
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
            disjoint_with: &[],
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
    ]
}
