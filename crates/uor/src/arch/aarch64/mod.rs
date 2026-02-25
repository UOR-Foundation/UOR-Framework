//! ARM aarch64 UOR executor with NEON SIMD support.
//!
//! This module provides NEON-accelerated implementations of the UorStep traits
//! for ARM aarch64 processors. NEON is ARM's SIMD architecture, providing 128-bit
//! vector operations.
//!
//! # Implementation Strategy
//!
//! On aarch64 targets, uses actual NEON intrinsics for maximum performance.
//! On other targets, delegates to ScalarExecutor for compilation compatibility.

use crate::arch::portable::ScalarExecutor;
use crate::isa::{UorStep, UorStepBinary, UorStepFused, UorStepLossless, Wavefront};
use crate::state::UorState;

/// ARM aarch64 executor using NEON SIMD instructions.
///
/// This executor targets ARM aarch64 processors with NEON support, which is
/// mandatory on all ARMv8-A processors. NEON provides 128-bit vector operations
/// (16 bytes per instruction).
///
/// # NEON Features
///
/// - **128-bit vectors**: Process 16 bytes per instruction (vs 32 for AVX2)
/// - **32 vector registers**: V0-V31 (128-bit each)
/// - **Crypto extensions**: SHA1, SHA256, AES acceleration (when available)
///
/// # Performance Targets
///
/// - Single wavefront: < 5 cycles (similar to Zen3)
/// - Element-wise ops: < 3 cycles per 16-byte chunk
/// - Matrix operations: Approach NEON GEMM performance
#[derive(Debug, Clone, Copy, Default)]
pub struct NeonExecutor {
    /// CPU feature flags detected at runtime
    features: CpuFeatures,
}

/// CPU feature detection for aarch64.
#[derive(Debug, Clone, Copy, Default)]
pub struct CpuFeatures {
    /// SHA1 and SHA256 crypto extensions
    pub has_sha: bool,
    /// AES and PMULL crypto extensions
    pub has_aes: bool,
    /// SHA3, SHA512, SM3, SM4 extensions (ARMv8.2+)
    pub has_sha3: bool,
}

impl NeonExecutor {
    /// Create a new NEON executor with runtime feature detection.
    #[must_use]
    pub fn new() -> Self {
        Self {
            features: CpuFeatures::detect(),
        }
    }

    /// Get the detected CPU features.
    #[inline]
    #[must_use]
    pub const fn features(&self) -> &CpuFeatures {
        &self.features
    }
}

impl CpuFeatures {
    /// Detect CPU features at runtime.
    ///
    /// On aarch64, NEON is always available (mandatory in ARMv8-A).
    /// Crypto extensions are optional. For now, we use compile-time detection
    /// via target features to avoid external dependencies.
    #[must_use]
    pub fn detect() -> Self {
        // Use compile-time feature detection
        // Runtime detection would require libc (getauxval) which violates
        // the zero-dependency constraint
        Self {
            has_sha: cfg!(target_feature = "sha2"),
            has_aes: cfg!(target_feature = "aes"),
            has_sha3: cfg!(target_feature = "sha3"),
        }
    }

    /// Check if all required features are available.
    #[inline]
    #[must_use]
    pub const fn is_supported(&self) -> bool {
        // NEON is mandatory on ARMv8-A, so always supported
        true
    }
}

// =============================================================================
// NEON Implementation (aarch64 only)
// =============================================================================

#[cfg(target_arch = "aarch64")]
mod neon_impl {
    use super::*;
    use crate::core::taxon::Taxon;
    use crate::isa::WavefrontOp;
    use crate::state::{GPR_COUNT, GPR_TAXONS, YMM_COUNT, YMM_TAXONS};
    use core::arch::aarch64::*;

    /// Number of 32-bit lanes per YMM register (256 bits / 32 = 8 lanes).
    const YMM_LANES: usize = YMM_TAXONS / 4;

