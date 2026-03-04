//! `query/` namespace — Information extraction queries.
//!
//! Queries are the user-initiated requests for information from the UOR kernel.
//! They are kernel-executed: the user initiates a query, the kernel resolves it.
//!
//! **Space classification:** `bridge` — user-initiated, kernel-executed.

use crate::model::iris::*;
use crate::model::{Class, Individual, Namespace, NamespaceModule, Property, PropertyKind, Space};

/// Returns the `query/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "query",
            iri: NS_QUERY,
            label: "UOR Queries",
            comment: "Information extraction queries. Users initiate queries; \
                      the kernel resolves them against the ring substrate.",
            space: Space::Bridge,
            imports: &[NS_SCHEMA, NS_U],
        },
        classes: classes(),
        properties: properties(),
        individuals: individuals(),
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/query/Query",
            label: "Query",
            comment: "A request for information from the UOR kernel. The root \
                      abstraction for all query types.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/query/CoordinateQuery",
            label: "CoordinateQuery",
            comment: "A query for the ring-coordinate position of a datum: its \
                      stratum, spectrum, and address within the ring geometry.",
            subclass_of: &["https://uor.foundation/query/Query"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/query/MetricQuery",
            label: "MetricQuery",
            comment: "A query for a metric value between two datums: ring distance, \
                      Hamming distance, or their divergence (curvature).",
            subclass_of: &["https://uor.foundation/query/Query"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/query/RepresentationQuery",
            label: "RepresentationQuery",
            comment: "A query for the canonical representation of a datum or term: \
                      its normal form under the active resolver strategy.",
            subclass_of: &["https://uor.foundation/query/Query"],
            disjoint_with: &[],
        },
        // Amendment 23: Typed controlled vocabulary class
        Class {
            id: "https://uor.foundation/query/CoordinateKind",
            label: "CoordinateKind",
            comment: "A classification of coordinate types that a CoordinateQuery \
                      can extract. Each CoordinateKind individual names a specific \
                      coordinate system (stratum, spectrum, address) replacing the \
                      string-valued query:coordinate property.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/query/inputType",
            label: "inputType",
            comment: "The type of input the query accepts.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/query/Query"),
            range: OWL_CLASS,
        },
        Property {
            id: "https://uor.foundation/query/outputType",
            label: "outputType",
            comment: "The type of output the query produces.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/query/Query"),
            range: OWL_CLASS,
        },
        // query:coordinate property removed by Amendment 23 (replaced by hasCoordinateKind)
        // Amendment 23: Typed controlled vocabulary property
        Property {
            id: "https://uor.foundation/query/hasCoordinateKind",
            label: "hasCoordinateKind",
            comment: "The typed coordinate kind this query extracts. Replaces \
                      the string-valued query:coordinate property with a typed \
                      reference to a CoordinateKind individual.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/query/CoordinateQuery"),
            range: "https://uor.foundation/query/CoordinateKind",
        },
    ]
}

// Amendment 23: Typed controlled vocabulary individuals
fn individuals() -> Vec<Individual> {
    vec![
        Individual {
            id: "https://uor.foundation/query/StratumCoordinate",
            type_: "https://uor.foundation/query/CoordinateKind",
            label: "StratumCoordinate",
            comment: "The stratum coordinate: the layer position of a datum \
                      within the ring's stratification.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/query/SpectrumCoordinate",
            type_: "https://uor.foundation/query/CoordinateKind",
            label: "SpectrumCoordinate",
            comment: "The spectrum coordinate: the spectral decomposition of a \
                      datum under the ring's Fourier analysis.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/query/AddressCoordinate",
            type_: "https://uor.foundation/query/CoordinateKind",
            label: "AddressCoordinate",
            comment: "The address coordinate: the content-addressable position \
                      of a datum in the Braille glyph encoding.",
            properties: &[],
        },
    ]
}
