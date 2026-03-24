//! SHACL test 215: `cascade:LeaseState` instance.

/// Instance graph for Test 215: LeaseState with leasePhase.
pub const TEST215_LEASE_STATE: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:ls_example a owl:NamedIndividual, cascade:LeaseState ;
    cascade:leasePhase "Active"^^xsd:string .
"#;
