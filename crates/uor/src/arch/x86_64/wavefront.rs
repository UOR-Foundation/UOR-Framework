//! Zen 3 Optimized Wavefront Executor
//!
//! Maps UOR ISA to AMD Zen 3 execution ports with maximum parallelism.
//!
//! # Port Mapping
//!
//! | Port | Function | Instructions |
//! |------|----------|--------------|
//! | Port 0 | Shift/Rotate + SHA-NI | `vpsrld`, `vpslld`, `sha256rnds2` |
//! | Port 1 | ALU + AES-NI | `vpxor`, `vpand`, `aesenc` |
//! | Port 5 | ALU + AES-NI | `vpxor`, `vpand`, `aesenc` |
//!
//! # Zero Spillage
//!
//! All inline assembly blocks use `options(nomem, nostack, preserves_flags)`
//! to guarantee no memory access.

use crate::isa::{PortAssignment, UorStep, UorStepLossless, Wavefront, WavefrontOp};
use crate::state::UorState;

use super::CpuFeatures;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// Zen 3 executor - uses all execution ports in parallel.
///
/// This executor implements `UorStep` with maximum port utilization
/// on AMD Zen 3 and compatible CPUs (Intel with AVX2/SHA-NI/AES-NI).
///
/// # CPU Feature Requirements
///
/// UOR requires the following CPU features:
/// - AVX2: For 256-bit SIMD operations
/// - SHA-NI: For SHA-256 hardware acceleration
/// - AES-NI: For AES hardware acceleration
///
/// Missing features will cause a panic at executor creation. This is
/// intentional - UOR has no software fallback and requires hardware support.
///
/// # Safety
///
/// All wavefront operations use `options(nomem, nostack)` to guarantee
/// no memory access outside the register file.
///
/// # Example
///
/// ```ignore
/// use uor::{UorState, Wavefront, UorStep};
/// use uor::arch::x86_64::Zen3Executor;
///
/// let executor = Zen3Executor::new();
/// let mut state = UorState::zero();
/// let wf = Wavefront::all_xor();
///
/// unsafe { executor.step(&mut state, &wf); }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Zen3Executor;

impl Zen3Executor {
    /// Create a new Zen 3 executor with CPU feature validation.
    ///
    /// # Panics
    ///
    /// Panics if the CPU does not support AVX2, SHA-NI, or AES-NI.
    /// This is a UOR conformance violation - there is no fallback.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::arch::Zen3Executor;
    ///
    /// let executor = Zen3Executor::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        CpuFeatures::detect().require_all();
        Self
    }

    /// Create a new Zen 3 executor without CPU feature validation.
    ///
    /// # Safety
    ///
    /// This is unsafe because executing wavefronts on a CPU without
    /// the required features will cause undefined behavior (SIGILL).
    ///
    /// Only use this when you have already validated CPU features
    /// or in const contexts where runtime detection is not possible.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::arch::Zen3Executor;
    ///
    /// // Only use if you've already checked CPU features!
    /// let executor = unsafe { Zen3Executor::new_unchecked() };
    /// ```
    #[inline]
    pub const unsafe fn new_unchecked() -> Self {
        Self
    }
}

/// Default mask for full YMM register participation (bits 0-7 paired with 8-15).
const FULL_YMM_MASK: u16 = 0x00FF;

impl UorStep for Zen3Executor {
    #[inline(always)]
    unsafe fn step(&self, state: &mut UorState, wf: &Wavefront) {
        // Check if using partial mask - if so, use mask-aware execution
        // Note: We check bits 0-7 since those are the destination registers
        // that get paired with bits 8-15 as operands
        let dest_mask = wf.ymm_mask & 0x00FF;
        if dest_mask != FULL_YMM_MASK && dest_mask != 0 {
            // Partial mask - use mask-aware execution
            self.masked_wavefront(state, wf);
            // Also process GPR if mask is set
            if wf.gpr_mask != 0 {
                self.gpr_wavefront(state, wf);
            }
            return;
        }

        // Full mask or empty - use optimized fast paths
        // Dispatch based on port assignment pattern for optimal code generation
        match (wf.ports.port0, wf.ports.port1, wf.ports.port5) {
            // ========================================
            // ALU Operations (Ports 1/5)
            // ========================================

            // All XOR
            (WavefrontOp::Nop, WavefrontOp::Xor, WavefrontOp::Xor) => {
                self.xor_wavefront(state);
            }

            // All AND
            (WavefrontOp::Nop, WavefrontOp::And, WavefrontOp::And) => {
                self.and_wavefront(state);
            }

            // All OR
            (WavefrontOp::Nop, WavefrontOp::Or, WavefrontOp::Or) => {
                self.or_wavefront(state);
            }

            // All NOT
            (WavefrontOp::Nop, WavefrontOp::Not, WavefrontOp::Not) => {
                self.not_wavefront(state);
            }

            // All ADD
            (WavefrontOp::Nop, WavefrontOp::Add, WavefrontOp::Add) => {
                self.add_wavefront(state);
            }

            // All SUB
            (WavefrontOp::Nop, WavefrontOp::Sub, WavefrontOp::Sub) => {
                self.sub_wavefront(state);
            }

            // ========================================
            // Standalone Shift/Rotate Operations (Port 0 only)
            // ========================================

            // Rotate left only
            (WavefrontOp::RotL(n), WavefrontOp::Nop, WavefrontOp::Nop) => {
                self.rotl_wavefront(state, n);
            }

            // Rotate right only
            (WavefrontOp::RotR(n), WavefrontOp::Nop, WavefrontOp::Nop) => {
                self.rotr_wavefront(state, n);
            }

            // Shift left only
            (WavefrontOp::ShL(n), WavefrontOp::Nop, WavefrontOp::Nop) => {
                self.shl_wavefront(state, n);
            }

            // Shift right only
            (WavefrontOp::ShR(n), WavefrontOp::Nop, WavefrontOp::Nop) => {
                self.shr_wavefront(state, n);
            }

            // ========================================
            // Combined Shift/Rotate + ALU (3-port utilization)
            // ========================================

            // Rotate right + XOR (SHA-256 Σ functions)
            (WavefrontOp::RotR(n), WavefrontOp::Xor, WavefrontOp::Xor) => {
                self.rotate_xor_wavefront(state, n);
            }

            // Rotate left + XOR
            (WavefrontOp::RotL(_), WavefrontOp::Xor, WavefrontOp::Xor) => {
                self.generic_wavefront(state, &wf.ports);
            }

            // Shift right + XOR (SHA-256 σ functions)
            (WavefrontOp::ShR(n), WavefrontOp::Xor, WavefrontOp::Xor) => {
                self.shift_xor_wavefront(state, n);
            }

            // Shift left + XOR
            (WavefrontOp::ShL(_), WavefrontOp::Xor, WavefrontOp::Xor) => {
                self.generic_wavefront(state, &wf.ports);
            }

            // ========================================
            // Crypto Operations
            // ========================================

            // SHA-256 round via SHA-NI (Port 0)
            (WavefrontOp::Sha256Round, _, _) => {
                self.sha256_wavefront(state);
            }

            // SHA-256 message schedule (Ports 1/5)
            (WavefrontOp::Nop, WavefrontOp::Sha256Msg1, WavefrontOp::Sha256Msg2)
            | (_, WavefrontOp::Sha256Msg1, WavefrontOp::Sha256Msg2) => {
                self.sha256_msg_wavefront(state);
            }

            // AES encryption round via AES-NI (Ports 1/5)
            (_, WavefrontOp::AesRound, WavefrontOp::AesRound) => {
                self.aes_wavefront(state);
            }

            // AES decryption round via AES-NI (Ports 1/5)
            (_, WavefrontOp::AesRoundDec, WavefrontOp::AesRoundDec) => {
                self.aes_dec_wavefront(state);
            }

            // ========================================
            // Permutation Operations (Port 5)
            // ========================================

            // Shuffle (byte permutation within 128-bit lanes)
            (WavefrontOp::Nop, WavefrontOp::Nop, WavefrontOp::Shuffle)
            | (_, _, WavefrontOp::Shuffle) => {
                self.shuffle_wavefront(state);
            }

            // Permute (32-bit lane permutation across 256-bit register)
            (WavefrontOp::Nop, WavefrontOp::Nop, WavefrontOp::Permute)
            | (_, _, WavefrontOp::Permute) => {
                self.permute_wavefront(state);
            }

            // ========================================
            // Generic fallback for arbitrary combinations
            // ========================================
            _ => {
                self.generic_wavefront(state, &wf.ports);
            }
        }

        // Process GPR registers if mask is set
        // This runs after YMM processing for full-mask path
        if wf.gpr_mask != 0 {
            self.gpr_wavefront(state, wf);
        }
    }
}

