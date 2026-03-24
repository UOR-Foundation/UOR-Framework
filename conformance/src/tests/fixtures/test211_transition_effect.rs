//! SHACL test 211: `cascade:TransitionEffect` instance.

/// Instance graph for Test 211: TransitionEffect with effectBindings.
pub const TEST211_TRANSITION_EFFECT: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:effect_example a owl:NamedIndividual, cascade:TransitionEffect ;
    cascade:effectBindings cascade:noop_bind .
"#;
