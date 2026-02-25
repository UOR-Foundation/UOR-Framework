//! UOR Instruction Set Architecture
//!
//! Abstract operations that map to hardware wavefronts.
//! Each operation fires on ALL execution ports simultaneously.
//!
//! # Execution Model
//!
//! UOR is a cellular automaton where:
//! - State = entire register file (624 taxons = 4992 bits)
//! - Instruction = wavefront (all ports fire simultaneously)
//! - Step = one wavefront cycle
//!
//! # Port Mapping (AMD Zen 3)
//!
//! ```text
//! Port 0: Shift, Rotate, SHA-NI (sha256rnds2)
//! Port 1: ALU, AES-NI (aesenc)
//! Port 5: ALU, AES-NI (aesenc)
//! ```
//!
//! A wavefront utilizes ALL three ports in a single cycle.
//!
//! # Safety Contract
//!
//! All implementations of `UorStep` MUST:
//! 1. Use `options(nomem, nostack)` on all inline assembly
//! 2. Never access memory outside the state parameter
//! 3. Complete in bounded time (no data-dependent loops)

use super::state::UorState;

/// Wavefront operation - fires on a specific execution port.
///
/// Operations are classified by which port can execute them:
/// - Port 0: Shift, rotate, SHA-NI
/// - Port 1/5: ALU (logic, add/sub), AES-NI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WavefrontOp {
    /// No operation (identity transformation).
    #[default]
    Nop,

    // === ALU Operations (Ports 1/5) ===
    /// Bitwise XOR: a ^ b
    Xor,
    /// Bitwise AND: a & b
    And,
    /// Bitwise OR: a | b
    Or,
    /// Bitwise NOT: !a
    Not,
    /// Wrapping addition: a + b (mod 2^n)
    Add,
    /// Wrapping subtraction: a - b (mod 2^n)
    Sub,

    // === Shift/Rotate Operations (Port 0) ===
    /// Rotate left by n bits (per 32-bit lane).
    RotL(u8),
    /// Rotate right by n bits (per 32-bit lane).
    RotR(u8),
    /// Shift left by n bits (per 32-bit lane).
    ShL(u8),
    /// Shift right by n bits (per 32-bit lane).
    ShR(u8),

    // === Crypto Operations ===
    /// SHA-256 round via SHA-NI (Port 0).
    /// Executes `sha256rnds2` instruction.
    Sha256Round,
    /// SHA-256 message schedule part 1 (Ports 1/5).
    /// Executes `sha256msg1` instruction.
    Sha256Msg1,
    /// SHA-256 message schedule part 2 (Ports 1/5).
    /// Executes `sha256msg2` instruction.
    Sha256Msg2,
    /// AES encryption round via AES-NI (Ports 1/5).
    /// Executes `aesenc` instruction.
    AesRound,
    /// AES decryption round via AES-NI (Ports 1/5).
    /// Executes `aesdec` instruction.
    AesRoundDec,

    // === Permutation Operations (Port 5) ===
    /// Byte shuffle/permute within 128-bit lanes.
    Shuffle,
    /// Byte shuffle/permute across 256-bit register.
    Permute,
}

impl WavefrontOp {
    /// Returns true if this operation can execute on Port 0.
    #[inline]
    pub const fn is_port0(&self) -> bool {
        matches!(
            self,
            Self::Nop
                | Self::RotL(_)
                | Self::RotR(_)
                | Self::ShL(_)
                | Self::ShR(_)
                | Self::Sha256Round
        )
    }

    /// Returns true if this operation can execute on Port 1.
    #[inline]
    pub const fn is_port1(&self) -> bool {
        matches!(
            self,
            Self::Nop
                | Self::Xor
                | Self::And
                | Self::Or
                | Self::Not
                | Self::Add
                | Self::Sub
                | Self::AesRound
                | Self::AesRoundDec
                | Self::Sha256Msg1
                | Self::Sha256Msg2
        )
    }

