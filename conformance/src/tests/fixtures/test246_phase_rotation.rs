//! SHACL test 246: `cascade:PhaseRotationScheduler` and `cascade:TargetConvergenceAngle`.

/// Instance graph for Test 246: PhaseRotationScheduler and TargetConvergenceAngle.
pub const TEST246_PHASE_ROTATION: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:ex_scheduler_246 a owl:NamedIndividual, cascade:PhaseRotationScheduler ;
    cascade:rotationSchedule "0, 30, 60, 90, 120, 150" ;
    cascade:baseAngle "30" .

cascade:ex_target_246 a owl:NamedIndividual, cascade:TargetConvergenceAngle ;
    cascade:targetAngle "180" .
"#;
