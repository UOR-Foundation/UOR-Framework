/// SHACL fixture for observable:CascadeEntropy.
pub const TEST147_CASCADE_ENTROPY: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix observable: <https://uor.foundation/observable/> .

<urn:test:cascade_entropy_1> a owl:NamedIndividual , observable:CascadeEntropy ;
    observable:value "0.693"^^xsd:decimal .
"#;
