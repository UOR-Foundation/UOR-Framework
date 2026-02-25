//! Cross-verification tests for microcode operations.
//!
//! These tests verify that the microcode operations produce identical results
//! to native Rust operations and existing implementations. This provides
//! correctness guarantees for the microcode primitive layer.
//!
//! TASK-115: Prove microcode correctness against native implementations.

use uor::microcode::{
    MicrocodeOps, MicrocodePrimitives, ScalarMicrocodeExecutor, ScalarPrimitives,
};

// =============================================================================
// Exhaustive u8 Tests (256 values, fast to verify completely)
// =============================================================================

#[test]
fn cross_verify_inc_exhaustive_u8() {
    // Verify INC matches native wrapping_add(1) for all u8 values
    let p = ScalarPrimitives;
    for x in 0u8..=255 {
        let microcode_result = p.inc(x as u32) as u8;
        let native_result = x.wrapping_add(1);
        assert_eq!(
            microcode_result, native_result,
            "INC mismatch at x={x}: microcode={microcode_result}, native={native_result}"
        );
    }
}

#[test]
fn cross_verify_dec_exhaustive_u8() {
    // Verify DEC matches native wrapping_sub(1) for all u8 values
    let p = ScalarPrimitives;
    for x in 0u8..=255 {
        let microcode_result = p.dec(x as u32) as u8;
        let native_result = x.wrapping_sub(1);
        assert_eq!(
            microcode_result, native_result,
            "DEC mismatch at x={x}: microcode={microcode_result}, native={native_result}"
        );
    }
}

#[test]
fn cross_verify_add_exhaustive_u8_pairs() {
    // Verify ADD matches native wrapping_add for all (a, b) pairs in u8 range
    let p = ScalarPrimitives;
    for a in 0u8..=255 {
        for b in 0u8..=255 {
            let microcode_result = p.add(a as u32, b as u32) as u8;
            let native_result = a.wrapping_add(b);
            assert_eq!(
                microcode_result, native_result,
                "ADD mismatch at ({a}, {b}): microcode={microcode_result}, native={native_result}"
            );
        }
    }
}

#[test]
fn cross_verify_sub_exhaustive_u8_pairs() {
    // Verify SUB matches native wrapping_sub for all (a, b) pairs in u8 range
    let p = ScalarPrimitives;
    for a in 0u8..=255 {
        for b in 0u8..=255 {
            let microcode_result = p.sub(a as u32, b as u32) as u8;
            let native_result = a.wrapping_sub(b);
            assert_eq!(
                microcode_result, native_result,
                "SUB mismatch at ({a}, {b}): microcode={microcode_result}, native={native_result}"
            );
        }
    }
}

// =============================================================================
// Representative u32 Tests (edge cases and random samples)
// =============================================================================

const TEST_VALUES_U32: [u32; 20] = [
    0,
    1,
    2,
    42,
    127,
    128,
    255,
    256,
    1000,
    0x7FFF_FFFF, // i32::MAX as u32
    0x8000_0000, // i32::MIN as u32
    0xDEAD_BEEF,
    0xCAFE_BABE,
    0xFFFF_0000,
    0x0000_FFFF,
    0xAAAA_AAAA,
    0x5555_5555,
    u32::MAX - 1,
    u32::MAX,
    0x1234_5678,
];

#[test]
fn cross_verify_add_u32_representative() {
    let p = ScalarPrimitives;
    for &a in &TEST_VALUES_U32 {
        for &b in &TEST_VALUES_U32 {
            let microcode_result = p.add(a, b);
            let native_result = a.wrapping_add(b);
            assert_eq!(
                microcode_result, native_result,
                "ADD u32 mismatch at ({a:#x}, {b:#x}): microcode={microcode_result:#x}, native={native_result:#x}"
            );
        }
    }
}

#[test]
fn cross_verify_sub_u32_representative() {
    let p = ScalarPrimitives;
    for &a in &TEST_VALUES_U32 {
        for &b in &TEST_VALUES_U32 {
            let microcode_result = p.sub(a, b);
            let native_result = a.wrapping_sub(b);
            assert_eq!(
                microcode_result, native_result,
                "SUB u32 mismatch at ({a:#x}, {b:#x}): microcode={microcode_result:#x}, native={native_result:#x}"
            );
        }
    }
}

#[test]
fn cross_verify_inc_u32_representative() {
    let p = ScalarPrimitives;
    for &x in &TEST_VALUES_U32 {
        let microcode_result = p.inc(x);
        let native_result = x.wrapping_add(1);
        assert_eq!(
            microcode_result, native_result,
            "INC u32 mismatch at {x:#x}: microcode={microcode_result:#x}, native={native_result:#x}"
        );
    }
}

#[test]
fn cross_verify_dec_u32_representative() {
    let p = ScalarPrimitives;
    for &x in &TEST_VALUES_U32 {
        let microcode_result = p.dec(x);
        let native_result = x.wrapping_sub(1);
        assert_eq!(
            microcode_result, native_result,
            "DEC u32 mismatch at {x:#x}: microcode={microcode_result:#x}, native={native_result:#x}"
        );
    }
}

// =============================================================================
// Executor Cross-Verification
// =============================================================================

#[test]
fn cross_verify_executor_add_matches_direct() {
    // Verify executor produces same results as direct primitive calls
    let exec = ScalarMicrocodeExecutor::new();
    let p = ScalarPrimitives;

    for &a in &TEST_VALUES_U32 {
        for &b in &TEST_VALUES_U32 {
            let exec_result = exec.add(a, b);
            let prim_result = p.add(a, b);
            assert_eq!(
                exec_result, prim_result,
                "Executor/Primitive ADD mismatch at ({a:#x}, {b:#x})"
            );
        }
    }
}

