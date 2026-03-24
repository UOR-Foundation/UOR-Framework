//! SHACL test 216: `cascade:ManagedLease` instance.

/// Instance graph for Test 216: ManagedLease with lifecycle.
pub const TEST216_MANAGED_LEASE: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:ml_example a owl:NamedIndividual, cascade:ManagedLease ;
    cascade:managedLeaseId "lease-001"^^xsd:string ;
    cascade:leaseLifecycle cascade:Active .
"#;
