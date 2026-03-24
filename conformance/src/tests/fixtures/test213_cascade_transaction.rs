//! SHACL test 213: `cascade:CascadeTransaction` instance.

/// Instance graph for Test 213: CascadeTransaction with transactionPolicy.
pub const TEST213_CASCADE_TRANSACTION: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:tx_example a owl:NamedIndividual, cascade:CascadeTransaction ;
    cascade:transactionPolicy "AllOrNothing"^^xsd:string ;
    cascade:transactionOutcome "committed"^^xsd:string .
"#;
