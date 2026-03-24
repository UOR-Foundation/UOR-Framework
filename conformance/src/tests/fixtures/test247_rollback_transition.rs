//! SHACL test 247: `cascade:ComplexConjugateRollback` and `cascade:CascadeTransitionRule`.

/// Instance graph for Test 247: ComplexConjugateRollback and CascadeTransitionRule.
pub const TEST247_ROLLBACK_TRANSITION: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:ex_rollback_247 a owl:NamedIndividual, cascade:ComplexConjugateRollback ;
    cascade:rollbackTarget "stage_2" .

cascade:ex_rule_247 a owl:NamedIndividual, cascade:CascadeTransitionRule ;
    cascade:transitionGuard cascade:ex_guard_247 ;
    cascade:transitionEffect cascade:ex_effect_247 .
"#;
