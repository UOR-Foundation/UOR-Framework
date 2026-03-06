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
            imports: &[NS_OP, NS_SCHEMA, NS_PARTITION, NS_TYPE],
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
        // Gap E: Thermodynamic Observable branch
        Class {
            id: "https://uor.foundation/observable/ThermoObservable",
            label: "ThermoObservable",
            comment: "An observable measuring thermodynamic properties of the resolution process: \
                      residual entropy, Landauer cost, and cascade distribution statistics.",
            subclass_of: &["https://uor.foundation/observable/Observable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/ResidualEntropy",
            label: "ResidualEntropy",
            comment: "S_residual: the residual Shannon entropy of the fiber distribution after \
                      partial resolution. Computed as S = (Σ κ_k − χ(N(C))) × ln 2 (IT_7b). \
                      Unit: Nats.",
            subclass_of: &["https://uor.foundation/observable/ThermoObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/LandauerCost",
            label: "LandauerCost",
            comment: "The minimum thermodynamic cost (in units of k_B T ln 2) of erasing one \
                      bit of fiber uncertainty. The UOR ring operates at β* = ln 2 — the \
                      Landauer temperature.",
            subclass_of: &["https://uor.foundation/observable/ThermoObservable"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/CascadeEntropy",
            label: "CascadeEntropy",
            comment: "The Shannon entropy of the cascade distribution P(j) = 2^{−j}. At the \
                      Landauer temperature, this equals ln 2 per cascade step — each step erases \
                      exactly one bit of fiber uncertainty.",
            subclass_of: &["https://uor.foundation/observable/ThermoObservable"],
            disjoint_with: &[],
        },
        // Amendment 28: Type synthesis signature
        Class {
            id: "https://uor.foundation/observable/SynthesisSignature",
            label: "SynthesisSignature",
            comment: "A named topological signature: a pair (realised Euler characteristic, \
                      realised Betti profile). Linked from TypeSynthesisResult. Allows \
                      comparison between the goal signature and the actually achieved signature.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 29: Quantum level spectral sequence
        Class {
            id: "https://uor.foundation/observable/SpectralSequencePage",
            label: "SpectralSequencePage",
            comment:
                "A single page E_r of the quantum level spectral sequence. Carries the \
                      page index r and the differential d_r. The sequence converges when all \
                      differentials vanish — typically by E_3 for simple constraint configurations.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/LiftObstructionClass",
            label: "LiftObstructionClass",
            comment: "The cohomology class in H^2(N(C(T))) representing the LiftObstruction \
                      for a specific QuantumLift. The class is zero iff the obstruction is \
                      trivial. When non-zero, it indexes the specific fiber pair at Q_{n+1} \
                      that cannot be closed by the lifted constraint set alone.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 30: Monodromy observable machinery
        Class {
            id: "https://uor.foundation/observable/MonodromyClass",
            label: "MonodromyClass",
            comment: "A classification of a type's holonomy: the subgroup of D_{2^n} generated \
                      by all Monodromy observables computed over closed paths in the type's \
                      constraint nerve. Trivial iff every closed constraint path returns to its \
                      starting fiber assignment without net dihedral transformation.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/HolonomyGroup",
            label: "HolonomyGroup",
            comment:
                "The holonomy group of a ConstrainedType: the group of all Monodromy \
                      elements achievable by closed paths in the constraint nerve. Always a \
                      subgroup of D_{2^n}. Trivial iff the type has trivial monodromy everywhere; \
                      equals D_{2^n} iff paths involving both neg and bnot involutions are present.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/observable/ClosedConstraintPath",
            label: "ClosedConstraintPath",
            comment: "A sequence of constraint applications forming a closed loop in the \
                      constraint nerve — beginning and ending at the same fiber assignment. \
                      The Monodromy of the loop is the net DihedralElement accumulated \
                      when traversing it.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 31: PhaseBoundaryType enum class
        Class {
            id: "https://uor.foundation/observable/PhaseBoundaryType",
            label: "PhaseBoundaryType",
            comment: "A classification of phase boundary in the catastrophe \
                      diagram: period boundary (g divides 2^n − 1) or \
                      power-of-two boundary (g = 2^k).",
            subclass_of: &[OWL_THING],
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
        // Amendment 28: SynthesisSignature properties
        Property {
            id: "https://uor.foundation/observable/realisedEuler",
            label: "realisedEuler",
            comment: "The Euler characteristic actually achieved by this synthesis signature.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/SynthesisSignature"),
            range: XSD_INTEGER,
        },
        Property {
            id: "https://uor.foundation/observable/realisedBetti",
            label: "realisedBetti",
            comment: "Non-functional. Realised Betti number values, one assertion per \
                      homological degree.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/observable/SynthesisSignature"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // Amendment 29: SpectralSequencePage properties
        Property {
            id: "https://uor.foundation/observable/pageIndex",
            label: "pageIndex",
            comment: "The page r of this spectral sequence page. r=1 is the initial page; \
                      convergence is declared when all d_r are zero.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/SpectralSequencePage"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/observable/differentialIsZero",
            label: "differentialIsZero",
            comment: "True iff d_r = 0 on this page — no further corrections to the \
                      lifted homology.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/SpectralSequencePage"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/observable/convergedAt",
            label: "convergedAt",
            comment: "The page index r at which the spectral sequence converged \
                      (all subsequent differentials zero).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/SpectralSequencePage"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // Amendment 29: LiftObstructionClass property
        Property {
            id: "https://uor.foundation/observable/obstructionClass",
            label: "obstructionClass",
            comment: "The cohomology class in H^2(N(C(T))) representing this obstruction.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/observable/LiftObstructionClass"),
            range: "https://uor.foundation/cohomology/CohomologyGroup",
        },
        // Amendment 30: Monodromy extensions
        Property {
            id: "https://uor.foundation/observable/monodromyLoop",
            label: "monodromyLoop",
            comment: "The closed path that generates this monodromy value.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/observable/Monodromy"),
            range: "https://uor.foundation/observable/ClosedConstraintPath",
        },
        Property {
            id: "https://uor.foundation/observable/monodromyElement",
            label: "monodromyElement",
            comment: "The dihedral element g in D_{2^n} accumulated when traversing the loop. \
                      The monodromy is trivial iff this element is the group identity.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/observable/Monodromy"),
            range: "https://uor.foundation/observable/DihedralElement",
        },
        Property {
            id: "https://uor.foundation/observable/isTrivialMonodromy",
            label: "isTrivialMonodromy",
            comment: "True iff the monodromyElement is the identity in D_{2^n}.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/Monodromy"),
            range: XSD_BOOLEAN,
        },
        // Amendment 30: HolonomyGroup properties
        Property {
            id: "https://uor.foundation/observable/holonomyGroup",
            label: "holonomyGroup",
            comment: "Non-functional. The generators of the holonomy group: one DihedralElement \
                      per generating monodromy.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/observable/HolonomyGroup"),
            range: "https://uor.foundation/observable/DihedralElement",
        },
        Property {
            id: "https://uor.foundation/observable/holonomyGroupOrder",
            label: "holonomyGroupOrder",
            comment: "The order of the holonomy group as a subgroup of D_{2^n}. \
                      For a FlatType: 1. For full dihedral holonomy: 2^{n+1}.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/HolonomyGroup"),
            range: XSD_POSITIVE_INTEGER,
        },
        // Amendment 30: ClosedConstraintPath properties
        Property {
            id: "https://uor.foundation/observable/pathLength",
            label: "pathLength",
            comment: "The number of constraint application steps in this closed path.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/ClosedConstraintPath"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/observable/pathConstraints",
            label: "pathConstraints",
            comment: "Non-functional. The ordered sequence of constraints traversed. \
                      One assertion per step.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/observable/ClosedConstraintPath"),
            range: "https://uor.foundation/type/Constraint",
        },
        // Amendment 30: DihedralElement extensions
        Property {
            id: "https://uor.foundation/observable/dihedralElementValue",
            label: "dihedralElementValue",
            comment: "Non-functional. One assertion per generator in the normal form of the \
                      element — the sequence of neg and/or bnot operations that realises this \
                      dihedral element when composed.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/observable/DihedralElement"),
            range: "https://uor.foundation/op/Operation",
        },
        Property {
            id: "https://uor.foundation/observable/isIdentityElement",
            label: "isIdentityElement",
            comment: "True iff this element is the group identity (the trivial monodromy value).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/DihedralElement"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/observable/elementOrder",
            label: "elementOrder",
            comment: "The order of this element in D_{2^n}: the smallest k >= 1 such that \
                      g^k = id. For neg and bnot: order 2.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/DihedralElement"),
            range: XSD_POSITIVE_INTEGER,
        },
        // Amendment 31: Thermodynamic + Phase Diagram properties
        Property {
            id: "https://uor.foundation/observable/hardnessEstimate",
            label: "hardnessEstimate",
            comment: "An estimated computational hardness for a ThermoObservable, \
                      connecting thermodynamic cost to complexity (TH_9 realisation).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/ThermoObservable"),
            range: XSD_DECIMAL,
        },
        Property {
            id: "https://uor.foundation/observable/phaseN",
            label: "phaseN",
            comment: "The ring dimension coordinate n in the (n, g) catastrophe \
                      phase diagram (PD_1 n-coordinate).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/CatastropheObservable"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/observable/phaseG",
            label: "phaseG",
            comment: "The group-order coordinate g in the (n, g) catastrophe \
                      phase diagram (PD_1 g-coordinate).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/CatastropheObservable"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/observable/onResonanceLine",
            label: "onResonanceLine",
            comment: "True when g divides 2^n − 1, placing this observable on \
                      a resonance line in the phase diagram (PD_4).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/observable/CatastropheObservable"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/observable/phaseBoundaryType",
            label: "phaseBoundaryType",
            comment: "The type of phase boundary at this point in the catastrophe \
                      diagram: PeriodBoundary or PowerOfTwoBoundary (PD_2).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/observable/CatastropheObservable"),
            range: "https://uor.foundation/observable/PhaseBoundaryType",
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
        Individual {
            id: "https://uor.foundation/observable/Nats",
            type_: "https://uor.foundation/observable/MeasurementUnit",
            label: "Nats",
            comment: "Natural information unit: entropy measured in nats (using natural \
                      logarithm). S_residual is expressed in nats when computed as \
                      (Σ κ_k − χ) × ln 2.",
            properties: &[],
        },
        // Amendment 31: PhaseBoundaryType vocabulary individuals
        Individual {
            id: "https://uor.foundation/observable/PeriodBoundary",
            type_: "https://uor.foundation/observable/PhaseBoundaryType",
            label: "PeriodBoundary",
            comment: "A phase boundary where g divides 2^n − 1, meaning g is \
                      a period of the multiplicative structure of R_n.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/observable/PowerOfTwoBoundary",
            type_: "https://uor.foundation/observable/PhaseBoundaryType",
            label: "PowerOfTwoBoundary",
            comment: "A phase boundary where g = 2^k, meaning g aligns with \
                      the binary stratification of R_n.",
            properties: &[],
        },
    ]
}
