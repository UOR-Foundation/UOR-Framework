/// SHACL test 71: Measurement event — MeasurementEvent with measurementEvent,
/// preCollapseEntropy, postCollapseLandauerCost, collapseStep (Amendment 36).
pub const TEST71_MEASUREMENT_EVENT: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix observable: <https://uor.foundation/observable/> .
@prefix trace:      <https://uor.foundation/trace/> .

# 1. MeasurementEvent observable
observable:ex_me_71 a owl:NamedIndividual, observable:MeasurementEvent ;
    observable:measurementEvent trace:ex_step_71 ;
    observable:preCollapseEntropy "3.2"^^xsd:decimal ;
    observable:postCollapseLandauerCost "0.8"^^xsd:decimal ;
    observable:collapseStep "5"^^xsd:integer .

# 2. Referenced step
trace:ex_step_71 a owl:NamedIndividual, trace:ComputationTrace .
"#;
