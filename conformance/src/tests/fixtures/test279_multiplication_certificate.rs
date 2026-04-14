//! Test 279: v0.2.2 Phase C.4 — MultiplicationCertificate + MultiplicationResolver.
//!
//! Validates SHACL coverage for the Phase C.4 ontology additions backing the
//! multiplication resolver's closed-form Landauer cost decision procedure:
//! - cert:MultiplicationCertificate with splittingFactor, subMultiplicationCount,
//!   landauerCostNats evidence.
//! - resolver:MultiplicationResolver.
//! - resolver:multiplicationCertifyMapping CertifyMapping individual.
//! - linear:stackBudgetBytes on a LinearBudget.

/// Instance graph for Test 279: v0.2.2 Phase C.4 multiplication resolver.
pub const TEST279_MULTIPLICATION_CERTIFICATE: &str = r#"
@prefix rdf:       <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:       <http://www.w3.org/2002/07/owl#> .
@prefix xsd:       <http://www.w3.org/2001/XMLSchema#> .
@prefix cert:      <https://uor.foundation/cert/> .
@prefix resolver:  <https://uor.foundation/resolver/> .
@prefix linear:    <https://uor.foundation/linear/> .

# 1. cert:MultiplicationCertificate — resolver-issued multiplication cost proof.
<https://uor.foundation/instance/cert/mult_karatsuba_w4096>
    a                             owl:NamedIndividual, cert:MultiplicationCertificate ;
    cert:splittingFactor          "2"^^xsd:positiveInteger ;
    cert:subMultiplicationCount   "3"^^xsd:nonNegativeInteger ;
    cert:landauerCostNats         "0.0665"^^xsd:decimal .

# 2. resolver:MultiplicationResolver — the resolver class itself, used as a
# target for the CertifyMapping below.
<https://uor.foundation/instance/resolver/mult_resolver>
    a                             owl:NamedIndividual, resolver:MultiplicationResolver .

# 3. linear:LinearBudget extended with linear:stackBudgetBytes for the
# multiplication resolver's stack-budget reasoning.
<https://uor.foundation/instance/linear/mult_call_site>
    a                             owl:NamedIndividual, linear:LinearBudget ;
    linear:stackBudgetBytes       "16384"^^xsd:nonNegativeInteger .
"#;
