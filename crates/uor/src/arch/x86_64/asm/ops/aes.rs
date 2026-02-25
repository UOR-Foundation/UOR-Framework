//! AES wavefront operations via inline assembly.
//!
//! Uses AES-NI instructions available on modern AMD and Intel CPUs.
//! AES-NI operates on XMM (128-bit) registers, which are the lower half of YMM registers.

use core::arch::asm;

/// Execute AES encryption round via aesenc instruction.
///
/// Performs one AES encryption round using AES-NI.
///
/// # Register Layout
///
/// - xmm0, xmm1: State blocks (lower 128 bits of ymm0, ymm1)
/// - xmm8, xmm9: Round keys (lower 128 bits of ymm8, ymm9)
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Execution
///
/// - 2 `aesenc` instructions (parallel on Ports 1 and 5)
/// - ~4 cycles latency, 1 cycle throughput
///
/// # Safety
///
/// - Caller must ensure AES-NI (aes extension) is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn aes_enc_round_asm() {
    asm!(
        // AES encryption round: state = SubBytes + ShiftRows + MixColumns + AddRoundKey
        // aesenc xmm_state, xmm_roundkey

        // Two parallel AES rounds (Port 1 and Port 5)
        "aesenc xmm0, xmm8",
        "aesenc xmm1, xmm9",
        options(nomem, nostack, preserves_flags)
    );
}

/// Execute AES decryption round via aesdec instruction.
///
/// Performs one AES decryption round using AES-NI.
///
/// # Register Layout
///
/// - xmm0, xmm1: State blocks (lower 128 bits of ymm0, ymm1)
/// - xmm8, xmm9: Round keys (lower 128 bits of ymm8, ymm9)
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Execution
///
/// - 2 `aesdec` instructions (parallel on Ports 1 and 5)
/// - ~4 cycles latency, 1 cycle throughput
///
/// # Safety
///
/// - Caller must ensure AES-NI (aes extension) is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn aes_dec_round_asm() {
    asm!(
        // AES decryption round: inverse of encryption
        // aesdec xmm_state, xmm_roundkey

        // Two parallel AES decryption rounds
        "aesdec xmm0, xmm8",
        "aesdec xmm1, xmm9",
        options(nomem, nostack, preserves_flags)
    );
}
