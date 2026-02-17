//! Instance graph test fixtures for SHACL conformance validation.
//!
//! Each constant holds a Turtle 1.1 RDF graph that represents a valid
//! instance of UOR ontology terms, used to verify SHACL shape constraints.

mod test1_ring;
mod test2_primitives;
mod test3_term_graph;
mod test4_state_lifecycle;
mod test5_partition;
mod test6_critical_identity;
mod test7_end_to_end;

pub use test1_ring::TEST1_RING;
pub use test2_primitives::TEST2_PRIMITIVES;
pub use test3_term_graph::TEST3_TERM_GRAPH;
pub use test4_state_lifecycle::TEST4_STATE_LIFECYCLE;
pub use test5_partition::TEST5_PARTITION;
pub use test6_critical_identity::TEST6_CRITICAL_IDENTITY;
pub use test7_end_to_end::TEST7_END_TO_END;
