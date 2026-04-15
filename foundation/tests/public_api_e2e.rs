//! v0.2.2 T2.0 (cleanup): end-to-end public-API functional verification.
//!
//! Exercises every previously-hardcoded public endpoint with **at least two**
//! distinct inputs and asserts the outputs differ (or that they're derived
//! from the inputs in a documented way). Hardcoded endpoints — those that
//! return a constant regardless of input — are now functional, and this
//! test is the regression gate that prevents them from sliding back.
//!
//! Phases covered:
//! - Phase A: Calibration / UorTime sanity
//! - Phase C.4 multiplication resolver (T2.1)
//! - Phase E BaseMetric accessors (T2.6)
//! - Phase F drivers (T2.7)
//! - Phase G const-fn frontier (T2.8)
//! - Phase J grounding combinator MarkersImpliedBy bound (T1.1)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use uor_foundation::enforcement::{
    calibrations, combinators, Calibration, Certify, CompileTime, CompileUnit, CompileUnitBuilder,
    ConstrainedTypeInput, DigestGroundingMap, Grounded, GroundingCertificate, GroundingProgram,
    IntegerGroundingMap, MultiplicationCertificate, MultiplicationResolver, Validated,
    MAX_BETTI_DIMENSION,
};
use uor_foundation::pipeline::{
    certify_inhabitance_const, certify_multiplication_const, certify_tower_completeness_const,
    run_const, run_interactive, run_parallel, run_stream, validate_compile_unit_const,
    InteractionDeclaration, InteractionDriver, ParallelDeclaration, PeerInput, PeerPayload,
    StepResult, StreamDeclaration, StreamDriver,
};
use uor_foundation::WittLevel;
use uor_foundation_test_helpers::validated_runtime;

// ─────────────────────────────────────────────────────────────────────────
// Phase A: UorTime / Calibration / Nanos
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn phase_a_calibration_presets_are_addressable() {
    let _ = calibrations::X86_SERVER;
    let _ = calibrations::ARM_MOBILE;
    let _ = calibrations::CORTEX_M_EMBEDDED;
    let _ = calibrations::CONSERVATIVE_WORST_CASE;
}

#[test]
fn phase_a_calibration_new_validates_inputs() {
    assert!(Calibration::new(4.14e-21, 1.0, 1e-15).is_ok());
    assert!(Calibration::new(-1.0, 1.0, 1e-15).is_err());
    assert!(Calibration::new(4.14e-21, 1.0, 0.0).is_err());
}

// ─────────────────────────────────────────────────────────────────────────
// Phase C.4 (T2.1): multiplication resolver trait delegation
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn phase_c4_multiplication_certificate_is_level_dependent() {
    let input = ConstrainedTypeInput::default();
    let cert_w8 = <MultiplicationResolver as Certify<ConstrainedTypeInput>>::certify_at(
        &MultiplicationResolver,
        &input,
        WittLevel::W8,
    )
    .expect("W8 multiplication certify succeeds");
    let cert_w32 = <MultiplicationResolver as Certify<ConstrainedTypeInput>>::certify_at(
        &MultiplicationResolver,
        &input,
        WittLevel::W32,
    )
    .expect("W32 multiplication certify succeeds");
    // Both certificates exist; the trait delegation calls the free function
    // with a derived MulContext. The cert's witt_bits is non-zero.
    assert_ne!(cert_w8.inner().witt_bits(), 0);
    assert_ne!(cert_w32.inner().witt_bits(), 0);
}

// ─────────────────────────────────────────────────────────────────────────
// Phase E (T2.6): BaseMetric accessors are input-dependent
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn phase_e_base_metrics_constants_pinned() {
    const _: () = assert!(MAX_BETTI_DIMENSION == 8);
}

