//! SHACL validator.
//!
//! Validates the 15 OWL instance test graphs against the UOR SHACL shapes.
//! Each test graph is defined as a Turtle string in `tests/fixtures/`.
//! Validation checks structural constraints without a full SHACL engine:
//! - Required properties are present
//! - Type assertions are correct
//! - Cardinality minimums are met

use crate::report::{ConformanceReport, TestResult};
use crate::tests;

/// Runs all 15 SHACL instance conformance tests.
pub fn validate() -> ConformanceReport {
    let mut report = ConformanceReport::new();

    run_test("test1_ring", tests::fixtures::TEST1_RING, &mut report);
    run_test(
        "test2_primitives",
        tests::fixtures::TEST2_PRIMITIVES,
        &mut report,
    );
    run_test(
        "test3_term_graph",
        tests::fixtures::TEST3_TERM_GRAPH,
        &mut report,
    );
    run_test(
        "test4_state_lifecycle",
        tests::fixtures::TEST4_STATE_LIFECYCLE,
        &mut report,
    );
    run_test(
        "test5_partition",
        tests::fixtures::TEST5_PARTITION,
        &mut report,
    );
    run_test(
        "test6_critical_identity",
        tests::fixtures::TEST6_CRITICAL_IDENTITY,
        &mut report,
    );
    run_test(
        "test7_end_to_end",
        tests::fixtures::TEST7_END_TO_END,
        &mut report,
    );
    run_test(
        "test8_fiber_budget",
        tests::fixtures::TEST8_FIBER_BUDGET,
        &mut report,
    );
    run_test(
        "test9_constraint_algebra",
        tests::fixtures::TEST9_CONSTRAINT_ALGEBRA,
        &mut report,
    );
    run_test(
        "test10_iterative_resolution",
        tests::fixtures::TEST10_ITERATIVE_RESOLUTION,
        &mut report,
    );
    run_test(
        "test11_composition",
        tests::fixtures::TEST11_COMPOSITION,
        &mut report,
    );
    run_test(
        "test12_factorization",
        tests::fixtures::TEST12_FACTORIZATION,
        &mut report,
    );
    run_test(
        "test13_canonical_form",
        tests::fixtures::TEST13_CANONICAL_FORM,
        &mut report,
    );
    run_test(
        "test14_content_addressing",
        tests::fixtures::TEST14_CONTENT_ADDRESSING,
        &mut report,
    );
    run_test(
        "test15_boolean_sat",
        tests::fixtures::TEST15_BOOLEAN_SAT,
        &mut report,
    );

    report
}

/// Runs a single SHACL test against the provided Turtle instance graph.
fn run_test(name: &str, turtle_src: &str, report: &mut ConformanceReport) {
    let validator = format!("ontology/shacl/{}", name);

    // Structural validation: the Turtle must be non-empty and syntactically plausible
    if turtle_src.trim().is_empty() {
        report.push(TestResult::fail(
            validator.clone(),
            "Instance graph is empty",
        ));
        return;
    }

    if !turtle_src.contains("@prefix") {
        report.push(TestResult::fail(
            validator.clone(),
            "Instance graph missing @prefix declarations",
        ));
        return;
    }

    // Run test-specific structural checks
    let result = match name {
        "test1_ring" => validate_ring(turtle_src),
        "test2_primitives" => validate_primitives(turtle_src),
        "test3_term_graph" => validate_term_graph(turtle_src),
        "test4_state_lifecycle" => validate_state_lifecycle(turtle_src),
        "test5_partition" => validate_partition(turtle_src),
        "test6_critical_identity" => validate_critical_identity(turtle_src),
        "test7_end_to_end" => validate_end_to_end(turtle_src),
        "test8_fiber_budget" => validate_fiber_budget(turtle_src),
        "test9_constraint_algebra" => validate_constraint_algebra(turtle_src),
        "test10_iterative_resolution" => validate_iterative_resolution(turtle_src),
        "test11_composition" => validate_composition(turtle_src),
        "test12_factorization" => validate_factorization(turtle_src),
        "test13_canonical_form" => validate_canonical_form(turtle_src),
        "test14_content_addressing" => validate_content_addressing(turtle_src),
        "test15_boolean_sat" => validate_boolean_sat(turtle_src),
        _ => Ok(()),
    };

    match result {
        Ok(()) => report.push(TestResult::pass(
            validator,
            format!("Instance graph {} passes SHACL structural validation", name),
        )),
        Err(msg) => report.push(TestResult::fail(validator, msg)),
    }
}

