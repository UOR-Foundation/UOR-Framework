//! `type/` namespace — Runtime type declarations.
//!
//! Types are declared at runtime by Prism applications and parameterize the
//! resolution pipeline. A type declaration tells the resolver how to partition
//! the ring into irreducible, reducible, unit, and exterior elements.
//!
//! Amendment 10 adds constraint algebra: composable predicates that refine types
//! by pinning fibers, plus metric axes that classify constraints by their
//! geometric effect.
//!
//! **Space classification:** `user` — parameterizable at runtime.

use crate::model::iris::*;
use crate::model::{Class, Individual, Namespace, NamespaceModule, Property, PropertyKind, Space};

/// Returns the `type/` namespace module.
///
/// Note: the module is named `type_` because `type` is a reserved keyword in Rust.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "type",
            iri: NS_TYPE,
            label: "UOR Type System",
            comment: "Runtime type declarations that parameterize the resolution \
                      pipeline. Types are declared by Prism applications and \
                      resolved to partitions of the ring.",
            space: Space::User,
            imports: &[NS_SCHEMA, NS_U, NS_OP],
        },
        classes: classes(),
        properties: properties(),
        individuals: individuals(),
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/type/TypeDefinition",
            label: "TypeDefinition",
            comment: "A runtime type declaration. The root class for all UOR types. \
                      Each TypeDefinition, when resolved, produces a partition of \
                      the ring at the specified quantum level.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/type/Constraint",
                "https://uor.foundation/type/MetricAxis",
            ],
        },
        Class {
            id: "https://uor.foundation/type/PrimitiveType",
            label: "PrimitiveType",
            comment: "A primitive type defined by a fixed bit width. The carrier is \
                      the entire ring Z/(2^n)Z at the specified quantum level.",
            subclass_of: &["https://uor.foundation/type/TypeDefinition"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/type/ProductType",
            label: "ProductType",
            comment: "A product (Cartesian) type formed from multiple component \
                      types. The carrier is the product of the component carriers.",
            subclass_of: &["https://uor.foundation/type/TypeDefinition"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/type/SumType",
            label: "SumType",
            comment: "A sum (disjoint union) type formed from multiple variant \
                      types. The carrier is the disjoint union of the variant \
                      carriers.",
            subclass_of: &["https://uor.foundation/type/TypeDefinition"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/type/ConstrainedType",
            label: "ConstrainedType",
            comment: "A type formed by constraining a base type with a predicate. \
                      The carrier is the subset of the base carrier satisfying the \
                      constraint.",
            subclass_of: &["https://uor.foundation/type/TypeDefinition"],
            disjoint_with: &[],
        },
        // Amendment 10: Constraint Algebra classes
        Class {
            id: "https://uor.foundation/type/Constraint",
            label: "Constraint",
            comment: "A composable predicate that refines a type by pinning one or \
                      more fiber coordinates. Constraints are the parameterization \
                      mechanism for ConstrainedType.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/type/TypeDefinition",
                "https://uor.foundation/type/MetricAxis",
            ],
        },
        Class {
            id: "https://uor.foundation/type/ResidueConstraint",
            label: "ResidueConstraint",
            comment: "A constraint based on residue class membership: x ≡ r (mod m). \
                      Pins fibers corresponding to the residue pattern.",
            subclass_of: &["https://uor.foundation/type/Constraint"],
            disjoint_with: &[
                "https://uor.foundation/type/CarryConstraint",
                "https://uor.foundation/type/DepthConstraint",
                "https://uor.foundation/type/CompositeConstraint",
            ],
        },
        Class {
            id: "https://uor.foundation/type/CarryConstraint",
            label: "CarryConstraint",
            comment: "A constraint based on carry propagation patterns in ring \
                      arithmetic. Pins fibers corresponding to carry positions.",
            subclass_of: &["https://uor.foundation/type/Constraint"],
            disjoint_with: &[
                "https://uor.foundation/type/ResidueConstraint",
                "https://uor.foundation/type/DepthConstraint",
                "https://uor.foundation/type/CompositeConstraint",
            ],
        },
        Class {
            id: "https://uor.foundation/type/DepthConstraint",
            label: "DepthConstraint",
            comment: "A constraint on factorization depth: the minimum and maximum \
                      number of irreducible factors. Pins fibers by bounding the \
                      factorization tree depth.",
            subclass_of: &["https://uor.foundation/type/Constraint"],
            disjoint_with: &[
                "https://uor.foundation/type/ResidueConstraint",
                "https://uor.foundation/type/CarryConstraint",
                "https://uor.foundation/type/CompositeConstraint",
            ],
        },
        Class {
            id: "https://uor.foundation/type/CompositeConstraint",
            label: "CompositeConstraint",
            comment: "A constraint formed by composing two or more simpler \
                      constraints. The composite pins the union of fibers \
                      pinned by its components.",
            subclass_of: &["https://uor.foundation/type/Constraint"],
            disjoint_with: &[
                "https://uor.foundation/type/ResidueConstraint",
                "https://uor.foundation/type/CarryConstraint",
                "https://uor.foundation/type/DepthConstraint",
            ],
        },
        Class {
            id: "https://uor.foundation/type/MetricAxis",
            label: "MetricAxis",
            comment: "A classification axis for constraints by their geometric \
                      effect. The three axes — vertical (ring/additive), \
                      horizontal (Hamming/bitwise), diagonal (incompatibility) — \
                      form the tri-metric coordinate system of UOR.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/type/TypeDefinition",
                "https://uor.foundation/type/Constraint",
            ],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/type/bitWidth",
            label: "bitWidth",
            comment: "The bit width of a primitive type (the quantum level n). \
                      The carrier is Z/(2^n)Z.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/PrimitiveType"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/type/component",
            label: "component",
            comment: "A component type in a product type.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/type/ProductType"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/type/baseType",
            label: "baseType",
            comment: "The base type that a constrained type restricts.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/ConstrainedType"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/type/constraint",
            label: "constraint",
            comment: "The constraint predicate applied to the base type. \
                      Expressed as a string in the Prism constraint language. \
                      Deprecated in favor of type:hasConstraint (Amendment 10), \
                      which provides typed object references to Constraint \
                      individuals. Retained for backwards compatibility.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/type/ConstrainedType"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/type/contentAddress",
            label: "contentAddress",
            comment: "The content-derived address of this type definition, \
                      uniquely identifying the type in the UOR address space.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/TypeDefinition"),
            range: "https://uor.foundation/u/Address",
        },
        // Amendment 10: Constraint Algebra properties
        Property {
            id: "https://uor.foundation/type/hasConstraint",
            label: "hasConstraint",
            comment: "A typed constraint object applied to this constrained type. \
                      Replaces the deprecated string-based type:constraint property.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/type/ConstrainedType"),
            range: "https://uor.foundation/type/Constraint",
        },
        Property {
            id: "https://uor.foundation/type/modulus",
            label: "modulus",
            comment: "The modulus m of a residue constraint: x ≡ r (mod m).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/ResidueConstraint"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/type/residue",
            label: "residue",
            comment: "The residue value r of a residue constraint: x ≡ r (mod m).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/ResidueConstraint"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/type/carryPattern",
            label: "carryPattern",
            comment: "The carry propagation pattern of a carry constraint, \
                      expressed as a binary string (e.g., '1010').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/CarryConstraint"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/type/minDepth",
            label: "minDepth",
            comment: "The minimum factorization depth required by a depth \
                      constraint.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/DepthConstraint"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/type/maxDepth",
            label: "maxDepth",
            comment: "The maximum factorization depth allowed by a depth \
                      constraint.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/DepthConstraint"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/type/composedFrom",
            label: "composedFrom",
            comment: "A component constraint of this composite constraint.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/type/CompositeConstraint"),
            range: "https://uor.foundation/type/Constraint",
        },
        Property {
            id: "https://uor.foundation/type/metricAxis",
            label: "metricAxis",
            comment: "The metric axis along which this constraint operates: \
                      vertical (ring), horizontal (Hamming), or diagonal \
                      (incompatibility).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/Constraint"),
            range: "https://uor.foundation/type/MetricAxis",
        },
        Property {
            id: "https://uor.foundation/type/pinsFibers",
            label: "pinsFibers",
            comment: "A fiber coordinate that this constraint pins when applied.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/type/Constraint"),
            range: "https://uor.foundation/partition/FiberCoordinate",
        },
        Property {
            id: "https://uor.foundation/type/axisSignature",
            label: "axisSignature",
            comment: "The metric axis signature of an operation: a string encoding \
                      which axes the operation affects (e.g., 'V' for vertical, \
                      'H' for horizontal, 'VH' for both).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/op/Operation"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/type/crossingCost",
            label: "crossingCost",
            comment: "The cost of applying this constraint in terms of axis \
                      crossings: the number of metric boundaries that must be \
                      traversed.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/Constraint"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
    ]
}

fn individuals() -> Vec<Individual> {
    vec![
        Individual {
            id: "https://uor.foundation/type/verticalAxis",
            type_: "https://uor.foundation/type/MetricAxis",
            label: "verticalAxis",
            comment: "The vertical (ring/additive) metric axis. Constraints on \
                      this axis operate through ring arithmetic: residue classes, \
                      divisibility, and additive structure.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/type/horizontalAxis",
            type_: "https://uor.foundation/type/MetricAxis",
            label: "horizontalAxis",
            comment: "The horizontal (Hamming/bitwise) metric axis. Constraints on \
                      this axis operate through bitwise structure: carry patterns, \
                      bit positions, and Hamming distance.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/type/diagonalAxis",
            type_: "https://uor.foundation/type/MetricAxis",
            label: "diagonalAxis",
            comment: "The diagonal (incompatibility) metric axis. Constraints on \
                      this axis measure the gap between ring and Hamming metrics — \
                      the curvature of UOR geometry.",
            properties: &[],
        },
    ]
}
