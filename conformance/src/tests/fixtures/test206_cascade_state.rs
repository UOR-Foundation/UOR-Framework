//! SHACL test 206: `cascade:CascadeState` instance.

/// Instance graph for Test 206: CascadeState with currentStage and phaseAngle.
pub const TEST206_CASCADE_STATE: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:state_at_stage_2 a owl:NamedIndividual, cascade:CascadeState ;
    cascade:currentStage cascade:stage_factorize ;
    cascade:phaseAngle "Omega^2" ;
    cascade:pinnedMask "110000" ;
    cascade:freeCount "4"^^xsd:nonNegativeInteger .
"#;