    /// Number of 32-bit lanes per GPR (64 bits / 32 = 2 lanes).
    const GPR_LANES: usize = GPR_TAXONS / 4;

    /// Read a 32-bit lane from a taxon array.
    #[inline(always)]
    fn read_lane(taxons: &[Taxon], lane: usize) -> u32 {
        let base = lane * 4;
        u32::from_le_bytes([
            taxons[base].value(),
            taxons[base + 1].value(),
            taxons[base + 2].value(),
            taxons[base + 3].value(),
        ])
    }

    /// Write a 32-bit lane to a taxon array.
    #[inline(always)]
    fn write_lane(taxons: &mut [Taxon], lane: usize, value: u32) {
        let base = lane * 4;
        let bytes = value.to_le_bytes();
        taxons[base] = Taxon::new(bytes[0]);
        taxons[base + 1] = Taxon::new(bytes[1]);
        taxons[base + 2] = Taxon::new(bytes[2]);
        taxons[base + 3] = Taxon::new(bytes[3]);
    }

    /// Load 128 bits from taxon array into NEON register.
    #[inline(always)]
    unsafe fn load_128(taxons: &[Taxon], offset: usize) -> uint8x16_t {
        let ptr = taxons.as_ptr().add(offset) as *const u8;
        vld1q_u8(ptr)
    }

    /// Store 128 bits from NEON register to taxon array.
    #[inline(always)]
    unsafe fn store_128(taxons: &mut [Taxon], offset: usize, val: uint8x16_t) {
        let ptr = taxons.as_mut_ptr().add(offset) as *mut u8;
        vst1q_u8(ptr, val);
    }

