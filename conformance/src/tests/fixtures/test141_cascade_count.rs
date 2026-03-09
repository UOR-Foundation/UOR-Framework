/// SHACL fixture for observable:CascadeCount.
pub const TEST141_CASCADE_COUNT: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix observable: <https://uor.foundation/observable/> .

<urn:test:cascade_count_1> a owl:NamedIndividual , observable:CascadeCount ;
    observable:value "4"^^xsd:decimal .
"#;