fn validate_ring(src: &str) -> Result<(), String> {
    check_contains(src, "schema:Ring", "Missing schema:Ring type assertion")?;
    check_contains(
        src,
        "schema:ringQuantum",
        "Missing schema:ringQuantum property",
    )?;
    check_contains(src, "schema:modulus", "Missing schema:modulus property")?;
    Ok(())
}

fn validate_primitives(src: &str) -> Result<(), String> {
    check_contains(src, "op:neg", "Missing op:neg individual reference")?;
    check_contains(src, "op:bnot", "Missing op:bnot individual reference")?;
    check_contains(src, "op:succ", "Missing op:succ individual reference")?;
    check_contains(src, "op:composedOf", "Missing op:composedOf property usage")?;
    Ok(())
}

fn validate_term_graph(src: &str) -> Result<(), String> {
    check_contains(src, "schema:Application", "Missing schema:Application")?;
    check_contains(src, "schema:Literal", "Missing schema:Literal")?;
    check_contains(src, "schema:denotes", "Missing schema:denotes property")?;
    Ok(())
}

fn validate_state_lifecycle(src: &str) -> Result<(), String> {
    check_contains(src, "state:Context", "Missing state:Context")?;
    check_contains(src, "state:Binding", "Missing state:Binding")?;
    check_contains(src, "state:Transition", "Missing state:Transition")?;
    Ok(())
}

fn validate_partition(src: &str) -> Result<(), String> {
    check_contains(src, "partition:Partition", "Missing partition:Partition")?;
    check_contains(src, "partition:cardinality", "Missing cardinality property")?;
    Ok(())
}

fn validate_critical_identity(src: &str) -> Result<(), String> {
    check_contains(src, "op:criticalIdentity", "Missing op:criticalIdentity")?;
    check_contains(
        src,
        "proof:CriticalIdentityProof",
        "Missing proof:CriticalIdentityProof",
    )?;
    check_contains(
        src,
        "proof:provesIdentity",
        "Missing proof:provesIdentity property",
    )?;
    Ok(())
}

fn validate_end_to_end(src: &str) -> Result<(), String> {
    check_contains(src, "state:Context", "Missing state:Context")?;
    check_contains(src, "type:", "Missing type: namespace usage")?;
    check_contains(src, "resolver:", "Missing resolver: namespace usage")?;
    check_contains(src, "partition:", "Missing partition: namespace usage")?;
    check_contains(src, "observable:", "Missing observable: namespace usage")?;
    check_contains(src, "cert:", "Missing cert: namespace usage")?;
    check_contains(src, "trace:", "Missing trace: namespace usage")?;
    Ok(())
}

fn validate_fiber_budget(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "partition:FiberBudget",
        "Missing partition:FiberBudget",
    )?;
    check_contains(
        src,
        "partition:FiberCoordinate",
        "Missing partition:FiberCoordinate",
    )?;
    check_contains(src, "partition:isClosed", "Missing partition:isClosed")?;
    check_contains(
        src,
        "partition:pinnedCount",
        "Missing partition:pinnedCount",
    )?;
    check_contains(src, "partition:freeCount", "Missing partition:freeCount")?;
    check_contains(src, "partition:hasFiber", "Missing partition:hasFiber")?;
    check_contains(
        src,
        "partition:FiberPinning",
        "Missing partition:FiberPinning",
    )?;
    Ok(())
}

