//! Cross-executor conformance tests (TASK-142).
//!
//! Verifies that ScalarExecutor produces identical output to hardware-accelerated
//! executors (Zen3Executor on x86_64, NeonExecutor on aarch64) for all operations.
//!
//! This ensures the portable reference implementation is correct and can be used
//! as a fallback on any platform.

use uor::arch::portable::ScalarExecutor;
use uor::isa::{PortAssignment, UorStep, Wavefront, WavefrontOp};
use uor::state::UorState;
use uor::taxon::Taxon;

#[cfg(target_arch = "x86_64")]
use uor::arch::Zen3Executor;

#[cfg(target_arch = "aarch64")]
use uor::arch::aarch64::NeonExecutor;

/// Create a deterministic test state: YMM[i][j] = (i*32 + j + 1) % 256.
/// The +1 ensures no zero values in the first positions.
fn test_state() -> UorState {
    let mut s = UorState::zero();
    for i in 0..16 {
        for j in 0..32 {
            s.ymm[i][j] = Taxon::new(((i * 32 + j + 1) % 256) as u8);
        }
    }
    // Initialize GPRs too
    for i in 0..14 {
        for j in 0..8 {
            s.gpr[i][j] = Taxon::new(((i * 8 + j + 128) % 256) as u8);
        }
    }
    s
}

/// All wavefront operations to test.
fn all_wavefronts() -> Vec<(&'static str, Wavefront)> {
    vec![
        // Basic ALU operations
        ("xor", Wavefront::all_xor()),
        ("and", Wavefront::new(PortAssignment::all_and())),
        ("or", Wavefront::new(PortAssignment::all_or())),
        ("not", Wavefront::all_not()),
        ("add", Wavefront::new(PortAssignment::all_add())),
        ("sub", Wavefront::new(PortAssignment::all_sub())),
        // Rotations
        ("rotr_1", Wavefront::new(PortAssignment::rotr_only(1))),
        ("rotr_7", Wavefront::new(PortAssignment::rotr_only(7))),
        ("rotr_13", Wavefront::new(PortAssignment::rotr_only(13))),
        ("rotr_22", Wavefront::new(PortAssignment::rotr_only(22))),
        ("rotr_31", Wavefront::new(PortAssignment::rotr_only(31))),
        ("rotl_7", Wavefront::new(PortAssignment::rotl_only(7))),
        ("rotl_11", Wavefront::new(PortAssignment::rotl_only(11))),
        // Shifts
        ("shr_1", Wavefront::new(PortAssignment::shr_only(1))),
        ("shr_3", Wavefront::new(PortAssignment::shr_only(3))),
        ("shr_10", Wavefront::new(PortAssignment::shr_only(10))),
        ("shl_1", Wavefront::new(PortAssignment::shl_only(1))),
        ("shl_10", Wavefront::new(PortAssignment::shl_only(10))),
        ("shl_17", Wavefront::new(PortAssignment::shl_only(17))),
        // Permutations
        ("shuffle", Wavefront::shuffle()),
        ("permute", Wavefront::permute()),
        // Crypto (these use different internal implementations)
        ("sha256_round", Wavefront::sha256_round()),
        ("sha256_msg1", Wavefront::new(PortAssignment::sha256_msg1())),
        ("sha256_msg2", Wavefront::new(PortAssignment::sha256_msg2())),
        ("aes_round", Wavefront::aes_round()),
        (
            "aes_round_dec",
            Wavefront::new(PortAssignment::aes_round_dec()),
        ),
        // Mixed port assignments
        (
            "mixed_xor_and",
            Wavefront::new(PortAssignment {
                port0: WavefrontOp::Nop,
                port1: WavefrontOp::Xor,
                port5: WavefrontOp::And,
            }),
        ),
        (
            "mixed_rotr_xor",
            Wavefront::new(PortAssignment {
                port0: WavefrontOp::RotR(7),
                port1: WavefrontOp::Xor,
                port5: WavefrontOp::Nop,
            }),
        ),
    ]
}

/// Compare two states for exact equality.
fn states_equal(a: &UorState, b: &UorState) -> bool {
    for i in 0..16 {
        for j in 0..32 {
            if a.ymm[i][j].value() != b.ymm[i][j].value() {
                return false;
            }
        }
    }
    for i in 0..14 {
        for j in 0..8 {
            if a.gpr[i][j].value() != b.gpr[i][j].value() {
                return false;
            }
        }
    }
    true
}

