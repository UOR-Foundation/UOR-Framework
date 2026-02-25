//! x86_64 UOR implementation targeting AMD Zen 3+.
//!
//! This module provides the `Zen3Executor` which implements `UorStep`
//! with maximum port utilization on AMD Zen 3 and compatible CPUs.
//!
//! # Execution Ports (Zen 3)
//!
//! ```text
//! Port 0: Shift, Rotate, SHA-NI (sha256rnds2)
//! Port 1: ALU (XOR, AND, OR, ADD), AES-NI (aesenc)
//! Port 5: ALU, AES-NI, Shuffle/Permute
//! ```
//!
//! # Register Budget
//!
//! - 16 YMM registers (256-bit each)
//! - 14 GPRs available (excluding rsp/rbp)
//!
//! # Zero Spillage Guarantee
//!
//! All wavefront execution uses `options(nomem, nostack)` on inline
//! assembly to ensure no memory access.
//!
//! # CPU Feature Requirements
//!
//! UOR requires the following CPU features:
//! - AVX2: For 256-bit SIMD operations
//! - SHA-NI: For SHA-256 hardware acceleration
//! - AES-NI: For AES hardware acceleration
//!
//! Missing features are a conformance violation and will cause a panic.

pub mod asm;
mod features;
mod wavefront;

pub use asm::Zen3AsmExecutor;
pub use features::CpuFeatures;
pub use wavefront::Zen3Executor;