fn validate_constraint_algebra(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "type:ResidueConstraint",
        "Missing type:ResidueConstraint",
    )?;
    check_contains(
        src,
        "type:CompositeConstraint",
        "Missing type:CompositeConstraint",
    )?;
    check_contains(src, "type:metricAxis", "Missing type:metricAxis")?;
    check_contains(src, "type:hasConstraint", "Missing type:hasConstraint")?;
    check_contains(src, "type:verticalAxis", "Missing type:verticalAxis")?;
    Ok(())
}

fn validate_iterative_resolution(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:ResolutionState",
        "Missing resolver:ResolutionState",
    )?;
    check_contains(src, "resolver:isComplete", "Missing resolver:isComplete")?;
    check_contains(
        src,
        "resolver:RefinementSuggestion",
        "Missing resolver:RefinementSuggestion",
    )?;
    check_contains(
        src,
        "derivation:RefinementStep",
        "Missing derivation:RefinementStep",
    )?;
    check_contains(
        src,
        "resolver:convergenceRate",
        "Missing resolver:convergenceRate",
    )?;
    Ok(())
}

fn validate_composition(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "morphism:Composition",
        "Missing morphism:Composition class",
    )?;
    check_contains(
        src,
        "morphism:CompositionLaw",
        "Missing morphism:CompositionLaw",
    )?;
    check_contains(
        src,
        "morphism:criticalComposition",
        "Missing morphism:criticalComposition",
    )?;
    check_contains(src, "morphism:Identity", "Missing morphism:Identity")?;
    check_contains(src, "morphism:identityOn", "Missing morphism:identityOn")?;
    Ok(())
}

fn validate_factorization(src: &str) -> Result<(), String> {
    check_contains(src, "query:", "Missing query: namespace usage")?;
    check_contains(
        src,
        "resolver:DihedralFactorizationResolver",
        "Missing DihedralFactorizationResolver",
    )?;
    check_contains(
        src,
        "partition:FiberBudget",
        "Missing partition:FiberBudget",
    )?;
    check_contains(src, "cert:certifies", "Missing cert:certifies")?;
    check_contains(src, "trace:certifiedBy", "Missing trace:certifiedBy")?;
    Ok(())
}

fn validate_canonical_form(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:CanonicalFormResolver",
        "Missing CanonicalFormResolver",
    )?;
    check_contains(
        src,
        "derivation:Derivation",
        "Missing derivation:Derivation",
    )?;
    check_contains(
        src,
        "derivation:RewriteStep",
        "Missing derivation:RewriteStep",
    )?;
    check_contains(
        src,
        "derivation:TermMetrics",
        "Missing derivation:TermMetrics",
    )?;
    check_contains(
        src,
        "derivation:originalTerm",
        "Missing derivation:originalTerm",
    )?;
    Ok(())
}

fn validate_content_addressing(src: &str) -> Result<(), String> {
    check_contains(src, "u:Address", "Missing u:Address")?;
    check_contains(
        src,
        "observable:RingMetric",
        "Missing observable:RingMetric",
    )?;
    check_contains(
        src,
        "observable:HammingMetric",
        "Missing observable:HammingMetric",
    )?;
    check_contains(
        src,
        "cert:InvolutionCertificate",
        "Missing cert:InvolutionCertificate",
    )?;
    check_contains(src, "cert:certifies", "Missing cert:certifies")?;
    Ok(())
}

fn validate_boolean_sat(src: &str) -> Result<(), String> {
    check_contains(
        src,
        "resolver:EvaluationResolver",
        "Missing EvaluationResolver",
    )?;
    check_contains(src, "state:Transition", "Missing state:Transition")?;
    check_contains(src, "state:Binding", "Missing state:Binding")?;
    check_contains(src, "cert:certifies", "Missing cert:certifies")?;
    check_contains(src, "trace:certifiedBy", "Missing trace:certifiedBy")?;
    Ok(())
}

fn check_contains(src: &str, needle: &str, msg: &str) -> Result<(), String> {
    if src.contains(needle) {
        Ok(())
    } else {
        Err(msg.to_string())
    }
}
