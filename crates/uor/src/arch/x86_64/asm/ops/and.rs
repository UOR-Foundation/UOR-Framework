//! AND wavefront operation via inline assembly.
//!
//! Executes: ymm[i] &= ymm[i+8] for i in 0..8
//!
//! All 8 AND operations execute on Ports 1 and 5 in parallel.

use core::arch::asm;

/// Execute AND wavefront: ymm[i] &= ymm[i+8] for i in 0..8
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Execution
///
/// - 8 `vpand` instructions
/// - Execute on Ports 1 and 5 (both support AND)
/// - ~1 cycle total (superscalar execution)
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn and_wavefront_asm() {
    asm!(
        // All 8 AND operations - Ports 1 and 5 execute in parallel
        // Pattern: ymm[0-7] &= ymm[8-15]
        "vpand ymm0, ymm0, ymm8",
        "vpand ymm1, ymm1, ymm9",
        "vpand ymm2, ymm2, ymm10",
        "vpand ymm3, ymm3, ymm11",
        "vpand ymm4, ymm4, ymm12",
        "vpand ymm5, ymm5, ymm13",
        "vpand ymm6, ymm6, ymm14",
        "vpand ymm7, ymm7, ymm15",
        // No memory access, no stack, preserve flags
        options(nomem, nostack, preserves_flags)
    );
}