    /// Returns true if this operation can execute on Port 5.
    #[inline]
    pub const fn is_port5(&self) -> bool {
        matches!(
            self,
            Self::Nop
                | Self::Xor
                | Self::And
                | Self::Or
                | Self::Not
                | Self::Add
                | Self::Sub
                | Self::AesRound
                | Self::AesRoundDec
                | Self::Sha256Msg1
                | Self::Sha256Msg2
                | Self::Shuffle
                | Self::Permute
        )
    }

    /// Returns true if this operation is invertible without complement.
    ///
    /// Invertible operations can be reversed using only the result:
    /// - XOR, NOT: Self-inverse
    /// - RotL/RotR: Inverse is opposite rotation
    /// - ADD/SUB: Inverse is SUB/ADD
    /// - AesRound: AesRoundDec is inverse
    #[inline]
    pub const fn is_invertible(&self) -> bool {
        matches!(
            self,
            Self::Nop
                | Self::Xor
                | Self::Not
                | Self::Add
                | Self::Sub
                | Self::RotL(_)
                | Self::RotR(_)
                | Self::AesRound
                | Self::AesRoundDec
                | Self::Shuffle
                | Self::Permute
        )
    }

    /// Returns true if this operation requires complement for inversion.
    ///
    /// These operations lose information that must be tracked for lossless codec:
    /// - ShL/ShR: Bits shifted out
    /// - AND/OR: Masked/overwritten bits
    /// - SHA256Round: Pre-compression state
    #[inline]
    pub const fn requires_complement(&self) -> bool {
        matches!(
            self,
            Self::ShL(_) | Self::ShR(_) | Self::And | Self::Or | Self::Sha256Round
        )
    }
}

/// Port assignment for one wavefront cycle.
///
/// ALL ports fire in the same cycle - this is the "wavefront".
/// Operations must be compatible with their assigned ports.
///
/// # Example
///
/// ```
/// use uor::isa::{PortAssignment, WavefrontOp};
///
/// // SHA-256 round pattern: rotate + XOR + XOR
/// let ports = PortAssignment::rotate_and_xor(7);
/// assert!(ports.port0.is_port0());
/// assert!(ports.port1.is_port1());
/// assert!(ports.port5.is_port5());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PortAssignment {
    /// Port 0 operation (shift/rotate/SHA-NI).
    pub port0: WavefrontOp,
    /// Port 1 operation (ALU/AES-NI).
    pub port1: WavefrontOp,
    /// Port 5 operation (ALU/AES-NI/shuffle).
    pub port5: WavefrontOp,
}

