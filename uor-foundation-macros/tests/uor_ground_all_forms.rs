//! v0.2.1 Phase 8a.2: smoke test for every `uor_ground!` declaration form.
//!
//! The macro supports 7 forms (compile_unit, dispatch_rule, witt_level,
//! predicate, parallel, stream, lease). v0.2.1 Phase 7c.2 wired up all of
//! them but only compile_unit had test coverage. This file asserts that each
//! form compiles and emits the expected associated constants / types.
//!
//! compile_unit is exercised separately by `uor_ground.rs` (real pipeline
//! invocation with `Grounded<T>`); this file covers the other six forms.

#![allow(dead_code, non_camel_case_types)]

use uor_foundation_macros::uor_ground;

// ---------- dispatch_rule ----------

uor_ground! {
    dispatch_rule my_dispatch_rule {
        predicate: "https://uor.foundation/predicate/Is2SatShape";
        target_resolver: TwoSatDecider;
        priority: 0;
    }
}

#[test]
fn dispatch_rule_emits_iri_constants() {
    assert_eq!(
        my_dispatch_rule::PREDICATE_IRI,
        "https://uor.foundation/predicate/Is2SatShape"
    );
    assert_eq!(
        my_dispatch_rule::TARGET_RESOLVER_IRI,
        "https://uor.foundation/resolver/TwoSatDecider"
    );
    assert_eq!(my_dispatch_rule::PRIORITY, 0);
}

// ---------- witt_level ----------

uor_ground! {
    witt_level MY_W16 {
        bit_width: 16;
        cycle_size: 65536;
        predecessor_level: "W8";
    }
}

#[test]
fn witt_level_emits_wittlevel_const() {
    assert_eq!(MY_W16.witt_length(), 16);
}

// ---------- predicate ----------

uor_ground! {
    predicate MyPredicate {
        input_type: "https://uor.foundation/type/ConstrainedType";
        evaluator: "true";
        termination_witness: "https://uor.foundation/proof/AxiomaticDerivation";
    }
}

#[test]
fn predicate_emits_termination_witness_iri() {
    assert_eq!(
        MyPredicate::TERMINATION_WITNESS_IRI,
        "https://uor.foundation/proof/AxiomaticDerivation"
    );
}

// ---------- parallel ----------

uor_ground! {
    parallel MyParallel {
        site_partition: "https://uor.foundation/partition/TrivialPartition";
        disjointness_witness: "https://uor.foundation/proof/Trivial";
    }
}

#[test]
fn parallel_emits_partition_and_witness_iris() {
    assert_eq!(
        MyParallel::SITE_PARTITION_IRI,
        "https://uor.foundation/partition/TrivialPartition"
    );
    assert_eq!(
        MyParallel::DISJOINTNESS_WITNESS_IRI,
        "https://uor.foundation/proof/Trivial"
    );
}

// ---------- stream ----------

uor_ground! {
    stream MyStream {
        unfold_seed: "0";
        step: "n + 1";
        productivity_witness: "https://uor.foundation/proof/Cofixpoint";
    }
}

#[test]
fn stream_emits_seed_step_and_witness() {
    assert!(MyStream::UNFOLD_SEED_SRC.contains('0'));
    assert!(MyStream::STEP_SRC.contains("n + 1"));
    assert_eq!(
        MyStream::PRODUCTIVITY_WITNESS_IRI,
        "https://uor.foundation/proof/Cofixpoint"
    );
}

// ---------- lease ----------

uor_ground! {
    lease MyLease {
        linear_site: 7;
        lease_scope: "https://uor.foundation/state/SessionScope";
    }
}

#[test]
fn lease_emits_linear_site_and_scope() {
    assert_eq!(MyLease::LINEAR_SITE, 7);
    assert_eq!(
        MyLease::LEASE_SCOPE,
        "https://uor.foundation/state/SessionScope"
    );
}