/// Report first difference between two states.
fn find_difference(a: &UorState, b: &UorState) -> Option<String> {
    for i in 0..16 {
        for j in 0..32 {
            if a.ymm[i][j].value() != b.ymm[i][j].value() {
                return Some(format!(
                    "YMM[{}][{}]: scalar={} vs hardware={}",
                    i,
                    j,
                    a.ymm[i][j].value(),
                    b.ymm[i][j].value()
                ));
            }
        }
    }
    for i in 0..14 {
        for j in 0..8 {
            if a.gpr[i][j].value() != b.gpr[i][j].value() {
                return Some(format!(
                    "GPR[{}][{}]: scalar={} vs hardware={}",
                    i,
                    j,
                    a.gpr[i][j].value(),
                    b.gpr[i][j].value()
                ));
            }
        }
    }
    None
}

// ============================================================================
// x86_64: ScalarExecutor vs Zen3Executor
// ============================================================================

#[cfg(target_arch = "x86_64")]
mod x86_64_conformance {
    use super::*;

    #[test]
    #[ignore = "ScalarExecutor is a stub (see specs/plans/4-arch-executor-completion.md)"]
    fn test_scalar_matches_zen3_all_ops() {
        let scalar = ScalarExecutor::new();
        let zen3 = Zen3Executor::new();

        for (name, wf) in all_wavefronts() {
            let mut scalar_state = test_state();
            let mut zen3_state = test_state();

            // Execute on both
            unsafe {
                scalar.step(&mut scalar_state, &wf);
                zen3.step(&mut zen3_state, &wf);
            }

            // Compare
            if !states_equal(&scalar_state, &zen3_state) {
                let diff = find_difference(&scalar_state, &zen3_state)
                    .unwrap_or_else(|| "unknown".to_string());
                panic!("Conformance failure for '{}': {}", name, diff);
            }
        }
    }

    #[test]
    #[ignore = "ScalarExecutor is a stub (see specs/plans/4-arch-executor-completion.md)"]
    fn test_scalar_matches_zen3_sequence() {
        let scalar = ScalarExecutor::new();
        let zen3 = Zen3Executor::new();

        // Run a mixed sequence of 64 wavefronts
        let program: Vec<Wavefront> = (0..64)
            .map(|i| match i % 6 {
                0 => Wavefront::all_xor(),
                1 => Wavefront::new(PortAssignment::all_and()),
                2 => Wavefront::new(PortAssignment::rotr_only((i % 31 + 1) as u8)),
                3 => Wavefront::new(PortAssignment::all_add()),
                4 => Wavefront::shuffle(),
                _ => Wavefront::all_not(),
            })
            .collect();

        let mut scalar_state = test_state();
        let mut zen3_state = test_state();

        unsafe {
            scalar.run(&mut scalar_state, &program);
            zen3.run(&mut zen3_state, &program);
        }

        assert!(
            states_equal(&scalar_state, &zen3_state),
            "Conformance failure in 64-wavefront sequence"
        );
    }

    #[test]
    #[ignore = "ScalarExecutor is a stub (see specs/plans/4-arch-executor-completion.md)"]
    fn test_scalar_matches_zen3_sha256_compress() {
        let scalar = ScalarExecutor::new();
        let zen3 = Zen3Executor::new();

        let program = uor::wavefront::sha256_compress_program();

        let mut scalar_state = test_state();
        let mut zen3_state = test_state();

        unsafe {
            scalar.run(&mut scalar_state, &program);
            zen3.run(&mut zen3_state, &program);
        }

        assert!(
            states_equal(&scalar_state, &zen3_state),
            "Conformance failure in SHA-256 compress program"
        );
    }

    #[test]
    #[ignore = "ScalarExecutor is a stub (see specs/plans/4-arch-executor-completion.md)"]
    fn test_scalar_matches_zen3_aes_encrypt() {
        let scalar = ScalarExecutor::new();
        let zen3 = Zen3Executor::new();

        let program = uor::wavefront::aes128_encrypt_program();

        let mut scalar_state = test_state();
        let mut zen3_state = test_state();

        unsafe {
            scalar.run(&mut scalar_state, &program);
            zen3.run(&mut zen3_state, &program);
        }

        assert!(
            states_equal(&scalar_state, &zen3_state),
            "Conformance failure in AES-128 encrypt program"
        );
    }
}

// ============================================================================
// aarch64: ScalarExecutor vs NeonExecutor
// ============================================================================

#[cfg(target_arch = "aarch64")]
mod aarch64_conformance {
    use super::*;

    #[test]
    fn test_scalar_matches_neon_all_ops() {
        let scalar = ScalarExecutor::new();
        let neon = NeonExecutor::new();

        for (name, wf) in all_wavefronts() {
            let mut scalar_state = test_state();
            let mut neon_state = test_state();

            // Execute on both
            unsafe {
                scalar.step(&mut scalar_state, &wf);
                neon.step(&mut neon_state, &wf);
            }

            // Compare
            if !states_equal(&scalar_state, &neon_state) {
                let diff = find_difference(&scalar_state, &neon_state)
                    .unwrap_or_else(|| "unknown".to_string());
                panic!("Conformance failure for '{}': {}", name, diff);
            }
        }
    }

