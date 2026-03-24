//! SHACL test 251: `cascade:PreflightCheck` and `cascade:LeaseCheckpoint`.

/// Instance graph for Test 251: PreflightCheck and LeaseCheckpoint.
pub const TEST251_PREFLIGHT_CHECKPOINT: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:ex_preflight_251 a owl:NamedIndividual, cascade:PreflightCheck ;
    cascade:preflightKind "FeasibilityCheck" ;
    cascade:preflightResult "pass" .

cascade:ex_checkpoint_251 a owl:NamedIndividual, cascade:LeaseCheckpoint ;
    cascade:checkpointEpoch "5"^^xsd:nonNegativeInteger ;
    cascade:leaseRemainingBudget "3"^^xsd:nonNegativeInteger .
"#;
