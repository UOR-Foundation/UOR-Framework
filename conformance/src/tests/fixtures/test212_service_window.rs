//! SHACL test 212: `cascade:ServiceWindow` instance.

/// Instance graph for Test 212: ServiceWindow with windowSize.
pub const TEST212_SERVICE_WINDOW: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:window_example a owl:NamedIndividual, cascade:ServiceWindow ;
    cascade:windowSize "5"^^xsd:nonNegativeInteger ;
    cascade:windowOffset "0"^^xsd:nonNegativeInteger .
"#;
