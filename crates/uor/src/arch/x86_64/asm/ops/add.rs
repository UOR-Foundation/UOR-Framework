//! ADD wavefront operation via inline assembly.
//!
//! Executes: ymm[i] += ymm[i+8] for i in 0..8 (32-bit lanes)
//!
//! Uses vpaddd for 32-bit integer addition in each lane.

use core::arch::asm;

/// Execute ADD wavefront: ymm[i] += ymm[i+8] for i in 0..8
///
/// Addition is performed in 32-bit lanes (8 lanes per YMM register).
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Execution
///
/// - 8 `vpaddd` instructions
/// - Execute on Ports 1 and 5 (both support ADD)
/// - ~1 cycle total (superscalar execution)
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn add_wavefront_asm() {
    asm!(
        // All 8 ADD operations - Ports 1 and 5 execute in parallel
        // Pattern: ymm[0-7] += ymm[8-15] (32-bit lanes)
        "vpaddd ymm0, ymm0, ymm8",
        "vpaddd ymm1, ymm1, ymm9",
        "vpaddd ymm2, ymm2, ymm10",
        "vpaddd ymm3, ymm3, ymm11",
        "vpaddd ymm4, ymm4, ymm12",
        "vpaddd ymm5, ymm5, ymm13",
        "vpaddd ymm6, ymm6, ymm14",
        "vpaddd ymm7, ymm7, ymm15",
        // No memory access, no stack, preserve flags
        options(nomem, nostack, preserves_flags)
    );
}
