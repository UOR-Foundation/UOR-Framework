//! `schema/` namespace — Ring substrate, term language, and core value types.
//!
//! The `schema/` namespace defines the fundamental algebraic substrate of the
//! UOR Framework: the ring Z/(2^n)Z (`Datum`), its term language (`Term`,
//! `Literal`, `Application`), and the ring container itself (`Ring`).
//!
//! **Key invariant:** `Term` and `Datum` are `owl:disjointWith` — syntax and
//! semantics are strictly separated. A `Literal` *denotes* a `Datum` via
//! `schema:denotes` without *being* one.
//!
//! **Space classification:** `kernel` — compiled into ROM.

use crate::model::{Class, Individual, IndividualValue, Namespace, NamespaceModule, Property, PropertyKind, Space};
use crate::model::iris::*;

/// Returns the `schema/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "schema",
            iri: NS_SCHEMA,
            label: "UOR Schema",
            comment: "Core value types and term language for the UOR ring substrate. \
                      Defines Datum (ring element), Term (syntactic expression), and \
                      the Ring container.",
            space: Space::Kernel,
            imports: &[NS_U],
        },
        classes: classes(),
        properties: properties(),
        individuals: individuals(),
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/schema/Datum",
            label: "Datum",
            comment: "An element of the ring Z/(2^n)Z at a specific quantum level n. \
                      The primary semantic value type. Disjoint from Term: datums are \
                      values, terms are syntactic expressions that evaluate to datums.",
            subclass_of: &[OWL_THING],
            disjoint_with: &["https://uor.foundation/schema/Term"],
        },
        Class {
            id: "https://uor.foundation/schema/Term",
            label: "Term",
            comment: "A syntactic expression in the UOR term language. Terms are \
                      evaluated to produce Datums. Disjoint from Datum.",
            subclass_of: &[OWL_THING],
            disjoint_with: &["https://uor.foundation/schema/Datum"],
        },
        Class {
            id: "https://uor.foundation/schema/Triad",
            label: "Triad",
            comment: "A three-component structure encoding an element's position in \
                      the UOR address space: stratum (ring layer), spectrum (bit \
                      pattern), and glyph (Braille address).",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/schema/Literal",
            label: "Literal",
            comment: "A term that directly denotes a datum value. A Literal is a \
                      leaf node in the term language — it refers to a concrete Datum \
                      via schema:denotes without being a Datum itself.",
            subclass_of: &["https://uor.foundation/schema/Term"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/schema/Application",
            label: "Application",
            comment: "A term formed by applying an operation to one or more argument \
                      terms. The application's value is the result of evaluating the \
                      operator on the evaluated arguments.",
            subclass_of: &["https://uor.foundation/schema/Term"],
            disjoint_with: &[],
        },
        // Amendment 2: Ring class
        Class {
            id: "https://uor.foundation/schema/Ring",
            label: "Ring",
            comment: "The ambient ring Z/(2^n)Z at a specific quantum level n. \
                      The Ring is the primary data structure of the UOR kernel. \
                      Its two generators (negation and complement) produce the \
                      dihedral group D_{2^n} that governs the invariance frame.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/schema/value",
            label: "value",
            comment: "The integer value of a datum element. For a Datum in Z/(2^n)Z, \
                      this is an integer in [0, 2^n).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/schema/Datum"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/schema/quantum",
            label: "quantum",
            comment: "The quantum level n of a datum, where the datum's ring is \
                      Z/(2^n)Z. Determines the bit width and modulus of the datum.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/schema/Datum"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/schema/stratum",
            label: "stratum",
            comment: "The ring-layer index of a datum, indicating its position in \
                      the stratification of Z/(2^n)Z.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/schema/Datum"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/schema/spectrum",
            label: "spectrum",
            comment: "The bit-pattern representation of a datum, encoding its \
                      position in the hypercube geometry of Z/(2^n)Z.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/schema/Datum"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/schema/glyph",
            label: "glyph",
            comment: "The Braille address associated with this datum, linking the \
                      algebraic value to its content-addressable identifier.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/schema/Datum"),
            range: "https://uor.foundation/u/Address",
        },
        Property {
            id: "https://uor.foundation/schema/operator",
            label: "operator",
            comment: "The operation applied in an Application term.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/schema/Application"),
            range: "https://uor.foundation/op/Operation",
        },
        Property {
            id: "https://uor.foundation/schema/argument",
            label: "argument",
            comment: "An argument term in an Application. The ordering of arguments \
                      follows rdf:List semantics.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/schema/Application"),
            range: "https://uor.foundation/schema/Term",
        },
        // Amendment 2: Ring properties
        Property {
            id: "https://uor.foundation/schema/ringQuantum",
            label: "ringQuantum",
            comment: "The bit width n of the ring Z/(2^n)Z. Distinct from \
                      schema:quantum on Datum — ringQuantum is the container's \
                      bit width; datum quantum is a membership property.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/schema/Ring"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/schema/modulus",
            label: "modulus",
            comment: "The modulus 2^n of the ring. Equals 2 raised to the power \
                      of ringQuantum.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/schema/Ring"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/schema/generator",
            label: "generator",
            comment: "The generator element π₁ (value = 1) of the ring. Under \
                      iterated successor application, π₁ generates all ring elements.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/schema/Ring"),
            range: "https://uor.foundation/schema/Datum",
        },
        Property {
            id: "https://uor.foundation/schema/negation",
            label: "negation",
            comment: "The ring reflection involution: neg(x) = (-x) mod 2^n. \
                      One of the two generators of the dihedral group D_{2^n}.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/schema/Ring"),
            range: "https://uor.foundation/op/Involution",
        },
        Property {
            id: "https://uor.foundation/schema/complement",
            label: "complement",
            comment: "The hypercube reflection involution: bnot(x) = (2^n - 1) ⊕ x. \
                      The second generator of the dihedral group D_{2^n}.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/schema/Ring"),
            range: "https://uor.foundation/op/Involution",
        },
        Property {
            id: "https://uor.foundation/schema/denotes",
            label: "denotes",
            comment: "The datum value that a Literal term denotes. Bridges the \
                      Term/Datum disjointness: a Literal refers to a Datum without \
                      being one. Evaluation of a Literal produces its denoted Datum.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/schema/Literal"),
            range: "https://uor.foundation/schema/Datum",
        },
    ]
}

fn individuals() -> Vec<Individual> {
    vec![
        // Amendment 2: pi1 — the generator (value = 1)
        Individual {
            id: "https://uor.foundation/schema/pi1",
            type_: "https://uor.foundation/schema/Datum",
            label: "π₁",
            comment: "The unique generator of R_n under successor. Value = 1 at every \
                      quantum level. Under iterated application of succ, π₁ generates \
                      every element of the ring.",
            properties: &[
                ("https://uor.foundation/schema/value", IndividualValue::Int(1)),
            ],
        },
        // Amendment 2: zero — the additive identity
        Individual {
            id: "https://uor.foundation/schema/zero",
            type_: "https://uor.foundation/schema/Datum",
            label: "zero",
            comment: "The additive identity of the ring. Value = 0 at every quantum \
                      level. op:add(x, zero) = x for all x in R_n.",
            properties: &[
                ("https://uor.foundation/schema/value", IndividualValue::Int(0)),
            ],
        },
    ]
}
