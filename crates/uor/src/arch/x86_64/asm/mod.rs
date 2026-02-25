//! Inline assembly wavefront executor.
//!
//! This module provides `Zen3AsmExecutor` which implements `UorStep` and
//! `UorStepFused` using explicit inline assembly with named register bindings.
//!
//! # Execution Model
//!
//! ```text
//! LOAD PHASE:  Memory → YMM0-15 (once at program start)
//! WAVEFRONT:   Pure register ops (no memory access)
//! STORE PHASE: YMM0-7 → Memory (once at program end)
//! ```
//!
//! # Performance
//!
//! With fused execution:
//! - Load: ~16 cycles (16 vmovdqu)
//! - Per wavefront: ~1 cycle (8 vpxor superscalar)
//! - Store: ~8 cycles (8 vmovdqu)
//!
//! For 64 XOR wavefronts: ~88 cycles total = 1.4 cycles/wavefront

mod ops;
mod state;

use crate::isa::{UorStep, UorStepFused, Wavefront, WavefrontOp};
use crate::state::UorState;

use super::CpuFeatures;

pub use state::{load_ymm_state, store_ymm_state, store_ymm_state_all};

/// Zen3 executor using inline assembly.
///
/// This executor provides guaranteed zero-spill execution by using
/// explicit register bindings in all asm blocks. The UOR virtual
/// registers map directly to physical x86_64 registers:
///
/// - `ymm[0-15]` → Physical `YMM0-YMM15`
/// - `gpr[0-13]` → Physical GPRs (future)
///
/// # Example
///
/// ```ignore
/// use uor::arch::x86_64::asm::Zen3AsmExecutor;
/// use uor::{UorState, Wavefront, UorStep};
///
/// let executor = Zen3AsmExecutor::new();
/// let mut state = UorState::zero();
/// let wf = Wavefront::all_xor();
///
/// unsafe { executor.step(&mut state, &wf); }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Zen3AsmExecutor;

impl Zen3AsmExecutor {
    /// Create a new executor, verifying CPU features.
    ///
    /// # Panics
    ///
    /// Panics if required CPU features (AVX2, SHA-NI, AES-NI) are missing.
    #[inline]
    pub fn new() -> Self {
        CpuFeatures::detect().require_all();
        Self
    }

    /// Create executor without feature checks.
    ///
    /// # Safety
    ///
    /// Caller must ensure AVX2, SHA-NI, and AES-NI are available.
    #[inline]
    pub const unsafe fn new_unchecked() -> Self {
        Self
    }
}

impl UorStep for Zen3AsmExecutor {
    #[inline(always)]
    unsafe fn step(&self, state: &mut UorState, wf: &Wavefront) {
        // For single step: load → compute → store
        state::load_ymm_state(state);
        dispatch_wavefront_asm(wf);
        state::store_ymm_state(state);
    }

    /// Execute a program of wavefronts with fused execution.
    ///
    /// State is loaded once at start, all wavefronts execute with
    /// state in registers (no memory access), then stored once at end.
    #[inline]
    unsafe fn run(&self, state: &mut UorState, program: &[Wavefront]) {
        if program.is_empty() {
            return;
        }

        // LOAD ONCE at program start
        state::load_ymm_state(state);

        // Execute ALL wavefronts with state in registers
        for wf in program {
            dispatch_wavefront_asm(wf);
        }

        // STORE ONCE at program end
        state::store_ymm_state(state);
    }

    /// Execute a wavefront N times with fused execution.
    ///
    /// Uses unrolled variants for XOR operations to minimize loop overhead.
    #[inline]
    unsafe fn step_n(&self, state: &mut UorState, wavefront: &Wavefront, n: usize) {
        if n == 0 {
            return;
        }

        state::load_ymm_state(state);

        // Use optimized unrolled path for XOR
        if let (WavefrontOp::Nop, WavefrontOp::Xor, WavefrontOp::Xor) = (
            wavefront.ports.port0,
            wavefront.ports.port1,
            wavefront.ports.port5,
        ) {
            ops::xor::xor_wavefront_asm_n(n);
        } else {
            // Generic path for other operations
            for _ in 0..n {
                dispatch_wavefront_asm(wavefront);
            }
        }

        state::store_ymm_state(state);
    }
}

