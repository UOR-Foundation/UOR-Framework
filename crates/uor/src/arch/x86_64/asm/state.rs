//! State loading and storing for inline assembly execution.
//!
//! These functions transfer UorState between memory and CPU registers.
//! Load is performed once at program start, store once at program end.

use crate::state::UorState;
use core::arch::asm;

/// Load all 16 YMM registers from UorState.
///
/// After this call, physical registers contain:
/// - YMM0 = state.ymm[0]
/// - YMM1 = state.ymm[1]
/// - ... etc through YMM15
///
/// # Safety
///
/// - Caller must ensure AVX2 is available
/// - Caller must not call any function that clobbers YMM registers
///   before calling `store_ymm_state`
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn load_ymm_state(state: &UorState) {
    let ptr = state.ymm.as_ptr() as *const u8;

    asm!(
        // Load all 16 YMM registers from contiguous memory
        // Each YMM register is 32 bytes, offsets: 0, 32, 64, ...
        "vmovdqu ymm0, [{ptr}]",
        "vmovdqu ymm1, [{ptr} + 32]",
        "vmovdqu ymm2, [{ptr} + 64]",
        "vmovdqu ymm3, [{ptr} + 96]",
        "vmovdqu ymm4, [{ptr} + 128]",
        "vmovdqu ymm5, [{ptr} + 160]",
        "vmovdqu ymm6, [{ptr} + 192]",
        "vmovdqu ymm7, [{ptr} + 224]",
        "vmovdqu ymm8, [{ptr} + 256]",
        "vmovdqu ymm9, [{ptr} + 288]",
        "vmovdqu ymm10, [{ptr} + 320]",
        "vmovdqu ymm11, [{ptr} + 352]",
        "vmovdqu ymm12, [{ptr} + 384]",
        "vmovdqu ymm13, [{ptr} + 416]",
        "vmovdqu ymm14, [{ptr} + 448]",
        "vmovdqu ymm15, [{ptr} + 480]",
        ptr = in(reg) ptr,
        // All YMM registers are clobbered (we're loading into them)
        // Use lateout to indicate they're written
        lateout("ymm0") _,
        lateout("ymm1") _,
        lateout("ymm2") _,
        lateout("ymm3") _,
        lateout("ymm4") _,
        lateout("ymm5") _,
        lateout("ymm6") _,
        lateout("ymm7") _,
        lateout("ymm8") _,
        lateout("ymm9") _,
        lateout("ymm10") _,
        lateout("ymm11") _,
        lateout("ymm12") _,
        lateout("ymm13") _,
        lateout("ymm14") _,
        lateout("ymm15") _,
        options(nostack, preserves_flags)
    );
}

/// Store destination YMM registers (ymm0-7) back to UorState.
///
/// Only stores the first 8 registers since those are the destination
/// registers in the UOR pairing scheme (ymm[i] op= ymm[i+8]).
///
/// # Safety
///
/// - Must be called after `load_ymm_state` and wavefront execution
/// - Caller must ensure registers haven't been clobbered by other code
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn store_ymm_state(state: &mut UorState) {
    let ptr = state.ymm.as_mut_ptr() as *mut u8;

    // We need to tell LLVM that ymm0-7 are used as inputs.
    // Since we don't have Rust variables holding the values,
    // we use the clobber_abi approach with explicit instructions.
    asm!(
        // Store only destination registers (ymm0-7)
        // These are the registers modified by wavefront operations
        "vmovdqu [{ptr}], ymm0",
        "vmovdqu [{ptr} + 32], ymm1",
        "vmovdqu [{ptr} + 64], ymm2",
        "vmovdqu [{ptr} + 96], ymm3",
        "vmovdqu [{ptr} + 128], ymm4",
        "vmovdqu [{ptr} + 160], ymm5",
        "vmovdqu [{ptr} + 192], ymm6",
        "vmovdqu [{ptr} + 224], ymm7",
        ptr = in(reg) ptr,
        // Note: We can't specify ymm0-7 as "in" constraints without values.
        // The registers implicitly contain the values from previous asm blocks.
        // LLVM will see the register references in the asm string.
        options(nostack, preserves_flags)
    );
}

/// Store all 16 YMM registers back to UorState.
///
/// Use this when all registers may have been modified (rare).
///
/// # Safety
///
/// Same requirements as `store_ymm_state`.
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn store_ymm_state_all(state: &mut UorState) {
    let ptr = state.ymm.as_mut_ptr() as *mut u8;

    asm!(
        // Store all 16 YMM registers
        "vmovdqu [{ptr}], ymm0",
        "vmovdqu [{ptr} + 32], ymm1",
        "vmovdqu [{ptr} + 64], ymm2",
        "vmovdqu [{ptr} + 96], ymm3",
        "vmovdqu [{ptr} + 128], ymm4",
        "vmovdqu [{ptr} + 160], ymm5",
        "vmovdqu [{ptr} + 192], ymm6",
        "vmovdqu [{ptr} + 224], ymm7",
        "vmovdqu [{ptr} + 256], ymm8",
        "vmovdqu [{ptr} + 288], ymm9",
        "vmovdqu [{ptr} + 320], ymm10",
        "vmovdqu [{ptr} + 352], ymm11",
        "vmovdqu [{ptr} + 384], ymm12",
        "vmovdqu [{ptr} + 416], ymm13",
        "vmovdqu [{ptr} + 448], ymm14",
        "vmovdqu [{ptr} + 480], ymm15",
        ptr = in(reg) ptr,
        options(nostack, preserves_flags)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Taxon;

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_load_store_roundtrip() {
        let mut state = UorState::zero();

        // Initialize with pattern
        for i in 0..16 {
            for j in 0..32 {
                state.ymm[i][j] = Taxon::new(((i * 32 + j) % 256) as u8);
            }
        }

        let original = state;

        // Load into registers, then store back
        unsafe {
            load_ymm_state(&state);
            store_ymm_state_all(&mut state);
        }

        // Verify roundtrip
        for i in 0..16 {
            for j in 0..32 {
                assert_eq!(
                    state.ymm[i][j], original.ymm[i][j],
                    "Mismatch at ymm[{}][{}]",
                    i, j
                );
            }
        }
    }
}
