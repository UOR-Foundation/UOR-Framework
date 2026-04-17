//! Test-only helpers for constructing crate-internal `uor-foundation` values.
//!
//! v0.2.2 T2.5 (cleanup) deliverable. Not published to crates.io. Used as a
//! `[dev-dependencies]` path dependency by `uor-foundation-verify` and the
//! foundation's own integration tests. Re-exports the foundation's
//! `__test_helpers` module under stable test-only names.
//!
//! v0.2.2 T5: provides `Fnv1aHasher16` and `Fnv1aHasher32` substrate
//! `Hasher` impls used by the conformance round-trip tests. These are
//! TEST-ONLY substrates — the foundation itself does not ship a `Hasher`
//! impl, and production deployments use a cryptographic substrate (BLAKE3
//! recommended). The test-helpers FNV-1a impls exist solely to exercise
//! the `verify_trace` round-trip property end-to-end without depending on
//! a downstream crypto crate.

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::missing_errors_doc)]

use uor_foundation::enforcement::{
    ContentFingerprint, Hasher, MulContext, Trace, TraceEvent, Validated, __test_helpers,
    FINGERPRINT_MAX_BYTES,
};

/// Test-only ctor: build a Trace from a slice of events with a
/// `ContentFingerprint::zero()` placeholder. Tests that need a non-zero
/// fingerprint use `trace_with_fingerprint` instead.
#[must_use]
pub fn trace_from_events(events: &[TraceEvent]) -> Trace {
    __test_helpers::trace_from_events(events)
}

/// v0.2.2 T5: test-only ctor that takes an explicit `witt_level_bits` and
/// `ContentFingerprint`. Used by round-trip tests that need to verify the
/// `verify_trace` fingerprint passthrough invariant.
#[must_use]
pub fn trace_with_fingerprint(
    events: &[TraceEvent],
    witt_level_bits: u16,
    content_fingerprint: ContentFingerprint,
) -> Trace {
    __test_helpers::trace_with_fingerprint(events, witt_level_bits, content_fingerprint)
}

/// Test-only ctor: build a TraceEvent.
#[must_use]
pub fn trace_event(step_index: u32, target: u128) -> TraceEvent {
    __test_helpers::trace_event(step_index, target)
}

/// Test-only ctor: build a MulContext with the given fields.
#[must_use]
pub fn mul_context(stack_budget_bytes: u64, const_eval: bool, limb_count: usize) -> MulContext {
    __test_helpers::mul_context(stack_budget_bytes, const_eval, limb_count)
}

/// Test-only ctor: wrap any `T` in a `Validated<T>` (Runtime phase).
#[must_use]
pub fn validated_runtime<T>(inner: T) -> Validated<T> {
    __test_helpers::validated_runtime(inner)
}

/// v0.2.2 T5: test-only `Hasher` impl producing a 16-byte (128-bit) FNV-1a
/// fingerprint. Used by the conformance round-trip tests to exercise the
/// `verify_trace` round-trip property at the foundation's minimum width.
///
/// Not a production substrate. Production deployments use a cryptographic
/// hash (BLAKE3 recommended). FNV-1a is non-cryptographic — the test impl
/// exists only to satisfy `Hasher::OUTPUT_BYTES >= FINGERPRINT_MIN_BYTES`
/// without pulling in a crypto dependency.
#[derive(Debug, Clone, Copy, Default)]
pub struct Fnv1aHasher16 {
    state: u128,
}

impl Hasher for Fnv1aHasher16 {
    const OUTPUT_BYTES: usize = 16;

    #[inline]
    fn initial() -> Self {
        // FNV-1a 128-bit offset basis.
        Self {
            state: 0x6c62272e07bb014262b821756295c58d,
        }
    }

    #[inline]
    fn fold_byte(self, b: u8) -> Self {
        // FNV-1a 128-bit prime.
        const PRIME: u128 = 0x0000000001000000000000000000013b;
        Self {
            state: (self.state ^ (b as u128)).wrapping_mul(PRIME),
        }
    }

    #[inline]
    fn finalize(self) -> [u8; FINGERPRINT_MAX_BYTES] {
        let mut out = [0u8; FINGERPRINT_MAX_BYTES];
        let bytes = self.state.to_be_bytes();
        let mut i = 0;
        while i < 16 {
            out[i] = bytes[i];
            i += 1;
        }
        out
    }
}

/// v0.2.2 T5: test-only `Hasher` impl producing a 32-byte (256-bit) FNV-1a
/// fingerprint. Used by the parametric-width round-trip tests to confirm
/// the `verify_trace` round-trip property is orthogonal to the chosen
/// `OUTPUT_BYTES`. The state is two parallel FNV-1a 128 lanes seeded with
/// different offset basis values; the test does not require cryptographic
/// strength, only deterministic 32-byte output.
#[derive(Debug, Clone, Copy, Default)]
pub struct Fnv1aHasher32 {
    lo: u128,
    hi: u128,
}

impl Hasher for Fnv1aHasher32 {
    const OUTPUT_BYTES: usize = 32;

    #[inline]
    fn initial() -> Self {
        Self {
            lo: 0x6c62272e07bb014262b821756295c58d,
            // Distinct seed for the high lane so the two lanes don't collide
            // for byte sequences that happen to be palindromic.
            hi: 0x9e3779b97f4a7c15f39cc0605cedc834,
        }
    }

    #[inline]
    fn fold_byte(self, b: u8) -> Self {
        const PRIME: u128 = 0x0000000001000000000000000000013b;
        Self {
            lo: (self.lo ^ (b as u128)).wrapping_mul(PRIME),
            hi: (self.hi ^ ((b as u128) ^ 0x55)).wrapping_mul(PRIME),
        }
    }

    #[inline]
    fn finalize(self) -> [u8; FINGERPRINT_MAX_BYTES] {
        let mut out = [0u8; FINGERPRINT_MAX_BYTES];
        let lo_bytes = self.lo.to_be_bytes();
        let hi_bytes = self.hi.to_be_bytes();
        let mut i = 0;
        while i < 16 {
            out[i] = lo_bytes[i];
            out[i + 16] = hi_bytes[i];
            i += 1;
        }
        out
    }
}
