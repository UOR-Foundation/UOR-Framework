/// SHACL test 56: Domain saturation record — DomainSaturationRecord with
/// saturatedContext, saturatedDomain, domainFreeCount (Amendment 33).
pub const TEST56_DOMAIN_SATURATION_RECORD: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix state:      <https://uor.foundation/state/> .
@prefix observable: <https://uor.foundation/observable/> .

# 1. DomainSaturationRecord
observable:ex_dsr_56 a owl:NamedIndividual, observable:DomainSaturationRecord ;
    observable:saturatedContext state:ex_sc_56 ;
    observable:saturatedDomain "arithmetic"^^xsd:string ;
    observable:domainFreeCount "0"^^xsd:integer .

# 2. Referenced SaturatedContext
state:ex_sc_56 a owl:NamedIndividual, state:SaturatedContext .
"#;
