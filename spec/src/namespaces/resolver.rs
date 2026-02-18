//! `resolver/` namespace — Type resolution strategies.
//!
//! Resolvers implement the map Π : T_n → Part(R_n), transforming a type
//! declaration into a partition of the ring. Resolution requests come from
//! user-space; execution occurs in the kernel.
//!
//! **Space classification:** `bridge` — user-requested, kernel-executed.

use crate::model::iris::*;
use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};

/// Returns the `resolver/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "resolver",
            iri: NS_RESOLVER,
            label: "UOR Resolvers",
            comment: "Type resolution strategies implementing the partition map \
                      Π : T_n → Part(R_n). Resolvers transform type declarations \
                      into ring partitions.",
            space: Space::Bridge,
            imports: &[NS_SCHEMA, NS_QUERY],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/resolver/Resolver",
            label: "Resolver",
            comment: "A strategy for resolving a type declaration into a partition \
                      of the ring. The kernel dispatches to a specific resolver \
                      based on the type's structure.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/resolver/DihedralFactorizationResolver",
            label: "DihedralFactorizationResolver",
            comment: "Resolves types by factoring the ring under dihedral group \
                      action. Identifies orbits under D_{2^n} to determine \
                      irreducibility boundaries.",
            subclass_of: &["https://uor.foundation/resolver/Resolver"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/resolver/CanonicalFormResolver",
            label: "CanonicalFormResolver",
            comment: "Resolves types by computing canonical forms via term rewriting. \
                      Applies the critical identity and normalization rules to \
                      reduce terms to unique canonical representatives.",
            subclass_of: &["https://uor.foundation/resolver/Resolver"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/resolver/EvaluationResolver",
            label: "EvaluationResolver",
            comment: "Resolves types by direct evaluation: applies operations to \
                      enumerate ring elements and classify them as irreducible, \
                      reducible, unit, or exterior.",
            subclass_of: &["https://uor.foundation/resolver/Resolver"],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/resolver/inputType",
            label: "inputType",
            comment: "The type of input this resolver accepts.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/Resolver"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/resolver/outputType",
            label: "outputType",
            comment: "The type of output this resolver produces. For all UOR \
                      resolvers, the output is a partition:Partition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/Resolver"),
            range: "https://uor.foundation/partition/Partition",
        },
        Property {
            id: "https://uor.foundation/resolver/strategy",
            label: "strategy",
            comment: "A human-readable description of the resolution strategy \
                      this resolver implements.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/resolver/Resolver"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/resolver/complexity",
            label: "complexity",
            comment: "The computational complexity of this resolver, expressed as \
                      a big-O string (e.g., 'O(n)', 'O(2^n)').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/resolver/Resolver"),
            range: XSD_STRING,
        },
    ]
}
