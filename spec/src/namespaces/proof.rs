//! `proof/` namespace — Verification proof structures.
//!
//! Proofs are kernel-produced attestations of algebraic properties. The
//! critical proof asserts the foundational theorem `neg(bnot(x)) = succ(x)`.
//!
//! **Space classification:** `bridge` — kernel-produced, user-consumed.

use crate::model::iris::*;
use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};

/// Returns the `proof/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "proof",
            iri: NS_PROOF,
            label: "UOR Proofs",
            comment: "Kernel-produced verification proofs attesting to algebraic \
                      properties of UOR objects and operations.",
            space: Space::Bridge,
            imports: &[NS_SCHEMA, NS_OP],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/proof/Proof",
            label: "Proof",
            comment: "A kernel-produced attestation that a given algebraic property \
                      holds. The root class for all proof types.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/proof/CoherenceProof",
            label: "CoherenceProof",
            comment: "A proof of coherence: the type system and ring structure are \
                      mutually consistent at a given quantum level.",
            subclass_of: &["https://uor.foundation/proof/Proof"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/proof/CriticalIdentityProof",
            label: "CriticalIdentityProof",
            comment: "A proof of the critical identity: neg(bnot(x)) = succ(x) \
                      for all x in R_n. This is the foundational theorem of the \
                      UOR kernel.",
            subclass_of: &["https://uor.foundation/proof/Proof"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/proof/WitnessData",
            label: "WitnessData",
            comment: "Supporting data for a proof: specific examples, counter-examples \
                      checked, or intermediate computation results.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/proof/quantum",
            label: "quantum",
            comment: "The quantum level at which this proof was verified.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/proof/Proof"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/proof/verified",
            label: "verified",
            comment: "Whether this proof has been verified by the kernel.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/proof/Proof"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/proof/timestamp",
            label: "timestamp",
            comment: "The time at which this proof was produced.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/proof/Proof"),
            range: XSD_DATETIME,
        },
        Property {
            id: "https://uor.foundation/proof/witness",
            label: "witness",
            comment: "Supporting witness data for this proof.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/proof/Proof"),
            range: "https://uor.foundation/proof/WitnessData",
        },
        Property {
            id: "https://uor.foundation/proof/criticalIdentity",
            label: "criticalIdentity",
            comment: "Human-readable statement of the critical identity proven. \
                      E.g., 'neg(bnot(x)) = succ(x) for all x in R_n'.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/proof/CriticalIdentityProof"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/proof/x",
            label: "x",
            comment: "A specific input value used as a witness for the critical \
                      identity check.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/proof/WitnessData"),
            range: XSD_INTEGER,
        },
        Property {
            id: "https://uor.foundation/proof/bnot_x",
            label: "bnot_x",
            comment: "The value bnot(x) for a witness x.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/proof/WitnessData"),
            range: XSD_INTEGER,
        },
        Property {
            id: "https://uor.foundation/proof/neg_bnot_x",
            label: "neg_bnot_x",
            comment: "The value neg(bnot(x)) for a witness x.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/proof/WitnessData"),
            range: XSD_INTEGER,
        },
        Property {
            id: "https://uor.foundation/proof/succ_x",
            label: "succ_x",
            comment: "The value succ(x) for a witness x.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/proof/WitnessData"),
            range: XSD_INTEGER,
        },
        Property {
            id: "https://uor.foundation/proof/holds",
            label: "holds",
            comment: "Whether the identity neg(bnot(x)) = succ(x) holds for \
                      this specific witness.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/proof/WitnessData"),
            range: XSD_BOOLEAN,
        },
        // Amendment 3: provesIdentity — object property linking to op:Identity
        Property {
            id: "https://uor.foundation/proof/provesIdentity",
            label: "provesIdentity",
            comment: "The algebraic identity this proof establishes. Provides a \
                      canonical object reference alongside the existing \
                      proof:criticalIdentity string property, which remains for \
                      human readability.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/proof/Proof"),
            range: "https://uor.foundation/op/Identity",
        },
    ]
}
