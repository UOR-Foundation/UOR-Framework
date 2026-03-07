/// SHACL test 54: Saturated context — SaturatedContext with saturationDegree,
/// contextTemperature, isSaturated, saturationPhase (Amendment 33).
pub const TEST54_SATURATED_CONTEXT: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix state:      <https://uor.foundation/state/> .
@prefix observable: <https://uor.foundation/observable/> .

# 1. SaturatedContext with properties
state:ex_sc_54 a owl:NamedIndividual, state:SaturatedContext ;
    state:saturationDegree "0.95"^^xsd:decimal ;
    state:contextTemperature "42"^^xsd:integer ;
    state:isSaturated "true"^^xsd:boolean ;
    state:saturationPhase observable:FullSaturation .
"#;