    /// Apply XOR operation using NEON: ymm[i] ^= ymm[i+8]
    #[inline]
    pub unsafe fn apply_xor_neon(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
        for i in 0..8 {
            if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
                // Process 256 bits as 2 x 128-bit NEON operations
                for lane128 in 0..2 {
                    let offset = lane128 * 16;
                    let a = load_128(&state.ymm[i], offset);
                    let b = load_128(&state.ymm[i + 8], offset);
                    let result = veorq_u8(a, b);
                    store_128(&mut state.ymm[i], offset, result);
                }
            }
        }
        // GPRs are smaller, use scalar for simplicity
        for i in 0..7 {
            if i + 7 < GPR_COUNT && (gpr_mask >> i) & 1 == 1 && (gpr_mask >> (i + 7)) & 1 == 1 {
                for lane in 0..GPR_LANES {
                    let a = read_lane(&state.gpr[i], lane);
                    let b = read_lane(&state.gpr[i + 7], lane);
                    write_lane(&mut state.gpr[i], lane, a ^ b);
                }
            }
        }
    }

    /// Apply AND operation using NEON: ymm[i] &= ymm[i+8]
    #[inline]
    pub unsafe fn apply_and_neon(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
        for i in 0..8 {
            if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
                for lane128 in 0..2 {
                    let offset = lane128 * 16;
                    let a = load_128(&state.ymm[i], offset);
                    let b = load_128(&state.ymm[i + 8], offset);
                    let result = vandq_u8(a, b);
                    store_128(&mut state.ymm[i], offset, result);
                }
            }
        }
        for i in 0..7 {
            if i + 7 < GPR_COUNT && (gpr_mask >> i) & 1 == 1 && (gpr_mask >> (i + 7)) & 1 == 1 {
                for lane in 0..GPR_LANES {
                    let a = read_lane(&state.gpr[i], lane);
                    let b = read_lane(&state.gpr[i + 7], lane);
                    write_lane(&mut state.gpr[i], lane, a & b);
                }
            }
        }
    }

    /// Apply OR operation using NEON: ymm[i] |= ymm[i+8]
    #[inline]
    pub unsafe fn apply_or_neon(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
        for i in 0..8 {
            if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
                for lane128 in 0..2 {
                    let offset = lane128 * 16;
                    let a = load_128(&state.ymm[i], offset);
                    let b = load_128(&state.ymm[i + 8], offset);
                    let result = vorrq_u8(a, b);
                    store_128(&mut state.ymm[i], offset, result);
                }
            }
        }
        for i in 0..7 {
            if i + 7 < GPR_COUNT && (gpr_mask >> i) & 1 == 1 && (gpr_mask >> (i + 7)) & 1 == 1 {
                for lane in 0..GPR_LANES {
                    let a = read_lane(&state.gpr[i], lane);
                    let b = read_lane(&state.gpr[i + 7], lane);
                    write_lane(&mut state.gpr[i], lane, a | b);
                }
            }
        }
    }

    /// Apply NOT operation using NEON: ymm[i] = ~ymm[i]
    #[inline]
    pub unsafe fn apply_not_neon(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
        for i in 0..YMM_COUNT {
            if (ymm_mask >> i) & 1 == 1 {
                for lane128 in 0..2 {
                    let offset = lane128 * 16;
                    let a = load_128(&state.ymm[i], offset);
                    let result = vmvnq_u8(a);
                    store_128(&mut state.ymm[i], offset, result);
                }
            }
        }
        for i in 0..GPR_COUNT {
            if (gpr_mask >> i) & 1 == 1 {
                for lane in 0..GPR_LANES {
                    let a = read_lane(&state.gpr[i], lane);
                    write_lane(&mut state.gpr[i], lane, !a);
                }
            }
        }
    }

    /// Apply ADD operation using NEON: ymm[i] += ymm[i+8] (per 32-bit lane)
    #[inline]
    pub unsafe fn apply_add_neon(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
        for i in 0..8 {
            if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
                for lane128 in 0..2 {
                    let offset = lane128 * 16;
                    let a = vreinterpretq_u32_u8(load_128(&state.ymm[i], offset));
                    let b = vreinterpretq_u32_u8(load_128(&state.ymm[i + 8], offset));
                    let result = vaddq_u32(a, b);
                    store_128(&mut state.ymm[i], offset, vreinterpretq_u8_u32(result));
                }
            }
        }
        for i in 0..7 {
            if i + 7 < GPR_COUNT && (gpr_mask >> i) & 1 == 1 && (gpr_mask >> (i + 7)) & 1 == 1 {
                for lane in 0..GPR_LANES {
                    let a = read_lane(&state.gpr[i], lane);
                    let b = read_lane(&state.gpr[i + 7], lane);
                    write_lane(&mut state.gpr[i], lane, a.wrapping_add(b));
                }
            }
        }
    }

    /// Apply SUB operation using NEON: ymm[i] -= ymm[i+8] (per 32-bit lane)
    #[inline]
    pub unsafe fn apply_sub_neon(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
        for i in 0..8 {
            if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
                for lane128 in 0..2 {
                    let offset = lane128 * 16;
                    let a = vreinterpretq_u32_u8(load_128(&state.ymm[i], offset));
                    let b = vreinterpretq_u32_u8(load_128(&state.ymm[i + 8], offset));
                    let result = vsubq_u32(a, b);
                    store_128(&mut state.ymm[i], offset, vreinterpretq_u8_u32(result));
                }
            }
        }
        for i in 0..7 {
            if i + 7 < GPR_COUNT && (gpr_mask >> i) & 1 == 1 && (gpr_mask >> (i + 7)) & 1 == 1 {
                for lane in 0..GPR_LANES {
                    let a = read_lane(&state.gpr[i], lane);
                    let b = read_lane(&state.gpr[i + 7], lane);
                    write_lane(&mut state.gpr[i], lane, a.wrapping_sub(b));
                }
            }
        }
    }

    /// Apply rotate left (per 32-bit lane) - scalar fallback
    #[inline]
    pub unsafe fn apply_rotl_neon(state: &mut UorState, n: u8, ymm_mask: u16, gpr_mask: u16) {
        // NEON doesn't have direct rotate, use scalar for correctness
        for i in 0..YMM_COUNT {
            if (ymm_mask >> i) & 1 == 1 {
                for lane in 0..YMM_LANES {
                    let a = read_lane(&state.ymm[i], lane);
                    write_lane(&mut state.ymm[i], lane, a.rotate_left(n as u32));
                }
            }
        }
        for i in 0..GPR_COUNT {
            if (gpr_mask >> i) & 1 == 1 {
                for lane in 0..GPR_LANES {
                    let a = read_lane(&state.gpr[i], lane);
                    write_lane(&mut state.gpr[i], lane, a.rotate_left(n as u32));
                }
            }
        }
    }

    /// Apply rotate right (per 32-bit lane) - scalar fallback
    #[inline]
    pub unsafe fn apply_rotr_neon(state: &mut UorState, n: u8, ymm_mask: u16, gpr_mask: u16) {
        for i in 0..YMM_COUNT {
            if (ymm_mask >> i) & 1 == 1 {
                for lane in 0..YMM_LANES {
                    let a = read_lane(&state.ymm[i], lane);
                    write_lane(&mut state.ymm[i], lane, a.rotate_right(n as u32));
                }
            }
        }
        for i in 0..GPR_COUNT {
            if (gpr_mask >> i) & 1 == 1 {
                for lane in 0..GPR_LANES {
                    let a = read_lane(&state.gpr[i], lane);
                    write_lane(&mut state.gpr[i], lane, a.rotate_right(n as u32));
                }
            }
        }
    }

    /// Apply shift left (per 32-bit lane) - scalar fallback
    #[inline]
    pub unsafe fn apply_shl_neon(state: &mut UorState, n: u8, ymm_mask: u16, gpr_mask: u16) {
        for i in 0..YMM_COUNT {
            if (ymm_mask >> i) & 1 == 1 {
                for lane in 0..YMM_LANES {
                    let a = read_lane(&state.ymm[i], lane);
                    write_lane(&mut state.ymm[i], lane, a.wrapping_shl(n as u32));
                }
            }
        }
        for i in 0..GPR_COUNT {
            if (gpr_mask >> i) & 1 == 1 {
                for lane in 0..GPR_LANES {
                    let a = read_lane(&state.gpr[i], lane);
                    write_lane(&mut state.gpr[i], lane, a.wrapping_shl(n as u32));
                }
            }
        }
    }

    /// Apply shift right (per 32-bit lane) - scalar fallback
    #[inline]
    pub unsafe fn apply_shr_neon(state: &mut UorState, n: u8, ymm_mask: u16, gpr_mask: u16) {
        for i in 0..YMM_COUNT {
            if (ymm_mask >> i) & 1 == 1 {
                for lane in 0..YMM_LANES {
                    let a = read_lane(&state.ymm[i], lane);
                    write_lane(&mut state.ymm[i], lane, a.wrapping_shr(n as u32));
                }
            }
        }
        for i in 0..GPR_COUNT {
            if (gpr_mask >> i) & 1 == 1 {
                for lane in 0..GPR_LANES {
                    let a = read_lane(&state.gpr[i], lane);
                    write_lane(&mut state.gpr[i], lane, a.wrapping_shr(n as u32));
                }
            }
        }
    }

    /// Apply operation dispatch for NEON
    #[inline]
    pub unsafe fn apply_op_neon(
        state: &mut UorState,
        op: WavefrontOp,
        ymm_mask: u16,
        gpr_mask: u16,
    ) {
        match op {
            WavefrontOp::Nop => {}
            WavefrontOp::Xor => apply_xor_neon(state, ymm_mask, gpr_mask),
            WavefrontOp::And => apply_and_neon(state, ymm_mask, gpr_mask),
            WavefrontOp::Or => apply_or_neon(state, ymm_mask, gpr_mask),
            WavefrontOp::Not => apply_not_neon(state, ymm_mask, gpr_mask),
            WavefrontOp::Add => apply_add_neon(state, ymm_mask, gpr_mask),
            WavefrontOp::Sub => apply_sub_neon(state, ymm_mask, gpr_mask),
            WavefrontOp::RotL(n) => apply_rotl_neon(state, n, ymm_mask, gpr_mask),
            WavefrontOp::RotR(n) => apply_rotr_neon(state, n, ymm_mask, gpr_mask),
            WavefrontOp::ShL(n) => apply_shl_neon(state, n, ymm_mask, gpr_mask),
            WavefrontOp::ShR(n) => apply_shr_neon(state, n, ymm_mask, gpr_mask),
            // Crypto and permute operations use scalar fallback
            _ => crate::arch::portable::apply_op(state, op, ymm_mask, gpr_mask),
        }
    }
}

