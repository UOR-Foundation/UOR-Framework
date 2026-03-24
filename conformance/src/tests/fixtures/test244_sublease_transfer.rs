//! SHACL test 244: `cascade:SubleaseTransfer`.

/// Instance graph for Test 244: SubleaseTransfer.
pub const TEST244_SUBLEASE_TRANSFER: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:ex_sublease_244 a owl:NamedIndividual, cascade:SubleaseTransfer ;
    cascade:sourceLeaseRef "lease_alpha" ;
    cascade:targetLeaseRef "lease_beta" ;
    cascade:transferredBudget "5"^^xsd:nonNegativeInteger ;
    cascade:transferCompleted "true"^^xsd:boolean .
"#;
