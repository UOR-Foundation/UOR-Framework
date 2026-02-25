//! NOT wavefront operation via inline assembly.
//!
//! Executes: ymm[i] = ~ymm[i] for i in 0..8
//!
//! NOT is implemented as XOR with all-ones mask.

use core::arch::asm;

/// Execute NOT wavefront: ymm[i] = ~ymm[i] for i in 0..8
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Implementation
///
/// NOT is implemented as XOR with all-ones:
/// 1. Generate all-ones mask using `vpcmpeqd ymm15, ymm15, ymm15`
/// 2. XOR each destination with the mask
///
/// # Note
///
/// This operation clobbers ymm15 (used as scratch for all-ones mask).
///
/// # Execution
///
/// - 1 `vpcmpeqd` to generate mask
/// - 8 `vpxor` instructions
/// - Execute on Ports 1 and 5
/// - ~2-3 cycles total
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn not_wavefront_asm() {
    asm!(
        // Generate all-ones mask in ymm15
        "vpcmpeqd ymm15, ymm15, ymm15",
        // Invert all 8 destination registers
        "vpxor ymm0, ymm0, ymm15",
        "vpxor ymm1, ymm1, ymm15",
        "vpxor ymm2, ymm2, ymm15",
        "vpxor ymm3, ymm3, ymm15",
        "vpxor ymm4, ymm4, ymm15",
        "vpxor ymm5, ymm5, ymm15",
        "vpxor ymm6, ymm6, ymm15",
        "vpxor ymm7, ymm7, ymm15",
        // No memory access, no stack, preserve flags
        options(nomem, nostack, preserves_flags)
    );
}
