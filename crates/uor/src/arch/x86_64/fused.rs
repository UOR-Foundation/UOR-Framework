//! Register-resident fused wavefront execution.
//!
//! This module implements `UorStepFused` for the `Zen3Executor`, enabling
//! multi-wavefront execution with state in CPU registers throughout.
//!
//! # Execution Model
//!
//! Fused execution follows the correct UOR model:
//! 1. Load state into YMM/GPR registers ONCE at program start
//! 2. Execute all wavefronts with pure intrinsics (NO memory access)
//! 3. Store state ONCE at program end
//!
//! # Performance
//!
//! For 64 XOR wavefronts:
//! - Load: 16 cycles (16 YMM registers)
//! - Compute: 64 cycles (1 cycle per wavefront, pipelined)
//! - Store: 8 cycles (8 destination registers)
//! - Total: ~88 cycles = ~1.4 cycles/wavefront
//!
//! With larger programs, approaches 1 cycle/wavefront.

use crate::isa::{UorStep, UorStepFused, Wavefront, WavefrontOp};
use crate::state::UorState;

use super::Zen3Executor;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// Check if all wavefronts in a program are identical.
#[inline]
fn is_homogeneous(program: &[Wavefront]) -> bool {
    if program.is_empty() {
        return true;
    }
    let first = &program[0];
    program.iter().all(|wf| {
        wf.ports.port0 == first.ports.port0
            && wf.ports.port1 == first.ports.port1
            && wf.ports.port5 == first.ports.port5
    })
}

impl UorStepFused for Zen3Executor {
    #[inline]
    unsafe fn run_fused(&self, state: &mut UorState, program: &[Wavefront]) {
        if program.is_empty() {
            return;
        }

        // For homogeneous programs, use tight loop (most common case)
        if is_homogeneous(program) {
            self.step_n_fused(state, &program[0], program.len());
            return;
        }

        // Heterogeneous programs: load once, dispatch each, store once
        self.run_fused_heterogeneous(state, program);
    }

    #[inline]
    unsafe fn step_n_fused(&self, state: &mut UorState, wavefront: &Wavefront, n: usize) {
        if n == 0 {
            return;
        }

        // Dispatch to specialized fused implementation based on operation type
        match (wavefront.ports.port0, wavefront.ports.port1, wavefront.ports.port5) {
            // XOR (most common)
            (WavefrontOp::Nop, WavefrontOp::Xor, WavefrontOp::Xor) => {
                self.run_fused_xor(state, n);
            }

            // AND
            (WavefrontOp::Nop, WavefrontOp::And, WavefrontOp::And) => {
                self.run_fused_and(state, n);
            }

            // OR
            (WavefrontOp::Nop, WavefrontOp::Or, WavefrontOp::Or) => {
                self.run_fused_or(state, n);
            }

            // ADD
            (WavefrontOp::Nop, WavefrontOp::Add, WavefrontOp::Add) => {
                self.run_fused_add(state, n);
            }

            // Rotate right only
            (WavefrontOp::RotR(amt), WavefrontOp::Nop, WavefrontOp::Nop) => {
                self.run_fused_rotr(state, n, amt);
            }

            // Rotate left only
            (WavefrontOp::RotL(amt), WavefrontOp::Nop, WavefrontOp::Nop) => {
                self.run_fused_rotl(state, n, amt);
            }

            // Rotate right + XOR (SHA-256 Σ functions)
            (WavefrontOp::RotR(amt), WavefrontOp::Xor, WavefrontOp::Xor) => {
                self.run_fused_rotr_xor(state, n, amt);
            }

            // SHA-256 round
            (WavefrontOp::Sha256Round, _, _) => {
                self.run_fused_sha256_round(state, n);
            }

            // AES round
            (_, WavefrontOp::AesRound, WavefrontOp::AesRound) => {
                self.run_fused_aes_round(state, n);
            }

            // Fallback: execute individually (still correct, just not optimal)
            _ => {
                for _ in 0..n {
                    self.step(state, wavefront);
                }
            }
        }
    }
}

impl Zen3Executor {
    // ========================================================================
    // Fused XOR - The primary operation
    // ========================================================================

