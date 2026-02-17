//! `type/` namespace — Runtime type declarations.
//!
//! Types are declared at runtime by Prism applications and parameterize the
//! resolution pipeline. A type declaration tells the resolver how to partition
//! the ring into irreducible, reducible, unit, and exterior elements.
//!
//! **Space classification:** `user` — parameterizable at runtime.

use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};
use crate::model::iris::*;

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
            imports: &[NS_SCHEMA, NS_U],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
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
            disjoint_with: &[],
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
                      Expressed as a string in the Prism constraint language.",
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
    ]
}
