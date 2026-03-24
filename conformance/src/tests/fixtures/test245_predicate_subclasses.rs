//! SHACL test 245: cascade predicate subclasses.

/// Instance graph for Test 245: Ten predicate subclasses.
pub const TEST245_PREDICATE_SUBCLASSES: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:ex_comparison a owl:NamedIndividual, cascade:ComparisonPredicate ;
    cascade:comparisonField "freeCount" ;
    cascade:comparisonOperator ">=" ;
    cascade:comparisonValue "3" .

cascade:ex_conjunction a owl:NamedIndividual, cascade:ConjunctionPredicate ;
    cascade:conjuncts "p1" ;
    cascade:conjuncts "p2" .

cascade:ex_disjunction a owl:NamedIndividual, cascade:DisjunctionPredicate ;
    cascade:disjuncts "q1" ;
    cascade:disjuncts "q2" .

cascade:ex_negation a owl:NamedIndividual, cascade:NegationPredicate ;
    cascade:negatedPredicate "p1" .

cascade:ex_membership a owl:NamedIndividual, cascade:MembershipPredicate ;
    cascade:membershipSet "active_fibers" ;
    cascade:membershipElement "fiber_7" .

cascade:ex_saturation a owl:NamedIndividual, cascade:SaturationPredicate ;
    cascade:saturationThreshold "0.75" .

cascade:ex_coverage a owl:NamedIndividual, cascade:FiberCoveragePredicate ;
    cascade:coverageTarget "all_critical" .

cascade:ex_equals a owl:NamedIndividual, cascade:EqualsPredicate ;
    cascade:equalityLeft "x" ;
    cascade:equalityRight "y" .

cascade:ex_non_null a owl:NamedIndividual, cascade:NonNullPredicate ;
    cascade:nonNullField "resolver_output" .

cascade:ex_query_subtype a owl:NamedIndividual, cascade:QuerySubtypePredicate ;
    cascade:queryTypeRef "SessionQuery" .
"#;
