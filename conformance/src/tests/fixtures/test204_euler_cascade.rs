//! SHACL test 204: `cascade:EulerCascade` instance.

/// Instance graph for Test 204: EulerCascade with phaseParameter and stageCount.
pub const TEST204_EULER_CASCADE: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:euler_cascade_instance a owl:NamedIndividual, cascade:EulerCascade ;
    cascade:phaseParameter "e^{i*pi/6}" ;
    cascade:stageCount "6"^^xsd:nonNegativeInteger ;
    cascade:convergenceAngle "pi" .
"#;
