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
//! Amendment 25 adds the completeness certification pathway: `CompletenessCandidate`
//! (a ConstrainedType undergoing certification) and `CompletenessWitness` (a single
//! fiber-closing event), with four new properties supporting the certification loop.
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
        // Gap D: CompleteType
        Class {
            id: "https://uor.foundation/type/CompleteType",
            label: "CompleteType",
            comment: "A TypeDefinition certified to satisfy the UOR completeness criterion \
                      (IT_7d): its constraint nerve N(C) has Euler characteristic χ = n and \
                      all Betti numbers β_k = 0. A CompleteType guarantees that resolution \
                      closes the fiber budget in O(1) — no iterative refinement is required. \
                      Completeness is attested by a cert:CompletenessCertificate linked via \
                      cert:certifiedType.",
            subclass_of: &["https://uor.foundation/type/TypeDefinition"],
            disjoint_with: &[],
        },
        // Amendment 25: Completeness Certification Pathway
        Class {
            id: "https://uor.foundation/type/CompletenessCandidate",
            label: "CompletenessCandidate",
            comment: "A ConstrainedType actively undergoing the completeness certification \
                      pipeline. Links to the resolver:ResolutionState tracking the current \
                      iteration and to the resolver:ConstraintNerve being computed. \
                      Disjoint from CompleteType (which is already certified).",
            subclass_of: &["https://uor.foundation/type/ConstrainedType"],
            disjoint_with: &["https://uor.foundation/type/CompleteType"],
        },
        Class {
            id: "https://uor.foundation/type/CompletenessWitness",
            label: "CompletenessWitness",
            comment: "A record of a single fiber-closing event: one constraint application \
                      that reduced the FiberBudget deficit. Carries the applied constraint \
                      and the fibersClosed count. Forms the ordered audit trail between \
                      ConstrainedType and CompleteType.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 28: ψ-Pipeline Inversion (Type Synthesis)
        Class {
            id: "https://uor.foundation/type/TypeSynthesisGoal",
            label: "TypeSynthesisGoal",
            comment: "A specification of the desired topological properties of a type to be \
                      synthesised. Carries a target Euler characteristic \
                      (targetEulerCharacteristic) and a target Betti profile (zero or more \
                      targetBettiNumber assertions). The minimal goal for O(1) resolution is: \
                      targetEulerCharacteristic = n and all targetBettiNumber = 0 — the IT_7d \
                      profile.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/type/TypeSynthesisResult",
            label: "TypeSynthesisResult",
            comment: "The output of a TypeSynthesisResolver run. Contains the SynthesizedType, \
                      the realised topological signature (as a SynthesisSignature), and the \
                      SynthesisTrace recording the construction steps.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/type/SynthesizedType",
            label: "SynthesizedType",
            comment: "A ConstrainedType produced by the TypeSynthesisResolver. Distinguished \
                      from a hand-authored ConstrainedType by the presence of a \
                      type:synthesisResult link. May or may not be a CompleteType, depending \
                      on the synthesis goal.",
            subclass_of: &["https://uor.foundation/type/ConstrainedType"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/type/MinimalConstraintBasis",
            label: "MinimalConstraintBasis",
            comment: "The minimal set of constraints in the SynthesizedType's constraint set \
                      that is sufficient to realise the target topological signature. The \
                      minimality criterion is that removing any single member changes the \
                      realised signature.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 29: Quantum Level Spectral Sequence
        Class {
            id: "https://uor.foundation/type/QuantumLift",
            label: "QuantumLift",
            comment: "A ConstrainedType T' over R_{n+1} obtained by extending a \
                      ConstrainedType T over R_n. Carries a link to the base type (liftBase), \
                      the quantum level it lifts to (liftTargetLevel), and the LiftObstruction \
                      (if the lift fails to transfer completeness). A QuantumLift is a \
                      CompleteType iff its LiftObstruction is trivial.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/type/LiftObstruction",
            label: "LiftObstruction",
            comment: "The algebraic obstruction to a QuantumLift inheriting the completeness \
                      of its base type. Computed as the image of the spectral sequence \
                      differential d_2. If trivial (zero), the base type's completeness lifts. \
                      If non-trivial, at least one additional constraint is needed at the new \
                      quantum level.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 30: Monodromy Observables
        Class {
            id: "https://uor.foundation/type/TwistedType",
            label: "TwistedType",
            comment: "A ConstrainedType whose HolonomyGroup is non-trivial — at least one \
                      closed constraint path produces a non-identity dihedral element. A \
                      TwistedType may still be a CompleteType (IT_7d is a homological, not \
                      holonomic, criterion), but its resolution paths require tracking dihedral \
                      accumulation.",
            subclass_of: &["https://uor.foundation/type/ConstrainedType"],
            disjoint_with: &["https://uor.foundation/type/FlatType"],
        },
        Class {
            id: "https://uor.foundation/type/FlatType",
            label: "FlatType",
            comment: "A ConstrainedType whose HolonomyGroup is trivial — all closed constraint \
                      paths have identity monodromy. The constraint configuration is \
                      topologically flat: resolution is path-independent.",
            subclass_of: &["https://uor.foundation/type/ConstrainedType"],
            disjoint_with: &["https://uor.foundation/type/TwistedType"],
        },
        // Amendment 32: Superposed fiber state (RC_5)
        Class {
            id: "https://uor.foundation/type/SuperposedFiberState",
            label: "SuperposedFiberState",
            comment: "A type representing a superposition of fiber states where fibers \
                      carry complex amplitudes rather than binary pinned/free \
                      assignments. Ontological realisation of RC_5 (Amendment 32).",
            subclass_of: &["https://uor.foundation/type/TypeDefinition"],
            disjoint_with: &[],
        },
        // Amendment 34: Morphospace Achievability
        Class {
            id: "https://uor.foundation/type/ForbiddenSignature",
            label: "ForbiddenSignature",
            comment: "A topological signature (χ, β_k) that is formally impossible \
                      to achieve for any ConstrainedType. Witnessed by an \
                      ImpossibilityWitness in proof/.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 36: Measurement Boundary
        Class {
            id: "https://uor.foundation/type/CollapsedFiberState",
            label: "CollapsedFiberState",
            comment: "A fiber state that has undergone projective collapse from a \
                      SuperposedFiberState to a definitive classical value. \
                      Topologically equivalent to a classically pinned fiber (QM_2).",
            subclass_of: &[OWL_THING],
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
        // type:axisSignature property removed by Amendment 23 (replaced by affectsAxis + axisSignatureNote)
        // Amendment 23: Typed controlled vocabulary properties
        Property {
            id: "https://uor.foundation/type/affectsAxis",
            label: "affectsAxis",
            comment: "The metric axis that this operation affects. Replaces \
                      the string-valued type:axisSignature property with a typed \
                      reference to MetricAxis individuals. Non-functional: an \
                      operation may affect multiple axes.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/op/Operation"),
            range: "https://uor.foundation/type/MetricAxis",
        },
        Property {
            id: "https://uor.foundation/type/axisSignatureNote",
            label: "axisSignatureNote",
            comment: "Human-readable annotation of the operation's axis signature \
                      (e.g., 'V', 'H', 'VH'). Demoted from a datatype property to \
                      an annotation property by Amendment 23.",
            kind: PropertyKind::Annotation,
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
        // Amendment 25: Completeness Certification Pathway properties
        Property {
            id: "https://uor.foundation/type/completenessCandidate",
            label: "completenessCandidate",
            comment: "The ConstrainedType being evaluated for completeness \
                      by this CompletenessCandidate.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/type/CompletenessCandidate"),
            range: "https://uor.foundation/type/ConstrainedType",
        },
        Property {
            id: "https://uor.foundation/type/candidateNerve",
            label: "candidateNerve",
            comment: "The constraint nerve being computed for this candidate. \
                      The CompletenessResolver reads χ(N(C)) from this nerve \
                      at each iteration via resolver:nerveEulerCharacteristic.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/CompletenessCandidate"),
            range: "https://uor.foundation/resolver/ConstraintNerve",
        },
        Property {
            id: "https://uor.foundation/type/witnessConstraint",
            label: "witnessConstraint",
            comment: "The constraint applied in this witness step.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/CompletenessWitness"),
            range: "https://uor.foundation/type/Constraint",
        },
        Property {
            id: "https://uor.foundation/type/fibersClosed",
            label: "fibersClosed",
            comment: "Number of fibers closed by this witness step.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/CompletenessWitness"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // Amendment 28: TypeSynthesisGoal properties
        Property {
            id: "https://uor.foundation/type/targetEulerCharacteristic",
            label: "targetEulerCharacteristic",
            comment: "The target χ(N(C)) value. For O(1) resolution: set equal to n (the \
                      quantum level).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/TypeSynthesisGoal"),
            range: XSD_INTEGER,
        },
        Property {
            id: "https://uor.foundation/type/targetBettiNumber",
            label: "targetBettiNumber",
            comment: "Non-functional. Each assertion specifies a target Betti number value \
                      for a given homological degree. Multiple assertions permitted, one per \
                      degree.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/type/TypeSynthesisGoal"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // Amendment 28: SynthesizedType property
        Property {
            id: "https://uor.foundation/type/synthesisResult",
            label: "synthesisResult",
            comment: "Links a SynthesizedType back to the synthesis run that produced it.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/SynthesizedType"),
            range: "https://uor.foundation/type/TypeSynthesisResult",
        },
        // Amendment 28: MinimalConstraintBasis properties
        Property {
            id: "https://uor.foundation/type/basisConstraint",
            label: "basisConstraint",
            comment: "Non-functional. One assertion per constraint in the minimal basis.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/type/MinimalConstraintBasis"),
            range: "https://uor.foundation/type/Constraint",
        },
        Property {
            id: "https://uor.foundation/type/basisSize",
            label: "basisSize",
            comment: "The cardinality of the minimal basis. The theoretical lower bound is \
                      n (one constraint per fiber).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/MinimalConstraintBasis"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // Amendment 29: QuantumLift properties
        Property {
            id: "https://uor.foundation/type/liftBase",
            label: "liftBase",
            comment: "The base type being lifted to the next quantum level.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/QuantumLift"),
            range: "https://uor.foundation/type/ConstrainedType",
        },
        Property {
            id: "https://uor.foundation/type/liftTargetLevel",
            label: "liftTargetLevel",
            comment: "The quantum level this lift targets.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/QuantumLift"),
            range: "https://uor.foundation/schema/QuantumLevel",
        },
        Property {
            id: "https://uor.foundation/type/liftObstruction",
            label: "liftObstruction",
            comment: "The LiftObstruction for this lift. Trivial (zero class) iff the lift \
                      inherits completeness.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/QuantumLift"),
            range: "https://uor.foundation/type/LiftObstruction",
        },
        // Amendment 29: LiftObstruction properties
        Property {
            id: "https://uor.foundation/type/obstructionTrivial",
            label: "obstructionTrivial",
            comment: "True iff the obstruction class is zero — the base type's completeness \
                      transfers to the lifted quantum level without additional constraints.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/LiftObstruction"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/type/obstructionFiber",
            label: "obstructionFiber",
            comment: "The fiber at the new quantum level where the obstruction is located. \
                      Ranges over the new bit position introduced at Q_{n+1}.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/LiftObstruction"),
            range: "https://uor.foundation/partition/FiberCoordinate",
        },
        // Amendment 30: ConstrainedType extensions
        Property {
            id: "https://uor.foundation/type/holonomyGroup",
            label: "holonomyGroup",
            comment: "The HolonomyGroup of this type. Computed by the MonodromyResolver.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/ConstrainedType"),
            range: "https://uor.foundation/observable/HolonomyGroup",
        },
        Property {
            id: "https://uor.foundation/type/monodromyClass",
            label: "monodromyClass",
            comment: "The MonodromyClass classifying this type as flat or twisted.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/ConstrainedType"),
            range: "https://uor.foundation/observable/MonodromyClass",
        },
        // Amendment 32: Superposed fiber state amplitude (RC_5)
        Property {
            id: "https://uor.foundation/type/amplitude",
            label: "amplitude",
            comment: "The amplitude coefficient for this superposed fiber state.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/SuperposedFiberState"),
            range: XSD_DECIMAL,
        },
        // Amendment 34: Morphospace Achievability
        Property {
            id: "https://uor.foundation/type/targetForbidden",
            label: "targetForbidden",
            comment: "Whether the target signature of this TypeSynthesisGoal is a \
                      ForbiddenSignature. If true, synthesis is provably impossible.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/TypeSynthesisGoal"),
            range: XSD_BOOLEAN,
        },
        // Amendment 36: Measurement Boundary
        Property {
            id: "https://uor.foundation/type/collapsedFrom",
            label: "collapsedFrom",
            comment: "The SuperposedFiberState from which this CollapsedFiberState \
                      was produced by projective measurement.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/type/CollapsedFiberState"),
            range: "https://uor.foundation/type/SuperposedFiberState",
        },
        Property {
            id: "https://uor.foundation/type/survivingAmplitude",
            label: "survivingAmplitude",
            comment: "The amplitude of the surviving branch after projective \
                      collapse. |α|² is the probability of this outcome under \
                      the Born rule.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/CollapsedFiberState"),
            range: XSD_DECIMAL,
        },
        // Amendment 37: Amplitude normalization verification (Gap 1)
        Property {
            id: "https://uor.foundation/type/normalizationVerified",
            label: "normalizationVerified",
            comment: "Whether the amplitude vector of this SuperposedFiberState \
                      satisfies the normalization condition Σ|αᵢ|² = 1 (QM_5). \
                      Set by the SuperpositionResolver after verification.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/SuperposedFiberState"),
            range: XSD_BOOLEAN,
        },
        // Amendment 37: Holonomy classification flag (Gap 4)
        Property {
            id: "https://uor.foundation/type/holonomyClassified",
            label: "holonomyClassified",
            comment: "Whether this ConstrainedType has been classified as FlatType \
                      or TwistedType by the MonodromyResolver. The MN_8 identity \
                      guarantees this is a bivalent classification: \
                      holonomyClassified(T) iff isFlatType(T) xor isTwistedType(T).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/type/ConstrainedType"),
            range: XSD_BOOLEAN,
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
