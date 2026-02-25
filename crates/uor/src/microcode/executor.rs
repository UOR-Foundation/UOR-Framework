//! Scalar reference executor for microcode sequences.
//!
//! This module provides a pure Rust, no-SIMD executor for running microcode
//! derivations. It serves as:
//!
//! 1. **Reference implementation**: Correctness baseline for other executors
//! 2. **Portable fallback**: Works on any platform without SIMD
//! 3. **Test oracle**: Verify optimized implementations match reference
//!
//! # Usage
//!
//! ```
//! use uor::microcode::{ScalarMicrocodeExecutor, MicrocodeStep, Derivation};
//! use uor::microcode::derivation::standard;
//!
//! let mut executor = ScalarMicrocodeExecutor::new();
//!
//! // Execute INC derivation: result = x + 1
//! executor.set_register(0, 41);
//! executor.execute_derivation(&standard::inc());
//! assert_eq!(executor.get_register(0), 42);
//! ```

use super::derivation::{Derivation, MicrocodeStep};
use super::primitives::{MicrocodePrimitives, ScalarPrimitives};

/// Number of registers in the executor.
pub const REGISTER_COUNT: usize = 16;

/// Scalar reference executor for microcode operations.
///
/// Uses a simple register file with 16 u32 registers. All operations are
/// executed using `ScalarPrimitives` - pure Rust with no SIMD intrinsics.
///
/// # Register Convention
///
/// - r0: Primary input/output
/// - r1: Secondary input (for binary ops)
/// - r2-r15: Temporaries
///
/// Derivations typically expect inputs in r0/r1 and produce output in r0.
#[derive(Debug, Clone)]
pub struct ScalarMicrocodeExecutor {
    /// Register file (16 x u32).
    registers: [u32; REGISTER_COUNT],
    /// The primitive operations backend.
    primitives: ScalarPrimitives,
}

impl Default for ScalarMicrocodeExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl ScalarMicrocodeExecutor {
    /// Create a new executor with zeroed registers.
    #[inline]
    pub fn new() -> Self {
        Self {
            registers: [0u32; REGISTER_COUNT],
            primitives: ScalarPrimitives,
        }
    }

    /// Create an executor with registers initialized from a slice.
    ///
    /// Initializes registers starting from r0. Remaining registers are zeroed.
    pub fn with_registers(values: &[u32]) -> Self {
        let mut registers = [0u32; REGISTER_COUNT];
        let len = values.len().min(REGISTER_COUNT);
        registers[..len].copy_from_slice(&values[..len]);
        Self {
            registers,
            primitives: ScalarPrimitives,
        }
    }

    /// Get the value of a register.
    ///
    /// # Panics
    ///
    /// Panics if `reg >= REGISTER_COUNT`.
    #[inline]
    pub fn get_register(&self, reg: u8) -> u32 {
        self.registers[reg as usize]
    }

    /// Set the value of a register.
    ///
    /// # Panics
    ///
    /// Panics if `reg >= REGISTER_COUNT`.
    #[inline]
    pub fn set_register(&mut self, reg: u8, value: u32) {
        self.registers[reg as usize] = value;
    }

    /// Get all register values as a slice.
    #[inline]
    pub fn registers(&self) -> &[u32; REGISTER_COUNT] {
        &self.registers
    }

    /// Reset all registers to zero.
    #[inline]
    pub fn reset(&mut self) {
        self.registers = [0u32; REGISTER_COUNT];
    }

    /// Execute a single microcode step.
    ///
    /// Reads source registers, performs the operation, writes to destination.
    #[inline]
    pub fn execute_step(&mut self, step: &MicrocodeStep) {
        match step {
            MicrocodeStep::BNot { dst, src } => {
                let a = self.registers[*src as usize];
                self.registers[*dst as usize] = self.primitives.bnot(a);
            }
            MicrocodeStep::Neg { dst, src } => {
                let a = self.registers[*src as usize];
                self.registers[*dst as usize] = self.primitives.neg(a);
            }
            MicrocodeStep::Xor { dst, a, b } => {
                let va = self.registers[*a as usize];
                let vb = self.registers[*b as usize];
                self.registers[*dst as usize] = self.primitives.xor(va, vb);
            }
            MicrocodeStep::And { dst, a, b } => {
                let va = self.registers[*a as usize];
                let vb = self.registers[*b as usize];
                self.registers[*dst as usize] = self.primitives.and(va, vb);
            }
            MicrocodeStep::Or { dst, a, b } => {
                let va = self.registers[*a as usize];
                let vb = self.registers[*b as usize];
                self.registers[*dst as usize] = self.primitives.or(va, vb);
            }
        }
    }

