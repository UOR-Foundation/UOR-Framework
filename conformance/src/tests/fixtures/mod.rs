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
mod test1_ring;
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
pub use test1_ring::TEST1_RING;
pub use test2_primitives::TEST2_PRIMITIVES;
pub use test3_term_graph::TEST3_TERM_GRAPH;
pub use test4_state_lifecycle::TEST4_STATE_LIFECYCLE;
pub use test5_partition::TEST5_PARTITION;
pub use test6_critical_identity::TEST6_CRITICAL_IDENTITY;
pub use test7_end_to_end::TEST7_END_TO_END;
pub use test8_fiber_budget::TEST8_FIBER_BUDGET;
pub use test9_constraint_algebra::TEST9_CONSTRAINT_ALGEBRA;
