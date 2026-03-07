/// SHACL test 73: Collapsed fiber state — CollapsedFiberState with
/// collapsedFrom, survivingAmplitude (Amendment 36).
pub const TEST73_COLLAPSED_FIBER_STATE: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix type:       <https://uor.foundation/type/> .

# 1. CollapsedFiberState
type:ex_cfs_73 a owl:NamedIndividual, type:CollapsedFiberState ;
    type:collapsedFrom type:ex_sfs_73 ;
    type:survivingAmplitude "1.0"^^xsd:decimal .

# 2. Referenced SuperposedFiberState
type:ex_sfs_73 a owl:NamedIndividual, type:SuperposedFiberState .
"#;
