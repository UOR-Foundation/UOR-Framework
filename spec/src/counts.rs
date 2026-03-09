//! Authoritative ontology inventory counts.
//!
//! **This is the single file to update when ontology terms change.**
//! All crates import from here. The spec crate's tests verify that
//! [`Ontology::full()`](crate::Ontology::full) produces exactly these counts.

/// Number of namespace modules.
pub const NAMESPACES: usize = 16;

/// Total OWL classes across all namespaces.
pub const CLASSES: usize = 213;

/// Total properties including the global `uor:space` annotation.
pub const PROPERTIES: usize = 438;

/// Namespace-level properties only (excludes global annotation).
pub const NAMESPACE_PROPERTIES: usize = 437;

/// Total named individuals across all namespaces.
pub const INDIVIDUALS: usize = 820;

/// Number of SHACL test instance graphs.
pub const SHACL_TESTS: usize = 159;

/// Total conformance checks in the full suite.
pub const CONFORMANCE_CHECKS: usize = 256;

/// Number of amendments applied to the base ontology.
pub const AMENDMENTS: usize = 46;

/// Number of classes that become Rust enums/structs (not traits).
pub const ENUM_CLASSES: usize = 13;

/// Number of `op:Identity` individuals (and corresponding proofs).
pub const IDENTITY_COUNT: usize = 367;

/// Kernel-space namespace count.
pub const KERNEL_NAMESPACES: usize = 3;

/// Bridge-space namespace count.
pub const BRIDGE_NAMESPACES: usize = 10;

/// User-space namespace count.
pub const USER_NAMESPACES: usize = 3;

/// Number of trait methods generated (properties with domains,
/// excluding enum-class-domain and cross-namespace-domain properties).
pub const METHODS: usize = 367;

/// Number of individual constant modules generated.
pub const CONSTANT_MODULES: usize = 791;
