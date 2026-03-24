//! SHACL test 250: `cascade:PipelineSuccess` and `cascade:PipelineFailureReason`.

/// Instance graph for Test 250: PipelineSuccess and PipelineFailureReason.
pub const TEST250_PIPELINE_OUTCOME: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:ex_success_250 a owl:NamedIndividual, cascade:PipelineSuccess ;
    cascade:finalSaturation "1.0" .

cascade:ex_failure_250 a owl:NamedIndividual, cascade:PipelineFailureReason ;
    cascade:failureKind "DispatchMiss" ;
    cascade:failureDetail "No resolver found for query" ;
    cascade:failureStage "Declare" .
"#;
