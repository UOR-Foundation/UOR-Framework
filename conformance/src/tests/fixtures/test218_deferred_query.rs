//! SHACL test 218: `cascade:DeferredQuerySet` instance.

/// Instance graph for Test 218: DeferredQuerySet with count and epoch.
pub const TEST218_DEFERRED_QUERY: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:dq_example a owl:NamedIndividual, cascade:DeferredQuerySet ;
    cascade:deferredCount "5"^^xsd:nonNegativeInteger ;
    cascade:deferralEpoch "3"^^xsd:nonNegativeInteger .
"#;