impl PortAssignment {
    /// All ports idle (identity wavefront).
    pub const fn nop() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        }
    }

    /// All ALU ports perform XOR.
    pub const fn all_xor() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Xor,
            port5: WavefrontOp::Xor,
        }
    }

    /// All ALU ports perform AND.
    pub const fn all_and() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::And,
            port5: WavefrontOp::And,
        }
    }

    /// All ALU ports perform OR.
    pub const fn all_or() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Or,
            port5: WavefrontOp::Or,
        }
    }

    /// All ALU ports perform ADD.
    pub const fn all_add() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Add,
            port5: WavefrontOp::Add,
        }
    }

    /// Port 0 rotates right, Ports 1/5 XOR.
    /// Common in SHA-256 Σ functions.
    pub const fn rotate_and_xor(n: u8) -> Self {
        Self {
            port0: WavefrontOp::RotR(n),
            port1: WavefrontOp::Xor,
            port5: WavefrontOp::Xor,
        }
    }

    /// Port 0 shifts right, Ports 1/5 XOR.
    /// Common in SHA-256 σ functions.
    pub const fn shift_and_xor(n: u8) -> Self {
        Self {
            port0: WavefrontOp::ShR(n),
            port1: WavefrontOp::Xor,
            port5: WavefrontOp::Xor,
        }
    }

    /// SHA-256 round pattern: SHA-NI round + message schedule.
    pub const fn sha256_round() -> Self {
        Self {
            port0: WavefrontOp::Sha256Round,
            port1: WavefrontOp::Sha256Msg1,
            port5: WavefrontOp::Sha256Msg2,
        }
    }

    /// AES round pattern: dual AES encryption rounds.
    pub const fn aes_round() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::AesRound,
            port5: WavefrontOp::AesRound,
        }
    }

    /// AES decryption round pattern: dual AES decryption rounds.
    pub const fn aes_dec_round() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::AesRoundDec,
            port5: WavefrontOp::AesRoundDec,
        }
    }

    /// All ALU ports perform NOT.
    pub const fn all_not() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Not,
            port5: WavefrontOp::Not,
        }
    }

    /// All ALU ports perform SUB.
    pub const fn all_sub() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Sub,
            port5: WavefrontOp::Sub,
        }
    }

    /// Shuffle pattern: byte permutation within 128-bit lanes.
    pub const fn shuffle() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Shuffle,
        }
    }

    /// Permute pattern: 32-bit lane permutation across 256-bit register.
    pub const fn permute() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Permute,
        }
    }

    /// SHA-256 message schedule pattern.
    pub const fn sha256_msg() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Sha256Msg1,
            port5: WavefrontOp::Sha256Msg2,
        }
    }

    /// SHA-256 message schedule part 1 only.
    pub const fn sha256_msg1() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Sha256Msg1,
            port5: WavefrontOp::Nop,
        }
    }

    /// SHA-256 message schedule part 2 only.
    pub const fn sha256_msg2() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Sha256Msg2,
        }
    }

    /// AES decryption round only (without dual execution).
    pub const fn aes_round_dec() -> Self {
        Self {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::AesRoundDec,
            port5: WavefrontOp::Nop,
        }
    }

    /// Rotate left + XOR pattern.
    pub const fn rotate_left_and_xor(n: u8) -> Self {
        Self {
            port0: WavefrontOp::RotL(n),
            port1: WavefrontOp::Xor,
            port5: WavefrontOp::Xor,
        }
    }

    /// Shift left + XOR pattern.
    pub const fn shift_left_and_xor(n: u8) -> Self {
        Self {
            port0: WavefrontOp::ShL(n),
            port1: WavefrontOp::Xor,
            port5: WavefrontOp::Xor,
        }
    }

    // ========================================
    // Full Port Utilization Patterns (100% efficiency)
    // ========================================

    /// Rotate right + AND pattern (100% port utilization).
    pub const fn rotr_and_and(n: u8) -> Self {
        Self {
            port0: WavefrontOp::RotR(n),
            port1: WavefrontOp::And,
            port5: WavefrontOp::And,
        }
    }

    /// Rotate right + OR pattern (100% port utilization).
    pub const fn rotr_and_or(n: u8) -> Self {
        Self {
            port0: WavefrontOp::RotR(n),
            port1: WavefrontOp::Or,
            port5: WavefrontOp::Or,
        }
    }

    /// Rotate right + ADD pattern (100% port utilization).
    pub const fn rotr_and_add(n: u8) -> Self {
        Self {
            port0: WavefrontOp::RotR(n),
            port1: WavefrontOp::Add,
            port5: WavefrontOp::Add,
        }
    }

    /// Rotate left + AND pattern (100% port utilization).
    pub const fn rotl_and_and(n: u8) -> Self {
        Self {
            port0: WavefrontOp::RotL(n),
            port1: WavefrontOp::And,
            port5: WavefrontOp::And,
        }
    }

    /// Rotate left + OR pattern (100% port utilization).
    pub const fn rotl_and_or(n: u8) -> Self {
        Self {
            port0: WavefrontOp::RotL(n),
            port1: WavefrontOp::Or,
            port5: WavefrontOp::Or,
        }
    }

    /// Rotate left + ADD pattern (100% port utilization).
    pub const fn rotl_and_add(n: u8) -> Self {
        Self {
            port0: WavefrontOp::RotL(n),
            port1: WavefrontOp::Add,
            port5: WavefrontOp::Add,
        }
    }

    /// Shift right + AND pattern (100% port utilization).
    pub const fn shr_and_and(n: u8) -> Self {
        Self {
            port0: WavefrontOp::ShR(n),
            port1: WavefrontOp::And,
            port5: WavefrontOp::And,
        }
    }

    /// Shift left + ADD pattern (100% port utilization).
    pub const fn shl_and_add(n: u8) -> Self {
        Self {
            port0: WavefrontOp::ShL(n),
            port1: WavefrontOp::Add,
            port5: WavefrontOp::Add,
        }
    }

    /// Standalone rotate left (Port 0 only).
    pub const fn rotl_only(n: u8) -> Self {
        Self {
            port0: WavefrontOp::RotL(n),
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        }
    }

    /// Standalone rotate right (Port 0 only).
    pub const fn rotr_only(n: u8) -> Self {
        Self {
            port0: WavefrontOp::RotR(n),
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        }
    }

    /// Standalone shift left (Port 0 only).
    pub const fn shl_only(n: u8) -> Self {
        Self {
            port0: WavefrontOp::ShL(n),
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        }
    }

    /// Standalone shift right (Port 0 only).
    pub const fn shr_only(n: u8) -> Self {
        Self {
            port0: WavefrontOp::ShR(n),
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        }
    }

    /// Validate that all operations are compatible with their ports.
    pub const fn is_valid(&self) -> bool {
        self.port0.is_port0() && self.port1.is_port1() && self.port5.is_port5()
    }
}

