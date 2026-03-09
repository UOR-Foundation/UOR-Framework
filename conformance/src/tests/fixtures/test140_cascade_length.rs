/// SHACL fixture for observable:CascadeLength.
pub const TEST140_CASCADE_LENGTH: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix observable: <https://uor.foundation/observable/> .

<urn:test:cascade_len_1> a owl:NamedIndividual , observable:CascadeLength ;
    observable:value "8"^^xsd:decimal .
"#;
