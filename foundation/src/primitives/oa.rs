// @codegen-exempt — Phase 12 hand-written verification bodies.
// Initial baseline emitted by `uor-crate`; subsequent edits
// are preserved by emit::write_file's banner check.

//! Phase 12 verification primitives for the `oa` theorem family.
//!
//! Each `verify_*` validates a `Mint{Foo}Inputs<H>` against the
//! theorem its `Mint{Foo}` witness attests, then mints the
//! witness with a content-addressed fingerprint derived from
//! `(THEOREM_IDENTITY, canonical(inputs))`. On theorem failure
//! the function returns a typed `GenericImpossibilityWitness`
//! whose IRI cites the specific failing identity.
//!
//! The Phase-12 baseline accepts every input unconditionally
//! because `Mint{Foo}Inputs<H>` is currently a `PhantomData<H>`
//! placeholder. Hand-edit each body with the per-theorem checks
//! once Phase 10b's R5 field mapping populates the inputs with
//! per-property fields.

use crate::enforcement::{ContentFingerprint, GenericImpossibilityWitness};
use crate::witness_scaffolds::{
    MintMorphismGroundingWitness, MintMorphismGroundingWitnessInputs, MintProjectionWitness,
    MintProjectionWitnessInputs, MintStateGroundingWitness, MintStateGroundingWitnessInputs,
    MintWitness, MintWitnessInputs,
};
use crate::HostTypes;

/// Deterministic 32-byte fingerprint derived from `iri` via
/// index-salted XOR fold across the full byte sequence. Every
/// IRI byte contributes to the output buffer cyclically; the
/// `i as u8` salt prevents byte-swap collisions. The fold is
/// `no_std` + `const`-friendly and avoids the host-supplied
/// `Hasher` dependency that the production mint paths use.
fn fingerprint_for_identity(iri: &str) -> ContentFingerprint {
    let mut buf = [0u8; crate::enforcement::FINGERPRINT_MAX_BYTES];
    let bytes = iri.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let pos = i % crate::enforcement::FINGERPRINT_MAX_BYTES;
        #[allow(clippy::cast_possible_truncation)]
        let salt = i as u8;
        buf[pos] ^= bytes[i].wrapping_add(salt);
        i += 1;
    }
    #[allow(clippy::cast_possible_truncation)]
    ContentFingerprint::from_buffer(buf, crate::enforcement::FINGERPRINT_MAX_BYTES as u8)
}

/// Phase-12 verification primitive for `https://uor.foundation/morphism/GroundingWitness`.
///
/// Theorem identity: `https://uor.foundation/op/surfaceSymmetry`.
///
/// Phase-12 baseline: accepts every input and mints a
/// witness with a fingerprint derived from the class
/// IRI. Replace this body with theorem-specific checks
/// once `MintMorphismGroundingWitnessInputs<H>` carries per-property fields.
///
/// # Errors
///
/// Returns a `GenericImpossibilityWitness::for_identity(IRI)`
/// citing the specific failing op-namespace identity
/// when a future hand-edited body rejects the inputs.
#[allow(unused_variables)]
pub fn verify_morphism_grounding_witness<H: HostTypes + 'static>(
    inputs: MintMorphismGroundingWitnessInputs<H>,
) -> Result<MintMorphismGroundingWitness, GenericImpossibilityWitness> {
    let _ = inputs;
    let fp = fingerprint_for_identity("https://uor.foundation/morphism/GroundingWitness");
    Ok(MintMorphismGroundingWitness::from_fingerprint(fp))
}

/// Phase-12 verification primitive for `https://uor.foundation/morphism/ProjectionWitness`.
///
/// Theorem identity: `https://uor.foundation/op/surfaceSymmetry`.
///
/// Phase-12 baseline: accepts every input and mints a
/// witness with a fingerprint derived from the class
/// IRI. Replace this body with theorem-specific checks
/// once `MintProjectionWitnessInputs<H>` carries per-property fields.
///
/// # Errors
///
/// Returns a `GenericImpossibilityWitness::for_identity(IRI)`
/// citing the specific failing op-namespace identity
/// when a future hand-edited body rejects the inputs.
#[allow(unused_variables)]
pub fn verify_morphism_projection_witness<H: HostTypes + 'static>(
    inputs: MintProjectionWitnessInputs<H>,
) -> Result<MintProjectionWitness, GenericImpossibilityWitness> {
    let _ = inputs;
    let fp = fingerprint_for_identity("https://uor.foundation/morphism/ProjectionWitness");
    Ok(MintProjectionWitness::from_fingerprint(fp))
}

/// Phase-12 verification primitive for `https://uor.foundation/morphism/Witness`.
///
/// Theorem identity: `https://uor.foundation/op/surfaceSymmetry`.
///
/// Phase-12 baseline: accepts every input and mints a
/// witness with a fingerprint derived from the class
/// IRI. Replace this body with theorem-specific checks
/// once `MintWitnessInputs<H>` carries per-property fields.
///
/// # Errors
///
/// Returns a `GenericImpossibilityWitness::for_identity(IRI)`
/// citing the specific failing op-namespace identity
/// when a future hand-edited body rejects the inputs.
#[allow(unused_variables)]
pub fn verify_morphism_witness<H: HostTypes + 'static>(
    inputs: MintWitnessInputs<H>,
) -> Result<MintWitness, GenericImpossibilityWitness> {
    let _ = inputs;
    let fp = fingerprint_for_identity("https://uor.foundation/morphism/Witness");
    Ok(MintWitness::from_fingerprint(fp))
}

/// Phase-12 verification primitive for `https://uor.foundation/state/GroundingWitness`.
///
/// Theorem identity: `https://uor.foundation/op/surfaceSymmetry`.
///
/// Phase-12 baseline: accepts every input and mints a
/// witness with a fingerprint derived from the class
/// IRI. Replace this body with theorem-specific checks
/// once `MintStateGroundingWitnessInputs<H>` carries per-property fields.
///
/// # Errors
///
/// Returns a `GenericImpossibilityWitness::for_identity(IRI)`
/// citing the specific failing op-namespace identity
/// when a future hand-edited body rejects the inputs.
#[allow(unused_variables)]
pub fn verify_state_grounding_witness<H: HostTypes + 'static>(
    inputs: MintStateGroundingWitnessInputs<H>,
) -> Result<MintStateGroundingWitness, GenericImpossibilityWitness> {
    let _ = inputs;
    let fp = fingerprint_for_identity("https://uor.foundation/state/GroundingWitness");
    Ok(MintStateGroundingWitness::from_fingerprint(fp))
}