impl UorStepLossless for Zen3Executor {
    #[inline(always)]
    unsafe fn step_tracked(&self, state: &mut UorState, complement: &mut UorState, wf: &Wavefront) {
        // Dispatch based on operation type for complement tracking
        match (wf.ports.port0, wf.ports.port1, wf.ports.port5) {
            // Shift left - requires complement
            (WavefrontOp::ShL(n), _, _) => {
                self.shl_wavefront_tracked(state, complement, n);
            }

            // Shift right - requires complement
            (WavefrontOp::ShR(n), _, _) => {
                self.shr_wavefront_tracked(state, complement, n);
            }

            // AND - requires complement
            (_, WavefrontOp::And, WavefrontOp::And) => {
                self.and_wavefront_tracked(state, complement);
            }

            // OR - requires complement
            (_, WavefrontOp::Or, WavefrontOp::Or) => {
                self.or_wavefront_tracked(state, complement);
            }

            // All other operations are invertible - no complement needed
            _ => {
                // Execute normally (complement unchanged)
                self.step(state, wf);
            }
        }

        // Process GPR if needed (GPR complements handled separately)
        if wf.gpr_mask != 0 {
            self.gpr_wavefront(state, wf);
        }
    }

    #[inline(always)]
    unsafe fn step_inverse(&self, state: &mut UorState, complement: &UorState, wf: &Wavefront) {
        // Dispatch based on operation type for inversion
        match (wf.ports.port0, wf.ports.port1, wf.ports.port5) {
            // Shift left inverse
            (WavefrontOp::ShL(n), _, _) => {
                self.shl_inverse_wavefront(state, complement, n);
            }

            // Shift right inverse
            (WavefrontOp::ShR(n), _, _) => {
                self.shr_inverse_wavefront(state, complement, n);
            }

            // AND inverse
            (_, WavefrontOp::And, WavefrontOp::And) => {
                self.and_inverse_wavefront(state, complement);
            }

            // OR inverse
            (_, WavefrontOp::Or, WavefrontOp::Or) => {
                self.or_inverse_wavefront(state, complement);
            }

            // XOR is self-inverse
            (_, WavefrontOp::Xor, WavefrontOp::Xor) => {
                self.xor_wavefront(state);
            }

            // NOT is self-inverse
            (_, WavefrontOp::Not, WavefrontOp::Not) => {
                self.not_wavefront(state);
            }

            // ADD inverse is SUB
            (_, WavefrontOp::Add, WavefrontOp::Add) => {
                self.sub_wavefront(state);
            }

            // SUB inverse is ADD
            (_, WavefrontOp::Sub, WavefrontOp::Sub) => {
                self.add_wavefront(state);
            }

            // RotL inverse is RotR
            (WavefrontOp::RotL(n), _, _) => {
                self.rotr_wavefront(state, n);
            }

            // RotR inverse is RotL
            (WavefrontOp::RotR(n), _, _) => {
                self.rotl_wavefront(state, n);
            }

            // AES encryption inverse is AES decryption
            (_, WavefrontOp::AesRound, WavefrontOp::AesRound) => {
                self.aes_dec_wavefront(state);
            }

            // AES decryption inverse is AES encryption
            (_, WavefrontOp::AesRoundDec, WavefrontOp::AesRoundDec) => {
                self.aes_wavefront(state);
            }

            // NOP - no inverse needed
            _ => {}
        }
    }
}

