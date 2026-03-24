//! SHACL test 208: `cascade:Epoch` instance.

/// Instance graph for Test 208: Epoch with epochIndex.
pub const TEST208_EPOCH: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:epoch_0 a owl:NamedIndividual, cascade:Epoch ;
    cascade:epochIndex "0"^^xsd:nonNegativeInteger .
"#;