// =============================================================================
// UorStep Implementations
// =============================================================================

impl UorStep for NeonExecutor {
    #[cfg(target_arch = "aarch64")]
    unsafe fn step(&self, state: &mut UorState, wavefront: &Wavefront) {
        let ymm_mask = wavefront.ymm_mask;
        let gpr_mask = wavefront.gpr_mask;

        neon_impl::apply_op_neon(state, wavefront.ports.port0, ymm_mask, gpr_mask);
        neon_impl::apply_op_neon(state, wavefront.ports.port1, ymm_mask, gpr_mask);
        if wavefront.ports.port5 != wavefront.ports.port1 {
            neon_impl::apply_op_neon(state, wavefront.ports.port5, ymm_mask, gpr_mask);
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    unsafe fn step(&self, state: &mut UorState, wavefront: &Wavefront) {
        // Delegate to ScalarExecutor on non-aarch64 platforms
        ScalarExecutor::new().step(state, wavefront);
    }
}

impl UorStepLossless for NeonExecutor {
    unsafe fn step_tracked(
        &self,
        state: &mut UorState,
        complement: &mut UorState,
        wavefront: &Wavefront,
    ) {
        // Delegate to ScalarExecutor for lossless operations
        ScalarExecutor::new().step_tracked(state, complement, wavefront);
    }

    unsafe fn step_inverse(
        &self,
        state: &mut UorState,
        complement: &UorState,
        wavefront: &Wavefront,
    ) {
        ScalarExecutor::new().step_inverse(state, complement, wavefront);
    }
}

impl UorStepFused for NeonExecutor {
    unsafe fn run_fused(&self, state: &mut UorState, program: &[Wavefront]) {
        for wavefront in program {
            self.step(state, wavefront);
        }
    }

    unsafe fn step_n_fused(&self, state: &mut UorState, wavefront: &Wavefront, n: usize) {
        for _ in 0..n {
            self.step(state, wavefront);
        }
    }
}

impl UorStepBinary for NeonExecutor {
    unsafe fn step_binary(
        &self,
        state_a: &mut UorState,
        state_b: &UorState,
        wavefront: &Wavefront,
    ) {
        // Delegate to ScalarExecutor for binary operations
        ScalarExecutor::new().step_binary(state_a, state_b, wavefront);
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neon_executor_creation() {
        let executor = NeonExecutor::new();
        let default = NeonExecutor::default();

        // Both should have detected features
        assert!(executor.features().is_supported());
        assert!(default.features().is_supported());
    }

    #[test]
    fn test_cpu_features_detect() {
        let features = CpuFeatures::detect();
        // NEON is always supported on aarch64
        assert!(features.is_supported());
    }

    #[test]
    fn test_neon_xor_self_inverse() {
        let executor = NeonExecutor::new();
        let mut state = UorState::zero();

        // Set up initial values
        state.ymm[0][0] = crate::core::taxon::Taxon::new(0x12);
        state.ymm[0][1] = crate::core::taxon::Taxon::new(0x34);
        state.ymm[8][0] = crate::core::taxon::Taxon::new(0xAA);
        state.ymm[8][1] = crate::core::taxon::Taxon::new(0xBB);

        let original = state.ymm[0];
        let wavefront = Wavefront::all_xor();

        // First XOR
        unsafe { executor.step(&mut state, &wavefront) };
        assert_ne!(state.ymm[0], original);

        // Second XOR (should restore)
        unsafe { executor.step(&mut state, &wavefront) };
        assert_eq!(state.ymm[0], original);
    }

    #[test]
    fn test_neon_add_sub() {
        let executor = NeonExecutor::new();
        let mut state = UorState::zero();

        // Set ymm0 = 100, ymm8 = 50
        state.ymm[0][0] = crate::core::taxon::Taxon::new(100);
        state.ymm[8][0] = crate::core::taxon::Taxon::new(50);

        let add_wf = Wavefront::all_add();
        unsafe { executor.step(&mut state, &add_wf) };
        assert_eq!(state.ymm[0][0].value(), 150);

        let sub_wf = Wavefront::all_sub();
        unsafe { executor.step(&mut state, &sub_wf) };
        assert_eq!(state.ymm[0][0].value(), 100);
    }

    #[test]
    fn test_neon_and() {
        let executor = NeonExecutor::new();
        let mut state = UorState::zero();

        state.ymm[0][0] = crate::core::taxon::Taxon::new(0xFF);
        state.ymm[8][0] = crate::core::taxon::Taxon::new(0x0F);

        let wf = Wavefront::all_and();
        unsafe { executor.step(&mut state, &wf) };
        assert_eq!(state.ymm[0][0].value(), 0x0F);
    }

    #[test]
    fn test_neon_or() {
        let executor = NeonExecutor::new();
        let mut state = UorState::zero();

        state.ymm[0][0] = crate::core::taxon::Taxon::new(0xF0);
        state.ymm[8][0] = crate::core::taxon::Taxon::new(0x0F);

        let wf = Wavefront::all_or();
        unsafe { executor.step(&mut state, &wf) };
        assert_eq!(state.ymm[0][0].value(), 0xFF);
    }

    #[test]
    fn test_neon_not() {
        let executor = NeonExecutor::new();
        let mut state = UorState::zero();

        state.ymm[0][0] = crate::core::taxon::Taxon::new(0x00);

        let wf = Wavefront::all_not();
        unsafe { executor.step(&mut state, &wf) };
        assert_eq!(state.ymm[0][0].value(), 0xFF);
    }

    #[test]
    fn test_neon_lossless_roundtrip() {
        let executor = NeonExecutor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        state.ymm[0][0] = crate::core::taxon::Taxon::new(0x42);
        state.ymm[8][0] = crate::core::taxon::Taxon::new(0x24);

        let original = state.ymm[0];
        let wf = Wavefront::all_xor();

        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };
        assert_ne!(state.ymm[0], original);

        unsafe { executor.step_inverse(&mut state, &complement, &wf) };
        assert_eq!(state.ymm[0], original);
    }

    #[test]
    fn test_neon_run_fused() {
        let executor = NeonExecutor::new();
        let mut state = UorState::zero();

        state.ymm[0][0] = crate::core::taxon::Taxon::new(0x12);
        state.ymm[8][0] = crate::core::taxon::Taxon::new(0x12);

        let program = [Wavefront::all_xor(), Wavefront::all_xor()];
        unsafe { executor.run_fused(&mut state, &program) };

        // XOR twice with same value = original
        assert_eq!(state.ymm[0][0].value(), 0x12);
    }

    #[test]
    fn test_neon_step_n_fused() {
        let executor = NeonExecutor::new();
        let mut state = UorState::zero();

        state.ymm[0][0] = crate::core::taxon::Taxon::new(100);
        state.ymm[8][0] = crate::core::taxon::Taxon::new(10);

        let wf = Wavefront::all_add();
        unsafe { executor.step_n_fused(&mut state, &wf, 5) };

        // 100 + (10 * 5) = 150
        assert_eq!(state.ymm[0][0].value(), 150);
    }
}
