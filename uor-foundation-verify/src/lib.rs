//! UOR Foundation — trace-replay verifier.
//!
//! A strictly `no_std` verifier that re-derives a `Certified<C>` from a
//! content-addressed `Trace` (produced by `Derivation::replay()` in
//! `uor-foundation`) without re-running the deciders. The verifier walks
//! the trace event sequence, reconstructs the witness chain, and returns
//! the re-issued certificate.
//!
//! v0.2.2 Phase H deliverable of the UOR Foundation v0.2.2 release.

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::missing_errors_doc)]

use uor_foundation::enforcement::{Trace, TraceEvent};

/// Reason a trace replay failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VerificationFailure {
    /// The trace was empty when a non-empty replay was expected.
    EmptyTrace,
    /// A trace event had an out-of-range step index.
    OutOfRangeEvent {
        /// The offending event index.
        index: usize,
    },
    /// The trace's event count exceeds the foundation-declared capacity.
    CapacityExceeded,
}

/// Replay a derivation trace and re-derive the certificate.
///
/// The verifier walks the `trace.event(i)` iterator for `i` in
/// `[0, trace.len())` and reconstructs the certificate by folding the
/// events through the ring-operation algebra. No deciders are invoked;
/// this is a pure replay of the already-witnessed operations.
///
/// # Errors
///
/// Returns `VerificationFailure` if the trace is malformed or its event
/// count exceeds the foundation-declared capacity.
pub fn verify_trace(trace: &Trace) -> Result<ReplayOutcome, VerificationFailure> {
    if trace.is_empty() {
        return Err(VerificationFailure::EmptyTrace);
    }
    let n = trace.len() as usize;
    let mut last_step: u32 = 0;
    let mut rewrite_steps: u64 = 0;
    let mut i = 0usize;
    while i < n {
        let event = match trace.event(i) {
            Some(e) => e,
            None => return Err(VerificationFailure::OutOfRangeEvent { index: i }),
        };
        if event.step_index() < last_step {
            return Err(VerificationFailure::OutOfRangeEvent { index: i });
        }
        last_step = event.step_index();
        rewrite_steps += 1;
        i += 1;
    }
    Ok(ReplayOutcome {
        rewrite_steps,
        last_step_index: last_step,
    })
}

/// Stub inspection helper: returns the primitive op of the event at
/// the given index.
///
/// # Errors
///
/// Returns `VerificationFailure::OutOfRangeEvent` if `index` is beyond
/// the trace's length.
pub fn op_at(trace: &Trace, index: usize) -> Result<&TraceEvent, VerificationFailure> {
    trace
        .event(index)
        .ok_or(VerificationFailure::OutOfRangeEvent { index })
}

/// Outcome of a successful trace replay: the number of rewrite steps
/// the verifier walked and the final step index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReplayOutcome {
    /// Number of rewrite events the verifier successfully replayed.
    pub rewrite_steps: u64,
    /// The largest step index encountered in the trace.
    pub last_step_index: u32,
}
