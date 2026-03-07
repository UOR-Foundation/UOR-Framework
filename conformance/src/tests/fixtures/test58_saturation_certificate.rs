/// SHACL test 58: Saturation certificate — SaturationCertificate with
/// certifiedSaturation, saturationWitness (Amendment 33).
pub const TEST58_SATURATION_CERTIFICATE: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix cert:       <https://uor.foundation/cert/> .
@prefix state:      <https://uor.foundation/state/> .

# 1. SaturationCertificate
cert:ex_scert_58 a owl:NamedIndividual, cert:SaturationCertificate ;
    cert:certifiedSaturation state:ex_sc_58 ;
    cert:saturationWitness state:ex_sw_58 .

# 2. Referenced SaturatedContext and SaturationWitness
state:ex_sc_58 a owl:NamedIndividual, state:SaturatedContext .
state:ex_sw_58 a owl:NamedIndividual, state:SaturationWitness .
"#;
