//! `monoidal/` namespace — Monoidal composition.
//!
//! The `monoidal/` namespace formalizes sequential composition of computations
//! via the monoidal product A \u{2297} B: the output of A feeds the input of B.
//! Includes the identity computation I and the associativity isomorphism.
//!
//! - **Amendment 69**: 3 classes, 8 properties
//!
//! **Space classification:** `kernel` — immutable algebra.

use crate::model::iris::*;
use crate::model::{Class, Individual, Namespace, NamespaceModule, Property, PropertyKind, Space};

/// Returns the `monoidal/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "monoidal",
            iri: NS_MONOIDAL,
            label: "UOR Monoidal Composition",
            comment: "Sequential composition of computations via monoidal \
                      product A \u{2297} B.",
            space: Space::Kernel,
            imports: &[NS_OP, NS_CASCADE, NS_CERT],
        },
        classes: classes(),
        properties: properties(),
        individuals: individuals(),
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/monoidal/MonoidalProduct",
            label: "MonoidalProduct",
            comment: "A \u{2297} B: the sequential composition of two \
                      computations. Output of A feeds input of B.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/monoidal/MonoidalUnit",
            label: "MonoidalUnit",
            comment: "The identity computation I: passes input through \
                      unchanged. I \u{2297} A \u{2245} A \u{2245} A \
                      \u{2297} I.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/monoidal/MonoidalAssociator",
            label: "MonoidalAssociator",
            comment: "The witness that (A\u{2297}B)\u{2297}C \u{2245} \
                      A\u{2297}(B\u{2297}C). The associativity isomorphism.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        // MonoidalProduct properties
        Property {
            id: "https://uor.foundation/monoidal/leftComponent",
            label: "leftComponent",
            comment: "The left operand in the monoidal product A \u{2297} B.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/monoidal/MonoidalProduct"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/monoidal/rightComponent",
            label: "rightComponent",
            comment: "The right operand in the monoidal product A \u{2297} B.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/monoidal/MonoidalProduct"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/monoidal/composedEndpoint",
            label: "composedEndpoint",
            comment: "The endpoint of the composed computation A \u{2297} B.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/monoidal/MonoidalProduct"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/monoidal/monoidalSaturation",
            label: "monoidalSaturation",
            comment: "\u{03c3}(A\u{2297}B) relationship: saturation of the \
                      sequential composition.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/monoidal/MonoidalProduct"),
            range: XSD_STRING,
        },
        // MonoidalUnit property
        Property {
            id: "https://uor.foundation/monoidal/unitWitness",
            label: "unitWitness",
            comment: "Witness that I \u{2297} A \u{2245} A \u{2245} A \
                      \u{2297} I.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/monoidal/MonoidalUnit"),
            range: XSD_STRING,
        },
        // MonoidalAssociator properties
        Property {
            id: "https://uor.foundation/monoidal/associatorLeftTriple",
            label: "associatorLeftTriple",
            comment: "The left-grouped triple (A\u{2297}B)\u{2297}C.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/monoidal/MonoidalAssociator"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/monoidal/associatorRightTriple",
            label: "associatorRightTriple",
            comment: "The right-grouped triple A\u{2297}(B\u{2297}C).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/monoidal/MonoidalAssociator"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/monoidal/associatorWitness",
            label: "associatorWitness",
            comment: "Witness of the associativity isomorphism \
                      (A\u{2297}B)\u{2297}C \u{2245} A\u{2297}(B\u{2297}C).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/monoidal/MonoidalAssociator"),
            range: XSD_STRING,
        },
    ]
}

fn individuals() -> Vec<Individual> {
    vec![]
}
