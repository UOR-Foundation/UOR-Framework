//! Instance graph test fixtures for SHACL conformance validation.
//!
//! Each constant holds a Turtle 1.1 RDF graph that represents a valid
//! instance of UOR ontology terms, used to verify SHACL shape constraints.

mod test10_iterative_resolution;
mod test11_composition;
mod test12_factorization;
mod test13_canonical_form;
mod test14_content_addressing;
mod test15_boolean_sat;
mod test16_algebraic_identities;
mod test17_inter_algebra_maps;
mod test18_analytical_completeness;
mod test19_homological_pipeline;
mod test1_ring;
mod test20_sheaf_consistency;
mod test21_topological_delta;
mod test22_index_bridge;
mod test23_identity_grounding;
mod test24_verification_domain;
mod test25_geometric_character;
mod test26_complexity_class;
mod test27_rewrite_rule;
mod test28_measurement_unit;
mod test29_coordinate_kind;
mod test2_primitives;
mod test3_term_graph;
mod test4_state_lifecycle;
mod test5_partition;
mod test6_critical_identity;
mod test7_end_to_end;
mod test8_fiber_budget;
mod test9_constraint_algebra;

pub use test10_iterative_resolution::TEST10_ITERATIVE_RESOLUTION;
pub use test11_composition::TEST11_COMPOSITION;
pub use test12_factorization::TEST12_FACTORIZATION;
pub use test13_canonical_form::TEST13_CANONICAL_FORM;
pub use test14_content_addressing::TEST14_CONTENT_ADDRESSING;
pub use test15_boolean_sat::TEST15_BOOLEAN_SAT;
pub use test16_algebraic_identities::TEST16_ALGEBRAIC_IDENTITIES;
pub use test17_inter_algebra_maps::TEST17_INTER_ALGEBRA_MAPS;
pub use test18_analytical_completeness::TEST18_ANALYTICAL_COMPLETENESS;
pub use test19_homological_pipeline::TEST19_HOMOLOGICAL_PIPELINE;
pub use test1_ring::TEST1_RING;
pub use test20_sheaf_consistency::TEST20_SHEAF_CONSISTENCY;
pub use test21_topological_delta::TEST21_TOPOLOGICAL_DELTA;
pub use test22_index_bridge::TEST22_INDEX_BRIDGE;
pub use test23_identity_grounding::TEST23_IDENTITY_GROUNDING;
pub use test24_verification_domain::TEST24_VERIFICATION_DOMAIN;
pub use test25_geometric_character::TEST25_GEOMETRIC_CHARACTER;
pub use test26_complexity_class::TEST26_COMPLEXITY_CLASS;
pub use test27_rewrite_rule::TEST27_REWRITE_RULE;
pub use test28_measurement_unit::TEST28_MEASUREMENT_UNIT;
pub use test29_coordinate_kind::TEST29_COORDINATE_KIND;
pub use test2_primitives::TEST2_PRIMITIVES;
pub use test3_term_graph::TEST3_TERM_GRAPH;
pub use test4_state_lifecycle::TEST4_STATE_LIFECYCLE;
pub use test5_partition::TEST5_PARTITION;
pub use test6_critical_identity::TEST6_CRITICAL_IDENTITY;
pub use test7_end_to_end::TEST7_END_TO_END;
pub use test8_fiber_budget::TEST8_FIBER_BUDGET;
pub use test9_constraint_algebra::TEST9_CONSTRAINT_ALGEBRA;