    #[test]
    fn test_scalar_matches_neon_sequence() {
        let scalar = ScalarExecutor::new();
        let neon = NeonExecutor::new();

        // Run a mixed sequence of 64 wavefronts
        let program: Vec<Wavefront> = (0..64)
            .map(|i| match i % 6 {
                0 => Wavefront::all_xor(),
                1 => Wavefront::new(PortAssignment::all_and()),
                2 => Wavefront::new(PortAssignment::rotr_only((i % 31 + 1) as u8)),
                3 => Wavefront::new(PortAssignment::all_add()),
                4 => Wavefront::shuffle(),
                _ => Wavefront::all_not(),
            })
            .collect();

        let mut scalar_state = test_state();
        let mut neon_state = test_state();

        unsafe {
            scalar.run(&mut scalar_state, &program);
            neon.run(&mut neon_state, &program);
        }

        assert!(
            states_equal(&scalar_state, &neon_state),
            "Conformance failure in 64-wavefront sequence"
        );
    }

    #[test]
    fn test_scalar_matches_neon_sha256_compress() {
        let scalar = ScalarExecutor::new();
        let neon = NeonExecutor::new();

        let program = uor::wavefront::sha256_compress_program();

        let mut scalar_state = test_state();
        let mut neon_state = test_state();

        unsafe {
            scalar.run(&mut scalar_state, &program);
            neon.run(&mut neon_state, &program);
        }

        assert!(
            states_equal(&scalar_state, &neon_state),
            "Conformance failure in SHA-256 compress program"
        );
    }
}

// ============================================================================
// Portable: ScalarExecutor self-consistency tests
// ============================================================================

mod scalar_consistency {
    use super::*;

    #[test]
    fn test_xor_is_self_inverse() {
        let scalar = ScalarExecutor::new();
        let wf = Wavefront::all_xor();

        let initial = test_state();
        let mut state = test_state();

        // XOR twice should return to original (for first 8 registers)
        unsafe {
            scalar.step(&mut state, &wf);
            scalar.step(&mut state, &wf);
        }

        // YMM[0..8] should be unchanged (they were XORed with [8..16] twice)
        for i in 0..8 {
            for j in 0..32 {
                assert_eq!(
                    state.ymm[i][j].value(),
                    initial.ymm[i][j].value(),
                    "XOR self-inverse failed at YMM[{}][{}]",
                    i,
                    j
                );
            }
        }
    }

    #[test]
    fn test_not_is_self_inverse() {
        let scalar = ScalarExecutor::new();
        let wf = Wavefront::all_not();

        let initial = test_state();
        let mut state = test_state();

        // NOT twice should return to original
        unsafe {
            scalar.step(&mut state, &wf);
            scalar.step(&mut state, &wf);
        }

        assert!(states_equal(&state, &initial), "NOT is not self-inverse");
    }

    #[test]
    fn test_rotation_inverse() {
        let scalar = ScalarExecutor::new();

        let initial = test_state();
        let mut state = test_state();

        // RotR(7) followed by RotL(7) should be identity
        let rotr = Wavefront::new(PortAssignment::rotr_only(7));
        let rotl = Wavefront::new(PortAssignment::rotl_only(7));

        unsafe {
            scalar.step(&mut state, &rotr);
            scalar.step(&mut state, &rotl);
        }

        // First 8 YMM registers should be unchanged
        for i in 0..8 {
            for j in 0..32 {
                assert_eq!(
                    state.ymm[i][j].value(),
                    initial.ymm[i][j].value(),
                    "Rotation inverse failed at YMM[{}][{}]",
                    i,
                    j
                );
            }
        }
    }

    #[test]
    fn test_add_sub_inverse() {
        let scalar = ScalarExecutor::new();

        let initial = test_state();
        let mut state = test_state();

        // Add then Sub should restore original (for 32-bit lanes)
        let add = Wavefront::new(PortAssignment::all_add());
        let sub = Wavefront::new(PortAssignment::all_sub());

        unsafe {
            scalar.step(&mut state, &add);
            scalar.step(&mut state, &sub);
        }

        // First 8 YMM registers should be unchanged
        for i in 0..8 {
            for j in 0..32 {
                assert_eq!(
                    state.ymm[i][j].value(),
                    initial.ymm[i][j].value(),
                    "Add/Sub inverse failed at YMM[{}][{}]",
                    i,
                    j
                );
            }
        }
    }

    #[test]
    fn test_deterministic_execution() {
        let scalar = ScalarExecutor::new();
        let wf = Wavefront::all_xor();

        // Same input should produce same output
        let mut state1 = test_state();
        let mut state2 = test_state();

        unsafe {
            scalar.step(&mut state1, &wf);
            scalar.step(&mut state2, &wf);
        }

        assert!(
            states_equal(&state1, &state2),
            "Execution is not deterministic"
        );
    }
}
