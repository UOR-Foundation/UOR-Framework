//! SHA-256 wavefront operations via inline assembly.
//!
//! Uses SHA-NI instructions (sha256rnds2) available on modern AMD and Intel CPUs.
//! SHA-NI operates on XMM (128-bit) registers, which are the lower half of YMM registers.

use core::arch::asm;

/// Execute SHA-256 rounds via sha256rnds2 instruction.
///
/// Performs 2 SHA-256 rounds using the SHA-NI extension.
///
/// # Register Layout
///
/// - xmm0: State ABEF (lower 128 bits of ymm0)
/// - xmm1: State CDGH (lower 128 bits of ymm1)
/// - xmm2: Message schedule (lower 128 bits of ymm2)
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Execution
///
/// - 2 `sha256rnds2` instructions
/// - Execute on Port 0
/// - ~8 cycles for 4 rounds (2 per instruction)
///
/// # Safety
///
/// - Caller must ensure SHA-NI (sha extension) is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn sha256_round_asm() {
    asm!(
        // sha256rnds2: Perform 2 SHA-256 rounds
        // dst = sha256rnds2(dst, src, msg) uses xmm0 implicitly for msg
        // We need to shuffle msg for the second instruction

        // Save xmm2 (message) since sha256rnds2 uses xmm0 implicitly
        "vmovdqa xmm15, xmm2",
        // First 2 rounds: CDGH = sha256rnds2(CDGH, ABEF, msg[0:1])
        // Implicit: uses xmm0 (lower 64 bits) as message
        // We need xmm2 in xmm0 position for the implicit operand
        "vmovdqa xmm14, xmm0",     // Save ABEF
        "vmovdqa xmm0, xmm15",     // Put message in xmm0 (implicit operand)
        "sha256rnds2 xmm1, xmm14", // CDGH = sha256rnds2(CDGH, ABEF, msg)
        // Second 2 rounds: ABEF = sha256rnds2(ABEF, CDGH, msg[2:3])
        "vpshufd xmm0, xmm15, 0x0E", // Shuffle message for rounds 2-3
        "sha256rnds2 xmm14, xmm1",   // ABEF = sha256rnds2(ABEF, CDGH, msg)
        // Restore results
        "vmovdqa xmm0, xmm14", // Restore ABEF to xmm0
        // No memory access, no stack, preserve flags
        options(nomem, nostack, preserves_flags)
    );
}

/// Execute SHA-256 message schedule operations.
///
/// Performs message schedule expansion using sha256msg1 and sha256msg2.
///
/// # Register Layout
///
/// - xmm0: W[i-4:i-1] (previous message block)
/// - xmm1: W[i-8:i-5] (earlier message block)
/// - xmm2: W[i-16:i-13] (oldest message block)
///
/// # Safety
///
/// - Caller must ensure SHA-NI (sha extension) is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn sha256_msg_asm() {
    asm!(
        // sha256msg1: Sigma0 message expansion
        // sha256msg2: Sigma1 message expansion

        // W[i] += sha256msg1(W[i-4], W[i-8])
        "sha256msg1 xmm0, xmm1",
        // W[i] += sha256msg2(W[i], W[i-16])
        "sha256msg2 xmm0, xmm2",
        options(nomem, nostack, preserves_flags)
    );
}
