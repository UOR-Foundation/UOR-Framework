//! Shuffle wavefront operations via inline assembly.
//!
//! Uses vpshufb for byte-level shuffling within 128-bit lanes.

use core::arch::asm;

/// Execute shuffle wavefront using vpshufb.
///
/// Performs byte-level shuffle within each 128-bit lane.
/// ymm[i] = shuffle(ymm[i], ymm[i+8]) where ymm[i+8] provides the shuffle mask.
///
/// # Register Layout
///
/// - ymm0-7: Data to be shuffled (destinations)
/// - ymm8-15: Shuffle control masks (sources)
///
/// # Shuffle Semantics
///
/// For each byte position i in the destination:
/// - If mask[i] & 0x80, result[i] = 0
/// - Otherwise, result[i] = src[mask[i] & 0x0F] (within 128-bit lane)
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Execution
///
/// - 8 `vpshufb` instructions
/// - Execute on Port 5
/// - ~1 cycle throughput
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn shuffle_wavefront_asm() {
    asm!(
        // Byte shuffle: ymm[i] = pshufb(ymm[i], ymm[i+8])
        // Port 5 only (cannot parallelize with Port 1)
        "vpshufb ymm0, ymm0, ymm8",
        "vpshufb ymm1, ymm1, ymm9",
        "vpshufb ymm2, ymm2, ymm10",
        "vpshufb ymm3, ymm3, ymm11",
        "vpshufb ymm4, ymm4, ymm12",
        "vpshufb ymm5, ymm5, ymm13",
        "vpshufb ymm6, ymm6, ymm14",
        "vpshufb ymm7, ymm7, ymm15",
        options(nomem, nostack, preserves_flags)
    );
}