    /// Fused XOR execution - N iterations with single load/store.
    ///
    /// State remains in YMM registers throughout execution.
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn run_fused_xor(&self, state: &mut UorState, n: usize) {
        // LOAD ONCE - bring state into register variables
        let mut y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let mut y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let mut y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let mut y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let mut y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let mut y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let mut y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let mut y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        // EXECUTE N WAVEFRONTS - pure intrinsics, NO MEMORY ACCESS
        for _ in 0..n {
            y0 = _mm256_xor_si256(y0, y8);
            y1 = _mm256_xor_si256(y1, y9);
            y2 = _mm256_xor_si256(y2, y10);
            y3 = _mm256_xor_si256(y3, y11);
            y4 = _mm256_xor_si256(y4, y12);
            y5 = _mm256_xor_si256(y5, y13);
            y6 = _mm256_xor_si256(y6, y14);
            y7 = _mm256_xor_si256(y7, y15);
        }

        // STORE ONCE - write results back
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, y2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, y3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, y4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, y5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, y6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, y7);
    }

    // ========================================================================
    // Fused AND
    // ========================================================================

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn run_fused_and(&self, state: &mut UorState, n: usize) {
        let mut y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let mut y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let mut y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let mut y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let mut y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let mut y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let mut y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let mut y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        for _ in 0..n {
            y0 = _mm256_and_si256(y0, y8);
            y1 = _mm256_and_si256(y1, y9);
            y2 = _mm256_and_si256(y2, y10);
            y3 = _mm256_and_si256(y3, y11);
            y4 = _mm256_and_si256(y4, y12);
            y5 = _mm256_and_si256(y5, y13);
            y6 = _mm256_and_si256(y6, y14);
            y7 = _mm256_and_si256(y7, y15);
        }

        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, y2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, y3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, y4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, y5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, y6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, y7);
    }

    // ========================================================================
    // Fused OR
    // ========================================================================

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn run_fused_or(&self, state: &mut UorState, n: usize) {
        let mut y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let mut y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let mut y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let mut y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let mut y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let mut y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let mut y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let mut y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        for _ in 0..n {
            y0 = _mm256_or_si256(y0, y8);
            y1 = _mm256_or_si256(y1, y9);
            y2 = _mm256_or_si256(y2, y10);
            y3 = _mm256_or_si256(y3, y11);
            y4 = _mm256_or_si256(y4, y12);
            y5 = _mm256_or_si256(y5, y13);
            y6 = _mm256_or_si256(y6, y14);
            y7 = _mm256_or_si256(y7, y15);
        }

        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, y2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, y3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, y4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, y5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, y6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, y7);
    }

    // ========================================================================
    // Fused ADD
    // ========================================================================

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn run_fused_add(&self, state: &mut UorState, n: usize) {
        let mut y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let mut y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let mut y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let mut y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let mut y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let mut y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let mut y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let mut y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        for _ in 0..n {
            y0 = _mm256_add_epi32(y0, y8);
            y1 = _mm256_add_epi32(y1, y9);
            y2 = _mm256_add_epi32(y2, y10);
            y3 = _mm256_add_epi32(y3, y11);
            y4 = _mm256_add_epi32(y4, y12);
            y5 = _mm256_add_epi32(y5, y13);
            y6 = _mm256_add_epi32(y6, y14);
            y7 = _mm256_add_epi32(y7, y15);
        }

        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, y2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, y3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, y4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, y5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, y6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, y7);
    }

    // ========================================================================
    // Fused Rotate Right
    // ========================================================================

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn run_fused_rotr(&self, state: &mut UorState, n: usize, amt: u8) {
        let anti = 32u8.wrapping_sub(amt);
        let shift_r = _mm256_set1_epi32(amt as i32);
        let shift_l = _mm256_set1_epi32(anti as i32);

        let mut y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let mut y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let mut y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let mut y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let mut y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let mut y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let mut y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let mut y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);

        for _ in 0..n {
            // Rotate right = (x >> amt) | (x << (32-amt))
            let lo0 = _mm256_srlv_epi32(y0, shift_r);
            let hi0 = _mm256_sllv_epi32(y0, shift_l);
            y0 = _mm256_or_si256(lo0, hi0);

            let lo1 = _mm256_srlv_epi32(y1, shift_r);
            let hi1 = _mm256_sllv_epi32(y1, shift_l);
            y1 = _mm256_or_si256(lo1, hi1);

            let lo2 = _mm256_srlv_epi32(y2, shift_r);
            let hi2 = _mm256_sllv_epi32(y2, shift_l);
            y2 = _mm256_or_si256(lo2, hi2);

            let lo3 = _mm256_srlv_epi32(y3, shift_r);
            let hi3 = _mm256_sllv_epi32(y3, shift_l);
            y3 = _mm256_or_si256(lo3, hi3);

            let lo4 = _mm256_srlv_epi32(y4, shift_r);
            let hi4 = _mm256_sllv_epi32(y4, shift_l);
            y4 = _mm256_or_si256(lo4, hi4);

            let lo5 = _mm256_srlv_epi32(y5, shift_r);
            let hi5 = _mm256_sllv_epi32(y5, shift_l);
            y5 = _mm256_or_si256(lo5, hi5);

            let lo6 = _mm256_srlv_epi32(y6, shift_r);
            let hi6 = _mm256_sllv_epi32(y6, shift_l);
            y6 = _mm256_or_si256(lo6, hi6);

            let lo7 = _mm256_srlv_epi32(y7, shift_r);
            let hi7 = _mm256_sllv_epi32(y7, shift_l);
            y7 = _mm256_or_si256(lo7, hi7);
        }

        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, y2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, y3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, y4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, y5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, y6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, y7);
    }

    // ========================================================================
    // Fused Rotate Left
    // ========================================================================

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn run_fused_rotl(&self, state: &mut UorState, n: usize, amt: u8) {
        let anti = 32u8.wrapping_sub(amt);
        let shift_l = _mm256_set1_epi32(amt as i32);
        let shift_r = _mm256_set1_epi32(anti as i32);

        let mut y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let mut y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let mut y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let mut y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let mut y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let mut y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let mut y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let mut y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);

        for _ in 0..n {
            // Rotate left = (x << amt) | (x >> (32-amt))
            let hi0 = _mm256_sllv_epi32(y0, shift_l);
            let lo0 = _mm256_srlv_epi32(y0, shift_r);
            y0 = _mm256_or_si256(hi0, lo0);

            let hi1 = _mm256_sllv_epi32(y1, shift_l);
            let lo1 = _mm256_srlv_epi32(y1, shift_r);
            y1 = _mm256_or_si256(hi1, lo1);

            let hi2 = _mm256_sllv_epi32(y2, shift_l);
            let lo2 = _mm256_srlv_epi32(y2, shift_r);
            y2 = _mm256_or_si256(hi2, lo2);

            let hi3 = _mm256_sllv_epi32(y3, shift_l);
            let lo3 = _mm256_srlv_epi32(y3, shift_r);
            y3 = _mm256_or_si256(hi3, lo3);

            let hi4 = _mm256_sllv_epi32(y4, shift_l);
            let lo4 = _mm256_srlv_epi32(y4, shift_r);
            y4 = _mm256_or_si256(hi4, lo4);

            let hi5 = _mm256_sllv_epi32(y5, shift_l);
            let lo5 = _mm256_srlv_epi32(y5, shift_r);
            y5 = _mm256_or_si256(hi5, lo5);

            let hi6 = _mm256_sllv_epi32(y6, shift_l);
            let lo6 = _mm256_srlv_epi32(y6, shift_r);
            y6 = _mm256_or_si256(hi6, lo6);

            let hi7 = _mm256_sllv_epi32(y7, shift_l);
            let lo7 = _mm256_srlv_epi32(y7, shift_r);
            y7 = _mm256_or_si256(hi7, lo7);
        }

        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, y2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, y3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, y4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, y5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, y6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, y7);
    }

    // ========================================================================
    // Fused Rotate Right + XOR (SHA-256 Σ functions)
    // ========================================================================

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn run_fused_rotr_xor(&self, state: &mut UorState, n: usize, amt: u8) {
        let anti = 32u8.wrapping_sub(amt);
        let shift_r = _mm256_set1_epi32(amt as i32);
        let shift_l = _mm256_set1_epi32(anti as i32);

        let mut y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let mut y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);

        for _ in 0..n {
            // Port 0: Rotate right y0
            let lo0 = _mm256_srlv_epi32(y0, shift_r);
            let hi0 = _mm256_sllv_epi32(y0, shift_l);
            y0 = _mm256_or_si256(lo0, hi0);

            // Ports 1/5: XOR y0, y1 with y8, y9
            y0 = _mm256_xor_si256(y0, y9);
            y1 = _mm256_xor_si256(y1, y8);
        }

        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
    }

    // ========================================================================
    // Fused SHA-256 Round (placeholder - uses hardware)
    // ========================================================================

    #[inline]
    #[target_feature(enable = "sha", enable = "sse4.1")]
    unsafe fn run_fused_sha256_round(&self, state: &mut UorState, n: usize) {
        // SHA-256 rounds use 128-bit XMM registers via SHA-NI
        // For fused execution, we still need to interact with hardware
        // This is a placeholder - actual implementation would use sha256rnds2
        for _ in 0..n {
            self.step(state, &Wavefront::sha256_round());
        }
    }

    // ========================================================================
    // Fused AES Round (placeholder - uses hardware)
    // ========================================================================

    #[inline]
    #[target_feature(enable = "aes")]
    unsafe fn run_fused_aes_round(&self, state: &mut UorState, n: usize) {
        // AES rounds use dedicated hardware via AES-NI
        // For fused execution, we still need to interact with hardware
        for _ in 0..n {
            self.step(state, &Wavefront::aes_round());
        }
    }

    // ========================================================================
    // Heterogeneous Program Execution
    // ========================================================================

    /// Execute heterogeneous program with single load/store.
    ///
    /// For programs with different wavefront types, we load once,
    /// dispatch each wavefront in sequence, then store once.
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn run_fused_heterogeneous(&self, state: &mut UorState, program: &[Wavefront]) {
        // Load all YMM registers once
        let mut y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
        let mut y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
        let mut y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
        let mut y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
        let mut y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
        let mut y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
        let mut y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
        let mut y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
        let mut y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
        let mut y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
        let mut y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
        let mut y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
        let mut y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
        let mut y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
        let mut y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
        let mut y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);

        // Execute each wavefront with registers
        for wf in program {
            match (wf.ports.port0, wf.ports.port1, wf.ports.port5) {
                (WavefrontOp::Nop, WavefrontOp::Xor, WavefrontOp::Xor) => {
                    y0 = _mm256_xor_si256(y0, y8);
                    y1 = _mm256_xor_si256(y1, y9);
                    y2 = _mm256_xor_si256(y2, y10);
                    y3 = _mm256_xor_si256(y3, y11);
                    y4 = _mm256_xor_si256(y4, y12);
                    y5 = _mm256_xor_si256(y5, y13);
                    y6 = _mm256_xor_si256(y6, y14);
                    y7 = _mm256_xor_si256(y7, y15);
                }
                (WavefrontOp::Nop, WavefrontOp::And, WavefrontOp::And) => {
                    y0 = _mm256_and_si256(y0, y8);
                    y1 = _mm256_and_si256(y1, y9);
                    y2 = _mm256_and_si256(y2, y10);
                    y3 = _mm256_and_si256(y3, y11);
                    y4 = _mm256_and_si256(y4, y12);
                    y5 = _mm256_and_si256(y5, y13);
                    y6 = _mm256_and_si256(y6, y14);
                    y7 = _mm256_and_si256(y7, y15);
                }
                (WavefrontOp::Nop, WavefrontOp::Or, WavefrontOp::Or) => {
                    y0 = _mm256_or_si256(y0, y8);
                    y1 = _mm256_or_si256(y1, y9);
                    y2 = _mm256_or_si256(y2, y10);
                    y3 = _mm256_or_si256(y3, y11);
                    y4 = _mm256_or_si256(y4, y12);
                    y5 = _mm256_or_si256(y5, y13);
                    y6 = _mm256_or_si256(y6, y14);
                    y7 = _mm256_or_si256(y7, y15);
                }
                (WavefrontOp::Nop, WavefrontOp::Add, WavefrontOp::Add) => {
                    y0 = _mm256_add_epi32(y0, y8);
                    y1 = _mm256_add_epi32(y1, y9);
                    y2 = _mm256_add_epi32(y2, y10);
                    y3 = _mm256_add_epi32(y3, y11);
                    y4 = _mm256_add_epi32(y4, y12);
                    y5 = _mm256_add_epi32(y5, y13);
                    y6 = _mm256_add_epi32(y6, y14);
                    y7 = _mm256_add_epi32(y7, y15);
                }
                _ => {
                    // For unsupported operations, store/reload through state
                    // This is not ideal but ensures correctness
                    _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
                    _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
                    _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, y2);
                    _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, y3);
                    _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, y4);
                    _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, y5);
                    _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, y6);
                    _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, y7);
                    _mm256_storeu_si256(state.ymm[8].as_mut_ptr() as *mut __m256i, y8);
                    _mm256_storeu_si256(state.ymm[9].as_mut_ptr() as *mut __m256i, y9);
                    _mm256_storeu_si256(state.ymm[10].as_mut_ptr() as *mut __m256i, y10);
                    _mm256_storeu_si256(state.ymm[11].as_mut_ptr() as *mut __m256i, y11);
                    _mm256_storeu_si256(state.ymm[12].as_mut_ptr() as *mut __m256i, y12);
                    _mm256_storeu_si256(state.ymm[13].as_mut_ptr() as *mut __m256i, y13);
                    _mm256_storeu_si256(state.ymm[14].as_mut_ptr() as *mut __m256i, y14);
                    _mm256_storeu_si256(state.ymm[15].as_mut_ptr() as *mut __m256i, y15);

                    self.step(state, wf);

                    y0 = _mm256_loadu_si256(state.ymm[0].as_ptr() as *const __m256i);
                    y1 = _mm256_loadu_si256(state.ymm[1].as_ptr() as *const __m256i);
                    y2 = _mm256_loadu_si256(state.ymm[2].as_ptr() as *const __m256i);
                    y3 = _mm256_loadu_si256(state.ymm[3].as_ptr() as *const __m256i);
                    y4 = _mm256_loadu_si256(state.ymm[4].as_ptr() as *const __m256i);
                    y5 = _mm256_loadu_si256(state.ymm[5].as_ptr() as *const __m256i);
                    y6 = _mm256_loadu_si256(state.ymm[6].as_ptr() as *const __m256i);
                    y7 = _mm256_loadu_si256(state.ymm[7].as_ptr() as *const __m256i);
                    y8 = _mm256_loadu_si256(state.ymm[8].as_ptr() as *const __m256i);
                    y9 = _mm256_loadu_si256(state.ymm[9].as_ptr() as *const __m256i);
                    y10 = _mm256_loadu_si256(state.ymm[10].as_ptr() as *const __m256i);
                    y11 = _mm256_loadu_si256(state.ymm[11].as_ptr() as *const __m256i);
                    y12 = _mm256_loadu_si256(state.ymm[12].as_ptr() as *const __m256i);
                    y13 = _mm256_loadu_si256(state.ymm[13].as_ptr() as *const __m256i);
                    y14 = _mm256_loadu_si256(state.ymm[14].as_ptr() as *const __m256i);
                    y15 = _mm256_loadu_si256(state.ymm[15].as_ptr() as *const __m256i);
                }
            }
        }

        // Store all YMM registers once
        _mm256_storeu_si256(state.ymm[0].as_mut_ptr() as *mut __m256i, y0);
        _mm256_storeu_si256(state.ymm[1].as_mut_ptr() as *mut __m256i, y1);
        _mm256_storeu_si256(state.ymm[2].as_mut_ptr() as *mut __m256i, y2);
        _mm256_storeu_si256(state.ymm[3].as_mut_ptr() as *mut __m256i, y3);
        _mm256_storeu_si256(state.ymm[4].as_mut_ptr() as *mut __m256i, y4);
        _mm256_storeu_si256(state.ymm[5].as_mut_ptr() as *mut __m256i, y5);
        _mm256_storeu_si256(state.ymm[6].as_mut_ptr() as *mut __m256i, y6);
        _mm256_storeu_si256(state.ymm[7].as_mut_ptr() as *mut __m256i, y7);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::taxon::Taxon;

    #[test]
    fn test_fused_xor_correctness() {
        let executor = Zen3Executor::new();
        let mut state1 = UorState::zero();
        let mut state2 = UorState::zero();

        // Initialize with pattern
        for i in 0..32 {
            state1.ymm[0][i] = Taxon::new((i * 7) as u8);
            state1.ymm[8][i] = Taxon::new((i * 11) as u8);
            state2.ymm[0][i] = Taxon::new((i * 7) as u8);
            state2.ymm[8][i] = Taxon::new((i * 11) as u8);
        }

        let wf = Wavefront::all_xor();

        // Execute 10 times unfused
        for _ in 0..10 {
            unsafe { executor.step(&mut state1, &wf) };
        }

        // Execute 10 times fused
        unsafe { executor.step_n_fused(&mut state2, &wf, 10) };

        // Results should match
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_fused_and_correctness() {
        let executor = Zen3Executor::new();
        let mut state1 = UorState::zero();
        let mut state2 = UorState::zero();

        for i in 0..32 {
            state1.ymm[0][i] = Taxon::new(0xFF);
            state1.ymm[8][i] = Taxon::new((i * 7) as u8);
            state2.ymm[0][i] = Taxon::new(0xFF);
            state2.ymm[8][i] = Taxon::new((i * 7) as u8);
        }

        let wf = Wavefront::new(crate::isa::PortAssignment::all_and());

        for _ in 0..5 {
            unsafe { executor.step(&mut state1, &wf) };
        }

        unsafe { executor.step_n_fused(&mut state2, &wf, 5) };

        assert_eq!(state1, state2);
    }

    #[test]
    fn test_is_homogeneous() {
        let xor = Wavefront::all_xor();
        let and = Wavefront::new(crate::isa::PortAssignment::all_and());

        assert!(is_homogeneous(&[]));
        assert!(is_homogeneous(&[xor]));
        assert!(is_homogeneous(&[xor, xor, xor]));
        assert!(!is_homogeneous(&[xor, and]));
    }
}
