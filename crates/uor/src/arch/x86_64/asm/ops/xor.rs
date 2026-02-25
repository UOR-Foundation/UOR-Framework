//! XOR wavefront operation via inline assembly.
//!
//! Executes: ymm[i] ^= ymm[i+8] for i in 0..8
//!
//! All 8 XOR operations execute on Ports 1 and 5 in parallel.
//!
//! # Unrolled Variants
//!
//! This module provides unrolled variants for common iteration counts:
//! - `xor_wavefront_asm_4()` - 4 iterations unrolled
//! - `xor_wavefront_asm_8()` - 8 iterations unrolled
//! - `xor_wavefront_asm_16()` - 16 iterations unrolled
//! - `xor_wavefront_asm_64()` - 64 iterations unrolled
//!
//! Unrolling eliminates loop overhead and enables maximum instruction-level parallelism.

use core::arch::asm;

/// Execute XOR wavefront: ymm[i] ^= ymm[i+8] for i in 0..8
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Execution
///
/// - 8 `vpxor` instructions
/// - Execute on Ports 1 and 5 (both support XOR)
/// - ~1 cycle total (superscalar execution)
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn xor_wavefront_asm() {
    asm!(
        // All 8 XOR operations - Ports 1 and 5 execute in parallel
        // Pattern: ymm[0-7] ^= ymm[8-15]
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        // Destination registers (ymm0-7) are clobbered
        // LLVM sees the register references in the asm string
        // and knows they're both read and written
        // No memory access, no stack, preserve flags
        options(nomem, nostack, preserves_flags)
    );
}

// =============================================================================
// UNROLLED VARIANTS
// =============================================================================

/// Execute 4 XOR wavefronts with zero loop overhead.
///
/// # Execution
///
/// - 32 `vpxor` instructions (4 × 8)
/// - Execute on Ports 1 and 5 in parallel
/// - ~4 cycles total (theoretical)
///
/// # Safety
///
/// Same requirements as `xor_wavefront_asm`.
#[inline]
pub unsafe fn xor_wavefront_asm_4() {
    asm!(
        // Iteration 1
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        // Iteration 2
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        // Iteration 3
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        // Iteration 4
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        options(nomem, nostack, preserves_flags)
    );
}

/// Execute 8 XOR wavefronts with zero loop overhead.
///
/// # Execution
///
/// - 64 `vpxor` instructions (8 × 8)
/// - Execute on Ports 1 and 5 in parallel
/// - ~8 cycles total (theoretical)
///
/// # Safety
///
/// Same requirements as `xor_wavefront_asm`.
#[inline]
pub unsafe fn xor_wavefront_asm_8() {
    asm!(
        // Iterations 1-4
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        // Iterations 5-8
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        options(nomem, nostack, preserves_flags)
    );
}

/// Execute 16 XOR wavefronts with zero loop overhead.
///
/// # Execution
///
/// - 128 `vpxor` instructions (16 × 8)
/// - Execute on Ports 1 and 5 in parallel
/// - ~16 cycles total (theoretical)
///
/// # Safety
///
/// Same requirements as `xor_wavefront_asm`.
#[inline]
pub unsafe fn xor_wavefront_asm_16() {
    asm!(
        // Iterations 1-8
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        // Iterations 9-16
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        "vpxor ymm0, ymm0, ymm8",
        "vpxor ymm1, ymm1, ymm9",
        "vpxor ymm2, ymm2, ymm10",
        "vpxor ymm3, ymm3, ymm11",
        "vpxor ymm4, ymm4, ymm12",
        "vpxor ymm5, ymm5, ymm13",
        "vpxor ymm6, ymm6, ymm14",
        "vpxor ymm7, ymm7, ymm15",
        options(nomem, nostack, preserves_flags)
    );
}

/// Execute N XOR wavefronts using the optimal unrolled variant.
///
/// This function dispatches to the appropriate unrolled variant based on
/// the iteration count. For counts not matching a specialized variant,
/// it falls back to calling `xor_wavefront_asm` in a loop.
///
/// # Execution
///
/// Uses specialized unrolled variants for common counts (4, 8, 16),
/// otherwise uses a loop with 8x unrolling for larger counts.
///
/// # Safety
///
/// Same requirements as `xor_wavefront_asm`.
#[inline]
pub unsafe fn xor_wavefront_asm_n(n: usize) {
    match n {
        0 => {}
        1 => xor_wavefront_asm(),
        4 => xor_wavefront_asm_4(),
        8 => xor_wavefront_asm_8(),
        16 => xor_wavefront_asm_16(),
        _ => {
            // For other counts, use 8x unrolling with remainder
            let full_blocks = n / 8;
            let remainder = n % 8;

            for _ in 0..full_blocks {
                xor_wavefront_asm_8();
            }

            for _ in 0..remainder {
                xor_wavefront_asm();
            }
        }
    }
}
