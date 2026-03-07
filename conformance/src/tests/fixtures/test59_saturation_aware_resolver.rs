/// SHACL test 59: Saturation-aware resolver — SaturationAwareResolver with
/// usedSaturation (Amendment 33).
pub const TEST59_SATURATION_AWARE_RESOLVER: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix resolver:   <https://uor.foundation/resolver/> .
@prefix state:      <https://uor.foundation/state/> .

# 1. SaturationAwareResolver
resolver:ex_sar_59 a owl:NamedIndividual, resolver:SaturationAwareResolver ;
    resolver:usedSaturation state:ex_sc_59 .

# 2. Referenced SaturatedContext
state:ex_sc_59 a owl:NamedIndividual, state:SaturatedContext .
"#;