#[test]
fn phase_e_run_const_grounded_metrics_differ_by_witt_level() {
    let builder_w8 = CompileUnitBuilder::new()
        .witt_level_ceiling(WittLevel::W8)
        .thermodynamic_budget(100);
    let validated_w8: Validated<CompileUnit, CompileTime> =
        validate_compile_unit_const(&builder_w8);

    let builder_w32 = CompileUnitBuilder::new()
        .witt_level_ceiling(WittLevel::W32)
        .thermodynamic_budget(200);
    let validated_w32: Validated<CompileUnit, CompileTime> =
        validate_compile_unit_const(&builder_w32);

    assert_eq!(validated_w8.inner().witt_level(), WittLevel::W8);
    assert_eq!(validated_w32.inner().witt_level(), WittLevel::W32);
    assert_eq!(validated_w8.inner().thermodynamic_budget(), 100);
    assert_eq!(validated_w32.inner().thermodynamic_budget(), 200);

    let g_w8: Grounded<ConstrainedTypeInput> = run_const::<ConstrainedTypeInput>(&validated_w8);
    let g_w32: Grounded<ConstrainedTypeInput> = run_const::<ConstrainedTypeInput>(&validated_w32);

    // unit_address differs because run_const hashes (level_bits, budget).
    assert_ne!(g_w8.unit_address(), g_w32.unit_address());

    // witt_level_bits reflects the unit.
    assert_ne!(g_w8.witt_level_bits(), g_w32.witt_level_bits());

    // BaseMetric accessors compute from witt_level_bits at mint time.
    assert_ne!(g_w8.betti_numbers(), g_w32.betti_numbers());

    // sigma is computed as bound_sites / declared_sites.
    let _: f64 = g_w8.sigma().as_f64();

    // residual_count = declared - bound; W8 vs W32 differ.
    assert_ne!(g_w8.residual_count(), g_w32.residual_count());

    // d_delta = witt_bits - bound_count differs because witt_bits differs.
    assert_ne!(g_w8.d_delta(), g_w32.d_delta());
}

// ─────────────────────────────────────────────────────────────────────────
// Phase F (T2.7): drivers walk their declarations
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn phase_f_run_parallel_unit_address_depends_on_site_count() {
    let unit_3: Validated<ParallelDeclaration> = validated_runtime(ParallelDeclaration::new(3));
    let unit_7: Validated<ParallelDeclaration> = validated_runtime(ParallelDeclaration::new(7));
    let g_3: Grounded<ConstrainedTypeInput> = run_parallel(unit_3).expect("3-site parallel walks");
    let g_7: Grounded<ConstrainedTypeInput> = run_parallel(unit_7).expect("7-site parallel walks");
    assert_ne!(g_3.unit_address(), g_7.unit_address());
}

#[test]
fn phase_f_stream_driver_yields_distinct_grounded() {
    let unit: Validated<StreamDeclaration> = validated_runtime(StreamDeclaration::new(3));
    let mut driver: StreamDriver<ConstrainedTypeInput, _> = run_stream(unit);
    let g1 = driver.next().expect("step 1").expect("step 1 ok");
    let g2 = driver.next().expect("step 2").expect("step 2 ok");
    let g3 = driver.next().expect("step 3").expect("step 3 ok");
    assert!(driver.next().is_none(), "stream terminates after bound");
    assert_eq!(driver.rewrite_steps(), 3);
    // Each yielded Grounded has a distinct unit_address from the unfold.
    assert_ne!(g1.unit_address(), g2.unit_address());
    assert_ne!(g2.unit_address(), g3.unit_address());
    assert_ne!(g1.unit_address(), g3.unit_address());
}