/// A complete wavefront specification.
///
/// Describes one step of the cellular automaton:
/// - Which ports execute which operations
/// - Which registers participate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wavefront {
    /// Port operations for this wavefront.
    pub ports: PortAssignment,
    /// Which YMM registers participate (bitmask, bits 0-15).
    pub ymm_mask: u16,
    /// Which GPRs participate (bitmask, bits 0-13).
    pub gpr_mask: u16,
}

impl Wavefront {
    /// Create a new wavefront with all registers participating.
    pub const fn new(ports: PortAssignment) -> Self {
        Self {
            ports,
            ymm_mask: 0xFFFF, // All 16 YMM registers
            gpr_mask: 0x3FFF, // All 14 GPRs
        }
    }

    /// Create a wavefront with specific register masks.
    pub const fn with_masks(ports: PortAssignment, ymm_mask: u16, gpr_mask: u16) -> Self {
        Self {
            ports,
            ymm_mask,
            gpr_mask,
        }
    }

    /// All-XOR wavefront (common for self-inverse testing).
    pub const fn all_xor() -> Self {
        Self::new(PortAssignment::all_xor())
    }

    /// Rotate-and-XOR wavefront.
    pub const fn rotate_xor(n: u8) -> Self {
        Self::new(PortAssignment::rotate_and_xor(n))
    }

    /// SHA-256 round wavefront.
    pub const fn sha256_round() -> Self {
        Self::new(PortAssignment::sha256_round())
    }

    /// AES round wavefront.
    pub const fn aes_round() -> Self {
        Self::new(PortAssignment::aes_round())
    }

    /// AES decryption round wavefront.
    pub const fn aes_dec_round() -> Self {
        Self::new(PortAssignment::aes_dec_round())
    }

    /// NOT wavefront (bitwise complement).
    pub const fn all_not() -> Self {
        Self::new(PortAssignment::all_not())
    }

    /// SUB wavefront (subtraction).
    pub const fn all_sub() -> Self {
        Self::new(PortAssignment::all_sub())
    }

    /// Shuffle wavefront (byte permutation).
    pub const fn shuffle() -> Self {
        Self::new(PortAssignment::shuffle())
    }

    /// Permute wavefront (32-bit lane permutation).
    pub const fn permute() -> Self {
        Self::new(PortAssignment::permute())
    }

    /// SHA-256 message schedule wavefront.
    pub const fn sha256_msg() -> Self {
        Self::new(PortAssignment::sha256_msg())
    }

    /// Rotate left + XOR wavefront.
    pub const fn rotate_left_xor(n: u8) -> Self {
        Self::new(PortAssignment::rotate_left_and_xor(n))
    }

    /// Shift left + XOR wavefront.
    pub const fn shift_left_xor(n: u8) -> Self {
        Self::new(PortAssignment::shift_left_and_xor(n))
    }

    /// AND wavefront.
    pub const fn all_and() -> Self {
        Self::new(PortAssignment::all_and())
    }

    /// OR wavefront.
    pub const fn all_or() -> Self {
        Self::new(PortAssignment::all_or())
    }

    /// ADD wavefront.
    pub const fn all_add() -> Self {
        Self::new(PortAssignment::all_add())
    }
}

