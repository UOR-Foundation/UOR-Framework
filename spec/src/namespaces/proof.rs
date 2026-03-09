//! `proof/` namespace — Verification proof structures.
//!
//! Proofs are kernel-produced attestations of algebraic properties. The
//! critical proof asserts the foundational theorem `neg(bnot(x)) = succ(x)`.
//!
//! Amendment 48 adds 11 proofs: prf_SR_8–10 (multi-session coordination axioms)
//! and prf_MC_1–8 (derivational scaling theorem proofs).
//!
//! **Space classification:** `bridge` — kernel-produced, user-consumed.

use crate::model::iris::*;
use crate::model::{
    Class, Individual, IndividualValue, Namespace, NamespaceModule, Property, PropertyKind, Space,
};

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
            imports: &[NS_SCHEMA, NS_OP, NS_DERIVATION, NS_OBSERVABLE],
        },
        classes: classes(),
        properties: properties(),
        individuals: individuals(),
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
        // v3.2: Proof modality classes
        Class {
            id: "https://uor.foundation/proof/ComputationCertificate",
            label: "ComputationCertificate",
            comment: "A proof confirmed by exhaustive execution over R_n at a specific \
                      quantum level. The kernel ran the identity against all 2^n inputs \
                      and observed that it holds. The proof:atQuantumLevel property \
                      records the level; proof:witness links to the WitnessData. \
                      CriticalIdentityProof is a subclass of ComputationCertificate.",
            subclass_of: &["https://uor.foundation/proof/Proof"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "AxiomaticDerivation",
            comment: "A proof that follows from previously established axioms or \
                      definitions by equational, structural, or topological reasoning. \
                      The proof:derivationWitness property links to a \
                      derivation:Derivation individual recording the rewrite chain. \
                      All pipeline, constraint, observable, and topological identities \
                      are AxiomaticDerivations.",
            subclass_of: &["https://uor.foundation/proof/Proof"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/proof/CriticalIdentityProof",
            label: "CriticalIdentityProof",
            comment: "A proof of the critical identity: neg(bnot(x)) = succ(x) \
                      for all x in R_n. This is the foundational theorem of the \
                      UOR kernel.",
            subclass_of: &["https://uor.foundation/proof/ComputationCertificate"],
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
        // Amendment 31: EmpiricalVerification proof class
        Class {
            id: "https://uor.foundation/proof/EmpiricalVerification",
            label: "EmpiricalVerification",
            comment: "A proof grounded in empirical verification across a bounded \
                      range of quantum levels, rather than axiomatic derivation \
                      or exhaustive computation. Used for identities like AR_5 \
                      whose forAll quantifier is 'empirical, Q0--Q4'.",
            subclass_of: &["https://uor.foundation/proof/Proof"],
            disjoint_with: &[],
        },
        // Amendment 34: Morphospace Achievability
        Class {
            id: "https://uor.foundation/proof/ImpossibilityWitness",
            label: "ImpossibilityWitness",
            comment: "A formal witness that a topological signature (χ, β_k) is \
                      impossible to achieve for any ConstrainedType. Carries the \
                      algebraic reason and the verification domain grounding the \
                      impossibility.",
            subclass_of: &["https://uor.foundation/proof/Proof"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/proof/MorphospaceRecord",
            label: "MorphospaceRecord",
            comment: "A formal record of a morphospace boundary point — either an \
                      achievable or forbidden topological signature. Aggregated by \
                      MorphospaceBoundary to form the queryable morphospace map.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/proof/MorphospaceBoundary",
            label: "MorphospaceBoundary",
            comment: "An aggregate of ImpossibilityWitness and EmpiricalVerification \
                      instances forming the queryable morphospace map. SPARQL over \
                      this structure answers achievability queries in O(1).",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 41: InductiveProof
        Class {
            id: "https://uor.foundation/proof/InductiveProof",
            label: "InductiveProof",
            comment: "A proof by structural induction on the quantum level index k. \
                      Carries a base case proof, an inductive step proof, and the \
                      minimum k for which the induction holds.",
            subclass_of: &["https://uor.foundation/proof/Proof"],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
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
                      E.g., 'neg(bnot(x)) = succ(x) for all x in R_n'. \
                      Annotation only — proof:provesIdentity is the typed reference.",
            kind: PropertyKind::Annotation,
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
        // v3.2: ComputationCertificate properties
        Property {
            id: "https://uor.foundation/proof/atQuantumLevel",
            label: "atQuantumLevel",
            comment: "The quantum level at which this computation certificate was \
                      produced. A ComputationCertificate at schema:Q0 confirms the \
                      identity holds for all 256 inputs of R_8. A certificate at \
                      schema:Q1 confirms it for all 65,536 inputs of R_16.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/proof/ComputationCertificate"),
            range: "https://uor.foundation/schema/QuantumLevel",
        },
        Property {
            id: "https://uor.foundation/proof/quantumNote",
            label: "quantumNote",
            comment: "Human-readable quantum level note, e.g. 'n=8, 256 inputs'. \
                      Annotation only — proof:atQuantumLevel is the typed assertion.",
            kind: PropertyKind::Annotation,
            functional: true,
            domain: Some("https://uor.foundation/proof/ComputationCertificate"),
            range: XSD_STRING,
        },
        // v3.2: AxiomaticDerivation properties
        Property {
            id: "https://uor.foundation/proof/universalScope",
            label: "universalScope",
            comment: "True when this axiomatic derivation holds for all quantum levels \
                      by the definition of Z/(2^n)Z. False when the derivation depends \
                      on a property specific to a particular ring size. All current \
                      AxiomaticDerivation individuals in the spec carry universalScope \
                      true.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/proof/AxiomaticDerivation"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/proof/derivationWitness",
            label: "derivationWitness",
            comment: "The derivation chain that witnesses this axiomatic derivation. \
                      Links a proof:AxiomaticDerivation to the derivation:Derivation \
                      individual recording the rewrite sequence. Optional at the spec \
                      level — the conformance suite requires only that the proof \
                      individual exists; full derivation chains live in generated \
                      artifacts.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/proof/AxiomaticDerivation"),
            range: "https://uor.foundation/derivation/Derivation",
        },
        // Amendment 31: EmpiricalVerification properties
        Property {
            id: "https://uor.foundation/proof/quantumLevelRange",
            label: "quantumLevelRange",
            comment: "The range of quantum levels over which this empirical \
                      verification was conducted (e.g., 'Q0-Q4').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/proof/EmpiricalVerification"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/proof/verificationMethod",
            label: "verificationMethod",
            comment: "The empirical method used for verification (e.g., 'exhaustive \
                      enumeration over Z/(2^n)Z for n=8,16,32,64').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/proof/EmpiricalVerification"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/proof/verifiedAt",
            label: "verifiedAt",
            comment: "The timestamp at which empirical verification was conducted.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/proof/EmpiricalVerification"),
            range: XSD_DATETIME,
        },
        // Amendment 34: Morphospace Achievability
        Property {
            id: "https://uor.foundation/proof/forbidsSignature",
            label: "forbidsSignature",
            comment: "The topological signature (χ, β_k) that this \
                      ImpossibilityWitness formally forbids.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/proof/ImpossibilityWitness"),
            range: "https://uor.foundation/observable/SynthesisSignature",
        },
        Property {
            id: "https://uor.foundation/proof/impossibilityReason",
            label: "impossibilityReason",
            comment: "Human-readable statement of the algebraic reason the \
                      signature is impossible (e.g., 'β₀ = 0 violates MS_1').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/proof/ImpossibilityWitness"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/proof/impossibilityDomain",
            label: "impossibilityDomain",
            comment: "The verification domain grounding the impossibility \
                      (e.g., Pipeline for β₀ = 0, Algebraic for χ > n).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/proof/ImpossibilityWitness"),
            range: "https://uor.foundation/op/VerificationDomain",
        },
        Property {
            id: "https://uor.foundation/proof/achievabilityStatus",
            label: "achievabilityStatus",
            comment: "The achievability classification of a proof-linked observable \
                      signature: Achievable or Forbidden.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/proof/EmpiricalVerification"),
            range: "https://uor.foundation/observable/AchievabilityStatus",
        },
        Property {
            id: "https://uor.foundation/proof/verifiedAtLevel",
            label: "verifiedAtLevel",
            comment: "The specific quantum level at which an empirical verification \
                      or impossibility witness was established.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/proof/Proof"),
            range: "https://uor.foundation/schema/QuantumLevel",
        },
        Property {
            id: "https://uor.foundation/proof/morphospaceRecord",
            label: "morphospaceRecord",
            comment: "Links a MorphospaceBoundary to one of its constituent \
                      MorphospaceRecord individuals.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/proof/MorphospaceBoundary"),
            range: "https://uor.foundation/proof/MorphospaceRecord",
        },
        Property {
            id: "https://uor.foundation/proof/boundaryType",
            label: "boundaryType",
            comment: "Whether this MorphospaceRecord represents an impossibility \
                      boundary (from below) or an achievability boundary (from above).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/proof/MorphospaceRecord"),
            range: "https://uor.foundation/observable/AchievabilityStatus",
        },
        // Amendment 41: InductiveProof properties
        Property {
            id: "https://uor.foundation/proof/baseCase",
            label: "baseCase",
            comment: "The proof that the claim holds at the base level k_0.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/proof/InductiveProof"),
            range: "https://uor.foundation/proof/Proof",
        },
        Property {
            id: "https://uor.foundation/proof/inductiveStep",
            label: "inductiveStep",
            comment: "The proof that if the claim holds at Q_k, it holds at Q_{k+1}.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/proof/InductiveProof"),
            range: "https://uor.foundation/proof/Proof",
        },
        Property {
            id: "https://uor.foundation/proof/validForKAtLeast",
            label: "validForKAtLeast",
            comment: "The minimum k for which the induction is valid.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/proof/InductiveProof"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
    ]
}

