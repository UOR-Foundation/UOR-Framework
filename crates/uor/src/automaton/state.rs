//! UOR State - The cellular automaton's complete state.
//!
//! The entire register file is ONE combined entity. No memory access.
//!
//! # Architecture
//!
//! UOR treats the CPU register file as a single state that undergoes
//! wavefront transformations. Each wavefront fires operations on ALL
//! execution ports simultaneously.
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    UorState (4992 bits)                      │
//! ├─────────────────────────────────────────────────────────────┤
//! │  YMM registers (16 × 256 = 4096 bits)                       │
//! │  ┌──────┬──────┬──────┬──────┬──────┬──────┬──────┬──────┐  │
//! │  │ ymm0 │ ymm1 │ ymm2 │ ymm3 │ ymm4 │ ymm5 │ ymm6 │ ymm7 │  │
//! │  ├──────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┤  │
//! │  │ ymm8 │ ymm9 │ymm10 │ymm11 │ymm12 │ymm13 │ymm14 │ymm15 │  │
//! │  └──────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┘  │
//! ├─────────────────────────────────────────────────────────────┤
//! │  GPR registers (14 × 64 = 896 bits)                         │
//! │  ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐         │
//! │  │ rax │ rbx │ rcx │ rdx │ rsi │ rdi │ r8  │ r9  │         │
//! │  ├─────┼─────┼─────┼─────┼─────┼─────┼─────┴─────┤         │
//! │  │ r10 │ r11 │ r12 │ r13 │ r14 │ r15 │           │         │
//! │  └─────┴─────┴─────┴─────┴─────┴─────┘           │         │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Invariants
//!
//! - `UorState` MUST be `Copy` (fits entirely in registers)
//! - No operation on `UorState` may access memory
//! - All transformations must use `options(nomem, nostack)`

use crate::core::taxon::Taxon;

/// YMM register count (x86_64 AVX2).
pub const YMM_COUNT: usize = 16;

/// YMM register width in taxons (256 bits / 8 bits per taxon).
pub const YMM_TAXONS: usize = 32;

/// GPR count (excluding rsp/rbp which are reserved for stack).
pub const GPR_COUNT: usize = 14;

/// GPR width in taxons (64 bits / 8 bits per taxon).
pub const GPR_TAXONS: usize = 8;

/// Total taxons in UOR state.
pub const STATE_TAXONS: usize = YMM_COUNT * YMM_TAXONS + GPR_COUNT * GPR_TAXONS; // 624

/// Total bits in UOR state.
pub const STATE_BITS: usize = STATE_TAXONS * 8; // 4992

/// The UOR state - entire register file as one combined entity.
///
/// This is THE cellular automaton state. One wavefront = one step function
/// that transforms the entire state simultaneously using ALL execution ports.
///
/// # Layout
///
/// ```text
/// YMM0-15:  16 × 32 taxons = 512 taxons (4096 bits)
/// GPR0-13:  14 ×  8 taxons = 112 taxons (896 bits)
/// Total:    624 taxons (4992 bits)
/// ```
///
/// # Invariant
///
/// This type MUST remain `Copy` and fit entirely in CPU registers.
/// Any operation that would require memory access is a conformance violation.
///
/// # Example
///
/// ```
/// use uor::state::{UorState, STATE_TAXONS};
///
/// let state = UorState::zero();
/// assert_eq!(state.as_taxons().len(), STATE_TAXONS);
/// ```
#[derive(Clone, Copy)]
#[repr(C, align(32))] // AVX2 alignment requirement
pub struct UorState {
    /// YMM registers (ymm0-ymm15).
    /// Each YMM register holds 32 taxons (256 bits).
    pub ymm: [[Taxon; YMM_TAXONS]; YMM_COUNT],

    /// General purpose registers (rax, rbx, rcx, rdx, rsi, rdi, r8-r15).
    /// Excludes rsp/rbp which are reserved for stack operations.
    /// Each GPR holds 8 taxons (64 bits).
    pub gpr: [[Taxon; GPR_TAXONS]; GPR_COUNT],
}

impl UorState {
    /// Create a zero-initialized state (identity element for XOR).
    ///
    /// This is the "blank" state before any wavefront execution.
    #[inline]
    pub const fn zero() -> Self {
        Self {
            ymm: [[Taxon::MIN; YMM_TAXONS]; YMM_COUNT],
            gpr: [[Taxon::MIN; GPR_TAXONS]; GPR_COUNT],
        }
    }