impl Default for Wavefront {
    fn default() -> Self {
        Self::new(PortAssignment::nop())
    }
}

/// The UOR step function - core of the cellular automaton.
///
/// Implementations execute wavefronts on hardware with maximum parallelism.
///
/// # Safety Contract
///
/// Implementations MUST:
/// 1. Use `options(nomem, nostack)` on all inline assembly
/// 2. Never access memory outside the state parameter
/// 3. Complete in bounded time (no loops based on state values)
/// 4. Maintain determinism (same input + wavefront = same output)
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
/// // Execute one wavefront cycle
/// unsafe { executor.step(&mut state, &wf); }
/// ```
pub trait UorStep: Send + Sync {
    /// Execute one wavefront cycle.
    ///
    /// Transforms `state` according to `wavefront` using all execution ports.
    ///
    /// # Safety
    ///
    /// - Caller must ensure required CPU features are available (AVX2, SHA-NI, AES-NI)
    /// - State must be properly aligned (32-byte for AVX2)
    unsafe fn step(&self, state: &mut UorState, wavefront: &Wavefront);

    /// Execute a sequence of wavefronts.
    ///
    /// # Safety
    ///
    /// Same requirements as `step`.
    #[inline]
    unsafe fn run(&self, state: &mut UorState, program: &[Wavefront]) {
        for wf in program {
            self.step(state, wf);
        }
    }

    /// Execute a wavefront N times.
    ///
    /// # Safety
    ///
    /// Same requirements as `step`.
    #[inline]
    unsafe fn step_n(&self, state: &mut UorState, wavefront: &Wavefront, n: usize) {
        for _ in 0..n {
            self.step(state, wavefront);
        }
    }
}

/// Extended step function for lossless codec operations.
///
/// This trait extends `UorStep` with complement tracking for non-invertible
/// operations, enabling lossless encode/decode cycles.
///
/// # Complement Storage
///
/// Complements are stored in a separate `UorState` (same layout, zero-copy):
/// - ShL(n): High n bits of each 32-bit lane
/// - ShR(n): Low n bits of each 32-bit lane
/// - AND: `dest & ~operand` (bits masked out)
/// - OR: `~dest & operand` (bits overwritten)
///
/// # Safety Contract
///
/// Same requirements as `UorStep`, plus:
/// - Complement must be valid `UorState` with same alignment
/// - Inverse operations assume complement was captured by matching tracked op
pub trait UorStepLossless: UorStep {
    /// Execute wavefront with complement capture (for non-invertible ops).
    ///
    /// Complement is stored in a separate `UorState` (same size, zero-copy).
    /// For invertible operations, complement is unchanged.
    ///
    /// # Safety
    ///
    /// Same requirements as `UorStep::step`.
    unsafe fn step_tracked(
        &self,
        state: &mut UorState,
        complement: &mut UorState,
        wavefront: &Wavefront,
    );

    /// Execute inverse wavefront using complement.
    ///
    /// Restores `state` to its value before the corresponding `step_tracked`.
    ///
    /// # Safety
    ///
    /// Same requirements as `UorStep::step`.
    unsafe fn step_inverse(
        &self,
        state: &mut UorState,
        complement: &UorState,
        wavefront: &Wavefront,
    );
}

/// Fused program execution - 1 cycle per wavefront (amortized).
///
/// This trait enables register-resident execution where state remains
/// in CPU registers across multiple wavefront executions. This is the
/// correct execution model for UOR - each wavefront should be just the
/// intrinsic operation, with load/store only at program boundaries.
///
/// # Performance Target
///
/// With fused execution:
/// - Load state: 16 cycles (once at program start)
/// - Per wavefront: ~1 cycle (pure intrinsic)
/// - Store state: 8 cycles (once at program end)
///
/// For 64 wavefronts: ~88 cycles total = ~1.4 cycles/wavefront
/// With larger programs, approaches **1 cycle/wavefront**.
///
/// # Hierarchical Fusion
///
/// UOR requires fusion at every level:
/// ```text
/// Taxon → Channel (SIMD fusion)     ✅
/// Channel → Wavefront (Port fusion) ✅
/// Wavefront → Program (Register fusion) ← THIS TRAIT
/// ```
///
/// # Safety Contract
///
/// Same requirements as `UorStep`, plus:
/// - State MUST remain in registers for the entire fused block
/// - Implementations MUST NOT access memory between wavefronts
pub trait UorStepFused: UorStep {
    /// Execute program with register-resident state.
    ///
    /// Loads state into registers once, executes all wavefronts with
    /// state in registers (no memory access), then stores once.
    ///
    /// # Safety
    ///
    /// Same requirements as `UorStep::step`.
    unsafe fn run_fused(&self, state: &mut UorState, program: &[Wavefront]);