#[test]
fn cross_verify_executor_sub_matches_direct() {
    let exec = ScalarMicrocodeExecutor::new();
    let p = ScalarPrimitives;

    for &a in &TEST_VALUES_U32 {
        for &b in &TEST_VALUES_U32 {
            let exec_result = exec.sub(a, b);
            let prim_result = p.sub(a, b);
            assert_eq!(
                exec_result, prim_result,
                "Executor/Primitive SUB mismatch at ({a:#x}, {b:#x})"
            );
        }
    }
}

// =============================================================================
// Bitwise Operation Verification
// =============================================================================

#[test]
fn cross_verify_bnot_matches_native() {
    let p = ScalarPrimitives;
    for &x in &TEST_VALUES_U32 {
        let microcode_result = p.bnot(x);
        let native_result = !x;
        assert_eq!(
            microcode_result, native_result,
            "BNOT mismatch at {x:#x}: microcode={microcode_result:#x}, native={native_result:#x}"
        );
    }
}

#[test]
fn cross_verify_neg_matches_native() {
    let p = ScalarPrimitives;
    for &x in &TEST_VALUES_U32 {
        let microcode_result = p.neg(x);
        let native_result = x.wrapping_neg();
        assert_eq!(
            microcode_result, native_result,
            "NEG mismatch at {x:#x}: microcode={microcode_result:#x}, native={native_result:#x}"
        );
    }
}

#[test]
fn cross_verify_xor_matches_native() {
    let p = ScalarPrimitives;
    for &a in &TEST_VALUES_U32 {
        for &b in &TEST_VALUES_U32 {
            let microcode_result = p.xor(a, b);
            let native_result = a ^ b;
            assert_eq!(
                microcode_result, native_result,
                "XOR mismatch at ({a:#x}, {b:#x})"
            );
        }
    }
}

#[test]
fn cross_verify_and_matches_native() {
    let p = ScalarPrimitives;
    for &a in &TEST_VALUES_U32 {
        for &b in &TEST_VALUES_U32 {
            let microcode_result = p.and(a, b);
            let native_result = a & b;
            assert_eq!(
                microcode_result, native_result,
                "AND mismatch at ({a:#x}, {b:#x})"
            );
        }
    }
}

#[test]
fn cross_verify_or_matches_native() {
    let p = ScalarPrimitives;
    for &a in &TEST_VALUES_U32 {
        for &b in &TEST_VALUES_U32 {
            let microcode_result = p.or(a, b);
            let native_result = a | b;
            assert_eq!(
                microcode_result, native_result,
                "OR mismatch at ({a:#x}, {b:#x})"
            );
        }
    }
}

// =============================================================================
// Kogge-Stone Adder Specific Verification
// =============================================================================

#[test]
fn cross_verify_kogge_stone_carry_propagation() {
    // Test carry propagation patterns that stress the Kogge-Stone adder
    let p = ScalarPrimitives;

    // Long carry chain: all 1s + 1 = overflow
    assert_eq!(p.add(u32::MAX, 1u32), 0);

    // Partial carry chains
    assert_eq!(p.add(0x0000_FFFFu32, 1), 0x0001_0000);
    assert_eq!(p.add(0x00FF_FFFFu32, 1), 0x0100_0000);
    assert_eq!(p.add(0x0FFF_FFFFu32, 1), 0x1000_0000);
    assert_eq!(p.add(0x7FFF_FFFFu32, 1), 0x8000_0000);

    // Alternating bit patterns
    assert_eq!(
        p.add(0x5555_5555, 0xAAAA_AAAA),
        0x5555_5555u32.wrapping_add(0xAAAA_AAAA)
    );
    assert_eq!(
        p.add(0xAAAA_AAAA, 0x5555_5555),
        0xAAAA_AAAAu32.wrapping_add(0x5555_5555)
    );
}

#[test]
fn cross_verify_kogge_stone_boundary_values() {
    let p = ScalarPrimitives;

    // Test all boundary combinations
    let boundaries = [0, 1, u32::MAX - 1, u32::MAX];
    for &a in &boundaries {
        for &b in &boundaries {
            let microcode_result = p.add(a, b);
            let native_result = a.wrapping_add(b);
            assert_eq!(
                microcode_result, native_result,
                "Kogge-Stone boundary mismatch at ({a}, {b})"
            );
        }
    }
}

// =============================================================================
// Critical UOR Identities
// =============================================================================

#[test]
fn cross_verify_uor_inc_identity() {
    // The UOR framework's critical identity: neg(bnot(x)) = x + 1
    let p = ScalarPrimitives;
    for &x in &TEST_VALUES_U32 {
        let uor_inc = p.neg(p.bnot(x));
        let native_inc = x.wrapping_add(1);
        assert_eq!(uor_inc, native_inc, "UOR INC identity violated at {x:#x}");
    }
}

#[test]
fn cross_verify_uor_dec_identity() {
    // The UOR framework's critical identity: bnot(neg(x)) = x - 1
    let p = ScalarPrimitives;
    for &x in &TEST_VALUES_U32 {
        let uor_dec = p.bnot(p.neg(x));
        let native_dec = x.wrapping_sub(1);
        assert_eq!(uor_dec, native_dec, "UOR DEC identity violated at {x:#x}");
    }
}
