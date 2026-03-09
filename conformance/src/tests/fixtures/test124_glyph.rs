/// SHACL fixture for u:Glyph.
pub const TEST124_GLYPH: &str = r#"
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix u:   <https://uor.foundation/u/> .

<urn:test:glyph_1> a owl:NamedIndividual , u:Glyph ;
    u:codepoint "10240"^^xsd:nonNegativeInteger ;
    u:byteValue "0"^^xsd:nonNegativeInteger .
"#;
