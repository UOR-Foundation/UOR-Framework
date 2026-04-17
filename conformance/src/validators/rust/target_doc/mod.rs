//! Target-document cross-reference validators.
//!
//! The correctness suite already pins behavioral contracts per endpoint.
//! These validators add a second layer: they scan the target document
//! `external/uor-foundation-target-v2.md` for structural prescriptions
//! (sealed-type table, resolver signature shape, closed enumerations,
//! trait-shape commitments) and cross-check the foundation source
//! against them.
//!
//! Together the two layers give the suite self-enforcement: behavioral
//! regressions fail a `correctness/*` validator; target-document
//! deviations fail a `rust/target/*` validator.

pub mod constraint_encoder_completeness;
pub mod resolver_signature_shape;
pub mod sealed_type_coverage;
pub mod spectral_sequence_walk;
pub mod w4_grounding_closure;