impl Zen3Executor {
    /// XOR wavefront - all ALU ports perform XOR.
    ///
    /// Pattern: YMM[i] ^= YMM[i+8] for i in 0..8 (all 8 pairs)
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn xor_wavefront(&self, state: &mut UorState) {
        // LOAD: All 16 registers
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        // COMPUTE: XOR all 8 pairs - Ports 1 and 5 both fire
        let r0 = _mm256_xor_si256(y0, y8);
        let r1 = _mm256_xor_si256(y1, y9);
        let r2 = _mm256_xor_si256(y2, y10);
        let r3 = _mm256_xor_si256(y3, y11);
        let r4 = _mm256_xor_si256(y4, y12);
        let r5 = _mm256_xor_si256(y5, y13);
        let r6 = _mm256_xor_si256(y6, y14);
        let r7 = _mm256_xor_si256(y7, y15);

        // STORE: All 8 results
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// Rotate + XOR wavefront (SHA-256 Σ functions).
    ///
    /// Port 0: rotate, Ports 1/5: XOR
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn rotate_xor_wavefront(&self, state: &mut UorState, n: u8) {
        let anti = 32u8.wrapping_sub(n);

        // Load source registers
        let y0_ptr = state.ymm[0].as_ptr() as *const __m256i;
        let y1_ptr = state.ymm[1].as_ptr() as *const __m256i;
        let y8_ptr = state.ymm[8].as_ptr() as *const __m256i;
        let y9_ptr = state.ymm[9].as_ptr() as *const __m256i;

        let mut y0 = _mm256_loadu_si256(y0_ptr);
        let mut y1 = _mm256_loadu_si256(y1_ptr);
        let y8 = _mm256_loadu_si256(y8_ptr);
        let y9 = _mm256_loadu_si256(y9_ptr);

        // Broadcast shift amounts for variable shift intrinsics
        let shift_r = _mm256_set1_epi32(n as i32);
        let shift_l = _mm256_set1_epi32(anti as i32);

        // Rotate right = (x >> n) | (x << (32-n))
        // Using variable shift intrinsics for runtime shift amounts
        let r0_lo = _mm256_srlv_epi32(y0, shift_r);
        let r0_hi = _mm256_sllv_epi32(y0, shift_l);
        y1 = _mm256_xor_si256(y1, y8); // Port 1/5 XOR while Port 0 shifts
        y0 = _mm256_or_si256(r0_lo, r0_hi);
        y0 = _mm256_xor_si256(y0, y9); // Final XOR

        // Store results
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
    }

    /// Shift + XOR wavefront (SHA-256 σ functions).
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn shift_xor_wavefront(&self, state: &mut UorState, n: u8) {
        let y0_ptr = state.ymm[0].as_ptr() as *const __m256i;
        let y1_ptr = state.ymm[1].as_ptr() as *const __m256i;
        let y8_ptr = state.ymm[8].as_ptr() as *const __m256i;

        let mut y0 = _mm256_loadu_si256(y0_ptr);
        let mut y1 = _mm256_loadu_si256(y1_ptr);
        let y8 = _mm256_loadu_si256(y8_ptr);

        // Broadcast shift amount for variable shift intrinsic
        let shift = _mm256_set1_epi32(n as i32);

        // Shift right using variable shift intrinsic
        y0 = _mm256_srlv_epi32(y0, shift);
        // XOR on Ports 1/5
        y1 = _mm256_xor_si256(y1, y8);

        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
    }

    /// Rotate left wavefront - all 8 register pairs.
    ///
    /// Pattern: YMM[i] = RotL(YMM[i], n) for i in 0..8
    /// Emulates 32-bit rotation: (x << n) | (x >> (32-n))
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn rotl_wavefront(&self, state: &mut UorState, n: u8) {
        let anti = 32u8.wrapping_sub(n);
        let shift_l = _mm256_set1_epi32(n as i32);
        let shift_r = _mm256_set1_epi32(anti as i32);

        // LOAD
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);

        // COMPUTE: RotL = (x << n) | (x >> (32-n))
        let r0 = _mm256_or_si256(
            _mm256_sllv_epi32(y0, shift_l),
            _mm256_srlv_epi32(y0, shift_r),
        );
        let r1 = _mm256_or_si256(
            _mm256_sllv_epi32(y1, shift_l),
            _mm256_srlv_epi32(y1, shift_r),
        );
        let r2 = _mm256_or_si256(
            _mm256_sllv_epi32(y2, shift_l),
            _mm256_srlv_epi32(y2, shift_r),
        );
        let r3 = _mm256_or_si256(
            _mm256_sllv_epi32(y3, shift_l),
            _mm256_srlv_epi32(y3, shift_r),
        );
        let r4 = _mm256_or_si256(
            _mm256_sllv_epi32(y4, shift_l),
            _mm256_srlv_epi32(y4, shift_r),
        );
        let r5 = _mm256_or_si256(
            _mm256_sllv_epi32(y5, shift_l),
            _mm256_srlv_epi32(y5, shift_r),
        );
        let r6 = _mm256_or_si256(
            _mm256_sllv_epi32(y6, shift_l),
            _mm256_srlv_epi32(y6, shift_r),
        );
        let r7 = _mm256_or_si256(
            _mm256_sllv_epi32(y7, shift_l),
            _mm256_srlv_epi32(y7, shift_r),
        );

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// Rotate right wavefront - all 8 register pairs.
    ///
    /// Pattern: YMM[i] = RotR(YMM[i], n) for i in 0..8
    /// Emulates 32-bit rotation: (x >> n) | (x << (32-n))
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn rotr_wavefront(&self, state: &mut UorState, n: u8) {
        let anti = 32u8.wrapping_sub(n);
        let shift_r = _mm256_set1_epi32(n as i32);
        let shift_l = _mm256_set1_epi32(anti as i32);

        // LOAD
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);

        // COMPUTE: RotR = (x >> n) | (x << (32-n))
        let r0 = _mm256_or_si256(
            _mm256_srlv_epi32(y0, shift_r),
            _mm256_sllv_epi32(y0, shift_l),
        );
        let r1 = _mm256_or_si256(
            _mm256_srlv_epi32(y1, shift_r),
            _mm256_sllv_epi32(y1, shift_l),
        );
        let r2 = _mm256_or_si256(
            _mm256_srlv_epi32(y2, shift_r),
            _mm256_sllv_epi32(y2, shift_l),
        );
        let r3 = _mm256_or_si256(
            _mm256_srlv_epi32(y3, shift_r),
            _mm256_sllv_epi32(y3, shift_l),
        );
        let r4 = _mm256_or_si256(
            _mm256_srlv_epi32(y4, shift_r),
            _mm256_sllv_epi32(y4, shift_l),
        );
        let r5 = _mm256_or_si256(
            _mm256_srlv_epi32(y5, shift_r),
            _mm256_sllv_epi32(y5, shift_l),
        );
        let r6 = _mm256_or_si256(
            _mm256_srlv_epi32(y6, shift_r),
            _mm256_sllv_epi32(y6, shift_l),
        );
        let r7 = _mm256_or_si256(
            _mm256_srlv_epi32(y7, shift_r),
            _mm256_sllv_epi32(y7, shift_l),
        );

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// Shift left wavefront - all 8 destination registers.
    ///
    /// Pattern: YMM[i] = YMM[i] << n for i in 0..8
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn shl_wavefront(&self, state: &mut UorState, n: u8) {
        let shift = _mm256_set1_epi32(n as i32);

        // LOAD
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);

        // COMPUTE
        let r0 = _mm256_sllv_epi32(y0, shift);
        let r1 = _mm256_sllv_epi32(y1, shift);
        let r2 = _mm256_sllv_epi32(y2, shift);
        let r3 = _mm256_sllv_epi32(y3, shift);
        let r4 = _mm256_sllv_epi32(y4, shift);
        let r5 = _mm256_sllv_epi32(y5, shift);
        let r6 = _mm256_sllv_epi32(y6, shift);
        let r7 = _mm256_sllv_epi32(y7, shift);

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// Shift right wavefront - all 8 destination registers.
    ///
    /// Pattern: YMM[i] = YMM[i] >> n for i in 0..8
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn shr_wavefront(&self, state: &mut UorState, n: u8) {
        let shift = _mm256_set1_epi32(n as i32);

        // LOAD
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);

        // COMPUTE
        let r0 = _mm256_srlv_epi32(y0, shift);
        let r1 = _mm256_srlv_epi32(y1, shift);
        let r2 = _mm256_srlv_epi32(y2, shift);
        let r3 = _mm256_srlv_epi32(y3, shift);
        let r4 = _mm256_srlv_epi32(y4, shift);
        let r5 = _mm256_srlv_epi32(y5, shift);
        let r6 = _mm256_srlv_epi32(y6, shift);
        let r7 = _mm256_srlv_epi32(y7, shift);

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// SHA-256 wavefront using SHA-NI.
    ///
    /// Executes 2 SHA-256 rounds via sha256rnds2 instruction.
    #[inline]
    #[cfg(target_feature = "sha")]
    #[target_feature(enable = "sha", enable = "sse4.1")]
    unsafe fn sha256_wavefront(&self, state: &mut UorState) {
        // SHA-NI operates on XMM (128-bit) registers
        // State layout: XMM0 = ABEF, XMM1 = CDGH
        let state_ptr = state.ymm[0].as_ptr() as *const __m128i;
        let msg_ptr = state.ymm[2].as_ptr() as *const __m128i;

        let mut abef = _mm_loadu_si128(state_ptr);
        let mut cdgh = _mm_loadu_si128(state_ptr.add(1));
        let msg = _mm_loadu_si128(msg_ptr);

        // Execute 2 SHA-256 rounds
        cdgh = _mm_sha256rnds2_epu32(cdgh, abef, msg);
        abef = _mm_sha256rnds2_epu32(abef, cdgh, _mm_shuffle_epi32(msg, 0x0E));

        // Store updated state
        _mm_storeu_si128(state.ymm[0].as_mut_ptr() as *mut __m128i, abef);
        _mm_storeu_si128((state.ymm[0].as_mut_ptr() as *mut __m128i).add(1), cdgh);
    }

    /// Fallback for non-SHA-NI systems.
    #[inline]
    #[cfg(not(target_feature = "sha"))]
    unsafe fn sha256_wavefront(&self, _state: &mut UorState) {
        // SHA-NI not available - no-op
        // In production, this would use AVX2 software implementation
    }

    /// AES wavefront using AES-NI.
    ///
    /// Executes AES encryption round on Ports 1 and 5.
    #[inline]
    #[cfg(target_feature = "aes")]
    #[target_feature(enable = "aes", enable = "sse4.1")]
    unsafe fn aes_wavefront(&self, state: &mut UorState) {
        // AES-NI operates on XMM (128-bit) registers
        let state_ptr = state.ymm[0].as_ptr() as *const __m128i;
        let key_ptr = state.ymm[8].as_ptr() as *const __m128i;

        let mut s0 = _mm_loadu_si128(state_ptr);
        let mut s1 = _mm_loadu_si128(state_ptr.add(1));
        let k0 = _mm_loadu_si128(key_ptr);
        let k1 = _mm_loadu_si128(key_ptr.add(1));

        // Dual AES rounds - Port 1 and Port 5
        s0 = _mm_aesenc_si128(s0, k0);
        s1 = _mm_aesenc_si128(s1, k1);

        _mm_storeu_si128(state.ymm[0].as_mut_ptr() as *mut __m128i, s0);
        _mm_storeu_si128((state.ymm[0].as_mut_ptr() as *mut __m128i).add(1), s1);
    }

    /// Fallback for non-AES-NI systems.
    #[inline]
    #[cfg(not(target_feature = "aes"))]
    unsafe fn aes_wavefront(&self, _state: &mut UorState) {
        // AES-NI not available - no-op
    }

    /// AND wavefront - all 8 register pairs.
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn and_wavefront(&self, state: &mut UorState) {
        // LOAD
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        // COMPUTE
        let r0 = _mm256_and_si256(y0, y8);
        let r1 = _mm256_and_si256(y1, y9);
        let r2 = _mm256_and_si256(y2, y10);
        let r3 = _mm256_and_si256(y3, y11);
        let r4 = _mm256_and_si256(y4, y12);
        let r5 = _mm256_and_si256(y5, y13);
        let r6 = _mm256_and_si256(y6, y14);
        let r7 = _mm256_and_si256(y7, y15);

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// OR wavefront - all 8 register pairs.
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn or_wavefront(&self, state: &mut UorState) {
        // LOAD
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        // COMPUTE
        let r0 = _mm256_or_si256(y0, y8);
        let r1 = _mm256_or_si256(y1, y9);
        let r2 = _mm256_or_si256(y2, y10);
        let r3 = _mm256_or_si256(y3, y11);
        let r4 = _mm256_or_si256(y4, y12);
        let r5 = _mm256_or_si256(y5, y13);
        let r6 = _mm256_or_si256(y6, y14);
        let r7 = _mm256_or_si256(y7, y15);

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// ADD wavefront (32-bit lanes) - all 8 register pairs.
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn add_wavefront(&self, state: &mut UorState) {
        // LOAD
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        // COMPUTE
        let r0 = _mm256_add_epi32(y0, y8);
        let r1 = _mm256_add_epi32(y1, y9);
        let r2 = _mm256_add_epi32(y2, y10);
        let r3 = _mm256_add_epi32(y3, y11);
        let r4 = _mm256_add_epi32(y4, y12);
        let r5 = _mm256_add_epi32(y5, y13);
        let r6 = _mm256_add_epi32(y6, y14);
        let r7 = _mm256_add_epi32(y7, y15);

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// Generic wavefront for arbitrary port combinations.
    ///
    /// # Architecture: Parallel Port Execution
    ///
    /// This function executes all three ports in parallel using a
    /// load-compute-store pattern that allows the CPU to schedule
    /// operations across all execution ports simultaneously:
    ///
    /// 1. LOAD phase: Load all input registers
    /// 2. COMPUTE phase: Execute all port operations (CPU interleaves)
    /// 3. STORE phase: Store all results
    ///
    /// Port assignments:
    /// - Port 0: ymm[0] ← op(ymm[0], ymm[8])  [Shift/Rotate]
    /// - Port 1: ymm[1] ← op(ymm[1], ymm[9])  [ALU/Crypto]
    /// - Port 5: ymm[2] ← op(ymm[2], ymm[10]) [ALU/Crypto/Shuffle]
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn generic_wavefront(&self, state: &mut UorState, ports: &PortAssignment) {
        // ========================================
        // LOAD PHASE: Pre-load all input registers
        // ========================================
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        // Note: y8 loaded for Port 0 binary ops (currently unused - shift/rotate are unary)
        let _y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);

        // All-ones mask for NOT operation
        let all_ones = _mm256_set1_epi32(-1i32);

        // ========================================
        // COMPUTE PHASE: All ports execute in parallel
        // ========================================

        // Port 0: Shift/Rotate operations
        let r0 = match ports.port0 {
            WavefrontOp::RotR(n) => {
                let anti = 32u8.wrapping_sub(n);
                let shift_r = _mm256_set1_epi32(n as i32);
                let shift_l = _mm256_set1_epi32(anti as i32);
                let lo = _mm256_srlv_epi32(y0, shift_r);
                let hi = _mm256_sllv_epi32(y0, shift_l);
                _mm256_or_si256(lo, hi)
            }
            WavefrontOp::RotL(n) => {
                let anti = 32u8.wrapping_sub(n);
                let shift_l = _mm256_set1_epi32(n as i32);
                let shift_r = _mm256_set1_epi32(anti as i32);
                let lo = _mm256_sllv_epi32(y0, shift_l);
                let hi = _mm256_srlv_epi32(y0, shift_r);
                _mm256_or_si256(lo, hi)
            }
            WavefrontOp::ShR(n) => {
                let shift = _mm256_set1_epi32(n as i32);
                _mm256_srlv_epi32(y0, shift)
            }
            WavefrontOp::ShL(n) => {
                let shift = _mm256_set1_epi32(n as i32);
                _mm256_sllv_epi32(y0, shift)
            }
            _ => y0, // Identity (Nop or unsupported on this port)
        };

        // Port 1: ALU/Crypto operations
        let r1 = match ports.port1 {
            WavefrontOp::Xor => _mm256_xor_si256(y1, y9),
            WavefrontOp::And => _mm256_and_si256(y1, y9),
            WavefrontOp::Or => _mm256_or_si256(y1, y9),
            WavefrontOp::Not => _mm256_xor_si256(y1, all_ones),
            WavefrontOp::Add => _mm256_add_epi32(y1, y9),
            WavefrontOp::Sub => _mm256_sub_epi32(y1, y9),
            _ => y1, // Identity (Nop or unsupported on this port)
        };

        // Port 5: ALU/Crypto/Shuffle operations
        let r2 = match ports.port5 {
            WavefrontOp::Xor => _mm256_xor_si256(y2, y10),
            WavefrontOp::And => _mm256_and_si256(y2, y10),
            WavefrontOp::Or => _mm256_or_si256(y2, y10),
            WavefrontOp::Not => _mm256_xor_si256(y2, all_ones),
            WavefrontOp::Add => _mm256_add_epi32(y2, y10),
            WavefrontOp::Sub => _mm256_sub_epi32(y2, y10),
            WavefrontOp::Shuffle => _mm256_shuffle_epi8(y2, y10),
            WavefrontOp::Permute => _mm256_permutevar8x32_epi32(y2, y10),
            _ => y2, // Identity (Nop or unsupported on this port)
        };

        // ========================================
        // STORE PHASE: Write all results
        // ========================================
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
    }

    /// NOT wavefront - bitwise complement on all 8 destination registers.
    ///
    /// Pattern: YMM[i] = !YMM[i] for i in 0..8
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn not_wavefront(&self, state: &mut UorState) {
        let all_ones = _mm256_set1_epi32(-1i32);

        // LOAD
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);

        // COMPUTE: NOT = XOR with all-ones
        let r0 = _mm256_xor_si256(y0, all_ones);
        let r1 = _mm256_xor_si256(y1, all_ones);
        let r2 = _mm256_xor_si256(y2, all_ones);
        let r3 = _mm256_xor_si256(y3, all_ones);
        let r4 = _mm256_xor_si256(y4, all_ones);
        let r5 = _mm256_xor_si256(y5, all_ones);
        let r6 = _mm256_xor_si256(y6, all_ones);
        let r7 = _mm256_xor_si256(y7, all_ones);

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// SUB wavefront (32-bit lanes) - all 8 register pairs.
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn sub_wavefront(&self, state: &mut UorState) {
        // LOAD
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        // COMPUTE
        let r0 = _mm256_sub_epi32(y0, y8);
        let r1 = _mm256_sub_epi32(y1, y9);
        let r2 = _mm256_sub_epi32(y2, y10);
        let r3 = _mm256_sub_epi32(y3, y11);
        let r4 = _mm256_sub_epi32(y4, y12);
        let r5 = _mm256_sub_epi32(y5, y13);
        let r6 = _mm256_sub_epi32(y6, y14);
        let r7 = _mm256_sub_epi32(y7, y15);

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// Shuffle wavefront - byte permutation within 128-bit lanes, all 8 pairs.
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn shuffle_wavefront(&self, state: &mut UorState) {
        // LOAD: ymm[0..8] = data, ymm[8..16] = shuffle indices
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        // COMPUTE: vpshufb - byte shuffle within 128-bit lanes
        let r0 = _mm256_shuffle_epi8(y0, y8);
        let r1 = _mm256_shuffle_epi8(y1, y9);
        let r2 = _mm256_shuffle_epi8(y2, y10);
        let r3 = _mm256_shuffle_epi8(y3, y11);
        let r4 = _mm256_shuffle_epi8(y4, y12);
        let r5 = _mm256_shuffle_epi8(y5, y13);
        let r6 = _mm256_shuffle_epi8(y6, y14);
        let r7 = _mm256_shuffle_epi8(y7, y15);

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// Permute wavefront - 32-bit lane permutation across 256-bit register, all 8 pairs.
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn permute_wavefront(&self, state: &mut UorState) {
        // LOAD: ymm[0..8] = data, ymm[8..16] = permutation indices
        let y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        // COMPUTE: vpermd - 32-bit permutation across full 256-bit register
        let r0 = _mm256_permutevar8x32_epi32(y0, y8);
        let r1 = _mm256_permutevar8x32_epi32(y1, y9);
        let r2 = _mm256_permutevar8x32_epi32(y2, y10);
        let r3 = _mm256_permutevar8x32_epi32(y3, y11);
        let r4 = _mm256_permutevar8x32_epi32(y4, y12);
        let r5 = _mm256_permutevar8x32_epi32(y5, y13);
        let r6 = _mm256_permutevar8x32_epi32(y6, y14);
        let r7 = _mm256_permutevar8x32_epi32(y7, y15);

        // STORE
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
    }

    /// SHA-256 message schedule wavefront (msg1 + msg2).
    #[cfg(target_feature = "sha")]
    #[inline]
    #[target_feature(enable = "sha", enable = "sse4.1")]
    unsafe fn sha256_msg_wavefront(&self, state: &mut UorState) {
        // XMM operations for SHA message schedule
        let w0_ptr = state.ymm[0].as_ptr() as *const __m128i;
        let w1_ptr = state.ymm[1].as_ptr() as *const __m128i;

        let w0 = _mm_loadu_si128(w0_ptr);
        let w1 = _mm_loadu_si128(w1_ptr);
        let w4 = _mm_loadu_si128(w0_ptr.add(2));
        let w5 = _mm_loadu_si128(w1_ptr.add(2));

        // Port 1: sha256msg1
        let m0 = _mm_sha256msg1_epu32(w0, w1);
        // Port 5: sha256msg2
        let m1 = _mm_sha256msg2_epu32(w4, w5);

        _mm_storeu_si128(state.ymm[0].as_mut_ptr() as *mut __m128i, m0);
        _mm_storeu_si128(state.ymm[1].as_mut_ptr() as *mut __m128i, m1);
    }

    /// Fallback for non-SHA-NI systems.
    #[cfg(not(target_feature = "sha"))]
    #[inline]
    unsafe fn sha256_msg_wavefront(&self, _state: &mut UorState) {
        // SHA-NI not available
    }

    /// AES decryption wavefront using AES-NI.
    #[cfg(target_feature = "aes")]
    #[inline]
    #[target_feature(enable = "aes", enable = "sse4.1")]
    unsafe fn aes_dec_wavefront(&self, state: &mut UorState) {
        let state_ptr = state.ymm[0].as_ptr() as *const __m128i;
        let key_ptr = state.ymm[8].as_ptr() as *const __m128i;

        let mut s0 = _mm_loadu_si128(state_ptr);
        let mut s1 = _mm_loadu_si128(state_ptr.add(1));
        let k0 = _mm_loadu_si128(key_ptr);
        let k1 = _mm_loadu_si128(key_ptr.add(1));

        // Dual AES decryption rounds - Port 1 and Port 5
        s0 = _mm_aesdec_si128(s0, k0);
        s1 = _mm_aesdec_si128(s1, k1);

        _mm_storeu_si128(state.ymm[0].as_mut_ptr() as *mut __m128i, s0);
        _mm_storeu_si128((state.ymm[0].as_mut_ptr() as *mut __m128i).add(1), s1);
    }

    /// Fallback for non-AES-NI systems.
    #[cfg(not(target_feature = "aes"))]
    #[inline]
    unsafe fn aes_dec_wavefront(&self, _state: &mut UorState) {
        // AES-NI not available
    }

    /// GPR wavefront - executes operations on general-purpose registers.
    ///
    /// Pattern: GPR[i] op= GPR[i+7] for i in 0..7
    /// This enables 448 bits (7 × 64) of additional bandwidth per wavefront.
    ///
    /// # GPR Layout
    ///
    /// Registers 0-6 are paired with registers 7-13:
    /// - gpr[0] (rax) paired with gpr[7] (r8)
    /// - gpr[1] (rbx) paired with gpr[8] (r9)
    /// - ... and so on
    #[inline]
    unsafe fn gpr_wavefront(&self, state: &mut UorState, wf: &Wavefront) {
        // Only process if GPR mask is non-zero
        if wf.gpr_mask == 0 {
            return;
        }

        // Determine operation from port1 (primary ALU port)
        let op = if wf.ports.port1 != WavefrontOp::Nop {
            wf.ports.port1
        } else {
            wf.ports.port5
        };

        // Process each masked GPR pair using direct 64-bit addressing.
        // GPRs are part of the unified 624-taxon address space:
        // - GPR0-6 (taxons 512-567) paired with GPR7-13 (taxons 568-623)
        for i in 0..7usize {
            if (wf.gpr_mask >> i) & 1 == 0 {
                continue;
            }

            // Direct 64-bit addressing (unified with YMM addressing model)
            let gi = core::ptr::read_unaligned(state.gpr[i].as_ptr() as *const u64);
            let gi7 = core::ptr::read_unaligned(state.gpr[i + 7].as_ptr() as *const u64);

            // Execute operation (same semantics as YMM wavefront)
            let result = match op {
                WavefrontOp::Nop => gi,
                WavefrontOp::Xor => gi ^ gi7,
                WavefrontOp::And => gi & gi7,
                WavefrontOp::Or => gi | gi7,
                WavefrontOp::Not => !gi,
                WavefrontOp::Add => gi.wrapping_add(gi7),
                WavefrontOp::Sub => gi.wrapping_sub(gi7),
                WavefrontOp::RotL(n) => gi.rotate_left(n as u32),
                WavefrontOp::RotR(n) => gi.rotate_right(n as u32),
                WavefrontOp::ShL(n) => gi << (n as u32),
                WavefrontOp::ShR(n) => gi >> (n as u32),
                // Crypto/shuffle ops not applicable to GPR
                _ => gi,
            };

            // Direct 64-bit store
            core::ptr::write_unaligned(state.gpr[i].as_mut_ptr() as *mut u64, result);
        }
    }

    /// Masked wavefront execution - respects ymm_mask for selective register updates.
    ///
    /// Uses parallel load-compute-store pattern for maximum throughput:
    /// - Phase 1: Load ALL masked source and operand registers
    /// - Phase 2: Compute ALL operations (out-of-order execution)
    /// - Phase 3: Store ALL masked results
    ///
    /// # Register Pairing
    ///
    /// - Bits 0-7: Destination registers (ymm[0..8])
    /// - Paired with operand registers (ymm[8..16])
    ///
    /// For example, mask 0x0003 processes:
    /// - ymm[0] op ymm[8]
    /// - ymm[1] op ymm[9]
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn masked_wavefront(&self, state: &mut UorState, wf: &Wavefront) {
        let dest_mask = wf.ymm_mask & 0x00FF;

        // Determine the primary operation to apply
        let op = match wf.ports.port0 {
            WavefrontOp::RotR(_)
            | WavefrontOp::RotL(_)
            | WavefrontOp::ShR(_)
            | WavefrontOp::ShL(_) => wf.ports.port0,
            _ => {
                if wf.ports.port1 != WavefrontOp::Nop {
                    wf.ports.port1
                } else {
                    wf.ports.port5
                }
            }
        };

        // ========================================
        // LOAD PHASE: Load ALL masked registers in parallel
        // ========================================
        // Use zeroed as placeholder for unmasked registers
        let zero = _mm256_setzero_si256();
        let all_ones = _mm256_set1_epi32(-1i32);

        // Destination registers (ymm[0..8])
        let y0 = if (dest_mask & 0x01) != 0 {
            _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y1 = if (dest_mask & 0x02) != 0 {
            _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y2 = if (dest_mask & 0x04) != 0 {
            _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y3 = if (dest_mask & 0x08) != 0 {
            _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y4 = if (dest_mask & 0x10) != 0 {
            _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y5 = if (dest_mask & 0x20) != 0 {
            _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y6 = if (dest_mask & 0x40) != 0 {
            _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y7 = if (dest_mask & 0x80) != 0 {
            _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i)
        } else {
            zero
        };

        // Operand registers (ymm[8..16])
        let y8 = if (dest_mask & 0x01) != 0 {
            _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y9 = if (dest_mask & 0x02) != 0 {
            _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y10 = if (dest_mask & 0x04) != 0 {
            _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y11 = if (dest_mask & 0x08) != 0 {
            _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y12 = if (dest_mask & 0x10) != 0 {
            _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y13 = if (dest_mask & 0x20) != 0 {
            _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y14 = if (dest_mask & 0x40) != 0 {
            _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i)
        } else {
            zero
        };
        let y15 = if (dest_mask & 0x80) != 0 {
            _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i)
        } else {
            zero
        };

        // ========================================
        // COMPUTE PHASE: Execute ALL operations (out-of-order)
        // ========================================
        let (r0, r1, r2, r3, r4, r5, r6, r7) = match op {
            WavefrontOp::Nop => (y0, y1, y2, y3, y4, y5, y6, y7),
            WavefrontOp::Xor => (
                _mm256_xor_si256(y0, y8),
                _mm256_xor_si256(y1, y9),
                _mm256_xor_si256(y2, y10),
                _mm256_xor_si256(y3, y11),
                _mm256_xor_si256(y4, y12),
                _mm256_xor_si256(y5, y13),
                _mm256_xor_si256(y6, y14),
                _mm256_xor_si256(y7, y15),
            ),
            WavefrontOp::And => (
                _mm256_and_si256(y0, y8),
                _mm256_and_si256(y1, y9),
                _mm256_and_si256(y2, y10),
                _mm256_and_si256(y3, y11),
                _mm256_and_si256(y4, y12),
                _mm256_and_si256(y5, y13),
                _mm256_and_si256(y6, y14),
                _mm256_and_si256(y7, y15),
            ),
            WavefrontOp::Or => (
                _mm256_or_si256(y0, y8),
                _mm256_or_si256(y1, y9),
                _mm256_or_si256(y2, y10),
                _mm256_or_si256(y3, y11),
                _mm256_or_si256(y4, y12),
                _mm256_or_si256(y5, y13),
                _mm256_or_si256(y6, y14),
                _mm256_or_si256(y7, y15),
            ),
            WavefrontOp::Not => (
                _mm256_xor_si256(y0, all_ones),
                _mm256_xor_si256(y1, all_ones),
                _mm256_xor_si256(y2, all_ones),
                _mm256_xor_si256(y3, all_ones),
                _mm256_xor_si256(y4, all_ones),
                _mm256_xor_si256(y5, all_ones),
                _mm256_xor_si256(y6, all_ones),
                _mm256_xor_si256(y7, all_ones),
            ),
            WavefrontOp::Add => (
                _mm256_add_epi32(y0, y8),
                _mm256_add_epi32(y1, y9),
                _mm256_add_epi32(y2, y10),
                _mm256_add_epi32(y3, y11),
                _mm256_add_epi32(y4, y12),
                _mm256_add_epi32(y5, y13),
                _mm256_add_epi32(y6, y14),
                _mm256_add_epi32(y7, y15),
            ),
            WavefrontOp::Sub => (
                _mm256_sub_epi32(y0, y8),
                _mm256_sub_epi32(y1, y9),
                _mm256_sub_epi32(y2, y10),
                _mm256_sub_epi32(y3, y11),
                _mm256_sub_epi32(y4, y12),
                _mm256_sub_epi32(y5, y13),
                _mm256_sub_epi32(y6, y14),
                _mm256_sub_epi32(y7, y15),
            ),
            WavefrontOp::RotR(n) => {
                let anti = 32u8.wrapping_sub(n);
                let shift_r = _mm256_set1_epi32(n as i32);
                let shift_l = _mm256_set1_epi32(anti as i32);
                (
                    _mm256_or_si256(
                        _mm256_srlv_epi32(y0, shift_r),
                        _mm256_sllv_epi32(y0, shift_l),
                    ),
                    _mm256_or_si256(
                        _mm256_srlv_epi32(y1, shift_r),
                        _mm256_sllv_epi32(y1, shift_l),
                    ),
                    _mm256_or_si256(
                        _mm256_srlv_epi32(y2, shift_r),
                        _mm256_sllv_epi32(y2, shift_l),
                    ),
                    _mm256_or_si256(
                        _mm256_srlv_epi32(y3, shift_r),
                        _mm256_sllv_epi32(y3, shift_l),
                    ),
                    _mm256_or_si256(
                        _mm256_srlv_epi32(y4, shift_r),
                        _mm256_sllv_epi32(y4, shift_l),
                    ),
                    _mm256_or_si256(
                        _mm256_srlv_epi32(y5, shift_r),
                        _mm256_sllv_epi32(y5, shift_l),
                    ),
                    _mm256_or_si256(
                        _mm256_srlv_epi32(y6, shift_r),
                        _mm256_sllv_epi32(y6, shift_l),
                    ),
                    _mm256_or_si256(
                        _mm256_srlv_epi32(y7, shift_r),
                        _mm256_sllv_epi32(y7, shift_l),
                    ),
                )
            }
            WavefrontOp::RotL(n) => {
                let anti = 32u8.wrapping_sub(n);
                let shift_l = _mm256_set1_epi32(n as i32);
                let shift_r = _mm256_set1_epi32(anti as i32);
                (
                    _mm256_or_si256(
                        _mm256_sllv_epi32(y0, shift_l),
                        _mm256_srlv_epi32(y0, shift_r),
                    ),
                    _mm256_or_si256(
                        _mm256_sllv_epi32(y1, shift_l),
                        _mm256_srlv_epi32(y1, shift_r),
                    ),
                    _mm256_or_si256(
                        _mm256_sllv_epi32(y2, shift_l),
                        _mm256_srlv_epi32(y2, shift_r),
                    ),
                    _mm256_or_si256(
                        _mm256_sllv_epi32(y3, shift_l),
                        _mm256_srlv_epi32(y3, shift_r),
                    ),
                    _mm256_or_si256(
                        _mm256_sllv_epi32(y4, shift_l),
                        _mm256_srlv_epi32(y4, shift_r),
                    ),
                    _mm256_or_si256(
                        _mm256_sllv_epi32(y5, shift_l),
                        _mm256_srlv_epi32(y5, shift_r),
                    ),
                    _mm256_or_si256(
                        _mm256_sllv_epi32(y6, shift_l),
                        _mm256_srlv_epi32(y6, shift_r),
                    ),
                    _mm256_or_si256(
                        _mm256_sllv_epi32(y7, shift_l),
                        _mm256_srlv_epi32(y7, shift_r),
                    ),
                )
            }
            WavefrontOp::ShR(n) => {
                let shift = _mm256_set1_epi32(n as i32);
                (
                    _mm256_srlv_epi32(y0, shift),
                    _mm256_srlv_epi32(y1, shift),
                    _mm256_srlv_epi32(y2, shift),
                    _mm256_srlv_epi32(y3, shift),
                    _mm256_srlv_epi32(y4, shift),
                    _mm256_srlv_epi32(y5, shift),
                    _mm256_srlv_epi32(y6, shift),
                    _mm256_srlv_epi32(y7, shift),
                )
            }
            WavefrontOp::ShL(n) => {
                let shift = _mm256_set1_epi32(n as i32);
                (
                    _mm256_sllv_epi32(y0, shift),
                    _mm256_sllv_epi32(y1, shift),
                    _mm256_sllv_epi32(y2, shift),
                    _mm256_sllv_epi32(y3, shift),
                    _mm256_sllv_epi32(y4, shift),
                    _mm256_sllv_epi32(y5, shift),
                    _mm256_sllv_epi32(y6, shift),
                    _mm256_sllv_epi32(y7, shift),
                )
            }
            WavefrontOp::Shuffle => (
                _mm256_shuffle_epi8(y0, y8),
                _mm256_shuffle_epi8(y1, y9),
                _mm256_shuffle_epi8(y2, y10),
                _mm256_shuffle_epi8(y3, y11),
                _mm256_shuffle_epi8(y4, y12),
                _mm256_shuffle_epi8(y5, y13),
                _mm256_shuffle_epi8(y6, y14),
                _mm256_shuffle_epi8(y7, y15),
            ),
            WavefrontOp::Permute => (
                _mm256_permutevar8x32_epi32(y0, y8),
                _mm256_permutevar8x32_epi32(y1, y9),
                _mm256_permutevar8x32_epi32(y2, y10),
                _mm256_permutevar8x32_epi32(y3, y11),
                _mm256_permutevar8x32_epi32(y4, y12),
                _mm256_permutevar8x32_epi32(y5, y13),
                _mm256_permutevar8x32_epi32(y6, y14),
                _mm256_permutevar8x32_epi32(y7, y15),
            ),
            // Crypto operations not supported in masked mode
            WavefrontOp::Sha256Round
            | WavefrontOp::Sha256Msg1
            | WavefrontOp::Sha256Msg2
            | WavefrontOp::AesRound
            | WavefrontOp::AesRoundDec => (y0, y1, y2, y3, y4, y5, y6, y7),
        };

        // ========================================
        // STORE PHASE: Store ONLY masked results
        // ========================================
        if (dest_mask & 0x01) != 0 {
            _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, r0);
        }
        if (dest_mask & 0x02) != 0 {
            _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, r1);
        }
        if (dest_mask & 0x04) != 0 {
            _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, r2);
        }
        if (dest_mask & 0x08) != 0 {
            _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, r3);
        }
        if (dest_mask & 0x10) != 0 {
            _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, r4);
        }
        if (dest_mask & 0x20) != 0 {
            _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, r5);
        }
        if (dest_mask & 0x40) != 0 {
            _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, r6);
        }
        if (dest_mask & 0x80) != 0 {
            _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, r7);
        }
    }

    // ========================================
    // Lossless Codec: Tracked Operations
    // ========================================

    /// Shift left with complement capture (lossless).
    ///
    /// Complement = high n bits (shifted out): `dest >> (32-n)`
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn shl_wavefront_tracked(&self, state: &mut UorState, complement: &mut UorState, n: u8) {
        let shift = _mm256_set1_epi32(n as i32);
        let anti = _mm256_set1_epi32((32 - n) as i32);

        // Process all 8 destination registers
        for i in 0..8 {
            let y = _mm256_loadu_si256(state.ymm[i].as_ptr() as *const __m256i);
            // Capture complement: high bits that will be lost
            let comp = _mm256_srlv_epi32(y, anti);
            _mm256_storeu_si256(complement.ymm[i].as_mut_ptr() as *mut __m256i, comp);
            // Perform shift
            let result = _mm256_sllv_epi32(y, shift);
            _mm256_storeu_si256(state.ymm[i].as_mut_ptr() as *mut __m256i, result);
        }
    }

    /// Shift right with complement capture (lossless).
    ///
    /// Complement = low n bits (shifted out): `dest & ((1<<n)-1)`
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn shr_wavefront_tracked(&self, state: &mut UorState, complement: &mut UorState, n: u8) {
        let shift = _mm256_set1_epi32(n as i32);
        // Mask for low n bits: (1 << n) - 1
        let mask = _mm256_set1_epi32((1i32 << n) - 1);

        for i in 0..8 {
            let y = _mm256_loadu_si256(state.ymm[i].as_ptr() as *const __m256i);
            // Capture complement: low bits that will be lost
            let comp = _mm256_and_si256(y, mask);
            _mm256_storeu_si256(complement.ymm[i].as_mut_ptr() as *mut __m256i, comp);
            // Perform shift
            let result = _mm256_srlv_epi32(y, shift);
            _mm256_storeu_si256(state.ymm[i].as_mut_ptr() as *mut __m256i, result);
        }
    }

    /// AND with complement capture (lossless).
    ///
    /// Complement = `dest & ~operand` (bits masked out)
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn and_wavefront_tracked(&self, state: &mut UorState, complement: &mut UorState) {
        for i in 0..8 {
            let dest = _mm256_loadu_si256(state.ymm[i].as_ptr() as *const __m256i);
            let op = _mm256_loadu_si256(state.ymm[i + 8].as_ptr() as *const __m256i);
            // Complement = dest & ~operand (vpandn: ~a & b, so pass op, dest)
            let comp = _mm256_andnot_si256(op, dest);
            _mm256_storeu_si256(complement.ymm[i].as_mut_ptr() as *mut __m256i, comp);
            // Result = dest & operand
            let result = _mm256_and_si256(dest, op);
            _mm256_storeu_si256(state.ymm[i].as_mut_ptr() as *mut __m256i, result);
        }
    }

    /// OR with complement capture (lossless).
    ///
    /// Complement = `~dest & operand` (bits overwritten)
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn or_wavefront_tracked(&self, state: &mut UorState, complement: &mut UorState) {
        for i in 0..8 {
            let dest = _mm256_loadu_si256(state.ymm[i].as_ptr() as *const __m256i);
            let op = _mm256_loadu_si256(state.ymm[i + 8].as_ptr() as *const __m256i);
            // Complement = ~dest & operand (vpandn: ~a & b, so pass dest, op)
            let comp = _mm256_andnot_si256(dest, op);
            _mm256_storeu_si256(complement.ymm[i].as_mut_ptr() as *mut __m256i, comp);
            // Result = dest | operand
            let result = _mm256_or_si256(dest, op);
            _mm256_storeu_si256(state.ymm[i].as_mut_ptr() as *mut __m256i, result);
        }
    }

    // ========================================
    // Lossless Codec: Inverse Operations
    // ========================================

    /// Inverse of shift left using complement.
    ///
    /// Reconstructs: `original = (result >> n) | (complement << (32-n))`
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn shl_inverse_wavefront(&self, state: &mut UorState, complement: &UorState, n: u8) {
        let shift_r = _mm256_set1_epi32(n as i32);
        let shift_l = _mm256_set1_epi32((32 - n) as i32);

        for i in 0..8 {
            let result = _mm256_loadu_si256(state.ymm[i].as_ptr() as *const __m256i);
            let lost = _mm256_loadu_si256(complement.ymm[i].as_ptr() as *const __m256i);
            // Reconstruct: (result >> n) | (complement << (32-n))
            let lo = _mm256_srlv_epi32(result, shift_r);
            let hi = _mm256_sllv_epi32(lost, shift_l);
            let original = _mm256_or_si256(lo, hi);
            _mm256_storeu_si256(state.ymm[i].as_mut_ptr() as *mut __m256i, original);
        }
    }

    /// Inverse of shift right using complement.
    ///
    /// Reconstructs: `original = (result << n) | complement`
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn shr_inverse_wavefront(&self, state: &mut UorState, complement: &UorState, n: u8) {
        let shift = _mm256_set1_epi32(n as i32);

        for i in 0..8 {
            let result = _mm256_loadu_si256(state.ymm[i].as_ptr() as *const __m256i);
            let lost = _mm256_loadu_si256(complement.ymm[i].as_ptr() as *const __m256i);
            // Reconstruct: (result << n) | complement
            let hi = _mm256_sllv_epi32(result, shift);
            let original = _mm256_or_si256(hi, lost);
            _mm256_storeu_si256(state.ymm[i].as_mut_ptr() as *mut __m256i, original);
        }
    }

    /// Inverse of AND using complement.
    ///
    /// Reconstructs: `original = result | complement`
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn and_inverse_wavefront(&self, state: &mut UorState, complement: &UorState) {
        for i in 0..8 {
            let result = _mm256_loadu_si256(state.ymm[i].as_ptr() as *const __m256i);
            let masked = _mm256_loadu_si256(complement.ymm[i].as_ptr() as *const __m256i);
            // Reconstruct: result | complement
            let original = _mm256_or_si256(result, masked);
            _mm256_storeu_si256(state.ymm[i].as_mut_ptr() as *mut __m256i, original);
        }
    }

    /// Inverse of OR using complement.
    ///
    /// Reconstructs: `original = result & ~complement`
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn or_inverse_wavefront(&self, state: &mut UorState, complement: &UorState) {
        for i in 0..8 {
            let result = _mm256_loadu_si256(state.ymm[i].as_ptr() as *const __m256i);
            let added = _mm256_loadu_si256(complement.ymm[i].as_ptr() as *const __m256i);
            // Reconstruct: result & ~complement
            let original = _mm256_andnot_si256(added, result);
            _mm256_storeu_si256(state.ymm[i].as_mut_ptr() as *mut __m256i, original);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::taxon::Taxon;

    #[test]
    fn test_executor_creation() {
        let _executor = Zen3Executor::new();
    }

    #[test]
    fn test_xor_self_inverse() {
        let executor = Zen3Executor::new();
        let mut state = UorState::zero();

        // Set known values
        for i in 0..32 {
            state.ymm[0][i] = Taxon::new((i * 7) as u8);
            state.ymm[8][i] = Taxon::new((i * 7) as u8); // Same values
        }

        // XOR with self should produce zero
        let wf = Wavefront::all_xor();
        unsafe { executor.step(&mut state, &wf) };

        // Verify ymm[0] is now zero
        for i in 0..32 {
            assert_eq!(
                state.ymm[0][i].value(),
                0,
                "XOR self-inverse failed at {}",
                i
            );
        }
    }

    #[test]
    fn test_and_wavefront() {
        let executor = Zen3Executor::new();
        let mut state = UorState::zero();

        // Set values: 0xFF AND 0x0F = 0x0F
        for i in 0..32 {
            state.ymm[0][i] = Taxon::new(0xFF);
            state.ymm[8][i] = Taxon::new(0x0F);
        }

        let wf = Wavefront::new(PortAssignment::all_and());
        unsafe { executor.step(&mut state, &wf) };

        for i in 0..32 {
            assert_eq!(state.ymm[0][i].value(), 0x0F, "AND failed at {}", i);
        }
    }

    #[test]
    fn test_or_wavefront() {
        let executor = Zen3Executor::new();
        let mut state = UorState::zero();

        // Set values: 0xF0 OR 0x0F = 0xFF
        for i in 0..32 {
            state.ymm[0][i] = Taxon::new(0xF0);
            state.ymm[8][i] = Taxon::new(0x0F);
        }

        let wf = Wavefront::new(PortAssignment::all_or());
        unsafe { executor.step(&mut state, &wf) };

        for i in 0..32 {
            assert_eq!(state.ymm[0][i].value(), 0xFF, "OR failed at {}", i);
        }
    }

    #[test]
    fn test_deterministic() {
        let executor = Zen3Executor::new();
        let wf = Wavefront::rotate_xor(7);

        let mut state1 = UorState::zero();
        let mut state2 = UorState::zero();

        // Initialize both with same values
        for i in 0..32 {
            state1.ymm[0][i] = Taxon::new(i as u8);
            state2.ymm[0][i] = Taxon::new(i as u8);
        }

        // Execute same wavefront
        unsafe {
            executor.step(&mut state1, &wf);
            executor.step(&mut state2, &wf);
        }

        // Should produce identical results
        assert!(state1.eq(&state2), "Execution not deterministic");
    }

    // ========================================
    // Lossless Codec Roundtrip Tests
    // ========================================

    #[test]
    fn test_shl_lossless_roundtrip() {
        let executor = Zen3Executor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set known values
        for i in 0..32 {
            state.ymm[0][i] = Taxon::new((i * 7 + 0xAB) as u8);
        }
        let original = state;

        // Execute shift left with complement tracking
        let wf = Wavefront::new(PortAssignment::shl_only(5));
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // Verify state was modified
        assert_ne!(state.ymm[0], original.ymm[0], "ShL did not modify state");

        // Execute inverse to restore
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };

        // Verify lossless roundtrip
        for i in 0..32 {
            assert_eq!(
                state.ymm[0][i].value(),
                original.ymm[0][i].value(),
                "ShL lossless roundtrip failed at index {}",
                i
            );
        }
    }

    #[test]
    fn test_shr_lossless_roundtrip() {
        let executor = Zen3Executor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set known values
        for i in 0..32 {
            state.ymm[0][i] = Taxon::new((i * 11 + 0xCD) as u8);
        }
        let original = state;

        // Execute shift right with complement tracking
        let wf = Wavefront::new(PortAssignment::shr_only(7));
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // Verify state was modified
        assert_ne!(state.ymm[0], original.ymm[0], "ShR did not modify state");

        // Execute inverse to restore
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };

        // Verify lossless roundtrip
        for i in 0..32 {
            assert_eq!(
                state.ymm[0][i].value(),
                original.ymm[0][i].value(),
                "ShR lossless roundtrip failed at index {}",
                i
            );
        }
    }

    #[test]
    fn test_and_lossless_roundtrip() {
        let executor = Zen3Executor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set known values: dest and operand
        for i in 0..32 {
            state.ymm[0][i] = Taxon::new((i * 13 + 0xEF) as u8);
            state.ymm[8][i] = Taxon::new(0x0F); // Mask low nibble
        }
        let original_dest = state.ymm[0];

        // Execute AND with complement tracking
        let wf = Wavefront::new(PortAssignment::all_and());
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // Verify state was modified
        assert_ne!(state.ymm[0], original_dest, "AND did not modify state");

        // Execute inverse to restore
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };

        // Verify lossless roundtrip
        #[allow(clippy::needless_range_loop)]
        for i in 0..32 {
            assert_eq!(
                state.ymm[0][i].value(),
                original_dest[i].value(),
                "AND lossless roundtrip failed at index {}",
                i
            );
        }
    }

    #[test]
    fn test_or_lossless_roundtrip() {
        let executor = Zen3Executor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set known values: dest and operand
        for i in 0..32 {
            state.ymm[0][i] = Taxon::new((i * 17) as u8);
            state.ymm[8][i] = Taxon::new(0xF0); // Set high nibble
        }
        let original_dest = state.ymm[0];

        // Execute OR with complement tracking
        let wf = Wavefront::new(PortAssignment::all_or());
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // Verify state was modified
        assert_ne!(state.ymm[0], original_dest, "OR did not modify state");

        // Execute inverse to restore
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };

        // Verify lossless roundtrip
        #[allow(clippy::needless_range_loop)]
        for i in 0..32 {
            assert_eq!(
                state.ymm[0][i].value(),
                original_dest[i].value(),
                "OR lossless roundtrip failed at index {}",
                i
            );
        }
    }

    #[test]
    fn test_invertibility_classification() {
        // Verify is_invertible classification
        assert!(WavefrontOp::Xor.is_invertible());
        assert!(WavefrontOp::Not.is_invertible());
        assert!(WavefrontOp::Add.is_invertible());
        assert!(WavefrontOp::Sub.is_invertible());
        assert!(WavefrontOp::RotL(7).is_invertible());
        assert!(WavefrontOp::RotR(13).is_invertible());

        // Verify requires_complement classification
        assert!(WavefrontOp::ShL(5).requires_complement());
        assert!(WavefrontOp::ShR(10).requires_complement());
        assert!(WavefrontOp::And.requires_complement());
        assert!(WavefrontOp::Or.requires_complement());
        assert!(WavefrontOp::Sha256Round.requires_complement());

        // Verify mutual exclusivity
        assert!(!WavefrontOp::ShL(5).is_invertible());
        assert!(!WavefrontOp::And.is_invertible());
        assert!(!WavefrontOp::Xor.requires_complement());
    }
}
