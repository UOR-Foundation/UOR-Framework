//! Permute wavefront operations via inline assembly.
//!
//! Uses vpermd for 32-bit element permutation across the full 256-bit register.

use core::arch::asm;

/// Execute permute wavefront using vpermd.
///
/// Performs 32-bit element permutation across the full 256-bit register.
/// ymm[i] = permd(ymm[i], ymm[i+8]) where ymm[i+8] provides the permute indices.
///
/// # Register Layout
///
/// - ymm0-7: Data to be permuted (destinations)
/// - ymm8-15: Permute index masks (sources)
///
/// # Permute Semantics
///
/// For each 32-bit element position i (0-7) in the destination:
/// - result[i] = src[index[i] & 0x7]
///
/// Unlike vpshufb, vpermd can move elements across 128-bit lane boundaries.
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Execution
///
/// - 8 `vpermd` instructions
/// - Execute on Port 5
/// - ~3 cycles latency, 1 cycle throughput
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn permute_wavefront_asm() {
    asm!(
        // 32-bit permute: ymm[i] = permd(ymm[i], ymm[i+8])
        // vpermd ymm_dst, ymm_idx, ymm_src (VEX encoding)
        // Note: vpermd uses idx, src order (not dst, src, idx)
        "vpermd ymm0, ymm8, ymm0",
        "vpermd ymm1, ymm9, ymm1",
        "vpermd ymm2, ymm10, ymm2",
        "vpermd ymm3, ymm11, ymm3",
        "vpermd ymm4, ymm12, ymm4",
        "vpermd ymm5, ymm13, ymm5",
        "vpermd ymm6, ymm14, ymm6",
        "vpermd ymm7, ymm15, ymm7",
        options(nomem, nostack, preserves_flags)
    );
}