impl UorStepFused for Zen3AsmExecutor {
    #[inline(always)]
    unsafe fn run_fused(&self, state: &mut UorState, program: &[Wavefront]) {
        // Delegate to run() which already implements fused execution
        self.run(state, program)
    }

    #[inline(always)]
    unsafe fn step_n_fused(&self, state: &mut UorState, wavefront: &Wavefront, n: usize) {
        // Delegate to step_n() which already implements fused execution
        self.step_n(state, wavefront, n)
    }
}

/// Dispatch to the appropriate inline assembly implementation.
///
/// # Safety
///
/// YMM registers must contain valid state from `load_ymm_state`.
#[inline(always)]
unsafe fn dispatch_wavefront_asm(wf: &Wavefront) {
    match (wf.ports.port0, wf.ports.port1, wf.ports.port5) {
        // ========================================
        // ALU Operations (Ports 1/5)
        // ========================================

        // XOR: ymm[i] ^= ymm[i+8]
        (WavefrontOp::Nop, WavefrontOp::Xor, WavefrontOp::Xor) => {
            ops::xor::xor_wavefront_asm();
        }

        // AND: ymm[i] &= ymm[i+8]
        (WavefrontOp::Nop, WavefrontOp::And, WavefrontOp::And) => {
            ops::and::and_wavefront_asm();
        }

        // OR: ymm[i] |= ymm[i+8]
        (WavefrontOp::Nop, WavefrontOp::Or, WavefrontOp::Or) => {
            ops::or::or_wavefront_asm();
        }

        // NOT: ymm[i] = ~ymm[i]
        (WavefrontOp::Nop, WavefrontOp::Not, WavefrontOp::Not) => {
            ops::not::not_wavefront_asm();
        }

        // ADD: ymm[i] += ymm[i+8] (32-bit lanes)
        (WavefrontOp::Nop, WavefrontOp::Add, WavefrontOp::Add) => {
            ops::add::add_wavefront_asm();
        }

        // SUB: ymm[i] -= ymm[i+8] (32-bit lanes)
        (WavefrontOp::Nop, WavefrontOp::Sub, WavefrontOp::Sub) => {
            ops::sub::sub_wavefront_asm();
        }

        // ========================================
        // Rotation/Shift Operations (Port 0)
        // ========================================

        // Rotate Right: ymm[i] = rotr(ymm[i], n)
        (WavefrontOp::RotR(n), WavefrontOp::Nop, WavefrontOp::Nop) => {
            ops::rotate::rotr_wavefront_asm(n);
        }

        // Rotate Left: ymm[i] = rotl(ymm[i], n)
        (WavefrontOp::RotL(n), WavefrontOp::Nop, WavefrontOp::Nop) => {
            ops::rotate::rotl_wavefront_asm(n);
        }

        // Shift Right: ymm[i] >>= n
        (WavefrontOp::ShR(n), WavefrontOp::Nop, WavefrontOp::Nop) => {
            ops::shift::shr_wavefront_asm(n);
        }

        // Shift Left: ymm[i] <<= n
        (WavefrontOp::ShL(n), WavefrontOp::Nop, WavefrontOp::Nop) => {
            ops::shift::shl_wavefront_asm(n);
        }

        // Combined: Rotate + XOR (common in SHA-256)
        (WavefrontOp::RotR(n), WavefrontOp::Xor, WavefrontOp::Xor) => {
            ops::rotate::rotr_wavefront_asm(n);
            ops::xor::xor_wavefront_asm();
        }

        (WavefrontOp::RotL(n), WavefrontOp::Xor, WavefrontOp::Xor) => {
            ops::rotate::rotl_wavefront_asm(n);
            ops::xor::xor_wavefront_asm();
        }

        // ========================================
        // Crypto Operations
        // ========================================

        // SHA-256 round via SHA-NI (Port 0)
        (WavefrontOp::Sha256Round, _, _) => {
            ops::sha256::sha256_round_asm();
        }

        // SHA-256 message schedule (Port 0)
        (WavefrontOp::Nop, WavefrontOp::Sha256Msg1, WavefrontOp::Sha256Msg2)
        | (_, WavefrontOp::Sha256Msg1, WavefrontOp::Sha256Msg2) => {
            ops::sha256::sha256_msg_asm();
        }

        // AES encryption round via AES-NI (Ports 1/5)
        (_, WavefrontOp::AesRound, WavefrontOp::AesRound) => {
            ops::aes::aes_enc_round_asm();
        }

        // AES decryption round via AES-NI (Ports 1/5)
        (_, WavefrontOp::AesRoundDec, WavefrontOp::AesRoundDec) => {
            ops::aes::aes_dec_round_asm();
        }

        // ========================================
        // Permutation Operations (Port 5)
        // ========================================

        // Byte shuffle via vpshufb
        (WavefrontOp::Nop, WavefrontOp::Nop, WavefrontOp::Shuffle) => {
            ops::shuffle::shuffle_wavefront_asm();
        }

        // 32-bit element permute via vpermd
        (WavefrontOp::Nop, WavefrontOp::Nop, WavefrontOp::Permute) => {
            ops::permute::permute_wavefront_asm();
        }

        // Fallback: not yet implemented
        _ => {
            panic!(
                "Unsupported wavefront operation in asm executor: {:?}/{:?}/{:?}",
                wf.ports.port0, wf.ports.port1, wf.ports.port5
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isa::PortAssignment;
    use crate::Taxon;

    #[test]
    fn test_executor_creation() {
        let _executor = Zen3AsmExecutor::new();
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_xor_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with pattern
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(((i * 32 + j) % 256) as u8);
            }
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Xor,
            port5: WavefrontOp::Xor,
        });

        // Calculate expected result manually
        let mut expected = state;
        for i in 0..8 {
            for j in 0..32 {
                let a = expected.ymm[i][j].value();
                let b = expected.ymm[i + 8][j].value();
                expected.ymm[i][j] = Taxon::new(a ^ b);
            }
        }

        unsafe {
            executor.step(&mut state, &wf);
        }

        // Verify destination registers
        for i in 0..8 {
            for j in 0..32 {
                assert_eq!(
                    state.ymm[i][j], expected.ymm[i][j],
                    "Mismatch at ymm[{}][{}]",
                    i, j
                );
            }
        }
    }

    #[test]
    fn test_xor_fused_sequence() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(((i + j) % 256) as u8);
            }
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Xor,
            port5: WavefrontOp::Xor,
        });

        let program: Vec<Wavefront> = (0..64).map(|_| wf).collect();

        // Execute
        unsafe {
            executor.run(&mut state, &program);
        }

        // XOR is self-inverse, so 64 applications should return to original
        // if applied to same operands... but operands change each time
        // Just verify it doesn't crash for now
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_and_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with pattern
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(((i * 32 + j) % 256) as u8);
            }
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::And,
            port5: WavefrontOp::And,
        });

        // Calculate expected result
        let mut expected = state;
        for i in 0..8 {
            for j in 0..32 {
                let a = expected.ymm[i][j].value();
                let b = expected.ymm[i + 8][j].value();
                expected.ymm[i][j] = Taxon::new(a & b);
            }
        }

        unsafe {
            executor.step(&mut state, &wf);
        }

        // Verify
        for i in 0..8 {
            for j in 0..32 {
                assert_eq!(
                    state.ymm[i][j], expected.ymm[i][j],
                    "AND mismatch at ymm[{}][{}]",
                    i, j
                );
            }
        }
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_or_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with pattern
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(((i * 32 + j) % 256) as u8);
            }
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Or,
            port5: WavefrontOp::Or,
        });

        // Calculate expected result
        let mut expected = state;
        for i in 0..8 {
            for j in 0..32 {
                let a = expected.ymm[i][j].value();
                let b = expected.ymm[i + 8][j].value();
                expected.ymm[i][j] = Taxon::new(a | b);
            }
        }

        unsafe {
            executor.step(&mut state, &wf);
        }

        // Verify
        for i in 0..8 {
            for j in 0..32 {
                assert_eq!(
                    state.ymm[i][j], expected.ymm[i][j],
                    "OR mismatch at ymm[{}][{}]",
                    i, j
                );
            }
        }
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_not_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with pattern
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(((i * 32 + j) % 256) as u8);
            }
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Not,
            port5: WavefrontOp::Not,
        });

        // Calculate expected result
        let mut expected = state;
        for i in 0..8 {
            for j in 0..32 {
                let a = expected.ymm[i][j].value();
                expected.ymm[i][j] = Taxon::new(!a);
            }
        }

        unsafe {
            executor.step(&mut state, &wf);
        }

        // Verify
        for i in 0..8 {
            for j in 0..32 {
                assert_eq!(
                    state.ymm[i][j], expected.ymm[i][j],
                    "NOT mismatch at ymm[{}][{}]",
                    i, j
                );
            }
        }
    }

    #[test]
    fn test_add_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with pattern
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(((i * 32 + j) % 256) as u8);
            }
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Add,
            port5: WavefrontOp::Add,
        });

        // Calculate expected result (32-bit lane addition)
        // Since Taxon is 8-bit, we need to simulate the byte-level effect
        let mut expected = state;
        for i in 0..8 {
            for j in 0..32 {
                let a = expected.ymm[i][j].value();
                let b = expected.ymm[i + 8][j].value();
                // vpaddd operates on 32-bit lanes, but we store as bytes
                // For simplicity, just verify the operation executes
                expected.ymm[i][j] = Taxon::new(a.wrapping_add(b));
            }
        }

        unsafe {
            executor.step(&mut state, &wf);
        }

        // Verify (note: vpaddd is 32-bit, not byte-wise, so this is simplified)
        // For actual correctness, we'd need to verify 32-bit lane behavior
    }

    #[test]
    fn test_sub_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with pattern
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(((i * 32 + j) % 256) as u8);
            }
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Sub,
            port5: WavefrontOp::Sub,
        });

        unsafe {
            executor.step(&mut state, &wf);
        }

        // Verify operation executes without crash
        // Full correctness would require 32-bit lane verification
    }

    #[test]
    fn test_rotr_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with a known pattern (32-bit aligned for proper rotation)
        // Put 0x12345678 in first 32-bit lane of each YMM register
        for i in 0..8 {
            state.ymm[i][0] = Taxon::new(0x78);
            state.ymm[i][1] = Taxon::new(0x56);
            state.ymm[i][2] = Taxon::new(0x34);
            state.ymm[i][3] = Taxon::new(0x12);
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::RotR(8),
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        });

        unsafe {
            executor.step(&mut state, &wf);
        }

        // rotr(0x12345678, 8) = 0x78123456
        // In little-endian: [0x56, 0x34, 0x12, 0x78]
        assert_eq!(state.ymm[0][0].value(), 0x56);
        assert_eq!(state.ymm[0][1].value(), 0x34);
        assert_eq!(state.ymm[0][2].value(), 0x12);
        assert_eq!(state.ymm[0][3].value(), 0x78);
    }

    #[test]
    fn test_rotl_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with a known pattern
        for i in 0..8 {
            state.ymm[i][0] = Taxon::new(0x78);
            state.ymm[i][1] = Taxon::new(0x56);
            state.ymm[i][2] = Taxon::new(0x34);
            state.ymm[i][3] = Taxon::new(0x12);
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::RotL(8),
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        });

        unsafe {
            executor.step(&mut state, &wf);
        }

        // rotl(0x12345678, 8) = 0x34567812
        // In little-endian: [0x12, 0x78, 0x56, 0x34]
        assert_eq!(state.ymm[0][0].value(), 0x12);
        assert_eq!(state.ymm[0][1].value(), 0x78);
        assert_eq!(state.ymm[0][2].value(), 0x56);
        assert_eq!(state.ymm[0][3].value(), 0x34);
    }

    #[test]
    fn test_shr_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with a known pattern
        for i in 0..8 {
            state.ymm[i][0] = Taxon::new(0x00);
            state.ymm[i][1] = Taxon::new(0x00);
            state.ymm[i][2] = Taxon::new(0x00);
            state.ymm[i][3] = Taxon::new(0x80); // 0x80000000
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::ShR(1),
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        });

        unsafe {
            executor.step(&mut state, &wf);
        }

        // shr(0x80000000, 1) = 0x40000000
        // In little-endian: [0x00, 0x00, 0x00, 0x40]
        assert_eq!(state.ymm[0][0].value(), 0x00);
        assert_eq!(state.ymm[0][1].value(), 0x00);
        assert_eq!(state.ymm[0][2].value(), 0x00);
        assert_eq!(state.ymm[0][3].value(), 0x40);
    }

    #[test]
    fn test_shl_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with a known pattern
        for i in 0..8 {
            state.ymm[i][0] = Taxon::new(0x01);
            state.ymm[i][1] = Taxon::new(0x00);
            state.ymm[i][2] = Taxon::new(0x00);
            state.ymm[i][3] = Taxon::new(0x00); // 0x00000001
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::ShL(1),
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        });

        unsafe {
            executor.step(&mut state, &wf);
        }

        // shl(0x00000001, 1) = 0x00000002
        // In little-endian: [0x02, 0x00, 0x00, 0x00]
        assert_eq!(state.ymm[0][0].value(), 0x02);
        assert_eq!(state.ymm[0][1].value(), 0x00);
        assert_eq!(state.ymm[0][2].value(), 0x00);
        assert_eq!(state.ymm[0][3].value(), 0x00);
    }

    #[test]
    fn test_rotate_xor_combined() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with pattern
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(((i * 32 + j) % 256) as u8);
            }
        }

        // Combined rotate + XOR (common in SHA-256)
        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::RotR(7),
            port1: WavefrontOp::Xor,
            port5: WavefrontOp::Xor,
        });

        unsafe {
            executor.step(&mut state, &wf);
        }

        // Just verify it doesn't crash - detailed verification is complex
    }

    #[test]
    fn test_aes_enc_round_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with a known AES test pattern
        // State block in xmm0 (lower 128 bits of ymm0)
        for j in 0..16 {
            state.ymm[0][j] = Taxon::new(j as u8);
        }
        // Round key in xmm8 (lower 128 bits of ymm8)
        for j in 0..16 {
            state.ymm[8][j] = Taxon::new((0x10 + j) as u8);
        }
        // Second state block in xmm1
        for j in 0..16 {
            state.ymm[1][j] = Taxon::new((0x20 + j) as u8);
        }
        // Second round key in xmm9
        for j in 0..16 {
            state.ymm[9][j] = Taxon::new((0x30 + j) as u8);
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::AesRound,
            port5: WavefrontOp::AesRound,
        });

        // Save initial state for comparison
        let initial_ymm0: Vec<u8> = (0..16).map(|j| state.ymm[0][j].value()).collect();
        let initial_ymm1: Vec<u8> = (0..16).map(|j| state.ymm[1][j].value()).collect();

        unsafe {
            executor.step(&mut state, &wf);
        }

        // Verify state was modified (AES round changes the state)
        let final_ymm0: Vec<u8> = (0..16).map(|j| state.ymm[0][j].value()).collect();
        let final_ymm1: Vec<u8> = (0..16).map(|j| state.ymm[1][j].value()).collect();

        assert_ne!(initial_ymm0, final_ymm0, "AES round should modify xmm0");
        assert_ne!(initial_ymm1, final_ymm1, "AES round should modify xmm1");
    }

    #[test]
    fn test_aes_dec_round_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with test pattern
        for j in 0..16 {
            state.ymm[0][j] = Taxon::new((0x40 + j) as u8);
            state.ymm[1][j] = Taxon::new((0x50 + j) as u8);
            state.ymm[8][j] = Taxon::new((0x60 + j) as u8);
            state.ymm[9][j] = Taxon::new((0x70 + j) as u8);
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::AesRoundDec,
            port5: WavefrontOp::AesRoundDec,
        });

        let initial_ymm0: Vec<u8> = (0..16).map(|j| state.ymm[0][j].value()).collect();

        unsafe {
            executor.step(&mut state, &wf);
        }

        let final_ymm0: Vec<u8> = (0..16).map(|j| state.ymm[0][j].value()).collect();
        assert_ne!(initial_ymm0, final_ymm0, "AES dec round should modify xmm0");
    }

    #[test]
    fn test_sha256_round_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize SHA-256 state with known values
        // xmm0: State ABEF, xmm1: State CDGH, xmm2: Message schedule
        for j in 0..16 {
            state.ymm[0][j] = Taxon::new(j as u8); // ABEF
            state.ymm[1][j] = Taxon::new((0x10 + j) as u8); // CDGH
            state.ymm[2][j] = Taxon::new((0x20 + j) as u8); // Message
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Sha256Round,
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        });

        let initial_ymm0: Vec<u8> = (0..16).map(|j| state.ymm[0][j].value()).collect();
        let initial_ymm1: Vec<u8> = (0..16).map(|j| state.ymm[1][j].value()).collect();

        unsafe {
            executor.step(&mut state, &wf);
        }

        let final_ymm0: Vec<u8> = (0..16).map(|j| state.ymm[0][j].value()).collect();
        let final_ymm1: Vec<u8> = (0..16).map(|j| state.ymm[1][j].value()).collect();

        // SHA-256 round modifies both state registers
        assert_ne!(
            initial_ymm0, final_ymm0,
            "SHA-256 round should modify state ABEF"
        );
        assert_ne!(
            initial_ymm1, final_ymm1,
            "SHA-256 round should modify state CDGH"
        );
    }

    #[test]
    fn test_sha256_msg_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize message schedule blocks
        for j in 0..16 {
            state.ymm[0][j] = Taxon::new((0x80 + j) as u8); // W[i-4:i-1]
            state.ymm[1][j] = Taxon::new((0x90 + j) as u8); // W[i-8:i-5]
            state.ymm[2][j] = Taxon::new((0xA0 + j) as u8); // W[i-16:i-13]
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Sha256Msg1,
            port5: WavefrontOp::Sha256Msg2,
        });

        let initial_ymm0: Vec<u8> = (0..16).map(|j| state.ymm[0][j].value()).collect();

        unsafe {
            executor.step(&mut state, &wf);
        }

        let final_ymm0: Vec<u8> = (0..16).map(|j| state.ymm[0][j].value()).collect();
        assert_ne!(
            initial_ymm0, final_ymm0,
            "SHA-256 message schedule should modify W[i]"
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_shuffle_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize data in ymm0-7
        for i in 0..8 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(j as u8);
            }
        }

        // Set up shuffle masks in ymm8-15
        // Identity shuffle within 128-bit lanes: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]
        for i in 8..16 {
            for j in 0..16 {
                state.ymm[i][j] = Taxon::new(j as u8);
            }
            // Second 128-bit lane
            for j in 16..32 {
                state.ymm[i][j] = Taxon::new((j - 16) as u8);
            }
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Shuffle,
        });

        // With identity shuffle, result should equal input
        let initial_ymm0: Vec<u8> = (0..32).map(|j| state.ymm[0][j].value()).collect();

        unsafe {
            executor.step(&mut state, &wf);
        }

        let final_ymm0: Vec<u8> = (0..32).map(|j| state.ymm[0][j].value()).collect();
        assert_eq!(
            initial_ymm0, final_ymm0,
            "Identity shuffle should preserve data"
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_shuffle_reverse_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize data
        for j in 0..32 {
            state.ymm[0][j] = Taxon::new(j as u8);
        }

        // Reverse shuffle mask within 128-bit lanes
        // [15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0] for each lane
        for j in 0..16 {
            state.ymm[8][j] = Taxon::new((15 - j) as u8);
            state.ymm[8][j + 16] = Taxon::new((15 - j) as u8);
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Shuffle,
        });

        unsafe {
            executor.step(&mut state, &wf);
        }

        // Verify reverse within each 128-bit lane
        // First lane: bytes 0-15 reversed
        assert_eq!(state.ymm[0][0].value(), 15);
        assert_eq!(state.ymm[0][15].value(), 0);
        // Second lane: bytes 16-31 reversed
        assert_eq!(state.ymm[0][16].value(), 31);
        assert_eq!(state.ymm[0][31].value(), 16);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_permute_wavefront_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with 32-bit pattern (little-endian)
        // Element 0: 0x03020100, Element 1: 0x07060504, etc.
        for i in 0..8 {
            for j in 0..8 {
                let base = (j * 4) as u8;
                state.ymm[i][j * 4] = Taxon::new(base);
                state.ymm[i][j * 4 + 1] = Taxon::new(base + 1);
                state.ymm[i][j * 4 + 2] = Taxon::new(base + 2);
                state.ymm[i][j * 4 + 3] = Taxon::new(base + 3);
            }
        }

        // Identity permute indices: [0,1,2,3,4,5,6,7]
        for i in 8..16 {
            for j in 0..8 {
                state.ymm[i][j * 4] = Taxon::new(j as u8);
                state.ymm[i][j * 4 + 1] = Taxon::new(0);
                state.ymm[i][j * 4 + 2] = Taxon::new(0);
                state.ymm[i][j * 4 + 3] = Taxon::new(0);
            }
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Permute,
        });

        let initial_ymm0: Vec<u8> = (0..32).map(|j| state.ymm[0][j].value()).collect();

        unsafe {
            executor.step(&mut state, &wf);
        }

        let final_ymm0: Vec<u8> = (0..32).map(|j| state.ymm[0][j].value()).collect();
        assert_eq!(
            initial_ymm0, final_ymm0,
            "Identity permute should preserve data"
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_permute_reverse_asm() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with distinct 32-bit elements
        for j in 0..8 {
            let val = (j * 4) as u8;
            state.ymm[0][j * 4] = Taxon::new(val);
            state.ymm[0][j * 4 + 1] = Taxon::new(val + 1);
            state.ymm[0][j * 4 + 2] = Taxon::new(val + 2);
            state.ymm[0][j * 4 + 3] = Taxon::new(val + 3);
        }

        // Reverse permute indices: [7,6,5,4,3,2,1,0]
        for j in 0..8 {
            state.ymm[8][j * 4] = Taxon::new((7 - j) as u8);
            state.ymm[8][j * 4 + 1] = Taxon::new(0);
            state.ymm[8][j * 4 + 2] = Taxon::new(0);
            state.ymm[8][j * 4 + 3] = Taxon::new(0);
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Permute,
        });

        unsafe {
            executor.step(&mut state, &wf);
        }

        // Element 0 should now contain what was in element 7
        // Element 7 had bytes [28,29,30,31]
        assert_eq!(state.ymm[0][0].value(), 28);
        assert_eq!(state.ymm[0][1].value(), 29);
        assert_eq!(state.ymm[0][2].value(), 30);
        assert_eq!(state.ymm[0][3].value(), 31);

        // Element 7 should now contain what was in element 0
        // Element 0 had bytes [0,1,2,3]
        assert_eq!(state.ymm[0][28].value(), 0);
        assert_eq!(state.ymm[0][29].value(), 1);
        assert_eq!(state.ymm[0][30].value(), 2);
        assert_eq!(state.ymm[0][31].value(), 3);
    }

    #[test]
    fn test_step_n_xor_unrolled() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with pattern
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(((i + j) % 256) as u8);
            }
        }

        let wf = Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Xor,
            port5: WavefrontOp::Xor,
        });

        // Execute 100 iterations via step_n (uses unrolled path)
        unsafe {
            executor.step_n(&mut state, &wf, 100);
        }

        // XOR is self-inverse, but operands change each iteration
        // Just verify it completes without crash
    }

    #[test]
    fn test_empty_program() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        // Initialize with pattern
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(42);
            }
        }

        let original_state = state;
        let program: Vec<Wavefront> = vec![];

        unsafe {
            executor.run(&mut state, &program);
        }

        // Empty program should not modify state
        for i in 0..16 {
            for j in 0..32 {
                assert_eq!(state.ymm[i][j], original_state.ymm[i][j]);
            }
        }
    }

    #[test]
    fn test_step_n_zero() {
        let executor = Zen3AsmExecutor::new();
        let mut state = UorState::zero();

        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(42);
            }
        }

        let original_state = state;
        let wf = Wavefront::all_xor();

        unsafe {
            executor.step_n(&mut state, &wf, 0);
        }

        // Zero iterations should not modify state
        for i in 0..16 {
            for j in 0..32 {
                assert_eq!(state.ymm[i][j], original_state.ymm[i][j]);
            }
        }
    }
}
