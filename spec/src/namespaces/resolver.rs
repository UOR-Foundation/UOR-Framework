//! `resolver/` namespace — Type resolution strategies.
//!
//! Resolvers implement the map Π : T_n → Part(R_n), transforming a type
//! declaration into a partition of the ring. Resolution requests come from
//! user-space; execution occurs in the kernel.
//!
//! Amendment 11 adds iterative resolution: resolution state tracking,
//! refinement suggestions, and convergence metrics for the resolution-as-learning
//! loop.
//!
//! Amendment 25 adds `CompletenessResolver` — a specialised resolver that drives
//! the completeness certification loop for `CompletenessCandidate` instances.
//!
//! Amendment 26 adds `QuantumLevelResolver` — a resolver parameterised by quantum
//! level, allowing the same strategy to run at Q0, Q1, or any future level.
//!
//! Amendment 27 adds `SessionResolver` — a resolver that maintains a
//! `BindingAccumulator` across multiple `RelationQuery` evaluations for
//! multi-turn Prism deployments.
//!
//! **Space classification:** `bridge` — user-requested, kernel-executed.

use crate::model::iris::*;
use crate::model::{Class, Individual, Namespace, NamespaceModule, Property, PropertyKind, Space};

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
            imports: &[NS_SCHEMA, NS_QUERY, NS_PARTITION, NS_TYPE],
        },
        classes: classes(),
        properties: properties(),
        individuals: individuals(),
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
            disjoint_with: &[
                "https://uor.foundation/resolver/ResolutionState",
                "https://uor.foundation/resolver/RefinementSuggestion",
            ],
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
        // Amendment 11: Iterative Resolution classes
        Class {
            id: "https://uor.foundation/resolver/ResolutionState",
            label: "ResolutionState",
            comment: "The current state of an iterative resolution: tracks how many \
                      iterations have been performed, whether the resolution is \
                      complete, and the current fiber deficit.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/resolver/Resolver",
                "https://uor.foundation/resolver/RefinementSuggestion",
            ],
        },
        Class {
            id: "https://uor.foundation/resolver/RefinementSuggestion",
            label: "RefinementSuggestion",
            comment: "A suggestion from the resolver for how to refine an incomplete \
                      resolution: which metric axis to explore, which class to narrow \
                      to, and which fibers to target.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/resolver/Resolver",
                "https://uor.foundation/resolver/ResolutionState",
            ],
        },
        // Amendment 23: Typed controlled vocabulary class
        Class {
            id: "https://uor.foundation/resolver/ComplexityClass",
            label: "ComplexityClass",
            comment: "A computational complexity classification for resolvers. \
                      Each resolver's asymptotic runtime is typed as a named \
                      ComplexityClass individual rather than a free string.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 18: Constraint Nerve
        Class {
            id: "https://uor.foundation/resolver/ConstraintNerve",
            label: "ConstraintNerve",
            comment: "The simplicial complex whose vertices are constraints and \
                      where a k-simplex exists iff the corresponding k+1 constraints \
                      have nonempty intersection. The nerve's topology governs \
                      resolution convergence: trivial homology ↔ smooth convergence, \
                      non-trivial homology ↔ potential stalls.",
            subclass_of: &[
                OWL_THING,
                "https://uor.foundation/homology/SimplicialComplex",
            ],
            disjoint_with: &[],
        },
        // Amendment 25: Completeness Certification Pathway
        Class {
            id: "https://uor.foundation/resolver/CompletenessResolver",
            label: "CompletenessResolver",
            comment: "A specialisation of Resolver driving the completeness certification \
                      loop. Accepts a CompletenessCandidate, runs the ψ-pipeline (reading \
                      nerveEulerCharacteristic from ResolutionState), and either issues a \
                      CompletenessCertificate or produces a RefinementSuggestion.",
            subclass_of: &["https://uor.foundation/resolver/Resolver"],
            disjoint_with: &[],
        },
        // Amendment 26: Quantum Level Scaling
        Class {
            id: "https://uor.foundation/resolver/QuantumLevelResolver",
            label: "QuantumLevelResolver",
            comment: "A Resolver parameterised by quantum level. The same resolver \
                      strategy runs at any quantum level n ≥ 1 by substituting the \
                      appropriate R_n ring.",
            subclass_of: &["https://uor.foundation/resolver/Resolver"],
            disjoint_with: &[],
        },
        // Amendment 27: Session-Scoped Resolution
        Class {
            id: "https://uor.foundation/resolver/SessionResolver",
            label: "SessionResolver",
            comment: "A Resolver that maintains a BindingAccumulator across multiple \
                      RelationQuery evaluations. The top-level resolver for multi-turn \
                      Prism deployments.",
            subclass_of: &["https://uor.foundation/resolver/Resolver"],
            disjoint_with: &[],
        },
        // Amendment 28: ψ-Pipeline Inversion (Type Synthesis)
        Class {
            id: "https://uor.foundation/resolver/TypeSynthesisResolver",
            label: "TypeSynthesisResolver",
            comment: "A Resolver that runs the ψ-pipeline in inverse mode. Accepts a \
                      TypeSynthesisGoal and returns a TypeSynthesisResult. Internally \
                      maintains a ConstraintSearchState tracking which constraint \
                      combinations have been explored and which Betti profiles they realise.",
            subclass_of: &["https://uor.foundation/resolver/Resolver"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/resolver/ConstraintSearchState",
            label: "ConstraintSearchState",
            comment: "Internal resolver state tracking the boundary of explored constraint \
                      combinations during synthesis. Carries exploredCount, currentCandidate, \
                      and a link to the best SynthesisSignature achieved so far.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 29: Quantum Level Spectral Sequence
        Class {
            id: "https://uor.foundation/resolver/IncrementalCompletenessResolver",
            label: "IncrementalCompletenessResolver",
            comment: "A Resolver that determines whether a CompleteType T at Q_n lifts to a \
                      CompleteType at Q_{n+1} without re-running the full ψ-pipeline from \
                      scratch. It computes the SpectralSequencePage sequence, reads the \
                      LiftObstruction, and either confirms the lift or returns a \
                      LiftRefinementSuggestion.",
            subclass_of: &["https://uor.foundation/resolver/Resolver"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/resolver/LiftRefinementSuggestion",
            label: "LiftRefinementSuggestion",
            comment: "A RefinementSuggestion produced when a QuantumLift has a non-trivial \
                      LiftObstruction. Specialises RefinementSuggestion with liftFiberPosition \
                      (the new bit position n+1) and obstructionClass.",
            subclass_of: &["https://uor.foundation/resolver/RefinementSuggestion"],
            disjoint_with: &[],
        },
        // Amendment 30: Monodromy Observables
        Class {
            id: "https://uor.foundation/resolver/MonodromyResolver",
            label: "MonodromyResolver",
            comment: "A Resolver that computes the HolonomyGroup of a ConstrainedType by \
                      enumerating closed paths in the constraint nerve and accumulating \
                      DihedralElement values. Outputs a MonodromyClass and classifies the \
                      type as FlatType or TwistedType.",
            subclass_of: &["https://uor.foundation/resolver/Resolver"],
            disjoint_with: &[],
        },
        // Amendment 31: Jacobian-guided resolver (DC_10)
        Class {
            id: "https://uor.foundation/resolver/JacobianGuidedResolver",
            label: "JacobianGuidedResolver",
            comment: "A resolver that uses the Jacobian matrix to guide \
                      constraint selection, implementing DC_10: select the \
                      constraint that maximises total curvature reduction.",
            subclass_of: &["https://uor.foundation/resolver/Resolver"],
            disjoint_with: &[],
        },
        // Amendment 32: Superposition resolver (RC_5)
        Class {
            id: "https://uor.foundation/resolver/SuperpositionResolver",
            label: "SuperpositionResolver",
            comment: "A resolver that handles superposed fiber states, computing \
                      amplitudes and determining when superposition collapses to \
                      a classical fiber assignment (Amendment 32).",
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
            kind: PropertyKind::Annotation,
            functional: true,
            domain: Some("https://uor.foundation/resolver/Resolver"),
            range: XSD_STRING,
        },
        // complexity property removed by Amendment 23 (replaced by hasComplexityClass)
        // Amendment 11: Iterative Resolution properties
        Property {
            id: "https://uor.foundation/resolver/resolutionState",
            label: "resolutionState",
            comment: "The current resolution state of this resolver.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/Resolver"),
            range: "https://uor.foundation/resolver/ResolutionState",
        },
        Property {
            id: "https://uor.foundation/resolver/isComplete",
            label: "isComplete",
            comment: "Whether this resolution is complete: all fibers are pinned \
                      and the partition is fully determined.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ResolutionState"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/resolver/iterationCount",
            label: "iterationCount",
            comment: "The number of refinement iterations performed so far.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ResolutionState"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/resolver/fiberDeficit",
            label: "fiberDeficit",
            comment: "The fiber budget showing the remaining unpinned fibers. \
                      When all fibers are pinned, the deficit is zero and \
                      resolution is complete.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ResolutionState"),
            range: "https://uor.foundation/partition/FiberBudget",
        },
        Property {
            id: "https://uor.foundation/resolver/suggestion",
            label: "suggestion",
            comment: "A refinement suggestion for advancing this resolution.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/resolver/ResolutionState"),
            range: "https://uor.foundation/resolver/RefinementSuggestion",
        },
        Property {
            id: "https://uor.foundation/resolver/suggestedAxis",
            label: "suggestedAxis",
            comment: "The metric axis this suggestion recommends exploring.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/RefinementSuggestion"),
            range: "https://uor.foundation/type/MetricAxis",
        },
        Property {
            id: "https://uor.foundation/resolver/suggestedClass",
            label: "suggestedClass",
            comment: "The constraint class this suggestion recommends applying.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/RefinementSuggestion"),
            range: OWL_CLASS,
        },
        Property {
            id: "https://uor.foundation/resolver/targetFibers",
            label: "targetFibers",
            comment: "The fiber coordinates this suggestion targets for pinning.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/resolver/RefinementSuggestion"),
            range: "https://uor.foundation/partition/FiberCoordinate",
        },
        Property {
            id: "https://uor.foundation/resolver/convergenceRate",
            label: "convergenceRate",
            comment: "The rate at which fibers are being pinned per iteration. \
                      A higher rate indicates faster convergence toward a \
                      complete resolution.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ResolutionState"),
            range: XSD_DECIMAL,
        },
        // Amendment 23: Typed controlled vocabulary property
        Property {
            id: "https://uor.foundation/resolver/hasComplexityClass",
            label: "hasComplexityClass",
            comment: "The computational complexity class of this resolver. \
                      Replaces the string-valued resolver:complexity property.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/Resolver"),
            range: "https://uor.foundation/resolver/ComplexityClass",
        },
        // Amendment 18: Analytical resolver properties
        Property {
            id: "https://uor.foundation/resolver/constraintNerve",
            label: "constraintNerve",
            comment: "The constraint nerve associated with this resolution state.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ResolutionState"),
            range: "https://uor.foundation/resolver/ConstraintNerve",
        },
        Property {
            id: "https://uor.foundation/resolver/residualEntropy",
            label: "residualEntropy",
            comment: "The residual Shannon entropy of the resolution state: \
                      S = freeCount × ln 2. Measures remaining uncertainty.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ResolutionState"),
            range: XSD_DECIMAL,
        },
        Property {
            id: "https://uor.foundation/resolver/topologicallyComplete",
            label: "topologicallyComplete",
            comment: "Whether all Betti numbers of the constraint nerve are zero, \
                      indicating no topological obstructions to resolution.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ResolutionState"),
            range: XSD_BOOLEAN,
        },
        // Gap B: Euler characteristic cache
        Property {
            id: "https://uor.foundation/resolver/nerveEulerCharacteristic",
            label: "nerveEulerCharacteristic",
            comment: "The Euler characteristic χ(N(C)) of the active constraint nerve at this \
                      resolution state. IT_7d requires this value to equal n (the quantum level) \
                      for resolution to be complete. Cached here to avoid recomputing the full \
                      ψ pipeline on each iteration check.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ResolutionState"),
            range: XSD_INTEGER,
        },
        // Amendment 25: Completeness Certification Pathway property
        Property {
            id: "https://uor.foundation/resolver/completenessTarget",
            label: "completenessTarget",
            comment: "The CompletenessCandidate this resolver is certifying.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/CompletenessResolver"),
            range: "https://uor.foundation/type/CompletenessCandidate",
        },
        // Amendment 26: Quantum Level Scaling property
        Property {
            id: "https://uor.foundation/resolver/quantumLevel",
            label: "quantumLevel",
            comment: "The quantum level this resolver instance is configured for.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/QuantumLevelResolver"),
            range: "https://uor.foundation/schema/QuantumLevel",
        },
        // Amendment 27: Session-Scoped Resolution property
        Property {
            id: "https://uor.foundation/resolver/sessionAccumulator",
            label: "sessionAccumulator",
            comment: "The BindingAccumulator this session resolver maintains across \
                      multiple RelationQuery evaluations.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/SessionResolver"),
            range: "https://uor.foundation/state/BindingAccumulator",
        },
        // Amendment 28: TypeSynthesisResolver and ConstraintSearchState properties
        Property {
            id: "https://uor.foundation/resolver/synthesisGoal",
            label: "synthesisGoal",
            comment: "The goal this type synthesis resolver is working to achieve.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/TypeSynthesisResolver"),
            range: "https://uor.foundation/type/TypeSynthesisGoal",
        },
        Property {
            id: "https://uor.foundation/resolver/exploredCount",
            label: "exploredCount",
            comment: "Number of constraint combinations evaluated so far during synthesis.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ConstraintSearchState"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/resolver/currentCandidate",
            label: "currentCandidate",
            comment: "The type candidate currently being evaluated during synthesis.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ConstraintSearchState"),
            range: "https://uor.foundation/type/ConstrainedType",
        },
        // Amendment 29: IncrementalCompletenessResolver and LiftRefinementSuggestion properties
        Property {
            id: "https://uor.foundation/resolver/liftTarget",
            label: "liftTarget",
            comment: "The QuantumLift this incremental completeness resolver is evaluating.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/IncrementalCompletenessResolver"),
            range: "https://uor.foundation/type/QuantumLift",
        },
        Property {
            id: "https://uor.foundation/resolver/liftFiberPosition",
            label: "liftFiberPosition",
            comment: "The new fiber position at Q_{n+1} that the lift refinement suggestion \
                      targets.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/LiftRefinementSuggestion"),
            range: "https://uor.foundation/partition/FiberCoordinate",
        },
        Property {
            id: "https://uor.foundation/resolver/obstructionClass",
            label: "obstructionClass",
            comment: "The obstruction class this lift refinement suggestion is designed to kill.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/LiftRefinementSuggestion"),
            range: "https://uor.foundation/observable/LiftObstructionClass",
        },
        // Amendment 30: MonodromyResolver properties
        Property {
            id: "https://uor.foundation/resolver/monodromyTarget",
            label: "monodromyTarget",
            comment: "The type whose holonomy this monodromy resolver is computing.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/MonodromyResolver"),
            range: "https://uor.foundation/type/ConstrainedType",
        },
        Property {
            id: "https://uor.foundation/resolver/holonomyResult",
            label: "holonomyResult",
            comment: "The HolonomyGroup produced by this monodromy resolver run.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/MonodromyResolver"),
            range: "https://uor.foundation/observable/HolonomyGroup",
        },
        // Amendment 31: Jacobian-guided resolver property (DC_10)
        Property {
            id: "https://uor.foundation/resolver/guidingJacobian",
            label: "guidingJacobian",
            comment: "The Jacobian observable guiding constraint selection \
                      at this resolution state (DC_10).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/resolver/ResolutionState"),
            range: "https://uor.foundation/observable/Jacobian",
        },
    ]
}

// Amendment 23: Typed controlled vocabulary individuals
fn individuals() -> Vec<Individual> {
    vec![
        Individual {
            id: "https://uor.foundation/resolver/ConstantTime",
            type_: "https://uor.foundation/resolver/ComplexityClass",
            label: "ConstantTime",
            comment: "O(1) complexity — the resolver runs in constant time \
                      regardless of ring size.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/resolver/LogarithmicTime",
            type_: "https://uor.foundation/resolver/ComplexityClass",
            label: "LogarithmicTime",
            comment: "O(log n) complexity — the resolver runs in logarithmic time \
                      in the quantum level.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/resolver/LinearTime",
            type_: "https://uor.foundation/resolver/ComplexityClass",
            label: "LinearTime",
            comment: "O(n) complexity — the resolver runs in time linear in the \
                      quantum level.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/resolver/ExponentialTime",
            type_: "https://uor.foundation/resolver/ComplexityClass",
            label: "ExponentialTime",
            comment: "O(2^n) complexity — the resolver runs in time exponential \
                      in the quantum level.",
            properties: &[],
        },
    ]
}
