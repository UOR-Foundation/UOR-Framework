//! v0.2.2 T2.5 (cleanup): round-trip verification tests for the
//! `uor-foundation-verify` crate.
//!
//! Constructs Traces via the `uor-foundation-test-helpers` crate (which
//! re-exports the foundation's `__test_helpers` back-door module), then
//! verifies them via `verify_trace` and asserts the outcome reflects the
//! input trace's structure.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use uor_foundation::enforcement::Trace;
use uor_foundation_test_helpers::{trace_event, trace_from_events};
use uor_foundation_verify::{verify_trace, VerificationFailure};

#[test]
fn empty_trace_rejects() {
    let empty = Trace::empty();
    assert_eq!(verify_trace(&empty), Err(VerificationFailure::EmptyTrace));
}

#[test]
fn single_event_trace_round_trips() {
    let event = trace_event(0, 0x1234);
    let trace = trace_from_events(&[event]);
    let outcome = verify_trace(&trace).expect("single-event trace verifies");
    assert_eq!(outcome.rewrite_steps, 1);
    assert_eq!(outcome.last_step_index, 0);
}

#[test]
fn monotonic_trace_round_trips() {
    let events = [
        trace_event(0, 0x10),
        trace_event(1, 0x20),
        trace_event(2, 0x30),
    ];
    let trace = trace_from_events(&events);
    let outcome = verify_trace(&trace).expect("monotonic trace verifies");
    assert_eq!(outcome.rewrite_steps, 3);
    assert_eq!(outcome.last_step_index, 2);
}

#[test]
fn out_of_order_trace_rejects() {
    let events = [trace_event(5, 0x10), trace_event(2, 0x20)];
    let trace = trace_from_events(&events);
    assert_eq!(
        verify_trace(&trace),
        Err(VerificationFailure::OutOfRangeEvent { index: 1 })
    );
}

#[test]
fn distinct_traces_produce_distinct_outcomes() {
    let trace_a = trace_from_events(&[trace_event(0, 0xAA), trace_event(1, 0xBB)]);
    let trace_b = trace_from_events(&[
        trace_event(0, 0x11),
        trace_event(1, 0x22),
        trace_event(2, 0x33),
    ]);
    let outcome_a = verify_trace(&trace_a).expect("trace_a verifies");
    let outcome_b = verify_trace(&trace_b).expect("trace_b verifies");
    assert_ne!(outcome_a.rewrite_steps, outcome_b.rewrite_steps);
    assert_ne!(outcome_a.last_step_index, outcome_b.last_step_index);
}
