//! `cert/` namespace — Attestation certificates.
//!
//! Certificates are kernel-produced attestations of structural properties of
//! transforms and operations. They provide verifiable proofs that a specific
//! computation or operation satisfies a particular structural constraint.
//!
//! **Space classification:** `bridge` — kernel-produced, user-consumed.

use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};
use crate::model::iris::*;

/// Returns the `cert/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "cert",
            iri: NS_CERT,
            label: "UOR Certificates",
            comment: "Kernel-produced attestation certificates for transforms, \
                      isometries, and involutions. Each certificate verifies that \
                      a specific structural property holds.",
            space: Space::Bridge,
            imports: &[NS_OP, NS_PROOF],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/cert/Certificate",
            label: "Certificate",
            comment: "A kernel-produced attestation. The root class for all \
                      certificate types.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cert/TransformCertificate",
            label: "TransformCertificate",
            comment: "A certificate attesting to the properties of a morphism:Transform. \
                      Certifies that the transform maps source to target correctly.",
            subclass_of: &["https://uor.foundation/cert/Certificate"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cert/IsometryCertificate",
            label: "IsometryCertificate",
            comment: "A certificate attesting that a morphism:Isometry preserves \
                      metric distances. Certifies the transform is a metric isometry \
                      with respect to the specified metric.",
            subclass_of: &["https://uor.foundation/cert/Certificate"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cert/InvolutionCertificate",
            label: "InvolutionCertificate",
            comment: "A certificate attesting that an operation is an involution: \
                      f(f(x)) = x for all x in R_n.",
            subclass_of: &["https://uor.foundation/cert/Certificate"],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/cert/transformType",
            label: "transformType",
            comment: "The type of transform this certificate attests to \
                      (e.g., 'isometry', 'embedding', 'action').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cert/TransformCertificate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cert/method",
            label: "method",
            comment: "The verification method used to produce this certificate \
                      (e.g., 'exhaustive_check', 'symbolic_proof', 'sampling').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cert/Certificate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cert/operation",
            label: "operation",
            comment: "The operation this certificate applies to.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/cert/InvolutionCertificate"),
            range: "https://uor.foundation/op/Operation",
        },
        Property {
            id: "https://uor.foundation/cert/verified",
            label: "verified",
            comment: "Whether this certificate has been verified by the kernel.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cert/Certificate"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/cert/quantum",
            label: "quantum",
            comment: "The quantum level at which this certificate was produced.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cert/Certificate"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cert/timestamp",
            label: "timestamp",
            comment: "The time at which this certificate was issued.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cert/Certificate"),
            range: XSD_DATETIME,
        },
    ]
}
