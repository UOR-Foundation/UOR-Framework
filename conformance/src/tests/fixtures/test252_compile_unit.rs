//! SHACL test 252: `cascade:CompileUnit` with admission properties.

/// Instance graph for Test 252: CompileUnit with required properties.
pub const TEST252_COMPILE_UNIT: &str = r#"
@prefix rdf:         <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:         <http://www.w3.org/2002/07/owl#> .
@prefix xsd:         <http://www.w3.org/2001/XMLSchema#> .
@prefix schema:      <https://uor.foundation/schema/> .
@prefix op:          <https://uor.foundation/op/> .
@prefix u:           <https://uor.foundation/u/> .
@prefix cascade:     <https://uor.foundation/cascade/> .

cascade:ex_compile_unit_252 a owl:NamedIndividual, cascade:CompileUnit ;
    cascade:rootTerm         cascade:ex_root_term_252 ;
    cascade:unitQuantumLevel schema:Q0 ;
    cascade:targetDomains    op:Algebraic ;
    cascade:targetDomains    op:Pipeline ;
    cascade:thermodynamicBudget "6.0"^^xsd:decimal ;
    cascade:unitAddress      cascade:ex_address_252 .

cascade:ex_root_term_252 a owl:NamedIndividual, schema:Term .
cascade:ex_address_252   a owl:NamedIndividual, u:Address .
"#;