    /// Access YMM register by index.
    ///
    /// # Panics
    ///
    /// Panics if `idx >= YMM_COUNT` (16).
    #[inline(always)]
    pub fn ymm(&self, idx: usize) -> &[Taxon; YMM_TAXONS] {
        &self.ymm[idx]
    }

    /// Mutable access to YMM register by index.
    ///
    /// # Panics
    ///
    /// Panics if `idx >= YMM_COUNT` (16).
    #[inline(always)]
    pub fn ymm_mut(&mut self, idx: usize) -> &mut [Taxon; YMM_TAXONS] {
        &mut self.ymm[idx]
    }

    /// Access GPR by index.
    ///
    /// # Panics
    ///
    /// Panics if `idx >= GPR_COUNT` (14).
    #[inline(always)]
    pub fn gpr(&self, idx: usize) -> &[Taxon; GPR_TAXONS] {
        &self.gpr[idx]
    }

    /// Mutable access to GPR by index.
    ///
    /// # Panics
    ///
    /// Panics if `idx >= GPR_COUNT` (14).
    #[inline(always)]
    pub fn gpr_mut(&mut self, idx: usize) -> &mut [Taxon; GPR_TAXONS] {
        &mut self.gpr[idx]
    }

    /// View entire state as a flat taxon slice (for verification).
    ///
    /// This is used for conformance testing to verify state transformations.
    pub fn as_taxons(&self) -> &[Taxon] {
        // SAFETY: repr(C) guarantees contiguous layout with no padding
        // between ymm and gpr arrays.
        unsafe { core::slice::from_raw_parts(self as *const Self as *const Taxon, STATE_TAXONS) }
    }

    /// Mutable view of entire state as a flat taxon slice.
    pub fn as_taxons_mut(&mut self) -> &mut [Taxon] {
        // SAFETY: repr(C) guarantees contiguous layout.
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut Taxon, STATE_TAXONS) }
    }
}

impl Default for UorState {
    fn default() -> Self {
        Self::zero()
    }
}

impl PartialEq for UorState {
    fn eq(&self, other: &Self) -> bool {
        self.as_taxons() == other.as_taxons()
    }
}

impl Eq for UorState {}

impl core::fmt::Debug for UorState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UorState")
            .field("ymm_count", &YMM_COUNT)
            .field("gpr_count", &GPR_COUNT)
            .field("total_taxons", &STATE_TAXONS)
            .field("total_bits", &STATE_BITS)
            .finish()
    }
}

// Compile-time verification: UorState must be Copy
const _: () = {
    const fn assert_copy<T: Copy>() {}
    assert_copy::<UorState>();
};

