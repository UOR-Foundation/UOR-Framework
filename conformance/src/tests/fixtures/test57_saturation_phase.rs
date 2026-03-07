/// SHACL test 57: Saturation phase individuals — Unsaturated,
/// PartialSaturation, FullSaturation (Amendment 33).
pub const TEST57_SATURATION_PHASE: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix observable: <https://uor.foundation/observable/> .

# 1. SaturationPhase individuals
observable:Unsaturated a owl:NamedIndividual, observable:SaturationPhase .
observable:PartialSaturation a owl:NamedIndividual, observable:SaturationPhase .
observable:FullSaturation a owl:NamedIndividual, observable:SaturationPhase .
"#;
