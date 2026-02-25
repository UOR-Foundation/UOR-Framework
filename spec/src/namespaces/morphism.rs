//! `morphism/` namespace — Transforms and morphisms (Amendment 6).
//!
//! The morphism namespace defines the abstractions for maps between UOR objects.
//! These are the things that `cert/` certificates attest to, that `trace/`
//! traces record the execution of, and that `state/` transitions capture the
//! cumulative effect of.
//!
//! Amendment 12 adds the composition primitive: the categorical backbone that
//! turns transforms into a category with identity and associative composition.
//!
//! **Space classification:** `user` — transforms are instantiated by applications.

use crate::model::iris::*;
use crate::model::{
    Class, Individual, IndividualValue, Namespace, NamespaceModule, Property, PropertyKind, Space,
};

/// Returns the `morphism/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "morphism",
            iri: NS_MORPHISM,
            label: "UOR Transforms and Morphisms",
            comment: "Runtime abstractions for maps between UOR objects: transforms, \
                      isometries, embeddings, and group actions. The foundation \
                      provides the vocabulary; Prism writes the sentences.",
            space: Space::User,
            imports: &[
                NS_SCHEMA,
                NS_TYPE,
                NS_OP,
                NS_OBSERVABLE,
                NS_PARTITION,
                NS_TRACE,
            ],
        },
        classes: classes(),
        properties: properties(),
        individuals: individuals(),
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/morphism/Transform",
            label: "Transform",
            comment: "A map between UOR objects. The root abstraction: source, target, \
                      and optionally what structure (if any) is preserved. This is \
                      what cert:TransformCertificate certifies.",
            subclass_of: &[OWL_THING],
            disjoint_with: &["https://uor.foundation/morphism/CompositionLaw"],
        },
        Class {
            id: "https://uor.foundation/morphism/Isometry",
            label: "Isometry",
            comment: "A transform that preserves metric structure with respect to a \
                      specified metric. In UOR, isometry is metric-relative: neg is a \
                      ring isometry, bnot is a Hamming isometry. A transform can be \
                      an isometry with respect to one metric but not the other. This \
                      is what cert:IsometryCertificate certifies.",
            subclass_of: &["https://uor.foundation/morphism/Transform"],
            disjoint_with: &["https://uor.foundation/morphism/Composition"],
        },
        Class {
            id: "https://uor.foundation/morphism/Embedding",
            label: "Embedding",
            comment: "An injective, structure-preserving transform across quantum \
                      levels. The canonical instance is the level embedding \
                      ι : R_n → R_{n'} (n < n'), which preserves addition, \
                      multiplication, and content addressing.",
            subclass_of: &["https://uor.foundation/morphism/Transform"],
            disjoint_with: &["https://uor.foundation/morphism/Composition"],
        },
        Class {
            id: "https://uor.foundation/morphism/Action",
            label: "Action",
            comment: "The mechanism by which a group applies transforms systematically \
                      to a set. Each group element induces a transform of the set. \
                      The dihedral action on type space is an action by isometries — \
                      every element of D_{2^n} produces an isometric transform of 𝒯_n.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 12: Composition Primitive classes
        Class {
            id: "https://uor.foundation/morphism/Composition",
            label: "Composition",
            comment: "A transform formed by composing two or more transforms \
                      sequentially. The categorical composition operation that \
                      turns transforms into a category.",
            subclass_of: &["https://uor.foundation/morphism/Transform"],
            disjoint_with: &[
                "https://uor.foundation/morphism/Isometry",
                "https://uor.foundation/morphism/Embedding",
                "https://uor.foundation/morphism/Identity",
            ],
        },
        Class {
            id: "https://uor.foundation/morphism/Identity",
            label: "Identity",
            comment: "The identity transform on a type: maps every element to itself. \
                      The categorical identity morphism.",
            subclass_of: &["https://uor.foundation/morphism/Transform"],
            disjoint_with: &["https://uor.foundation/morphism/Composition"],
        },
        Class {
            id: "https://uor.foundation/morphism/CompositionLaw",
            label: "CompositionLaw",
            comment: "A law governing how operations compose. Records whether the \
                      composition is associative, commutative, and what the result \
                      operation is. The critical composition law (neg ∘ bnot = succ) \
                      is the foundational instance.",
            subclass_of: &[OWL_THING],
            disjoint_with: &["https://uor.foundation/morphism/Transform"],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/morphism/source",
            label: "source",
            comment: "The domain of the transform.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Transform"),
            range: OWL_THING,
        },
        Property {
            id: "https://uor.foundation/morphism/target",
            label: "target",
            comment: "The codomain of the transform.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Transform"),
            range: OWL_THING,
        },
        Property {
            id: "https://uor.foundation/morphism/preserves",
            label: "preserves",
            comment: "The structure preserved by this transform (if any). \
                      E.g., a ring homomorphism preserves addition and multiplication.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/morphism/Transform"),
            range: OWL_THING,
        },
        Property {
            id: "https://uor.foundation/morphism/preservesMetric",
            label: "preservesMetric",
            comment: "The specific metric this isometry preserves. Points to \
                      observable:RingMetric or observable:HammingMetric. A transform \
                      that preserves both is an isometry of the full UOR geometry. \
                      A transform that preserves one but not the other has nontrivial \
                      curvature — observable:CurvatureObservable measures this gap.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/morphism/Isometry"),
            range: "https://uor.foundation/observable/MetricObservable",
        },
        Property {
            id: "https://uor.foundation/morphism/sourceQuantum",
            label: "sourceQuantum",
            comment: "The quantum level n of the source ring for an embedding.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Embedding"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/morphism/targetQuantum",
            label: "targetQuantum",
            comment: "The quantum level n' of the target ring for an embedding. \
                      Must satisfy n' > n (embeddings go to larger rings).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Embedding"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/morphism/group",
            label: "group",
            comment: "The group acting in this group action.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Action"),
            range: "https://uor.foundation/op/Group",
        },
        Property {
            id: "https://uor.foundation/morphism/actingOn",
            label: "actingOn",
            comment: "The set being acted upon by this group action.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Action"),
            range: OWL_THING,
        },
        Property {
            id: "https://uor.foundation/morphism/actionIsometry",
            label: "actionIsometry",
            comment: "Whether every transform induced by this action is an isometry. \
                      True for the dihedral action on 𝒯_n (Frame Theorem).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Action"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/morphism/trace",
            label: "trace",
            comment: "The computation trace that realized this transform at runtime. \
                      A Transform is an abstraction; a trace is the kernel's record \
                      of how it was executed via concrete operations.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Transform"),
            range: "https://uor.foundation/trace/ComputationTrace",
        },
        // Amendment 12: Composition Primitive properties
        Property {
            id: "https://uor.foundation/morphism/composesWith",
            label: "composesWith",
            comment: "A transform that this transform can be composed with. \
                      The target of this transform must match the source of \
                      the composed transform.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/morphism/Transform"),
            range: "https://uor.foundation/morphism/Transform",
        },
        Property {
            id: "https://uor.foundation/morphism/compositionResult",
            label: "compositionResult",
            comment: "The transform that results from this composition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Composition"),
            range: "https://uor.foundation/morphism/Transform",
        },
        Property {
            id: "https://uor.foundation/morphism/compositionComponents",
            label: "compositionComponents",
            comment: "A component transform of this composition.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/morphism/Composition"),
            range: "https://uor.foundation/morphism/Transform",
        },
        Property {
            id: "https://uor.foundation/morphism/identityOn",
            label: "identityOn",
            comment: "The type on which this identity transform acts.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Identity"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/morphism/compositionOrder",
            label: "compositionOrder",
            comment: "The number of component transforms in a composition.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: None,
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/morphism/isAssociative",
            label: "isAssociative",
            comment: "Whether this composition law is associative.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/morphism/CompositionLaw"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/morphism/isCommutative",
            label: "isCommutative",
            comment: "Whether this composition law is commutative.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/morphism/CompositionLaw"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/morphism/lawComponents",
            label: "lawComponents",
            comment: "An operation that is a component of this composition law.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/morphism/CompositionLaw"),
            range: "https://uor.foundation/op/Operation",
        },
        Property {
            id: "https://uor.foundation/morphism/lawResult",
            label: "lawResult",
            comment: "The operation that results from this composition law.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/morphism/CompositionLaw"),
            range: "https://uor.foundation/op/Operation",
        },
        Property {
            id: "https://uor.foundation/morphism/preservesStructure",
            label: "preservesStructure",
            comment: "A human-readable description of the structure this transform \
                      preserves (e.g., 'ring homomorphism', 'metric isometry').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/morphism/Transform"),
            range: XSD_STRING,
        },
    ]
}

fn individuals() -> Vec<Individual> {
    vec![Individual {
        id: "https://uor.foundation/morphism/criticalComposition",
        type_: "https://uor.foundation/morphism/CompositionLaw",
        label: "criticalComposition",
        comment: "The critical composition law: neg ∘ bnot = succ. This is the \
                      operational form of the critical identity theorem. The \
                      composition of the two involutions (neg, bnot) yields the \
                      successor operation. Non-associative and non-commutative.",
        properties: &[
            (
                "https://uor.foundation/morphism/lawComponents",
                IndividualValue::IriRef("https://uor.foundation/op/neg"),
            ),
            (
                "https://uor.foundation/morphism/lawComponents",
                IndividualValue::IriRef("https://uor.foundation/op/bnot"),
            ),
            (
                "https://uor.foundation/morphism/lawResult",
                IndividualValue::IriRef("https://uor.foundation/op/succ"),
            ),
            (
                "https://uor.foundation/morphism/isAssociative",
                IndividualValue::Bool(false),
            ),
            (
                "https://uor.foundation/morphism/isCommutative",
                IndividualValue::Bool(false),
            ),
        ],
    }]
}
