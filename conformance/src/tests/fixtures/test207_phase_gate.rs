//! SHACL test 207: `cascade:PhaseGateAttestation` instance.

/// Instance graph for Test 207: PhaseGateAttestation with gateStage and gateResult.
pub const TEST207_PHASE_GATE: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:gate_initialization a owl:NamedIndividual, cascade:PhaseGateAttestation ;
    cascade:gateStage cascade:stage_initialization ;
    cascade:gateExpectedPhase "Omega^0" ;
    cascade:gateResult "true"^^xsd:boolean .
"#;
