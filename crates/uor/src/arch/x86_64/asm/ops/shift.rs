//! Shift wavefront operations via inline assembly.
//!
//! Implements logical shift right and left operations on 32-bit lanes.

use core::arch::asm;

/// Execute shift right wavefront: ymm[i] >>= n for i in 0..8 (32-bit lanes)
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Arguments
///
/// * `n` - Shift amount (0-31 bits)
///
/// # Execution
///
/// - 8 `vpsrld` instructions
/// - Execute on Port 0
/// - ~1-2 cycles total
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn shr_wavefront_asm(n: u8) {
    // Load shift count into xmm15 (lower 64 bits)
    // vpsrld takes shift count from bits 0-7 of the xmm register
    let shift_count = n as u64;
    asm!(
        // Load shift count into xmm15
        "vmovq xmm15, {count}",
        // Shift right all 8 destination registers
        "vpsrld ymm0, ymm0, xmm15",
        "vpsrld ymm1, ymm1, xmm15",
        "vpsrld ymm2, ymm2, xmm15",
        "vpsrld ymm3, ymm3, xmm15",
        "vpsrld ymm4, ymm4, xmm15",
        "vpsrld ymm5, ymm5, xmm15",
        "vpsrld ymm6, ymm6, xmm15",
        "vpsrld ymm7, ymm7, xmm15",
        count = in(reg) shift_count,
        // No memory access (besides the input), no stack, preserve flags
        options(nostack, preserves_flags)
    );
}

/// Execute shift left wavefront: ymm[i] <<= n for i in 0..8 (32-bit lanes)
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Arguments
///
/// * `n` - Shift amount (0-31 bits)
///
/// # Execution
///
/// - 8 `vpslld` instructions
/// - Execute on Port 0
/// - ~1-2 cycles total
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn shl_wavefront_asm(n: u8) {
    let shift_count = n as u64;
    asm!(
        // Load shift count into xmm15
        "vmovq xmm15, {count}",
        // Shift left all 8 destination registers
        "vpslld ymm0, ymm0, xmm15",
        "vpslld ymm1, ymm1, xmm15",
        "vpslld ymm2, ymm2, xmm15",
        "vpslld ymm3, ymm3, xmm15",
        "vpslld ymm4, ymm4, xmm15",
        "vpslld ymm5, ymm5, xmm15",
        "vpslld ymm6, ymm6, xmm15",
        "vpslld ymm7, ymm7, xmm15",
        count = in(reg) shift_count,
        options(nostack, preserves_flags)
    );
}
