//! Rotate wavefront operations via inline assembly.
//!
//! AVX2 does not have native rotate instructions, so rotation is emulated:
//! - RotateRight(n): (x >> n) | (x << (32-n))
//! - RotateLeft(n): (x << n) | (x >> (32-n))
//!
//! Uses ymm8-15 as scratch registers for intermediate values.

use core::arch::asm;

/// Execute rotate right wavefront: ymm[i] = rotr(ymm[i], n) for i in 0..8
///
/// Rotation is performed on 32-bit lanes.
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Note
///
/// This operation clobbers ymm8-15 (used as scratch registers).
///
/// # Arguments
///
/// * `n` - Rotation amount (1-31 bits)
///
/// # Execution
///
/// - 16 shift instructions + 8 OR instructions
/// - ~3-4 cycles total
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn rotr_wavefront_asm(n: u8) {
    let shift_right = n as u64;
    let shift_left = (32 - n) as u64;

    asm!(
        // Load shift counts into xmm14 and xmm15
        "vmovq xmm14, {sr}",
        "vmovq xmm15, {sl}",

        // For each register: copy to scratch, shift both, OR together
        // ymm0
        "vpsrld ymm8, ymm0, xmm14",     // ymm8 = ymm0 >> n
        "vpslld ymm0, ymm0, xmm15",     // ymm0 = ymm0 << (32-n)
        "vpor ymm0, ymm0, ymm8",        // ymm0 = (ymm0 << (32-n)) | (ymm0 >> n)

        // ymm1
        "vpsrld ymm9, ymm1, xmm14",
        "vpslld ymm1, ymm1, xmm15",
        "vpor ymm1, ymm1, ymm9",

        // ymm2
        "vpsrld ymm10, ymm2, xmm14",
        "vpslld ymm2, ymm2, xmm15",
        "vpor ymm2, ymm2, ymm10",

        // ymm3
        "vpsrld ymm11, ymm3, xmm14",
        "vpslld ymm3, ymm3, xmm15",
        "vpor ymm3, ymm3, ymm11",

        // ymm4
        "vpsrld ymm12, ymm4, xmm14",
        "vpslld ymm4, ymm4, xmm15",
        "vpor ymm4, ymm4, ymm12",

        // ymm5
        "vpsrld ymm13, ymm5, xmm14",
        "vpslld ymm5, ymm5, xmm15",
        "vpor ymm5, ymm5, ymm13",

        // ymm6 - reuse ymm8 as scratch
        "vpsrld ymm8, ymm6, xmm14",
        "vpslld ymm6, ymm6, xmm15",
        "vpor ymm6, ymm6, ymm8",

        // ymm7 - reuse ymm9 as scratch
        "vpsrld ymm9, ymm7, xmm14",
        "vpslld ymm7, ymm7, xmm15",
        "vpor ymm7, ymm7, ymm9",

        sr = in(reg) shift_right,
        sl = in(reg) shift_left,
        options(nostack, preserves_flags)
    );
}

/// Execute rotate left wavefront: ymm[i] = rotl(ymm[i], n) for i in 0..8
///
/// Rotation is performed on 32-bit lanes.
///
/// # Precondition
///
/// State must already be loaded into YMM0-15 via `load_ymm_state`.
///
/// # Note
///
/// This operation clobbers ymm8-15 (used as scratch registers).
///
/// # Arguments
///
/// * `n` - Rotation amount (1-31 bits)
///
/// # Execution
///
/// - 16 shift instructions + 8 OR instructions
/// - ~3-4 cycles total
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - YMM registers must contain valid state from `load_ymm_state`
#[inline]
pub unsafe fn rotl_wavefront_asm(n: u8) {
    let shift_left = n as u64;
    let shift_right = (32 - n) as u64;

    asm!(
        // Load shift counts
        "vmovq xmm14, {sl}",
        "vmovq xmm15, {sr}",

        // For each register: rotl = (x << n) | (x >> (32-n))
        // ymm0
        "vpsrld ymm8, ymm0, xmm15",
        "vpslld ymm0, ymm0, xmm14",
        "vpor ymm0, ymm0, ymm8",

        // ymm1
        "vpsrld ymm9, ymm1, xmm15",
        "vpslld ymm1, ymm1, xmm14",
        "vpor ymm1, ymm1, ymm9",

        // ymm2
        "vpsrld ymm10, ymm2, xmm15",
        "vpslld ymm2, ymm2, xmm14",
        "vpor ymm2, ymm2, ymm10",

        // ymm3
        "vpsrld ymm11, ymm3, xmm15",
        "vpslld ymm3, ymm3, xmm14",
        "vpor ymm3, ymm3, ymm11",

        // ymm4
        "vpsrld ymm12, ymm4, xmm15",
        "vpslld ymm4, ymm4, xmm14",
        "vpor ymm4, ymm4, ymm12",

        // ymm5
        "vpsrld ymm13, ymm5, xmm15",
        "vpslld ymm5, ymm5, xmm14",
        "vpor ymm5, ymm5, ymm13",

        // ymm6 - reuse ymm8
        "vpsrld ymm8, ymm6, xmm15",
        "vpslld ymm6, ymm6, xmm14",
        "vpor ymm6, ymm6, ymm8",

        // ymm7 - reuse ymm9
        "vpsrld ymm9, ymm7, xmm15",
        "vpslld ymm7, ymm7, xmm14",
        "vpor ymm7, ymm7, ymm9",

        sl = in(reg) shift_left,
        sr = in(reg) shift_right,
        options(nostack, preserves_flags)
    );
}
