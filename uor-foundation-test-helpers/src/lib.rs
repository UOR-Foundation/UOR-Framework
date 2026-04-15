//! Test-only helpers for constructing crate-internal `uor-foundation` values.
//!
//! v0.2.2 T2.5 (cleanup) deliverable. Not published to crates.io. Used as a
//! `[dev-dependencies]` path dependency by `uor-foundation-verify` and the
//! foundation's own integration tests. Re-exports the foundation's
//! `__test_helpers` module under stable test-only names.

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::missing_errors_doc)]

use uor_foundation::enforcement::{MulContext, Trace, TraceEvent, Validated, __test_helpers};

/// Test-only ctor: build a Trace from a slice of events.
#[must_use]
pub fn trace_from_events(events: &[TraceEvent]) -> Trace {
    __test_helpers::trace_from_events(events)
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
