//! Authoritative ontology inventory counts.
//!
//! **This is the single file to update when ontology terms change.**
//! All crates import from here. The spec crate's tests verify that
//! [`Ontology::full()`](crate::Ontology::full) produces exactly these counts.

/// Number of namespace modules.
pub const NAMESPACES: usize = 33;

/// Total OWL classes across all namespaces.
pub const CLASSES: usize = 396;

/// Total properties including the global `uor:space` annotation.
pub const PROPERTIES: usize = 837;

/// Namespace-level properties only (excludes global annotation).
pub const NAMESPACE_PROPERTIES: usize = 836;

/// Total named individuals across all namespaces.
pub const INDIVIDUALS: usize = 1432;

/// Number of SHACL test instance graphs.
pub const SHACL_TESTS: usize = 252;

/// Total conformance checks in the full suite.
pub const CONFORMANCE_CHECKS: usize = 396;

/// Number of amendments applied to the base ontology.
pub const AMENDMENTS: usize = 84;

/// Number of classes that become Rust enums/structs (not traits).
pub const ENUM_CLASSES: usize = 15;

/// Number of `op:Identity` individuals (and corresponding proofs).
pub const IDENTITY_COUNT: usize = 624;

/// Kernel-space namespace count.
pub const KERNEL_NAMESPACES: usize = 17;

/// Bridge-space namespace count.
pub const BRIDGE_NAMESPACES: usize = 13;

/// User-space namespace count.
pub const USER_NAMESPACES: usize = 3;

/// Number of trait methods generated (properties with domains,
/// excluding enum-class-domain and cross-namespace-domain properties).
pub const METHODS: usize = 790;

/// Number of individual constant modules generated.
pub const CONSTANT_MODULES: usize = 1452;

/// Number of concept pages on the website (one per content/concepts/*.md file).
pub const CONCEPT_PAGES: usize = 23;

/// Number of PRISM pipeline stages (Define / Resolve / Certify).
pub const PIPELINE_STAGES: usize = 3;

/// Minimum number of classes in a namespace to generate a class hierarchy SVG.
pub const MIN_HIERARCHY_CLASSES: usize = 3;
