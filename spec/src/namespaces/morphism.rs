//! `morphism/` namespace — Transforms and morphisms (Amendment 6).
//!
//! The morphism namespace defines the abstractions for maps between UOR objects.
//! These are the things that `cert/` certificates attest to, that `trace/`
//! traces record the execution of, and that `state/` transitions capture the
//! cumulative effect of.
//!
//! **No named individuals:** specific transforms (e.g., the dihedral action on
//! 𝒯_n, the level embedding Q0→Q2) are Prism-level declarations.
//!
//! **Space classification:** `user` — transforms are instantiated by applications.

use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};
use crate::model::iris::*;

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
            imports: &[NS_SCHEMA, NS_TYPE, NS_OP, NS_OBSERVABLE, NS_PARTITION, NS_TRACE],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
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
            disjoint_with: &[],
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
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/morphism/Embedding",
            label: "Embedding",
            comment: "An injective, structure-preserving transform across quantum \
                      levels. The canonical instance is the level embedding \
                      ι : R_n → R_{n'} (n < n'), which preserves addition, \
                      multiplication, and content addressing.",
            subclass_of: &["https://uor.foundation/morphism/Transform"],
            disjoint_with: &[],
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
    ]
}
