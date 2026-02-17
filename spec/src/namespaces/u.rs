//! `u/` namespace — Content addressing via Braille bijection.
//!
//! The `u/` namespace defines the content-addressing scheme used throughout
//! the UOR Framework. Addresses are represented as Braille strings, where
//! each Braille cell (glyph) encodes a 6-bit chunk of the address.
//!
//! **Space classification:** `kernel` — compiled into ROM.

use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};
use crate::model::iris::*;

/// Returns the `u/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "u",
            iri: NS_U,
            label: "UOR Content Addressing",
            comment: "Content-addressable identifiers represented as Braille strings. \
                      Each address is a sequence of Braille glyphs encoding a unique \
                      content-derived identifier.",
            space: Space::Kernel,
            imports: &[],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/u/Address",
            label: "Address",
            comment: "A content-addressable identifier represented as a Braille string. \
                      Each Address uniquely identifies a piece of content via its \
                      Braille-encoded hash.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/u/Glyph",
            label: "Glyph",
            comment: "A single Braille cell encoding 6 bits of an address. \
                      The bijection between Braille cells and 6-bit values is \
                      the foundational encoding of the UOR content-addressing scheme.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/u/glyph",
            label: "glyph",
            comment: "The Braille string representation of an address. \
                      Each character in the string is a Braille glyph (cell).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/u/Address"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/u/codepoint",
            label: "codepoint",
            comment: "The Unicode codepoint of a Braille glyph. \
                      Braille glyphs occupy the range U+2800–U+28FF.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/u/Glyph"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/u/byteValue",
            label: "byteValue",
            comment: "The 6-bit integer value (0–63) encoded by this Braille glyph. \
                      The bijection maps each of the 64 Braille patterns to a unique \
                      6-bit value.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/u/Glyph"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/u/length",
            label: "length",
            comment: "The number of Braille glyphs in an address string.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/u/Address"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
    ]
}