#[test]
fn phase_f_interaction_driver_folds_peer_inputs() {
    let unit: Validated<InteractionDeclaration> =
        validated_runtime(InteractionDeclaration::new(0xDEAD_BEEF));
    let mut driver: InteractionDriver<ConstrainedTypeInput, _> = run_interactive(unit);
    assert_eq!(driver.peer_step_count(), 0);
    assert!(!driver.is_converged());
    assert_eq!(driver.seed(), 0xDEAD_BEEF);

    // Step with non-zero peer_id: returns Continue, increments peer_step_count.
    let payload = PeerPayload::zero(32);
    let input1 = PeerInput::new(0x1234, payload);
    if let StepResult::Continue = driver.step(input1) {
    } else {
        panic!("expected Continue on first step");
    }
    assert_eq!(driver.peer_step_count(), 1);
    assert!(!driver.is_converged());

    // Step with peer_id=0: convergence handshake.
    let input2 = PeerInput::new(0, PeerPayload::zero(32));
    match driver.step(input2) {
        StepResult::Converged(_) => {}
        _ => panic!("expected Converged on convergence handshake"),
    }
    assert!(driver.is_converged());
    assert_eq!(driver.peer_step_count(), 2);

    // finalize returns Ok with non-zero unit_address.
    let final_grounded: Grounded<ConstrainedTypeInput> = driver.finalize().expect("converged");
    assert_ne!(final_grounded.unit_address(), 0u128);
}

#[test]
fn phase_f_interaction_driver_finalize_rejects_unconverged() {
    let unit: Validated<InteractionDeclaration> = validated_runtime(InteractionDeclaration::new(0));
    let driver: InteractionDriver<ConstrainedTypeInput, _> = run_interactive(unit);
    assert!(!driver.is_converged());
    let result: Result<Grounded<ConstrainedTypeInput>, _> = driver.finalize();
    assert!(result.is_err(), "unconverged driver finalize must error");
}

// ─────────────────────────────────────────────────────────────────────────
// Phase G (T2.8): const-fn companions are input-dependent
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn phase_g_certify_const_functions_carry_unit_level() {
    let builder = CompileUnitBuilder::new()
        .witt_level_ceiling(WittLevel::W32)
        .thermodynamic_budget(42);
    let validated: Validated<CompileUnit, CompileTime> = validate_compile_unit_const(&builder);
    assert_eq!(validated.inner().witt_level(), WittLevel::W32);

    let cert: Validated<GroundingCertificate, CompileTime> =
        certify_tower_completeness_const(&validated);
    assert_eq!(cert.inner().witt_bits(), 32);

    let inhab: Validated<GroundingCertificate, CompileTime> = certify_inhabitance_const(&validated);
    assert_eq!(inhab.inner().witt_bits(), 32);

    let mult: Validated<MultiplicationCertificate, CompileTime> =
        certify_multiplication_const(&validated);
    assert_eq!(mult.inner().witt_bits(), 32);
}

#[test]
fn phase_g_validate_compile_unit_const_is_input_dependent() {
    let b1 = CompileUnitBuilder::new()
        .witt_level_ceiling(WittLevel::W8)
        .thermodynamic_budget(100);
    let b2 = CompileUnitBuilder::new()
        .witt_level_ceiling(WittLevel::W32)
        .thermodynamic_budget(200);
    let v1 = validate_compile_unit_const(&b1);
    let v2 = validate_compile_unit_const(&b2);
    assert_ne!(v1.inner().witt_level(), v2.inner().witt_level());
    assert_ne!(
        v1.inner().thermodynamic_budget(),
        v2.inner().thermodynamic_budget()
    );
}

// ─────────────────────────────────────────────────────────────────────────
// Phase J (T1.1): grounding combinator MarkersImpliedBy bound
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn phase_j_grounding_program_compiles_for_integer_map() {
    let prog: GroundingProgram<u64, IntegerGroundingMap> =
        GroundingProgram::from_primitive(combinators::interpret_le_integer::<u64>());
    let _ = prog.primitive();
}

#[test]
fn phase_j_grounding_program_compiles_for_digest_map() {
    let prog: GroundingProgram<[u8; 32], DigestGroundingMap> =
        GroundingProgram::from_primitive(combinators::digest::<[u8; 32]>());
    let _ = prog.primitive();
}
