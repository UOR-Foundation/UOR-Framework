//! SHACL test 205: `cascade:CascadeStage` instance.

/// Instance graph for Test 205: CascadeStage with stageIndex and stageName.
pub const TEST205_CASCADE_STAGE: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:stage_initialization a owl:NamedIndividual, cascade:CascadeStage ;
    cascade:stageIndex "0"^^xsd:nonNegativeInteger ;
    cascade:stageName "Initialization" ;
    cascade:expectedPhase "Omega^0" .
"#;