    /// Execute single wavefront N times with register fusion.
    ///
    /// Optimized path for repeated identical wavefronts (e.g., SHA-256 rounds).
    ///
    /// # Safety
    ///
    /// Same requirements as `UorStep::step`.
    unsafe fn step_n_fused(&self, state: &mut UorState, wavefront: &Wavefront, n: usize);
}

/// Binary state operations - combine two states element-wise.
///
/// This trait enables operations that take two states as input (e.g., XOR, AND, OR)
/// and produce a result. Unlike `UorStep` which transforms a single state in-place,
/// `UorStepBinary` combines two states.
///
/// # Execution Model
///
/// ```text
/// (state_a, state_b) -> operation -> result
/// ```
///
/// The result is written to `state_a`, treating it as an in-out parameter.
///
/// # Safety Contract
///
/// Same requirements as `UorStep`:
/// 1. Use `options(nomem, nostack)` on all inline assembly
/// 2. Never access memory outside the state parameters
/// 3. Complete in bounded time
/// 4. Maintain determinism
pub trait UorStepBinary: Send + Sync {
    /// Execute a binary wavefront operation.
    ///
    /// Combines `state_a` and `state_b` according to `wavefront`, writing
    /// the result to `state_a`.
    ///
    /// # Safety
    ///
    /// - Caller must ensure required CPU features are available
    /// - Both states must be properly aligned (32-byte for AVX2)
    unsafe fn step_binary(&self, state_a: &mut UorState, state_b: &UorState, wavefront: &Wavefront);

    /// Execute a binary wavefront operation N times.
    ///
    /// # Safety
    ///
    /// Same requirements as `step_binary`.
    #[inline]
    unsafe fn step_binary_n(
        &self,
        state_a: &mut UorState,
        state_b: &UorState,
        wavefront: &Wavefront,
        n: usize,
    ) {
        for _ in 0..n {
            self.step_binary(state_a, state_b, wavefront);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_assignment_valid() {
        assert!(PortAssignment::nop().is_valid());
        assert!(PortAssignment::all_xor().is_valid());
        assert!(PortAssignment::rotate_and_xor(7).is_valid());
        assert!(PortAssignment::sha256_round().is_valid());
        assert!(PortAssignment::aes_round().is_valid());
    }

    #[test]
    fn test_wavefront_masks() {
        let wf = Wavefront::all_xor();
        assert_eq!(wf.ymm_mask, 0xFFFF);
        assert_eq!(wf.gpr_mask, 0x3FFF);

        let wf2 = Wavefront::with_masks(PortAssignment::all_xor(), 0x00FF, 0x003F);
        assert_eq!(wf2.ymm_mask, 0x00FF);
        assert_eq!(wf2.gpr_mask, 0x003F);
    }

    #[test]
    fn test_op_port_classification() {
        // Port 0 only
        assert!(WavefrontOp::RotR(7).is_port0());
        assert!(!WavefrontOp::RotR(7).is_port1());

        // Ports 1/5
        assert!(WavefrontOp::Xor.is_port1());
        assert!(WavefrontOp::Xor.is_port5());
        assert!(!WavefrontOp::Xor.is_port0());

        // SHA-NI
        assert!(WavefrontOp::Sha256Round.is_port0());
        assert!(WavefrontOp::Sha256Msg1.is_port1());

        // AES-NI
        assert!(WavefrontOp::AesRound.is_port1());
        assert!(WavefrontOp::AesRound.is_port5());
    }
}