fn individuals() -> Vec<Individual> {
    vec![
        // Critical identity: computation certificate (CriticalIdentityProof at Q0)
        Individual {
            id: "https://uor.foundation/proof/prf_criticalIdentity",
            type_: "https://uor.foundation/proof/CriticalIdentityProof",
            label: "prf_criticalIdentity",
            comment: "Computation certificate for the critical identity \
                      neg(bnot(x)) = succ(x) at Q0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/criticalIdentity"),
                ),
                (
                    "https://uor.foundation/proof/atQuantumLevel",
                    IndividualValue::IriRef("https://uor.foundation/schema/Q0"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Critical identity: axiomatic derivation (dual proof)
        Individual {
            id: "https://uor.foundation/proof/prf_criticalIdentity_axiomatic",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_criticalIdentity_axiomatic",
            comment: "Axiomatic derivation of the critical identity \
                      neg(bnot(x)) = succ(x). Holds at all quantum levels.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/criticalIdentity"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // phi_1 through phi_6: computation certificates at Q0
        Individual {
            id: "https://uor.foundation/proof/prf_phi_1",
            type_: "https://uor.foundation/proof/ComputationCertificate",
            label: "prf_phi_1",
            comment: "Computation certificate for phi_1 at Q0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/phi_1"),
                ),
                (
                    "https://uor.foundation/proof/atQuantumLevel",
                    IndividualValue::IriRef("https://uor.foundation/schema/Q0"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_phi_2",
            type_: "https://uor.foundation/proof/ComputationCertificate",
            label: "prf_phi_2",
            comment: "Computation certificate for phi_2 at Q0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/phi_2"),
                ),
                (
                    "https://uor.foundation/proof/atQuantumLevel",
                    IndividualValue::IriRef("https://uor.foundation/schema/Q0"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_phi_3",
            type_: "https://uor.foundation/proof/ComputationCertificate",
            label: "prf_phi_3",
            comment: "Computation certificate for phi_3 at Q0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/phi_3"),
                ),
                (
                    "https://uor.foundation/proof/atQuantumLevel",
                    IndividualValue::IriRef("https://uor.foundation/schema/Q0"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_phi_4",
            type_: "https://uor.foundation/proof/ComputationCertificate",
            label: "prf_phi_4",
            comment: "Computation certificate for phi_4 at Q0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/phi_4"),
                ),
                (
                    "https://uor.foundation/proof/atQuantumLevel",
                    IndividualValue::IriRef("https://uor.foundation/schema/Q0"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_phi_5",
            type_: "https://uor.foundation/proof/ComputationCertificate",
            label: "prf_phi_5",
            comment: "Computation certificate for phi_5 at Q0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/phi_5"),
                ),
                (
                    "https://uor.foundation/proof/atQuantumLevel",
                    IndividualValue::IriRef("https://uor.foundation/schema/Q0"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_phi_6",
            type_: "https://uor.foundation/proof/ComputationCertificate",
            label: "prf_phi_6",
            comment: "Computation certificate for phi_6 at Q0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/phi_6"),
                ),
                (
                    "https://uor.foundation/proof/atQuantumLevel",
                    IndividualValue::IriRef("https://uor.foundation/schema/Q0"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Axiomatic derivations for all remaining op identities
        Individual {
            id: "https://uor.foundation/proof/prf_AD_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AD_1",
            comment: "Axiomatic derivation of AD_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AD_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AD_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AD_2",
            comment: "Axiomatic derivation of AD_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AD_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_A1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_A1",
            comment: "Axiomatic derivation of R_A1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_A1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_A2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_A2",
            comment: "Axiomatic derivation of R_A2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_A2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_A3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_A3",
            comment: "Axiomatic derivation of R_A3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_A3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_A4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_A4",
            comment: "Axiomatic derivation of R_A4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_A4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_A5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_A5",
            comment: "Axiomatic derivation of R_A5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_A5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_A6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_A6",
            comment: "Axiomatic derivation of R_A6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_A6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_M1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_M1",
            comment: "Axiomatic derivation of R_M1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_M1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_M2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_M2",
            comment: "Axiomatic derivation of R_M2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_M2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_M3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_M3",
            comment: "Axiomatic derivation of R_M3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_M3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_M4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_M4",
            comment: "Axiomatic derivation of R_M4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_M4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_R_M5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_R_M5",
            comment: "Axiomatic derivation of R_M5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/R_M5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_1",
            comment: "Axiomatic derivation of B_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_2",
            comment: "Axiomatic derivation of B_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_3",
            comment: "Axiomatic derivation of B_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_4",
            comment: "Axiomatic derivation of B_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_5",
            comment: "Axiomatic derivation of B_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_6",
            comment: "Axiomatic derivation of B_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_7",
            comment: "Axiomatic derivation of B_7. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_8",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_8",
            comment: "Axiomatic derivation of B_8. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_9",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_9",
            comment: "Axiomatic derivation of B_9. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_9"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_10",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_10",
            comment: "Axiomatic derivation of B_10. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_10"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_11",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_11",
            comment: "Axiomatic derivation of B_11. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_11"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_12",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_12",
            comment: "Axiomatic derivation of B_12. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_12"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_B_13",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_B_13",
            comment: "Axiomatic derivation of B_13. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/B_13"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_X_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_X_1",
            comment: "Axiomatic derivation of X_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/X_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_X_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_X_2",
            comment: "Axiomatic derivation of X_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/X_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_X_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_X_3",
            comment: "Axiomatic derivation of X_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/X_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_X_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_X_4",
            comment: "Axiomatic derivation of X_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/X_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_X_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_X_5",
            comment: "Axiomatic derivation of X_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/X_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_X_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_X_6",
            comment: "Axiomatic derivation of X_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/X_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_X_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_X_7",
            comment: "Axiomatic derivation of X_7. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/X_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_D_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_D_1",
            comment: "Axiomatic derivation of D_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/D_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_D_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_D_3",
            comment: "Axiomatic derivation of D_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/D_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_D_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_D_4",
            comment: "Axiomatic derivation of D_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/D_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_D_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_D_5",
            comment: "Axiomatic derivation of D_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/D_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_U_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_U_1",
            comment: "Axiomatic derivation of U_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/U_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_U_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_U_2",
            comment: "Axiomatic derivation of U_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/U_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_U_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_U_3",
            comment: "Axiomatic derivation of U_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/U_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_U_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_U_4",
            comment: "Axiomatic derivation of U_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/U_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_U_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_U_5",
            comment: "Axiomatic derivation of U_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/U_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AG_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AG_1",
            comment: "Axiomatic derivation of AG_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AG_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AG_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AG_2",
            comment: "Axiomatic derivation of AG_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AG_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AG_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AG_3",
            comment: "Axiomatic derivation of AG_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AG_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AG_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AG_4",
            comment: "Axiomatic derivation of AG_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AG_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CA_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CA_1",
            comment: "Axiomatic derivation of CA_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CA_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CA_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CA_2",
            comment: "Axiomatic derivation of CA_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CA_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CA_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CA_3",
            comment: "Axiomatic derivation of CA_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CA_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CA_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CA_4",
            comment: "Axiomatic derivation of CA_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CA_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CA_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CA_5",
            comment: "Axiomatic derivation of CA_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CA_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CA_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CA_6",
            comment: "Axiomatic derivation of CA_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CA_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_C_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_C_1",
            comment: "Axiomatic derivation of C_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/C_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_C_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_C_2",
            comment: "Axiomatic derivation of C_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/C_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_C_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_C_3",
            comment: "Axiomatic derivation of C_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/C_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_C_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_C_4",
            comment: "Axiomatic derivation of C_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/C_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_C_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_C_5",
            comment: "Axiomatic derivation of C_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/C_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_C_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_C_6",
            comment: "Axiomatic derivation of C_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/C_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CDI",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CDI",
            comment: "Axiomatic derivation of CDI. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CDI"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CL_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CL_1",
            comment: "Axiomatic derivation of CL_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CL_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CL_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CL_2",
            comment: "Axiomatic derivation of CL_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CL_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CL_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CL_3",
            comment: "Axiomatic derivation of CL_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CL_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CL_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CL_4",
            comment: "Axiomatic derivation of CL_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CL_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CL_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CL_5",
            comment: "Axiomatic derivation of CL_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CL_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CM_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CM_1",
            comment: "Axiomatic derivation of CM_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CM_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CM_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CM_2",
            comment: "Axiomatic derivation of CM_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CM_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CM_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CM_3",
            comment: "Axiomatic derivation of CM_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CM_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CR_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CR_1",
            comment: "Axiomatic derivation of CR_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CR_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CR_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CR_2",
            comment: "Axiomatic derivation of CR_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CR_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CR_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CR_3",
            comment: "Axiomatic derivation of CR_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CR_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CR_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CR_4",
            comment: "Axiomatic derivation of CR_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CR_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CR_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CR_5",
            comment: "Axiomatic derivation of CR_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CR_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_F_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_F_1",
            comment: "Axiomatic derivation of F_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/F_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_F_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_F_2",
            comment: "Axiomatic derivation of F_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/F_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_F_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_F_3",
            comment: "Axiomatic derivation of F_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/F_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_F_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_F_4",
            comment: "Axiomatic derivation of F_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/F_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FL_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FL_1",
            comment: "Axiomatic derivation of FL_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FL_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FL_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FL_2",
            comment: "Axiomatic derivation of FL_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FL_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FL_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FL_3",
            comment: "Axiomatic derivation of FL_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FL_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FL_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FL_4",
            comment: "Axiomatic derivation of FL_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FL_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FPM_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FPM_1",
            comment: "Axiomatic derivation of FPM_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FPM_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FPM_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FPM_2",
            comment: "Axiomatic derivation of FPM_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FPM_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FPM_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FPM_3",
            comment: "Axiomatic derivation of FPM_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FPM_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FPM_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FPM_4",
            comment: "Axiomatic derivation of FPM_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FPM_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FPM_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FPM_5",
            comment: "Axiomatic derivation of FPM_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FPM_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FPM_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FPM_6",
            comment: "Axiomatic derivation of FPM_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FPM_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FPM_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FPM_7",
            comment: "Axiomatic derivation of FPM_7. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FPM_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FS_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FS_1",
            comment: "Axiomatic derivation of FS_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FS_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FS_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FS_2",
            comment: "Axiomatic derivation of FS_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FS_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FS_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FS_3",
            comment: "Axiomatic derivation of FS_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FS_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FS_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FS_4",
            comment: "Axiomatic derivation of FS_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FS_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FS_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FS_5",
            comment: "Axiomatic derivation of FS_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FS_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FS_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FS_6",
            comment: "Axiomatic derivation of FS_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FS_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FS_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FS_7",
            comment: "Axiomatic derivation of FS_7. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FS_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_RE_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_RE_1",
            comment: "Axiomatic derivation of RE_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/RE_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IR_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IR_1",
            comment: "Axiomatic derivation of IR_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IR_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IR_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IR_2",
            comment: "Axiomatic derivation of IR_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IR_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IR_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IR_3",
            comment: "Axiomatic derivation of IR_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IR_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IR_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IR_4",
            comment: "Axiomatic derivation of IR_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IR_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SF_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SF_1",
            comment: "Axiomatic derivation of SF_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SF_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SF_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SF_2",
            comment: "Axiomatic derivation of SF_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SF_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_RD_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_RD_1",
            comment: "Axiomatic derivation of RD_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/RD_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_RD_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_RD_2",
            comment: "Axiomatic derivation of RD_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/RD_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SE_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SE_1",
            comment: "Axiomatic derivation of SE_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SE_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SE_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SE_2",
            comment: "Axiomatic derivation of SE_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SE_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SE_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SE_3",
            comment: "Axiomatic derivation of SE_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SE_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SE_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SE_4",
            comment: "Axiomatic derivation of SE_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SE_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OO_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OO_1",
            comment: "Axiomatic derivation of OO_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OO_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OO_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OO_2",
            comment: "Axiomatic derivation of OO_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OO_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OO_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OO_3",
            comment: "Axiomatic derivation of OO_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OO_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OO_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OO_4",
            comment: "Axiomatic derivation of OO_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OO_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OO_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OO_5",
            comment: "Axiomatic derivation of OO_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OO_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CB_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CB_1",
            comment: "Axiomatic derivation of CB_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CB_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CB_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CB_2",
            comment: "Axiomatic derivation of CB_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CB_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CB_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CB_3",
            comment: "Axiomatic derivation of CB_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CB_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CB_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CB_4",
            comment: "Axiomatic derivation of CB_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CB_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CB_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CB_5",
            comment: "Axiomatic derivation of CB_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CB_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CB_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CB_6",
            comment: "Axiomatic derivation of CB_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CB_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_M1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_M1",
            comment: "Axiomatic derivation of OB_M1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_M1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_M2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_M2",
            comment: "Axiomatic derivation of OB_M2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_M2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_M3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_M3",
            comment: "Axiomatic derivation of OB_M3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_M3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_M4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_M4",
            comment: "Axiomatic derivation of OB_M4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_M4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_M5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_M5",
            comment: "Axiomatic derivation of OB_M5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_M5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_M6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_M6",
            comment: "Axiomatic derivation of OB_M6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_M6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_C1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_C1",
            comment: "Axiomatic derivation of OB_C1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_C1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_C2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_C2",
            comment: "Axiomatic derivation of OB_C2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_C2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_C3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_C3",
            comment: "Axiomatic derivation of OB_C3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_C3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_H1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_H1",
            comment: "Axiomatic derivation of OB_H1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_H1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_H2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_H2",
            comment: "Axiomatic derivation of OB_H2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_H2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_H3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_H3",
            comment: "Axiomatic derivation of OB_H3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_H3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_P1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_P1",
            comment: "Axiomatic derivation of OB_P1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_P1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_P2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_P2",
            comment: "Axiomatic derivation of OB_P2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_P2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_OB_P3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_OB_P3",
            comment: "Axiomatic derivation of OB_P3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/OB_P3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CT_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CT_1",
            comment: "Axiomatic derivation of CT_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CT_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CT_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CT_2",
            comment: "Axiomatic derivation of CT_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CT_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CT_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CT_3",
            comment: "Axiomatic derivation of CT_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CT_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CT_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CT_4",
            comment: "Axiomatic derivation of CT_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CT_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CF_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CF_1",
            comment: "Axiomatic derivation of CF_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CF_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CF_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CF_2",
            comment: "Axiomatic derivation of CF_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CF_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CF_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CF_3",
            comment: "Axiomatic derivation of CF_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CF_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CF_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CF_4",
            comment: "Axiomatic derivation of CF_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CF_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_HG_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_HG_1",
            comment: "Axiomatic derivation of HG_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/HG_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_HG_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_HG_2",
            comment: "Axiomatic derivation of HG_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/HG_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_HG_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_HG_3",
            comment: "Axiomatic derivation of HG_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/HG_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_HG_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_HG_4",
            comment: "Axiomatic derivation of HG_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/HG_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_HG_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_HG_5",
            comment: "Axiomatic derivation of HG_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/HG_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_C1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_C1",
            comment: "Axiomatic derivation of T_C1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_C1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_C2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_C2",
            comment: "Axiomatic derivation of T_C2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_C2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_C3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_C3",
            comment: "Axiomatic derivation of T_C3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_C3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_C4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_C4",
            comment: "Axiomatic derivation of T_C4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_C4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_I1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_I1",
            comment: "Axiomatic derivation of T_I1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_I1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_I2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_I2",
            comment: "Axiomatic derivation of T_I2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_I2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_I3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_I3",
            comment: "Axiomatic derivation of T_I3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_I3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_I4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_I4",
            comment: "Axiomatic derivation of T_I4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_I4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_I5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_I5",
            comment: "Axiomatic derivation of T_I5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_I5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_E1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_E1",
            comment: "Axiomatic derivation of T_E1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_E1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_E2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_E2",
            comment: "Axiomatic derivation of T_E2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_E2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_E3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_E3",
            comment: "Axiomatic derivation of T_E3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_E3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_E4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_E4",
            comment: "Axiomatic derivation of T_E4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_E4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_A1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_A1",
            comment: "Axiomatic derivation of T_A1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_A1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_A2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_A2",
            comment: "Axiomatic derivation of T_A2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_A2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_A3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_A3",
            comment: "Axiomatic derivation of T_A3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_A3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_T_A4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_T_A4",
            comment: "Axiomatic derivation of T_A4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/T_A4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AU_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AU_1",
            comment: "Axiomatic derivation of AU_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AU_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AU_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AU_2",
            comment: "Axiomatic derivation of AU_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AU_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AU_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AU_3",
            comment: "Axiomatic derivation of AU_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AU_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AU_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AU_4",
            comment: "Axiomatic derivation of AU_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AU_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AU_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AU_5",
            comment: "Axiomatic derivation of AU_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AU_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_EF_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_EF_1",
            comment: "Axiomatic derivation of EF_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/EF_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_EF_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_EF_2",
            comment: "Axiomatic derivation of EF_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/EF_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_EF_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_EF_3",
            comment: "Axiomatic derivation of EF_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/EF_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_EF_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_EF_4",
            comment: "Axiomatic derivation of EF_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/EF_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_EF_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_EF_5",
            comment: "Axiomatic derivation of EF_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/EF_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_EF_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_EF_6",
            comment: "Axiomatic derivation of EF_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/EF_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_EF_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_EF_7",
            comment: "Axiomatic derivation of EF_7. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/EF_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AA_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AA_1",
            comment: "Axiomatic derivation of AA_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AA_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AA_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AA_2",
            comment: "Axiomatic derivation of AA_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AA_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AA_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AA_3",
            comment: "Axiomatic derivation of AA_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AA_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AA_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AA_4",
            comment: "Axiomatic derivation of AA_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AA_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AA_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AA_5",
            comment: "Axiomatic derivation of AA_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AA_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AA_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AA_6",
            comment: "Axiomatic derivation of AA_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AA_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AM_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AM_1",
            comment: "Axiomatic derivation of AM_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AM_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AM_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AM_2",
            comment: "Axiomatic derivation of AM_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AM_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AM_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AM_3",
            comment: "Axiomatic derivation of AM_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AM_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AM_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AM_4",
            comment: "Axiomatic derivation of AM_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AM_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TH_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TH_1",
            comment: "Axiomatic derivation of TH_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TH_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TH_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TH_2",
            comment: "Axiomatic derivation of TH_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TH_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TH_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TH_3",
            comment: "Axiomatic derivation of TH_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TH_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TH_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TH_4",
            comment: "Axiomatic derivation of TH_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TH_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TH_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TH_5",
            comment: "Axiomatic derivation of TH_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TH_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TH_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TH_6",
            comment: "Axiomatic derivation of TH_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TH_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TH_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TH_7",
            comment: "Axiomatic derivation of TH_7. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TH_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TH_8",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TH_8",
            comment: "Axiomatic derivation of TH_8. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TH_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TH_9",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TH_9",
            comment: "Axiomatic derivation of TH_9. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TH_9"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TH_10",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TH_10",
            comment: "Axiomatic derivation of TH_10. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TH_10"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AR_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AR_1",
            comment: "Axiomatic derivation of AR_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AR_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AR_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AR_2",
            comment: "Axiomatic derivation of AR_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AR_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AR_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AR_3",
            comment: "Axiomatic derivation of AR_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AR_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_AR_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_AR_4",
            comment: "Axiomatic derivation of AR_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AR_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 31: prf_AR_5 retyped from AxiomaticDerivation to EmpiricalVerification
        // AR_5's forAll is 'empirical, Q0--Q4' — a computationally-verified bound
        Individual {
            id: "https://uor.foundation/proof/prf_AR_5",
            type_: "https://uor.foundation/proof/EmpiricalVerification",
            label: "prf_AR_5",
            comment: "Empirical verification of AR_5: greedy vs adiabatic cost \
                      difference is at most 5% for n ≤ 16. Verified by exhaustive \
                      enumeration at Q0 through Q4.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/AR_5"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/quantumLevelRange",
                    IndividualValue::Str("Q0-Q4"),
                ),
                (
                    "https://uor.foundation/proof/verificationMethod",
                    IndividualValue::Str("exhaustive enumeration over Z/(2^n)Z for n=8,16,32,64"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_PD_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PD_1",
            comment: "Axiomatic derivation of PD_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PD_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_PD_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PD_2",
            comment: "Axiomatic derivation of PD_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PD_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_PD_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PD_3",
            comment: "Axiomatic derivation of PD_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PD_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_PD_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PD_4",
            comment: "Axiomatic derivation of PD_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PD_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_PD_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PD_5",
            comment: "Axiomatic derivation of PD_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PD_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_RC_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_RC_1",
            comment: "Axiomatic derivation of RC_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/RC_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_RC_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_RC_2",
            comment: "Axiomatic derivation of RC_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/RC_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_RC_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_RC_3",
            comment: "Axiomatic derivation of RC_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/RC_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_RC_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_RC_4",
            comment: "Axiomatic derivation of RC_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/RC_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_RC_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_RC_5",
            comment: "Axiomatic derivation of RC_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/RC_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_1",
            comment: "Axiomatic derivation of DC_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_2",
            comment: "Axiomatic derivation of DC_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_3",
            comment: "Axiomatic derivation of DC_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_4",
            comment: "Axiomatic derivation of DC_4. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_5",
            comment: "Axiomatic derivation of DC_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_6",
            comment: "Axiomatic derivation of DC_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_7",
            comment: "Axiomatic derivation of DC_7. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_8",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_8",
            comment: "Axiomatic derivation of DC_8. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_9",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_9",
            comment: "Axiomatic derivation of DC_9. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_9"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_10",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_10",
            comment: "Axiomatic derivation of DC_10. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_10"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_DC_11",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_DC_11",
            comment: "Axiomatic derivation of DC_11. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/DC_11"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_HA_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_HA_1",
            comment: "Axiomatic derivation of HA_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/HA_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_HA_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_HA_2",
            comment: "Axiomatic derivation of HA_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/HA_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_HA_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_HA_3",
            comment: "Axiomatic derivation of HA_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/HA_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IT_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IT_2",
            comment: "Axiomatic derivation of IT_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IT_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IT_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IT_3",
            comment: "Axiomatic derivation of IT_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IT_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IT_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IT_6",
            comment: "Axiomatic derivation of IT_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IT_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IT_7a",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IT_7a",
            comment: "Axiomatic derivation of IT_7a. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IT_7a"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IT_7b",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IT_7b",
            comment: "Axiomatic derivation of IT_7b. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IT_7b"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IT_7c",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IT_7c",
            comment: "Axiomatic derivation of IT_7c. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IT_7c"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_IT_7d",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_IT_7d",
            comment: "Axiomatic derivation of IT_7d. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/IT_7d"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_psi_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_psi_1",
            comment: "Axiomatic derivation of psi_1. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/psi_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_psi_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_psi_2",
            comment: "Axiomatic derivation of psi_2. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/psi_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_psi_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_psi_3",
            comment: "Axiomatic derivation of psi_3. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/psi_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_psi_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_psi_5",
            comment: "Axiomatic derivation of psi_5. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/psi_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_psi_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_psi_6",
            comment: "Axiomatic derivation of psi_6. Holds at all quantum levels \
                      by definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/psi_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Axiomatic derivations for homology identities
        Individual {
            id: "https://uor.foundation/proof/prf_boundarySquaredZero",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_boundarySquaredZero",
            comment: "Axiomatic derivation of homology:boundarySquaredZero. Holds at all quantum \
                      levels by topological reasoning.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/homology/boundarySquaredZero"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_psi_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_psi_4",
            comment: "Axiomatic derivation of homology:psi_4. Holds at all quantum \
                      levels by topological reasoning.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/homology/psi_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_indexBridge",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_indexBridge",
            comment: "Axiomatic derivation of homology:indexBridge. Holds at all quantum \
                      levels by topological reasoning.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/homology/indexBridge"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Axiomatic derivations for cohomology identities
        Individual {
            id: "https://uor.foundation/proof/prf_coboundarySquaredZero",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_coboundarySquaredZero",
            comment: "Axiomatic derivation of cohomology:coboundarySquaredZero. Holds at all \
                      quantum levels by topological reasoning.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cohomology/coboundarySquaredZero",
                    ),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_deRhamDuality",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_deRhamDuality",
            comment: "Axiomatic derivation of cohomology:deRhamDuality. Holds at all \
                      quantum levels by topological reasoning.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/cohomology/deRhamDuality"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_sheafCohomologyBridge",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_sheafCohomologyBridge",
            comment: "Axiomatic derivation of cohomology:sheafCohomologyBridge. Holds at all \
                      quantum levels by topological reasoning.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cohomology/sheafCohomologyBridge",
                    ),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_localGlobalPrinciple",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_localGlobalPrinciple",
            comment: "Axiomatic derivation of cohomology:localGlobalPrinciple. Holds at all \
                      quantum levels by topological reasoning.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cohomology/localGlobalPrinciple",
                    ),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Gap C: Surface Symmetry proof
        Individual {
            id: "https://uor.foundation/proof/prf_surfaceSymmetry",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_surfaceSymmetry",
            comment: "Axiomatic derivation of the Surface Symmetry Theorem. Holds at all \
                      quantum levels: the composite P∘Π∘G is a well-typed morphism whenever \
                      G and P share the same state:Frame. Follows from the definition of \
                      the shared-frame condition and the type-equivalence algebra.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/surfaceSymmetry"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 25: Completeness Certification proofs (CC_1–CC_5)
        Individual {
            id: "https://uor.foundation/proof/prf_CC_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CC_1",
            comment: "Proof that a CompleteType T satisfies: resolution(x, T) terminates \
                      in O(1) for all x ∈ R_n. Follows from IT_7d: when χ(N(C)) = n and \
                      all β_k = 0, the resolver has no topological obstructions.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CC_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CC_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CC_2",
            comment: "Proof that the ψ pipeline is monotone: each constraint application \
                      cannot increase the fiber deficit. Derived from the definition \
                      of the partition refinement order.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CC_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CC_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CC_3",
            comment: "Proof that a CompletenessCertificate implies CompleteType: the \
                      certificate attestation is only issued when IT_7d holds, by \
                      construction of the CompletenessResolver.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CC_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CC_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CC_4",
            comment: "Proof that the CompletenessAuditTrail witnessCount equals the number \
                      of CompletenessWitness records in the trail. Structural invariant \
                      of the audit accumulation protocol.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CC_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CC_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CC_5",
            comment: "Proof that the ConstraintNerve nerve computation is deterministic: \
                      the same constraint set always produces the same nerve topology. \
                      Follows from the nerve functor being a functor (functoriality).",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CC_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 26: Quantum Level Scaling proofs (QL_1–QL_7)
        Individual {
            id: "https://uor.foundation/proof/prf_QL_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QL_1",
            comment: "Universal proof that neg(bnot(x)) = succ(x) in Z/(2^n)Z for all \
                      n ≥ 1. Derived symbolically from ring axioms: bnot is bitwise \
                      complement, neg is two's complement negation, succ is modular \
                      increment. The critical identity in universal form.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QL_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QL_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QL_2",
            comment: "Universal proof that the ring carrier set size is exactly 2^n for \
                      all n ≥ 1. Follows from the definition of Z/(2^n)Z.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QL_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QL_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QL_3",
            comment: "Universal proof that Landauer erasure cost scales as n × k_B T ln 2 \
                      at quantum level n. Follows from the thermodynamic interpretation: \
                      each bit erased from an n-bit ring costs k_B T ln 2.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QL_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QL_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QL_4",
            comment: "Universal proof that the dihedral group D_{2^n} action on Z/(2^n)Z \
                      is faithful for all n ≥ 1. The stabilizer of any element is trivial \
                      under the full dihedral action.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QL_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QL_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QL_5",
            comment: "Universal proof that canonical form rewriting terminates at all \
                      quantum levels. The rewriting system is terminating by lexicographic \
                      descent on the term complexity measure.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QL_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QL_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QL_6",
            comment: "Universal proof that the completeness criterion χ(N(C)) = n \
                      generalizes to all quantum levels. Derived from the definition \
                      of the nerve construction and the Euler characteristic formula \
                      for simplicial complexes.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QL_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QL_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QL_7",
            comment: "Universal proof of the ring topology Euler characteristic identity: \
                      χ = 1 − n at quantum level n. Derived from the CW decomposition \
                      of the n-dimensional torus formed by the ring's cyclic group action.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QL_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 27: Session-Scoped Resolution proofs (SR_1–SR_5)
        Individual {
            id: "https://uor.foundation/proof/prf_SR_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SR_1",
            comment: "Proof of binding monotonicity: freeCount(B_{i+1}) ≤ freeCount(B_i) \
                      for all i in a Session. Follows from the definition of the \
                      BindingAccumulator: each appended binding either pins fibers or \
                      is a no-op; it never frees them.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SR_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SR_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SR_2",
            comment: "Proof that the empty session is the identity element of the session \
                      algebra: freeCount(B_0) = total fiber space. The empty accumulator \
                      has no pinned fibers by definition.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SR_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SR_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SR_3",
            comment: "Proof of session convergence: a session terminates iff freeCount \
                      reaches its minimum (the maximum pinned by the given constraint set). \
                      Follows from the compactness of the fiber space and monotonicity.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SR_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SR_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SR_4",
            comment: "Proof that disjoint bindings compose without fiber conflict: if two \
                      bindings address disjoint fiber sets, their composition is \
                      well-defined and their union is also a valid binding.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SR_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SR_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SR_5",
            comment: "Proof of contradiction detection correctness: ContradictionBoundary \
                      fires iff there exist bindings b, b' in the same Context with the \
                      same address, different datum, and same constraint. This is the \
                      minimal condition for type contradiction.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SR_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 28: ψ-Pipeline Inversion proof coverage
        Individual {
            id: "https://uor.foundation/proof/prf_TS_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TS_1",
            comment: "Proof of nerve realisability: for any target profile with χ* ≤ n and \
                      β₀* = 1, there exists a ConstrainedType whose constraint nerve realises \
                      the target. Follows from the constructive synthesis algorithm.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TS_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TS_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TS_2",
            comment: "Proof of minimal basis bound: the MinimalConstraintBasis for the IT_7d \
                      target has size exactly n. Follows from the fiber-by-fiber construction \
                      and the minimality criterion.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TS_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TS_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TS_3",
            comment: "Proof of synthesis monotonicity: adding a constraint never decreases \
                      the Euler characteristic of the constraint nerve. Follows from the \
                      nerve inclusion principle.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TS_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TS_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TS_4",
            comment: "Proof of synthesis convergence: the TypeSynthesisResolver terminates in \
                      at most n steps. Follows from monotonicity (TS_3) and the finite fiber \
                      budget bound.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TS_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TS_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TS_5",
            comment: "Proof of synthesis-certification duality: a SynthesizedType achieves \
                      IT_7d iff the CompletenessResolver certifies it as CompleteType. The \
                      duality follows from the shared topological criterion.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TS_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TS_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TS_6",
            comment: "Proof of Jacobian-guided synthesis efficiency: the Jacobian oracle reduces \
                      expected steps from O(n²) to O(n log n). Follows from the information \
                      content of the Jacobian at each synthesis step.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TS_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TS_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TS_7",
            comment: "Proof of unreachable signatures: β₀ = 0 is unreachable by any non-empty \
                      ConstrainedType. Follows from the nerve connectedness of non-empty \
                      constraint sets.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TS_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 29/41: Quantum Level Spectral Sequence proof coverage
        // (reclassified from AxiomaticDerivation to InductiveProof in Amendment 41)
        Individual {
            id: "https://uor.foundation/proof/prf_QLS_1",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_QLS_1",
            comment: "Proof of lift unobstructedness criterion: QuantumLift T' is CompleteType \
                      iff the spectral sequence collapses at E_2. Follows from the Leray \
                      spectral sequence of the quantum level extension.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QLS_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_1"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_6"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(0),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QLS_2",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_QLS_2",
            comment: "Proof of obstruction localisation: a non-trivial LiftObstruction is \
                      localised to a specific fiber at bit position n+1. Follows from the \
                      local-to-global structure of the constraint nerve.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QLS_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_2"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_2"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(0),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QLS_3",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_QLS_3",
            comment: "Proof of monotone lifting: basisSize(T') = basisSize(T) + 1 for trivially \
                      obstructed lifts. Follows from the minimal basis construction at Q_{n+1}.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QLS_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_3"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_3"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(0),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QLS_4",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_QLS_4",
            comment: "Proof of spectral sequence convergence bound: the spectral sequence \
                      converges by page E_{d+2} for depth-d configurations. Follows from \
                      the filtration length of the constraint nerve chain complex.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QLS_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_4"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_4"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(0),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QLS_5",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_QLS_5",
            comment: "Proof of universal identity preservation under quantum lifts: every \
                      universallyValid identity holds in the lifted ring. Follows from the \
                      universal validity definition and ring extension properties.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QLS_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_5"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_6"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(0),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QLS_6",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_QLS_6",
            comment: "Proof of ψ-pipeline universality for quantum lifts: the ψ-pipeline \
                      produces a valid ChainComplex for any QuantumLift. Follows from the \
                      functorial construction of the chain complex.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QLS_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_6"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_6"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(0),
                ),
            ],
        },
        // Amendment 30: Monodromy Observables proof coverage
        Individual {
            id: "https://uor.foundation/proof/prf_MN_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MN_1",
            comment: "Proof of holonomy group containment: HolonomyGroup(T) ≤ D_{2^n}. Follows \
                      from the fact that all constraint applications are dihedral group elements.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MN_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MN_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MN_2",
            comment: "Proof of additive flatness: additive constraints (ResidueConstraint, \
                      DepthConstraint) generate only the identity in D_{2^n}. Follows from \
                      the additive structure of the dihedral action.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MN_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MN_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MN_3",
            comment: "Proof of dihedral generation: neg and bnot together generate D_{2^n}. \
                      Follows from the standard presentation of the dihedral group by \
                      involutions.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MN_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MN_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MN_4",
            comment: "Proof of holonomy-Betti implication: non-trivial holonomy implies β₁ ≥ 1. \
                      Follows from the fact that a non-trivial monodromy requires a topological \
                      loop in the constraint nerve.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MN_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MN_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MN_5",
            comment: "Proof of CompleteType holonomy: IT_7d (β₁ = 0) implies trivial holonomy \
                      (FlatType). Follows from MN_4 contrapositive: trivial holonomy ← β₁ = 0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MN_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MN_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MN_6",
            comment: "Proof of monodromy composition: the monodromy map is a group homomorphism \
                      from the loop space to D_{2^n}. Follows from the composition of dihedral \
                      group elements.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MN_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MN_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MN_7",
            comment: "Proof of TwistedType obstruction class: a TwistedType always contributes \
                      a non-zero class to H²(N(C(T')); ℤ/2ℤ) for any QuantumLift T'. Follows \
                      from MN_4 and the obstruction theory of dihedral torsors.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MN_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 31: PT_ and ST_ proof individuals
        Individual {
            id: "https://uor.foundation/proof/prf_PT_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PT_1",
            comment: "Proof of PT_1: product type fiber additivity. fiberBudget(A × B) = \
                      fiberBudget(A) + fiberBudget(B). Follows from the definition of \
                      ProductType as an independent concatenation of fiber spaces.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PT_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_PT_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PT_2",
            comment: "Proof of PT_2: product type partition factorisation. \
                      partition(A × B) = partition(A) ⊗ partition(B). Follows from the \
                      tensor product structure of constraint nerves over independent \
                      fiber spaces.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PT_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_PT_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PT_3",
            comment: "Proof of PT_3: product type Euler characteristic additivity. \
                      χ(N(C(A × B))) = χ(N(C(A))) + χ(N(C(B))). Follows from the \
                      Künneth formula applied to the join of constraint nerves.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PT_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_PT_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PT_4",
            comment: "Proof of PT_4: product type entropy additivity. \
                      S(A × B) = S(A) + S(B). Follows from PT_1 (fiber additivity) \
                      and TH_1 (S = freeCount × ln 2).",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PT_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_ST_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_ST_1",
            comment: "Proof of ST_1: sum type fiber budget maximum. \
                      fiberBudget(A + B) = max(fiberBudget(A), fiberBudget(B)). \
                      Follows from SumType requiring capacity for the larger variant.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/ST_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_ST_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_ST_2",
            comment: "Proof of ST_2: sum type entropy. \
                      S(A + B) = ln 2 + max(S(A), S(B)). The ln 2 term accounts \
                      for the variant discriminant bit; the max reflects that only \
                      one variant is active at a time.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/ST_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 33: Saturated Context Limit proof individuals
        Individual {
            id: "https://uor.foundation/proof/prf_SC_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SC_1",
            comment: "Proof of SC_1: context temperature. T_ctx(C) = \
                      freeCount(C) × ln 2 / n. Derived from TH_1 normalized per fiber.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SC_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SC_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SC_2",
            comment: "Proof of SC_2: saturation degree. σ(C) = (n − freeCount(C)) / n. \
                      Definitional identity.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SC_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SC_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SC_3",
            comment: "Proof of SC_3: saturation monotonicity. Corollary of SR_1 \
                      through order-reversing SC_2.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SC_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SC_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SC_4",
            comment: "Proof of SC_4: ground state equivalence. Four equivalent \
                      conditions for full saturation derived from SC_2, TH_1, SC_1.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SC_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SC_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SC_5",
            comment: "Proof of SC_5: O(1) resolution guarantee at saturation. \
                      Derived from SR_2 and FiberBudget.isClosed at freeCount = 0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SC_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SC_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SC_6",
            comment: "Proof of SC_6: pre-reduction of effective budget. Derived from \
                      session-scoped fiber reduction at partial saturation.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SC_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SC_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SC_7",
            comment: "Proof of SC_7: thermodynamic cooling cost. n fiber-closures \
                      at Landauer cost each via SR_1 + TH_4.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SC_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 34: Morphospace Achievability proof individuals
        Individual {
            id: "https://uor.foundation/proof/prf_MS_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MS_1",
            comment: "Proof of MS_1: connectivity lower bound β₀ ≥ 1. \
                      Formalisation of TS_7.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MS_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MS_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MS_2",
            comment: "Proof of MS_2: Euler capacity ceiling χ ≤ n. Derived from \
                      TS_1 constraint nerve dimension bound.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MS_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MS_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MS_3",
            comment: "Proof of MS_3: Betti monotonicity under constraint addition. \
                      Formalisation of TS_3.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MS_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MS_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MS_4",
            comment: "Proof of MS_4: level-relative achievability. Derived from \
                      QuantumLift construction (Amendment 29).",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MS_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MS_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MS_5",
            comment: "Proof of MS_5: empirical completeness convergence. \
                      Convergence statement for EmpiricalVerification accumulation.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MS_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 35: Computational Geodesic proof individuals
        Individual {
            id: "https://uor.foundation/proof/prf_GD_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_GD_1",
            comment: "Proof of GD_1: geodesic condition. Dual condition from \
                      AR_1 ordering + DC_10 selection.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/GD_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_GD_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_GD_2",
            comment: "Proof of GD_2: geodesic entropy bound. Each step on a \
                      geodesic erases exactly ln 2 nats.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/GD_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_GD_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_GD_3",
            comment: "Proof of GD_3: total geodesic cost equals Landauer bound \
                      TH_4 with equality.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/GD_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_GD_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_GD_4",
            comment: "Proof of GD_4: geodesic uniqueness up to step-order \
                      equivalence. Equal-J_k permutations are interchangeable.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/GD_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_GD_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_GD_5",
            comment: "Proof of GD_5: subgeodesic detectability via step-by-step \
                      J_k check.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/GD_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 36: Measurement Boundary proof individuals
        Individual {
            id: "https://uor.foundation/proof/prf_QM_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QM_1",
            comment: "Proof of QM_1: von Neumann–Landauer bridge. S_vN equals \
                      Landauer erasure cost at the Landauer temperature β* = ln 2.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QM_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QM_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QM_2",
            comment: "Proof of QM_2: measurement as fiber topology change. \
                      Projective collapse ≅ classical ResidueConstraint pinning.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QM_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QM_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QM_3",
            comment: "Proof of QM_3: superposition entropy bound. 0 ≤ S_vN ≤ ln 2 \
                      for single-fiber superpositions.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QM_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QM_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QM_4",
            comment: "Proof of QM_4: collapse idempotence. Re-measurement of a \
                      collapsed state is a no-op.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QM_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 37: Gap closure proof individuals (14)
        Individual {
            id: "https://uor.foundation/proof/prf_QM_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QM_5",
            comment: "Proof of QM_5: amplitude normalization (Born rule). \
                      Σ|αᵢ|² = 1 for well-formed SuperposedFiberState.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QM_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_RC_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_RC_6",
            comment: "Proof of RC_6: amplitude renormalization. Division by norm \
                      yields a normalized SuperposedFiberState.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/RC_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FPM_8",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FPM_8",
            comment: "Proof of FPM_8: partition exhaustiveness. The four component \
                      cardinalities sum to 2ⁿ.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FPM_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_FPM_9",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_FPM_9",
            comment: "Proof of FPM_9: exterior membership criterion. x ∈ Ext(T) \
                      iff x ∉ carrier(T).",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/FPM_9"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MN_8",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MN_8",
            comment: "Proof of MN_8: holonomy classification covering. Every \
                      ConstrainedType is flat xor twisted.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MN_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QL_8",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QL_8",
            comment: "Proof of QL_8: quantum level chain inverse. levelSuccessor \
                      is the left inverse of nextLevel.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QL_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_D_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_D_7",
            comment: "Proof of D_7: dihedral composition rule from the semidirect \
                      product presentation.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/D_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SP_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SP_1",
            comment: "Proof of SP_1: classical embedding. Superposition resolution \
                      of a classical datum reduces to classical resolution.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SP_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SP_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SP_2",
            comment: "Proof of SP_2: collapse–resolve commutativity. The collapse \
                      and resolve operations commute.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SP_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SP_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SP_3",
            comment: "Proof of SP_3: amplitude preservation. The SuperpositionResolver \
                      preserves the normalized amplitude vector.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SP_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SP_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SP_4",
            comment: "Proof of SP_4: Born rule outcome probability. P(collapse to \
                      fiber k) = |α_k|².",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SP_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_PT_2a",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PT_2a",
            comment: "Proof of PT_2a: product type partition tensor. Π(A × B) = \
                      PartitionProduct(Π(A), Π(B)).",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PT_2a"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_PT_2b",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_PT_2b",
            comment: "Proof of PT_2b: sum type partition coproduct. Π(A + B) = \
                      PartitionCoproduct(Π(A), Π(B)).",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/PT_2b"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_GD_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_GD_6",
            comment: "Proof of GD_6: geodesic predicate decomposition. isGeodesic = \
                      isAR1Ordered ∧ isDC10Selected.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/GD_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 34: Formal morphospace boundary individuals
        Individual {
            id: "https://uor.foundation/proof/iw_beta0_bound",
            type_: "https://uor.foundation/proof/ImpossibilityWitness",
            label: "iw_beta0_bound",
            comment: "Impossibility witness for MS_1: β₀ = 0 is forbidden for \
                      any non-empty ConstrainedType because the constraint nerve \
                      is always connected.",
            properties: &[
                (
                    "https://uor.foundation/proof/forbidsSignature",
                    IndividualValue::Str("β₀ = 0"),
                ),
                (
                    "https://uor.foundation/proof/impossibilityReason",
                    IndividualValue::Str(
                        "β₀ = 0 violates MS_1: constraint nerve of non-empty set is connected",
                    ),
                ),
                (
                    "https://uor.foundation/proof/impossibilityDomain",
                    IndividualValue::IriRef("https://uor.foundation/op/Pipeline"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/iw_chi_ceiling",
            type_: "https://uor.foundation/proof/ImpossibilityWitness",
            label: "iw_chi_ceiling",
            comment: "Impossibility witness for MS_2: χ > n is forbidden at \
                      quantum level n. The Euler characteristic cannot exceed \
                      the quantum level.",
            properties: &[
                (
                    "https://uor.foundation/proof/forbidsSignature",
                    IndividualValue::Str("χ > n"),
                ),
                (
                    "https://uor.foundation/proof/impossibilityReason",
                    IndividualValue::Str(
                        "χ > n violates MS_2: Euler characteristic bounded by quantum level",
                    ),
                ),
                (
                    "https://uor.foundation/proof/impossibilityDomain",
                    IndividualValue::IriRef("https://uor.foundation/op/Algebraic"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/mr_completeness_target",
            type_: "https://uor.foundation/proof/MorphospaceRecord",
            label: "mr_completeness_target",
            comment: "Morphospace record: the IT_7d completeness target χ = n is \
                      achievable (sits at the ceiling of MS_2).",
            properties: &[
                (
                    "https://uor.foundation/proof/boundaryType",
                    IndividualValue::IriRef("https://uor.foundation/observable/Achievable"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/mr_connectivity_lower",
            type_: "https://uor.foundation/proof/MorphospaceRecord",
            label: "mr_connectivity_lower",
            comment: "Morphospace record: the connectivity lower bound β₀ ≥ 1 \
                      marks the forbidden region from below.",
            properties: &[
                (
                    "https://uor.foundation/proof/boundaryType",
                    IndividualValue::IriRef("https://uor.foundation/observable/Forbidden"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 41: QT_ tower identity proofs
        Individual {
            id: "https://uor.foundation/proof/prf_QT_1",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_QT_1",
            comment: "Proof of tower chain validity by induction on chain length.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QT_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_1"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_6"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(0),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QT_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QT_2",
            comment: "Proof of obstruction count bound: direct from QLS_2 localization.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QT_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QT_3",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_QT_3",
            comment: "Proof of resolved basis size formula by induction on chain \
                      length.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QT_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QT_3"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QLS_3"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(0),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QT_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QT_4",
            comment: "Proof of flat tower characterization: isFlat iff trivial \
                      holonomy at every step.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QT_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QT_5",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_QT_5",
            comment: "Proof of LiftChainCertificate existence by induction on \
                      tower height.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QT_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QT_5"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_QT_1"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(0),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QT_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QT_6",
            comment: "Proof of single-step reduction: QT_3 with chainLength=1 \
                      reduces to QLS_3.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QT_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QT_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QT_7",
            comment: "Proof of flat chain basis size formula.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QT_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 44: Structural Gap Closure proofs
        // G7: CarryConstraint fiber-pinning map
        Individual {
            id: "https://uor.foundation/proof/prf_CC_PINS",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CC_PINS",
            comment: "Proof of CC_PINS: carry-constraint fiber-pinning map \
                      follows from ring carry propagation rule.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CC_PINS"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CC_COST_FIBER",
            type_: "https://uor.foundation/proof/ComputationCertificate",
            label: "prf_CC_COST_FIBER",
            comment: "Computation certificate for CC_COST_FIBER: exhaustive \
                      enumeration at Q0 confirms |pinsFibers| = popcount + 1.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CC_COST_FIBER"),
                ),
                (
                    "https://uor.foundation/proof/atQuantumLevel",
                    IndividualValue::IriRef("https://uor.foundation/schema/Q0"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // G1: Nerve joint satisfiability
        Individual {
            id: "https://uor.foundation/proof/prf_jsat_RR",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_jsat_RR",
            comment: "Proof of jsat_RR: CRT joint satisfiability follows \
                      from the Chinese Remainder Theorem.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/jsat_RR"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_jsat_CR",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_jsat_CR",
            comment: "Proof of jsat_CR: carry-residue joint satisfiability \
                      follows from the carry stopping rule and residue class \
                      intersection.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/jsat_CR"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_jsat_CC",
            type_: "https://uor.foundation/proof/ComputationCertificate",
            label: "prf_jsat_CC",
            comment: "Computation certificate for jsat_CC: bit-pattern \
                      exhaustive enumeration at Q0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/jsat_CC"),
                ),
                (
                    "https://uor.foundation/proof/atQuantumLevel",
                    IndividualValue::IriRef("https://uor.foundation/schema/Q0"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // G2: DihedralElement inverse and order
        Individual {
            id: "https://uor.foundation/proof/prf_D_8",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_D_8",
            comment: "Proof of D_8: dihedral inverse formula follows from \
                      D_5 group presentation and D_7 composition rule.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/D_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_D_9",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_D_9",
            comment: "Proof of D_9: reflection order 2 follows from D_7 \
                      composition: (r^k s)(r^k s) = identity.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/D_9"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // G5: Constraint language expressiveness
        Individual {
            id: "https://uor.foundation/proof/prf_EXP_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_EXP_1",
            comment: "Proof of EXP_1: monotone carrier characterization \
                      follows from fiber lattice monotonicity.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/EXP_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_EXP_2",
            type_: "https://uor.foundation/proof/ComputationCertificate",
            label: "prf_EXP_2",
            comment: "Computation certificate for EXP_2: principal filter \
                      count verified by exhaustive enumeration at Q0.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/EXP_2"),
                ),
                (
                    "https://uor.foundation/proof/atQuantumLevel",
                    IndividualValue::IriRef("https://uor.foundation/schema/Q0"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_EXP_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_EXP_3",
            comment: "Proof of EXP_3: SumType carrier is coproduct by \
                      definitional architectural decision.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/EXP_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // G4: SumType topology
        Individual {
            id: "https://uor.foundation/proof/prf_ST_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_ST_3",
            comment: "Proof of ST_3: Euler characteristic additivity for \
                      disjoint simplicial complexes.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/ST_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_ST_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_ST_4",
            comment: "Proof of ST_4: Betti number additivity via \
                      Mayer-Vietoris for disjoint union.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/ST_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_ST_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_ST_5",
            comment: "Proof of ST_5: SumType completeness transfer follows \
                      from ST_3 + ST_4 + IT_7d.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/ST_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // G3: TypeSynthesis reachability
        Individual {
            id: "https://uor.foundation/proof/prf_TS_8",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_TS_8",
            comment: "Inductive proof of TS_8: minimum constraint count for \
                      beta_1 = k is 2k + 1. Base case at k=1 requires 3 \
                      mutually overlapping constraints.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TS_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_HA_1"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_TS_4"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(1),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TS_9",
            type_: "https://uor.foundation/proof/InductiveProof",
            label: "prf_TS_9",
            comment: "Inductive proof of TS_9: TypeSynthesisResolver \
                      terminates in at most 2^n steps. Base case at n=1 has \
                      2 constraint combinations.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TS_9"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/baseCase",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_TS_1"),
                ),
                (
                    "https://uor.foundation/proof/inductiveStep",
                    IndividualValue::IriRef("https://uor.foundation/proof/prf_TS_4"),
                ),
                (
                    "https://uor.foundation/proof/validForKAtLeast",
                    IndividualValue::Int(1),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_TS_10",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_TS_10",
            comment: "Proof of TS_10: ForbiddenSignature membership follows \
                      from exhaustive enumeration bound.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/TS_10"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // G6: ObstructionChain termination
        Individual {
            id: "https://uor.foundation/proof/prf_QT_8",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QT_8",
            comment: "Proof of QT_8: ObstructionChain length bound follows \
                      from QLS_2 and spectral sequence convergence.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QT_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_QT_9",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_QT_9",
            comment: "Proof of QT_9: TowerCompletenessResolver termination \
                      follows from finite chain length and QT_8 bound.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QT_9"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // G11: Coefficient ring
        Individual {
            id: "https://uor.foundation/proof/prf_COEFF_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_COEFF_1",
            comment: "Proof of COEFF_1: Z/2Z coefficient ring is \
                      definitional, consistent with MN_7.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/COEFF_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // G9: GluingObstruction feedback
        Individual {
            id: "https://uor.foundation/proof/prf_GO_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_GO_1",
            comment: "Proof of GO_1: cohomology killing lemma for \
                      GluingObstruction feedback.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/GO_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // G8: Session saturation lifecycle bridge
        Individual {
            id: "https://uor.foundation/proof/prf_SR_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SR_6",
            comment: "Proof of SR_6: saturation re-entry free count follows \
                      from SR_1 monotone accumulation and SC_2.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SR_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SR_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SR_7",
            comment: "Proof of SR_7: saturation degree degradation follows \
                      from SC_2 definition and SR_1 monotonicity.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SR_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // G10: Amplitude index set
        Individual {
            id: "https://uor.foundation/proof/prf_QM_6",
            type_: "https://uor.foundation/proof/EmpiricalVerification",
            label: "prf_QM_6",
            comment: "Empirical verification of QM_6: amplitude index set \
                      equals monotone pinning trajectories consistent with \
                      constraints. Verified by exhaustive enumeration at Q0 \
                      through Q3.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/QM_6"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/quantumLevelRange",
                    IndividualValue::Str("Q0-Q3"),
                ),
                (
                    "https://uor.foundation/proof/verificationMethod",
                    IndividualValue::Str("exhaustive trajectory enumeration over fiber lattice"),
                ),
            ],
        },
        // Amendment 46: Certificate Issuance Coverage proofs
        Individual {
            id: "https://uor.foundation/proof/prf_CIC_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CIC_1",
            comment: "Proof of CIC_1: TransformCertificate issuance coverage.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CIC_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CIC_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CIC_2",
            comment: "Proof of CIC_2: IsometryCertificate issuance coverage.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CIC_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CIC_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CIC_3",
            comment: "Proof of CIC_3: InvolutionCertificate issuance coverage.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CIC_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CIC_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CIC_4",
            comment: "Proof of CIC_4: SaturationCertificate issuance coverage.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CIC_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CIC_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CIC_5",
            comment: "Proof of CIC_5: GeodesicCertificate issuance coverage.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CIC_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CIC_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_CIC_6",
            comment: "Proof of CIC_6: MeasurementCertificate issuance coverage.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CIC_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_CIC_7",
            type_: "https://uor.foundation/proof/EmpiricalVerification",
            label: "prf_CIC_7",
            comment: "Empirical verification of CIC_7: BornRuleVerification issuance coverage.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/CIC_7"),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/quantumLevelRange",
                    IndividualValue::Str("Q0-Q3"),
                ),
                (
                    "https://uor.foundation/proof/verificationMethod",
                    IndividualValue::Str("Born rule amplitude sum verification over fiber lattice"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_GC_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_GC_1",
            comment: "Proof of GC_1: GroundingCertificate issuance coverage.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/GC_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Amendment 48: Multi-Session Coordination proofs
        Individual {
            id: "https://uor.foundation/proof/prf_SR_8",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SR_8",
            comment: "Proof of SR_8: session composition tower consistency.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SR_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(false),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SR_9",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SR_9",
            comment: "Proof of SR_9: ContextLease fiber disjointness.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SR_9"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_SR_10",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_SR_10",
            comment: "Proof of SR_10: ExecutionPolicy confluence.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/SR_10"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MC_1",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MC_1",
            comment: "Proof of MC_1: lease partition conserves total budget.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MC_1"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MC_2",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MC_2",
            comment: "Proof of MC_2: per-lease binding monotonicity.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MC_2"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MC_3",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MC_3",
            comment: "Proof of MC_3: composition freeCount inclusion-exclusion.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MC_3"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MC_4",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MC_4",
            comment: "Proof of MC_4: disjoint-lease composition additivity.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MC_4"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MC_5",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MC_5",
            comment: "Proof of MC_5: policy-invariant final binding set.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MC_5"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MC_6",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MC_6",
            comment: "Proof of MC_6: full lease coverage implies composed saturation.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MC_6"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MC_7",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MC_7",
            comment: "Proof of MC_7: distributed O(1) resolution.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MC_7"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/proof/prf_MC_8",
            type_: "https://uor.foundation/proof/AxiomaticDerivation",
            label: "prf_MC_8",
            comment: "Proof of MC_8: parallelism bound on per-session resolution work.",
            properties: &[
                (
                    "https://uor.foundation/proof/provesIdentity",
                    IndividualValue::IriRef("https://uor.foundation/op/MC_8"),
                ),
                (
                    "https://uor.foundation/proof/universalScope",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/proof/verified",
                    IndividualValue::Bool(true),
                ),
            ],
        },
    ]
}
