/// SHACL test 55: Saturation witness — SaturationWitness with witnessBinding,
/// witnessStep, residualFreeCount (Amendment 33).
pub const TEST55_SATURATION_WITNESS: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix state:      <https://uor.foundation/state/> .

# 1. SaturationWitness with properties
state:ex_sw_55 a owl:NamedIndividual, state:SaturationWitness ;
    state:witnessBinding state:ex_bind_55 ;
    state:witnessStep "3"^^xsd:integer ;
    state:residualFreeCount "1"^^xsd:integer .

# 2. Referenced binding
state:ex_bind_55 a owl:NamedIndividual, state:Binding .
"#;
