//! `observable/` namespace — Observable quantities and metrics.
//!
//! Observables are kernel-computed measurements of UOR objects. They form a
//! rich taxonomy covering ring geometry, Hamming geometry, path-dependent
//! quantities, and catastrophe-theoretic measurements.
//!
//! **Space classification:** `bridge` — kernel-computed, user-requested.

use crate::model::iris::*;
use crate::model::{Class, Individual, Namespace, NamespaceModule, Property, PropertyKind, Space};

/// Returns the `observable/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "observable",
            iri: NS_OBSERVABLE,
            label: "UOR Observables",
            comment: "Observable quantities and metrics computed by the UOR kernel. \
                      Includes ring-metric, Hamming-metric, curvature, holonomy, \
                      and catastrophe-theoretic observables.",
            space: Space::Bridge,
            imports: &[NS_SCHEMA, NS_PARTITION],
        },
        classes: classes(),
        properties: properties(),
        individuals: individuals(),
    }
}

fn classes() -> Vec<Class> {
    vec![
        // Root
        Class {
            id: "https://uor.foundation/observable/Observable",
            label: "Observable",
            comment: "A measurable quantity in the UOR Framework. All observables \
                      are kernel-computed and user-consumed.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Observable categories
        Class {
            id: "https://uor.foundation/observable/StratumObservable",
            label: "StratumObservable",
            comment: "An observable measuring stratum-level properties: position \
                      within the ring's layer structure.",
            subclass_of: &["https://uor.foundation/observable/Observable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/MetricObservable",
            label: "MetricObservable",
            comment: "An observable measuring geometric distance between ring elements \
                      under a specific metric.",
            subclass_of: &["https://uor.foundation/observable/Observable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/PathObservable",
            label: "PathObservable",
            comment: "An observable measuring properties of paths through the ring: \
                      path length, total variation, winding number.",
            subclass_of: &["https://uor.foundation/observable/Observable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/CascadeObservable",
            label: "CascadeObservable",
            comment: "An observable measuring cascade properties: the length and \
                      count of operation sequences.",
            subclass_of: &["https://uor.foundation/observable/Observable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/CatastropheObservable",
            label: "CatastropheObservable",
            comment: "An observable measuring catastrophe-theoretic properties: \
                      thresholds at which qualitative changes occur in the partition.",
            subclass_of: &["https://uor.foundation/observable/Observable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/CurvatureObservable",
            label: "CurvatureObservable",
            comment: "An observable measuring the curvature of the UOR geometry: \
                      the gap between ring-isometry and Hamming-isometry for a \
                      given transform.",
            subclass_of: &["https://uor.foundation/observable/Observable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/HolonomyObservable",
            label: "HolonomyObservable",
            comment: "An observable measuring holonomy: the accumulated transformation \
                      when traversing a closed path in the ring.",
            subclass_of: &["https://uor.foundation/observable/Observable"],
            disjoint_with: &[],
        },
        // Metric subclasses
        Class {
            id: "https://uor.foundation/observable/RingMetric",
            label: "RingMetric",
            comment: "Distance between two ring elements under the ring metric: \
                      d_R(x, y) = |x - y| mod 2^n.",
            subclass_of: &["https://uor.foundation/observable/MetricObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/HammingMetric",
            label: "HammingMetric",
            comment: "Distance between two ring elements under the Hamming metric: \
                      the number of bit positions where they differ.",
            subclass_of: &["https://uor.foundation/observable/MetricObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/IncompatibilityMetric",
            label: "IncompatibilityMetric",
            comment: "The metric incompatibility between two ring elements: the \
                      divergence between their ring-metric and Hamming-metric \
                      distances, measuring geometric curvature.",
            subclass_of: &["https://uor.foundation/observable/MetricObservable"],
            disjoint_with: &[],
        },
        // Measurement result types
        Class {
            id: "https://uor.foundation/observable/StratumValue",
            label: "StratumValue",
            comment: "The stratum index of a ring element.",
            subclass_of: &["https://uor.foundation/observable/StratumObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/StratumDelta",
            label: "StratumDelta",
            comment: "The difference in stratum between two ring elements.",
            subclass_of: &["https://uor.foundation/observable/StratumObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/StratumTrajectory",
            label: "StratumTrajectory",
            comment: "The sequence of strata traversed by a path through the ring.",
            subclass_of: &["https://uor.foundation/observable/StratumObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/PathLength",
            label: "PathLength",
            comment: "The length of a path through the ring, measured in operation steps.",
            subclass_of: &["https://uor.foundation/observable/PathObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/TotalVariation",
            label: "TotalVariation",
            comment: "The total variation of a path: the sum of metric distances \
                      between consecutive elements.",
            subclass_of: &["https://uor.foundation/observable/PathObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/WindingNumber",
            label: "WindingNumber",
            comment: "The winding number of a closed path: the number of times \
                      the path wraps around the ring.",
            subclass_of: &["https://uor.foundation/observable/PathObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/CascadeLength",
            label: "CascadeLength",
            comment: "The number of operation applications in an operation cascade.",
            subclass_of: &["https://uor.foundation/observable/CascadeObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/CascadeCount",
            label: "CascadeCount",
            comment: "The number of distinct cascades in a computation.",
            subclass_of: &["https://uor.foundation/observable/CascadeObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/CatastropheThreshold",
            label: "CatastropheThreshold",
            comment: "A critical value at which a qualitative change occurs in \
                      the partition structure.",
            subclass_of: &["https://uor.foundation/observable/CatastropheObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/CatastropheCount",
            label: "CatastropheCount",
            comment: "The number of catastrophe events (qualitative partition changes) \
                      in a computation.",
            subclass_of: &["https://uor.foundation/observable/CatastropheObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/Commutator",
            label: "Commutator",
            comment: "The commutator [f, g](x) = f(g(x)) - g(f(x)) of two operations, \
                      measuring their non-commutativity.",
            subclass_of: &["https://uor.foundation/observable/CurvatureObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/CurvatureFlux",
            label: "CurvatureFlux",
            comment: "The integrated curvature over a region of type space: the \
                      total metric incompatibility accumulated.",
            subclass_of: &["https://uor.foundation/observable/CurvatureObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/Monodromy",
            label: "Monodromy",
            comment: "The monodromy of a closed path: the net transformation \
                      accumulated when traversing a loop in the type space.",
            subclass_of: &["https://uor.foundation/observable/HolonomyObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/ParallelTransport",
            label: "ParallelTransport",
            comment: "The parallel transport of a vector along a path: the canonical \
                      lift of the path to the tangent bundle of the ring.",
            subclass_of: &["https://uor.foundation/observable/HolonomyObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/DihedralElement",
            label: "DihedralElement",
            comment: "An element of the dihedral group D_{2^n} acting on the type \
                      space. Each dihedral element induces an isometry of 𝒯_n.",
            subclass_of: &["https://uor.foundation/observable/HolonomyObservable"],
            disjoint_with: &[],
        },
        // Amendment 23: Typed controlled vocabulary class
        Class {
            id: "https://uor.foundation/observable/MeasurementUnit",
            label: "MeasurementUnit",
            comment: "A unit of measurement for observable quantities. Each \
                      MeasurementUnit individual names a specific unit (bits, \
                      ring steps, dimensionless) replacing the string-valued \
                      observable:unit property.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 18: Analytical Observables
        Class {
            id: "https://uor.foundation/observable/Jacobian",
            label: "Jacobian",
            comment: "Fiber-by-fiber curvature decomposition. J_k measures the \
                      discrete derivative of the incompatibility metric at fiber \
                      position k: J_k = |d_R(x, succ(x)) - d_H(x, succ(x))| \
                      restricted to position k.",
            subclass_of: &["https://uor.foundation/observable/CurvatureObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/TopologicalObservable",
            label: "TopologicalObservable",
            comment: "An observable measuring a topological invariant of the \
                      resolution space. Topological observables are invariant \
                      under continuous deformations of the constraint configuration.",
            subclass_of: &["https://uor.foundation/observable/Observable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/BettiNumber",
            label: "BettiNumber",
            comment: "The rank of a homology group of the constraint nerve. \
                      β_k = rank(H_k(N(C))) counts the k-dimensional holes \
                      in the constraint configuration.",
            subclass_of: &["https://uor.foundation/observable/TopologicalObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/SpectralGap",
            label: "SpectralGap",
            comment: "The smallest positive eigenvalue of the constraint nerve \
                      Laplacian. Controls the convergence rate of iterative \
                      resolution: larger gap = faster convergence.",
            subclass_of: &["https://uor.foundation/observable/TopologicalObservable"],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/observable/value",
            label: "value",
            comment: "The numeric value of an observable measurement.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/Observable"),
            range: XSD_DECIMAL,
        },
        // observable:unit property removed by Amendment 23 (replaced by hasUnit)
        Property {
            id: "https://uor.foundation/observable/source",
            label: "source",
            comment: "The source object of this measurement (datum, partition, \
                      or path start point).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/observable/Observable"),
            range: OWL_THING,
        },
        Property {
            id: "https://uor.foundation/observable/target",
            label: "target",
            comment: "The target object of this measurement (for metrics and \
                      path-end measurements).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/observable/Observable"),
            range: OWL_THING,
        },
        // Amendment 23: Typed controlled vocabulary property
        Property {
            id: "https://uor.foundation/observable/hasUnit",
            label: "hasUnit",
            comment: "The measurement unit of this observable. Replaces the \
                      string-valued observable:unit property with a typed \
                      reference to a MeasurementUnit individual.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/observable/Observable"),
            range: "https://uor.foundation/observable/MeasurementUnit",
        },
        // Amendment 18: Analytical Observable properties
        Property {
            id: "https://uor.foundation/observable/fiberPosition",
            label: "fiberPosition",
            comment: "The fiber position k at which this Jacobian entry is measured.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/Jacobian"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/observable/derivativeValue",
            label: "derivativeValue",
            comment: "The discrete derivative value at this fiber position.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/Jacobian"),
            range: XSD_DECIMAL,
        },
        Property {
            id: "https://uor.foundation/observable/dimension",
            label: "dimension",
            comment: "The dimension k of the topological observable (e.g., the \
                      degree of the Betti number or the dimension of the spectral gap).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/TopologicalObservable"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
    ]
}

// Amendment 23: Typed controlled vocabulary individuals
fn individuals() -> Vec<Individual> {
    vec![
        Individual {
            id: "https://uor.foundation/observable/Bits",
            type_: "https://uor.foundation/observable/MeasurementUnit",
            label: "Bits",
            comment: "Information-theoretic unit: the measurement is in bits \
                      (e.g., Hamming weight, entropy).",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/observable/RingSteps",
            type_: "https://uor.foundation/observable/MeasurementUnit",
            label: "RingSteps",
            comment: "Ring-arithmetic unit: the measurement is in ring distance \
                      steps (|x - y| mod 2^n).",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/observable/Dimensionless",
            type_: "https://uor.foundation/observable/MeasurementUnit",
            label: "Dimensionless",
            comment: "Dimensionless unit: the measurement is a pure number \
                      (e.g., winding number, Betti number, spectral gap).",
            properties: &[],
        },
    ]
}