    /// Execute all steps in a derivation.
    ///
    /// Steps are executed in order. Each step reads from and writes to
    /// the register file, allowing data to flow between steps.
    #[inline]
    pub fn execute_derivation(&mut self, derivation: &Derivation) {
        for step in derivation.steps() {
            self.execute_step(step);
        }
    }

    /// Execute a slice of steps directly.
    #[inline]
    pub fn execute_steps(&mut self, steps: &[MicrocodeStep]) {
        for step in steps {
            self.execute_step(step);
        }
    }

    /// Execute INC on a register using the microcode identity: neg(bnot(x)).
    ///
    /// Reads from `src` register, writes result to `dst` register.
    #[inline]
    pub fn inc_reg(&mut self, dst: u8, src: u8) {
        let a = self.registers[src as usize];
        self.registers[dst as usize] = self.primitives.neg(self.primitives.bnot(a));
    }

    /// Execute DEC on a register using the microcode identity: bnot(neg(x)).
    ///
    /// Reads from `src` register, writes result to `dst` register.
    #[inline]
    pub fn dec_reg(&mut self, dst: u8, src: u8) {
        let a = self.registers[src as usize];
        self.registers[dst as usize] = self.primitives.bnot(self.primitives.neg(a));
    }
}

// Implement MicrocodePrimitives for the executor itself
impl MicrocodePrimitives<u32> for ScalarMicrocodeExecutor {
    #[inline(always)]
    fn bnot(&self, a: u32) -> u32 {
        self.primitives.bnot(a)
    }

    #[inline(always)]
    fn neg(&self, a: u32) -> u32 {
        self.primitives.neg(a)
    }

    #[inline(always)]
    fn xor(&self, a: u32, b: u32) -> u32 {
        self.primitives.xor(a, b)
    }

    #[inline(always)]
    fn and(&self, a: u32, b: u32) -> u32 {
        self.primitives.and(a, b)
    }