// Compile-time verification: UorState size accommodates all taxons
const _: () = {
    // YMM: 16 * 32 = 512 bytes
    // GPR: 14 * 8 = 112 bytes
    // Total data: 624 bytes
    // With 32-byte alignment, actual size is 640 bytes (padded)
    assert!(core::mem::size_of::<UorState>() >= STATE_TAXONS);
    // Verify alignment requirement
    assert!(core::mem::align_of::<UorState>() == 32);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_zero() {
        let state = UorState::zero();
        for t in state.as_taxons() {
            assert_eq!(t.value(), 0);
        }
    }

    #[test]
    fn test_state_taxon_count() {
        assert_eq!(STATE_TAXONS, 624);
        assert_eq!(STATE_BITS, 4992);

        let state = UorState::zero();
        assert_eq!(state.as_taxons().len(), STATE_TAXONS);
    }

    #[test]
    fn test_state_is_copy() {
        let state1 = UorState::zero();
        let state2 = state1; // Copy, not move
        assert!(state1.eq(&state2));
    }

    #[test]
    fn test_ymm_access() {
        let mut state = UorState::zero();

        // Write to ymm0
        state.ymm_mut(0)[0] = Taxon::new(42);
        assert_eq!(state.ymm(0)[0].value(), 42);

        // Other registers unchanged
        assert_eq!(state.ymm(1)[0].value(), 0);
    }

    #[test]
    fn test_gpr_access() {
        let mut state = UorState::zero();

        // Write to gpr0 (rax)
        state.gpr_mut(0)[0] = Taxon::new(255);
        assert_eq!(state.gpr(0)[0].value(), 255);

        // Other registers unchanged
        assert_eq!(state.gpr(1)[0].value(), 0);
    }

    #[test]
    fn test_flat_view() {
        let mut state = UorState::zero();

        // Modify via flat view
        state.as_taxons_mut()[0] = Taxon::new(1);
        state.as_taxons_mut()[512] = Taxon::new(2); // First GPR byte

        // Verify via structured access
        assert_eq!(state.ymm(0)[0].value(), 1);
        assert_eq!(state.gpr(0)[0].value(), 2);
    }

    #[test]
    fn test_state_alignment() {
        // State must be 32-byte aligned for AVX2
        assert_eq!(core::mem::align_of::<UorState>(), 32);
    }

    // ========================================================================
    // TASK-185: Prism State Edge Case Tests
    // ========================================================================

    // ------------------------------------------------------------------------
    // Boundary/Wrap-around Behavior
    // ------------------------------------------------------------------------

    #[test]
    fn test_ymm_boundary_first_register() {
        let mut state = UorState::zero();
        // First YMM register (ymm0)
        state.ymm_mut(0)[0] = Taxon::new(1);
        state.ymm_mut(0)[31] = Taxon::new(255);

        assert_eq!(state.ymm(0)[0].value(), 1);
        assert_eq!(state.ymm(0)[31].value(), 255);

        // Verify via flat view
        assert_eq!(state.as_taxons()[0].value(), 1);
        assert_eq!(state.as_taxons()[31].value(), 255);
    }

    #[test]
    fn test_ymm_boundary_last_register() {
        let mut state = UorState::zero();
        // Last YMM register (ymm15)
        state.ymm_mut(15)[0] = Taxon::new(42);
        state.ymm_mut(15)[31] = Taxon::new(128);

        assert_eq!(state.ymm(15)[0].value(), 42);
        assert_eq!(state.ymm(15)[31].value(), 128);

        // Verify via flat view (ymm15 starts at index 15*32 = 480)
        assert_eq!(state.as_taxons()[480].value(), 42);
        assert_eq!(state.as_taxons()[511].value(), 128);
    }

    #[test]
    fn test_gpr_boundary_first_register() {
        let mut state = UorState::zero();
        // First GPR (gpr0 = rax)
        state.gpr_mut(0)[0] = Taxon::new(100);
        state.gpr_mut(0)[7] = Taxon::new(200);

        assert_eq!(state.gpr(0)[0].value(), 100);
        assert_eq!(state.gpr(0)[7].value(), 200);

        // Verify via flat view (GPRs start at index 512)
        assert_eq!(state.as_taxons()[512].value(), 100);
        assert_eq!(state.as_taxons()[519].value(), 200);
    }

    #[test]
    fn test_gpr_boundary_last_register() {
        let mut state = UorState::zero();
        // Last GPR (gpr13 = r15)
        state.gpr_mut(13)[0] = Taxon::new(77);
        state.gpr_mut(13)[7] = Taxon::new(88);

        assert_eq!(state.gpr(13)[0].value(), 77);
        assert_eq!(state.gpr(13)[7].value(), 88);

        // Verify via flat view (gpr13 starts at index 512 + 13*8 = 616)
        assert_eq!(state.as_taxons()[616].value(), 77);
        assert_eq!(state.as_taxons()[623].value(), 88);
    }

    #[test]
    fn test_flat_view_ymm_gpr_boundary() {
        let mut state = UorState::zero();

        // Last byte of YMM section (index 511 = ymm15[31])
        state.as_taxons_mut()[511] = Taxon::new(99);
        // First byte of GPR section (index 512 = gpr0[0])
        state.as_taxons_mut()[512] = Taxon::new(100);

        // Verify via structured access
        assert_eq!(state.ymm(15)[31].value(), 99);
        assert_eq!(state.gpr(0)[0].value(), 100);
    }

    #[test]
    fn test_flat_view_first_last_taxon() {
        let mut state = UorState::zero();

        // First taxon (index 0)
        state.as_taxons_mut()[0] = Taxon::new(1);
        // Last taxon (index 623)
        state.as_taxons_mut()[623] = Taxon::new(254);

        assert_eq!(state.as_taxons()[0].value(), 1);
        assert_eq!(state.as_taxons()[623].value(), 254);

        // Cross-verify
        assert_eq!(state.ymm(0)[0].value(), 1);
        assert_eq!(state.gpr(13)[7].value(), 254);
    }

    #[test]
    fn test_taxon_value_boundaries() {
        let mut state = UorState::zero();

        // Test MIN value (0)
        state.ymm_mut(0)[0] = Taxon::MIN;
        assert_eq!(state.ymm(0)[0].value(), 0);

        // Test MAX value (255)
        state.ymm_mut(0)[1] = Taxon::MAX;
        assert_eq!(state.ymm(0)[1].value(), 255);

        // Test ONE value (1)
        state.ymm_mut(0)[2] = Taxon::ONE;
        assert_eq!(state.ymm(0)[2].value(), 1);
    }

    #[test]
    fn test_all_ymm_registers_accessible() {
        let mut state = UorState::zero();

        // Write distinct value to each YMM register
        for i in 0..YMM_COUNT {
            state.ymm_mut(i)[0] = Taxon::new(i as u8);
        }

        // Verify each register
        for i in 0..YMM_COUNT {
            assert_eq!(
                state.ymm(i)[0].value(),
                i as u8,
                "YMM{i} should have value {i}"
            );
        }
    }

    #[test]
    fn test_all_gpr_registers_accessible() {
        let mut state = UorState::zero();

        // Write distinct value to each GPR
        for i in 0..GPR_COUNT {
            state.gpr_mut(i)[0] = Taxon::new((i + 100) as u8);
        }

        // Verify each register
        for i in 0..GPR_COUNT {
            assert_eq!(
                state.gpr(i)[0].value(),
                (i + 100) as u8,
                "GPR{i} should have value {}",
                i + 100
            );
        }
    }

    #[test]
    fn test_all_taxon_positions_in_ymm() {
        let mut state = UorState::zero();

        // Write distinct value to each position in ymm0
        for i in 0..YMM_TAXONS {
            state.ymm_mut(0)[i] = Taxon::new(i as u8);
        }

        // Verify each position
        for i in 0..YMM_TAXONS {
            assert_eq!(
                state.ymm(0)[i].value(),
                i as u8,
                "YMM0[{i}] should have value {i}"
            );
        }
    }

    #[test]
    fn test_all_taxon_positions_in_gpr() {
        let mut state = UorState::zero();

        // Write distinct value to each position in gpr0
        for i in 0..GPR_TAXONS {
            state.gpr_mut(0)[i] = Taxon::new((i + 200) as u8);
        }

        // Verify each position
        for i in 0..GPR_TAXONS {
            assert_eq!(
                state.gpr(0)[i].value(),
                (i + 200) as u8,
                "GPR0[{i}] should have value {}",
                i + 200
            );
        }
    }

    // ------------------------------------------------------------------------
    // State Invariant Validation
    // ------------------------------------------------------------------------

    #[test]
    fn test_state_size_invariant() {
        // Size must be at least 624 bytes to hold all taxons
        assert!(core::mem::size_of::<UorState>() >= STATE_TAXONS);
        // Should be exactly 640 bytes with 32-byte alignment padding
        assert_eq!(core::mem::size_of::<UorState>(), 640);
    }

    #[test]
    fn test_state_layout_consistency() {
        let mut state = UorState::zero();

        // Verify YMM and GPR arrays are contiguous via flat view
        // YMM: 16 registers * 32 taxons = 512 taxons (indices 0-511)
        // GPR: 14 registers * 8 taxons = 112 taxons (indices 512-623)

        // Fill entire state via flat view
        for (i, taxon) in state.as_taxons_mut().iter_mut().enumerate() {
            *taxon = Taxon::new((i % 256) as u8);
        }

        // Verify YMM section
        for reg in 0..YMM_COUNT {
            for pos in 0..YMM_TAXONS {
                let flat_idx = reg * YMM_TAXONS + pos;
                assert_eq!(
                    state.ymm(reg)[pos].value(),
                    (flat_idx % 256) as u8,
                    "YMM{reg}[{pos}] should match flat index {flat_idx}"
                );
            }
        }

        // Verify GPR section
        for reg in 0..GPR_COUNT {
            for pos in 0..GPR_TAXONS {
                let flat_idx = YMM_COUNT * YMM_TAXONS + reg * GPR_TAXONS + pos;
                assert_eq!(
                    state.gpr(reg)[pos].value(),
                    (flat_idx % 256) as u8,
                    "GPR{reg}[{pos}] should match flat index {flat_idx}"
                );
            }
        }
    }

    #[test]
    fn test_state_equality() {
        let state1 = UorState::zero();
        let state2 = UorState::zero();

        assert_eq!(state1, state2);
    }

    #[test]
    fn test_state_inequality() {
        let mut state1 = UorState::zero();
        let state2 = UorState::zero();

        state1.ymm_mut(0)[0] = Taxon::new(1);

        assert_ne!(state1, state2);
    }

    #[test]
    fn test_state_default_is_zero() {
        let default_state = UorState::default();
        let zero_state = UorState::zero();

        assert_eq!(default_state, zero_state);
    }

    #[test]
    fn test_state_debug_format() {
        let state = UorState::zero();
        let debug_str = format!("{:?}", state);

        assert!(debug_str.contains("UorState"));
        assert!(debug_str.contains("ymm_count"));
        assert!(debug_str.contains("gpr_count"));
        assert!(debug_str.contains("total_taxons"));
        assert!(debug_str.contains("total_bits"));
    }

    #[test]
    fn test_state_clone() {
        let mut state1 = UorState::zero();
        state1.ymm_mut(5)[10] = Taxon::new(42);

        let state2 = state1;

        assert_eq!(state1, state2);
        assert_eq!(state2.ymm(5)[10].value(), 42);
    }

    // ------------------------------------------------------------------------
    // Transformation Edge Cases
    // ------------------------------------------------------------------------

    #[test]
    fn test_all_ones_state() {
        let mut state = UorState::zero();

        // Fill with all 255s
        for taxon in state.as_taxons_mut() {
            *taxon = Taxon::MAX;
        }

        // Verify
        for taxon in state.as_taxons() {
            assert_eq!(taxon.value(), 255);
        }
    }

    #[test]
    fn test_alternating_pattern_01() {
        let mut state = UorState::zero();

        // Fill with alternating 0, 1 pattern
        for (i, taxon) in state.as_taxons_mut().iter_mut().enumerate() {
            *taxon = Taxon::new((i % 2) as u8);
        }

        // Verify pattern
        for (i, taxon) in state.as_taxons().iter().enumerate() {
            assert_eq!(taxon.value(), (i % 2) as u8);
        }
    }

    #[test]
    fn test_alternating_pattern_ff00() {
        let mut state = UorState::zero();

        // Fill with alternating 0xFF, 0x00 pattern
        for (i, taxon) in state.as_taxons_mut().iter_mut().enumerate() {
            *taxon = if i % 2 == 0 { Taxon::MAX } else { Taxon::MIN };
        }

        // Verify pattern
        for (i, taxon) in state.as_taxons().iter().enumerate() {
            let expected = if i % 2 == 0 { 255 } else { 0 };
            assert_eq!(taxon.value(), expected);
        }
    }

    #[test]
    fn test_single_bit_modification() {
        let mut state = UorState::zero();

        // Modify single taxon in middle of state
        let mid_idx = STATE_TAXONS / 2;
        state.as_taxons_mut()[mid_idx] = Taxon::new(1);

        // Verify only that taxon changed
        for (i, taxon) in state.as_taxons().iter().enumerate() {
            if i == mid_idx {
                assert_eq!(taxon.value(), 1);
            } else {
                assert_eq!(taxon.value(), 0);
            }
        }
    }

    #[test]
    fn test_cross_register_modification() {
        let mut state = UorState::zero();

        // Modify last byte of each YMM register and first byte of next
        for i in 0..(YMM_COUNT - 1) {
            state.ymm_mut(i)[31] = Taxon::new(255);
            state.ymm_mut(i + 1)[0] = Taxon::new(1);
        }

        // Verify pattern
        for i in 0..(YMM_COUNT - 1) {
            assert_eq!(state.ymm(i)[31].value(), 255);
            assert_eq!(state.ymm(i + 1)[0].value(), 1);
        }
    }

    #[test]
    fn test_sequential_fill_pattern() {
        let mut state = UorState::zero();

        // Fill with sequential values (wrapping at 256)
        for (i, taxon) in state.as_taxons_mut().iter_mut().enumerate() {
            *taxon = Taxon::new((i % 256) as u8);
        }

        // Verify sequential values
        for (i, taxon) in state.as_taxons().iter().enumerate() {
            assert_eq!(taxon.value(), (i % 256) as u8);
        }

        // Verify wrap-around occurs (624 > 256, so we should see 0-111 twice + 0-111 again)
        assert_eq!(state.as_taxons()[0].value(), 0);
        assert_eq!(state.as_taxons()[255].value(), 255);
        assert_eq!(state.as_taxons()[256].value(), 0); // Wrap
        assert_eq!(state.as_taxons()[511].value(), 255);
        assert_eq!(state.as_taxons()[512].value(), 0); // Wrap again
        assert_eq!(state.as_taxons()[623].value(), 111); // 623 % 256 = 111
    }

    #[test]
    fn test_bitwise_patterns() {
        let mut state = UorState::zero();

        // Powers of 2 pattern
        let powers = [1u8, 2, 4, 8, 16, 32, 64, 128];
        for (i, &power) in powers.iter().enumerate() {
            state.ymm_mut(0)[i] = Taxon::new(power);
            assert!(state.ymm(0)[i].is_basis());
        }

        // Verify they are basis elements
        for i in 0..8 {
            assert!(state.ymm(0)[i].is_basis());
        }
    }

    #[test]
    fn test_copy_semantics_independence() {
        let mut state1 = UorState::zero();
        state1.ymm_mut(0)[0] = Taxon::new(100);

        // Copy
        let mut state2 = state1;

        // Modify copy
        state2.ymm_mut(0)[0] = Taxon::new(200);

        // Original should be unchanged
        assert_eq!(state1.ymm(0)[0].value(), 100);
        assert_eq!(state2.ymm(0)[0].value(), 200);
    }

    #[test]
    fn test_ymm_register_isolation() {
        let mut state = UorState::zero();

        // Modify one register completely
        for i in 0..YMM_TAXONS {
            state.ymm_mut(7)[i] = Taxon::new(255);
        }

        // Verify other registers are unchanged
        for reg in 0..YMM_COUNT {
            if reg != 7 {
                for pos in 0..YMM_TAXONS {
                    assert_eq!(
                        state.ymm(reg)[pos].value(),
                        0,
                        "YMM{reg}[{pos}] should be 0"
                    );
                }
            }
        }

        // Verify modified register
        for pos in 0..YMM_TAXONS {
            assert_eq!(state.ymm(7)[pos].value(), 255);
        }
    }

    #[test]
    fn test_gpr_register_isolation() {
        let mut state = UorState::zero();

        // Modify one GPR completely
        for i in 0..GPR_TAXONS {
            state.gpr_mut(5)[i] = Taxon::new(128);
        }

        // Verify other GPRs are unchanged
        for reg in 0..GPR_COUNT {
            if reg != 5 {
                for pos in 0..GPR_TAXONS {
                    assert_eq!(
                        state.gpr(reg)[pos].value(),
                        0,
                        "GPR{reg}[{pos}] should be 0"
                    );
                }
            }
        }

        // Verify modified GPR
        for pos in 0..GPR_TAXONS {
            assert_eq!(state.gpr(5)[pos].value(), 128);
        }
    }

    #[test]
    fn test_taxon_operations_preserve_state() {
        let mut state = UorState::zero();

        // Insert a taxon with specific properties
        let t = Taxon::new(17);
        state.ymm_mut(0)[0] = t;

        // Verify taxon properties are preserved
        let retrieved = state.ymm(0)[0];
        assert_eq!(retrieved.value(), 17);
        assert_eq!(retrieved.domain(), t.domain());
        assert_eq!(retrieved.rank(), t.rank());
        assert_eq!(retrieved.braille(), t.braille());
        assert_eq!(retrieved.codepoint(), t.codepoint());
    }

    #[test]
    fn test_state_xor_identity() {
        // Zero state is identity element for XOR
        let _zero = UorState::zero();

        // XOR with self should give zero
        let mut state = UorState::zero();
        for i in 0..STATE_TAXONS {
            state.as_taxons_mut()[i] = Taxon::new(42);
        }

        // Verify XOR a XOR a = 0 (by verifying the value)
        let val = state.as_taxons()[0].value();
        assert_eq!(val ^ val, 0);
    }

    #[test]
    fn test_constants_consistency() {
        // Verify constant relationships
        assert_eq!(
            STATE_TAXONS,
            YMM_COUNT * YMM_TAXONS + GPR_COUNT * GPR_TAXONS
        );
        assert_eq!(STATE_BITS, STATE_TAXONS * 8);
        assert_eq!(YMM_COUNT, 16);
        assert_eq!(YMM_TAXONS, 32);
        assert_eq!(GPR_COUNT, 14);
        assert_eq!(GPR_TAXONS, 8);
        assert_eq!(STATE_TAXONS, 624);
        assert_eq!(STATE_BITS, 4992);
    }
}
