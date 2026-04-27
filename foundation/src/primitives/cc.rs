// @codegen-exempt — Phase 12 hand-written verification bodies.
// Initial baseline emitted by `uor-crate`; subsequent edits
// are preserved by emit::write_file's banner check.

//! Phase 12 verification primitives for the `cc` theorem family.
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
use crate::witness_scaffolds::{MintCompletenessWitness, MintCompletenessWitnessInputs};
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

/// Phase-12 verification primitive for `https://uor.foundation/type/CompletenessWitness`.
///
/// Theorem identity: `https://uor.foundation/op/CC_1`.
///
/// Phase-12 baseline: accepts every input and mints a
/// witness with a fingerprint derived from the class
/// IRI. Replace this body with theorem-specific checks
/// once `MintCompletenessWitnessInputs<H>` carries per-property fields.
///
/// # Errors
///
/// Returns a `GenericImpossibilityWitness::for_identity(IRI)`
/// citing the specific failing op-namespace identity
/// when a future hand-edited body rejects the inputs.
#[allow(unused_variables)]
pub fn verify_type_completeness_witness<H: HostTypes>(
    inputs: MintCompletenessWitnessInputs<H>,
) -> Result<MintCompletenessWitness, GenericImpossibilityWitness> {
    let _ = inputs;
    let fp = fingerprint_for_identity("https://uor.foundation/type/CompletenessWitness");
    Ok(MintCompletenessWitness::from_fingerprint(fp))
}