    #[inline(always)]
    fn or(&self, a: u32, b: u32) -> u32 {
        self.primitives.or(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::microcode::derivation::standard;
    use crate::microcode::ops::MicrocodeOps;

    #[test]
    fn test_executor_creation() {
        let exec = ScalarMicrocodeExecutor::new();
        for i in 0..REGISTER_COUNT {
            assert_eq!(exec.get_register(i as u8), 0);
        }
    }

    #[test]
    fn test_executor_with_registers() {
        let exec = ScalarMicrocodeExecutor::with_registers(&[10, 20, 30]);
        assert_eq!(exec.get_register(0), 10);
        assert_eq!(exec.get_register(1), 20);
        assert_eq!(exec.get_register(2), 30);
        assert_eq!(exec.get_register(3), 0);
    }

    #[test]
    fn test_register_roundtrip() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(5, 0xDEAD_BEEF);
        assert_eq!(exec.get_register(5), 0xDEAD_BEEF);
    }

    #[test]
    fn test_execute_bnot() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 0x0000_FFFF);
        exec.execute_step(&MicrocodeStep::BNot { dst: 1, src: 0 });
        assert_eq!(exec.get_register(1), 0xFFFF_0000);
    }

    #[test]
    fn test_execute_neg() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 1);
        exec.execute_step(&MicrocodeStep::Neg { dst: 1, src: 0 });
        assert_eq!(exec.get_register(1), u32::MAX); // -1 in two's complement
    }

    #[test]
    fn test_execute_xor() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 0xAAAA_AAAA);
        exec.set_register(1, 0x5555_5555);
        exec.execute_step(&MicrocodeStep::Xor { dst: 2, a: 0, b: 1 });
        assert_eq!(exec.get_register(2), 0xFFFF_FFFF);
    }

    #[test]
    fn test_execute_and() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 0xFF00_FF00);
        exec.set_register(1, 0x0FF0_0FF0);
        exec.execute_step(&MicrocodeStep::And { dst: 2, a: 0, b: 1 });
        assert_eq!(exec.get_register(2), 0x0F00_0F00);
    }

    #[test]
    fn test_execute_or() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 0xFF00_0000);
        exec.set_register(1, 0x00FF_0000);
        exec.execute_step(&MicrocodeStep::Or { dst: 2, a: 0, b: 1 });
        assert_eq!(exec.get_register(2), 0xFFFF_0000);
    }

    #[test]
    fn test_execute_inc_derivation() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 41);
        exec.execute_derivation(&standard::inc());
        assert_eq!(exec.get_register(0), 42);
    }

    #[test]
    fn test_execute_dec_derivation() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 42);
        exec.execute_derivation(&standard::dec());
        assert_eq!(exec.get_register(0), 41);
    }

    #[test]
    fn test_execute_nand_derivation() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 0xFF00_FF00);
        exec.set_register(1, 0x0FF0_0FF0);
        exec.execute_derivation(&standard::nand());
        // NAND = NOT(AND)
        let expected = !(0xFF00_FF00u32 & 0x0FF0_0FF0);
        assert_eq!(exec.get_register(0), expected);
    }

    #[test]
    fn test_inc_reg_convenience_method() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 99);
        exec.inc_reg(0, 0);
        assert_eq!(exec.get_register(0), 100);
    }

    #[test]
    fn test_dec_reg_convenience_method() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 100);
        exec.dec_reg(0, 0);
        assert_eq!(exec.get_register(0), 99);
    }

    #[test]
    fn test_inc_dec_reg_roundtrip() {
        let mut exec = ScalarMicrocodeExecutor::new();
        for x in [0u32, 1, 42, 255, 1000, u32::MAX - 1, u32::MAX] {
            exec.set_register(0, x);
            exec.inc_reg(0, 0);
            exec.dec_reg(0, 0);
            assert_eq!(exec.get_register(0), x, "inc(dec(x)) != x for x={x}");
        }
    }

    #[test]
    fn test_executor_implements_primitives() {
        let exec = ScalarMicrocodeExecutor::new();
        // Verify it implements MicrocodePrimitives<u32>
        assert_eq!(exec.bnot(0u32), u32::MAX);
        assert_eq!(exec.neg(1u32), u32::MAX);
        assert_eq!(exec.xor(0xF0F0u32, 0x0F0F), 0xFFFF);
        assert_eq!(exec.and(0xFF00u32, 0x0FF0), 0x0F00);
        assert_eq!(exec.or(0xF000u32, 0x000F), 0xF00F);
    }

    #[test]
    fn test_executor_implements_ops() {
        let exec = ScalarMicrocodeExecutor::new();
        // Verify it gets MicrocodeOps via blanket impl
        assert_eq!(exec.inc(0u32), 1);
        assert_eq!(exec.dec(1u32), 0);
        assert_eq!(exec.add(10u32, 20), 30);
        assert_eq!(exec.sub(30u32, 10), 20);
    }

    #[test]
    fn test_reset() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, 100);
        exec.set_register(5, 200);
        exec.reset();
        assert_eq!(exec.get_register(0), 0);
        assert_eq!(exec.get_register(5), 0);
    }

    #[test]
    fn test_overflow_wrapping() {
        let mut exec = ScalarMicrocodeExecutor::new();
        exec.set_register(0, u32::MAX);
        exec.execute_derivation(&standard::inc());
        assert_eq!(exec.get_register(0), 0); // Wraps around

        exec.set_register(0, 0);
        exec.execute_derivation(&standard::dec());
        assert_eq!(exec.get_register(0), u32::MAX); // Wraps around
    }
}
