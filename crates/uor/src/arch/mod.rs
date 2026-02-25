//! Architecture-specific UOR implementations.
//!
//! This module provides hardware bindings for the UOR ISA on different
//! CPU architectures. Each architecture module implements the `UorStep`
//! trait with maximum port utilization and zero memory access.
//!
//! # Supported Architectures
//!
//! - `x86_64`: AMD Zen 3+ / Intel (AVX2, SHA-NI, AES-NI)
//! - `portable`: Pure Rust scalar implementation (all architectures)
//!
//! # Future Architectures
//!
//! - `aarch64`: ARM (NEON, SHA, AES crypto extensions)

// Portable scalar executor (available on all architectures)
pub mod portable;

// x86_64-specific implementations
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

// aarch64-specific implementations
#[cfg(target_arch = "aarch64")]
pub mod aarch64;

// Re-export the default executor for the current architecture
#[cfg(target_arch = "x86_64")]
pub use x86_64::Zen3Executor;

#[cfg(target_arch = "aarch64")]
pub use aarch64::NeonExecutor;

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub use portable::ScalarExecutor as DefaultExecutor;

// Re-export portable executor for all architectures
pub use portable::ScalarExecutor;

// Re-export CPU feature detection
#[cfg(target_arch = "x86_64")]
pub use x86_64::CpuFeatures;
