//! UOR Framework conformance suite.
//!
//! This crate is the workspace-wide standard index and validator. It documents
//! the professional specifications that every component must satisfy and
//! implements automated validators to enforce them.
//!
//! # Conformance Scope
//!
//! | Component | Standard |
//! |-----------|----------|
//! | Rust source | Rust API Guidelines, edition 2021, clippy deny list |
//! | Ontology (JSON-LD) | JSON-LD 1.1, OWL 2 DL |
//! | Ontology (Turtle/N-Triples) | RDF 1.1, Turtle 1.1 |
//! | Ontology (EBNF) | ISO/IEC 14977 EBNF |
//! | Ontology (OWL RDF/XML) | OWL 2 RDF/XML |
//! | Ontology (JSON Schema) | JSON Schema Draft 2020-12 |
//! | Ontology (SHACL shapes) | SHACL W3C shapes |
//! | Instance graphs | SHACL W3C spec |
//! | Documentation | Diataxis framework, completeness, accuracy |
//! | Website | HTML5, WCAG 2.1 AA, CSS |
//!
//! # Entry Point
//!
//! ```no_run
//! use uor_conformance::{WorkspacePaths, run_all};
//! use std::path::PathBuf;
//!
//! let paths = WorkspacePaths {
//!     workspace: PathBuf::from("."),
//!     artifacts: PathBuf::from("public"),
//! };
//! let report = run_all(&paths).expect("Failed to run conformance");
//! assert!(report.all_passed());
//! ```

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    missing_docs,
    clippy::missing_errors_doc
)]

pub mod report;
pub mod tests;
pub mod validators;

pub use report::{ConformanceReport, Severity, TestResult};

/// Paths required by the conformance runner.
pub struct WorkspacePaths {
    /// Root of the Rust workspace (contains Cargo.toml, spec/, clients/, etc.)
    pub workspace: std::path::PathBuf,
    /// Directory containing built artifacts (uor.foundation.jsonld, docs/, etc.)
    pub artifacts: std::path::PathBuf,
}

/// Runs all conformance validators and returns the aggregated report.
///
/// Validators are run in this order:
/// 1. Rust source standards (style, API surface)
/// 2. Ontology inventory (counts must match `uor_ontology::counts`)
/// 3. Ontology JSON-LD 1.1
/// 4. Ontology OWL 2 DL
/// 5. Ontology RDF 1.1 / Turtle 1.1
/// 6. Ontology EBNF grammar
/// 7. SHACL instance conformance
/// 8. Documentation completeness and accuracy
/// 9. Website HTML5, WCAG, CSS, coverage
///
/// # Errors
///
/// Returns an error only if a file system operation fails.
pub fn run_all(paths: &WorkspacePaths) -> anyhow::Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    // 1. Rust source standards
    report.extend(validators::rust::style::validate(&paths.workspace)?);
    report.extend(validators::rust::api::validate(&paths.workspace)?);
    // v0.2.2 W6: public-API snapshot pin (drift detector).
    report.extend(validators::rust::public_api_snapshot::validate(
        &paths.workspace,
    )?);

    // 2. Ontology inventory
    report.extend(validators::ontology::inventory::validate(&paths.artifacts)?);

    // 2b. Workspace-level inventory (shapes count)
    report.extend(validators::ontology::inventory::validate_workspace(
        &paths.workspace,
    )?);

    // 2c. Required-property coverage: fail if any `required: true`
    //     property is missing an assertion on any matching individual.
    //     Fast in-memory drift guard — runs before any codegen.
    let ontology = uor_ontology::Ontology::full();
    report.extend(validators::ontology::required_property_coverage::validate(
        ontology,
    )?);

    // 3. JSON-LD 1.1
    report.extend(validators::ontology::jsonld::validate(&paths.artifacts)?);

    // 4. OWL 2 DL (operates on live spec, no file I/O)
    report.extend(validators::ontology::owl::validate());

    // 5. RDF 1.1 / Turtle 1.1
    report.extend(validators::ontology::rdf::validate(&paths.artifacts)?);

    // 5b. EBNF grammar (Amendment 42)
    report.extend(validators::ontology::ebnf::validate(&paths.artifacts)?);

    // 5c. OWL RDF/XML artifact
    report.extend(validators::ontology::owl_xml::validate(&paths.artifacts)?);

    // 5d. JSON Schema artifact
    report.extend(validators::ontology::json_schema::validate(
        &paths.artifacts,
    )?);

    // 5e. SHACL shapes artifact
    report.extend(validators::ontology::shacl_shapes::validate(
        &paths.artifacts,
    )?);

    // 6. SHACL instance conformance
    report.extend(validators::ontology::shacl::validate());

    // 6b. Generated crate conformance
    report.extend(validators::ontology::crate_::validate(&paths.workspace)?);

    // 6b2. Declarative enforcement module
    report.extend(validators::ontology::enforcement::validate(
        &paths.workspace,
    )?);

    // 6c. Standards document counts
    report.extend(validators::ontology::standards::validate(&paths.workspace)?);

    // 7. Documentation
    report.extend(validators::docs::completeness::validate(&paths.artifacts)?);
    report.extend(validators::docs::accuracy::validate(&paths.artifacts)?);
    report.extend(validators::docs::structure::validate(&paths.artifacts)?);
    report.extend(validators::docs::links::validate(&paths.artifacts)?);
    // v0.2.2 W5: ψ vocabulary leak gate (consumer-facing surface).
    report.extend(validators::docs::psi_leakage::validate(&paths.workspace)?);

    // 8. Website
    report.extend(validators::website::html::validate(&paths.artifacts)?);
    report.extend(validators::website::accessibility::validate(
        &paths.artifacts,
    )?);
    report.extend(validators::website::coverage::validate(&paths.artifacts)?);
    report.extend(validators::website::css::validate(&paths.artifacts)?);
    report.extend(validators::website::links::validate(&paths.artifacts)?);

    // 8b. Website nav structure
    report.extend(validators::website::nav::validate(&paths.artifacts)?);
    // 8c. Website design system
    report.extend(validators::website::design::validate(&paths.artifacts)?);
    // 8d. New page existence
    report.extend(validators::website::pages::validate(&paths.artifacts)?);
    // 8e. Visual elements (SVG)
    report.extend(validators::website::visual::validate(&paths.artifacts)?);
    // 8f. Bootstrap framework integration
    report.extend(validators::website::bootstrap::validate(&paths.artifacts)?);

    // 9. Lean 4 formalization
    report.extend(validators::lean4::structure::validate(&paths.workspace)?);
    report.extend(validators::lean4::build::validate(&paths.workspace)?);
    report.extend(validators::lean4::individual_proof::validate(
        &paths.workspace,
    )?);

    Ok(report)
}

#[cfg(test)]
mod tests_unit {
    use super::*;

    #[test]
    fn spec_inventory_passes() {
        let ontology = uor_ontology::Ontology::full();
        assert_eq!(ontology.namespaces.len(), uor_ontology::counts::NAMESPACES);
        assert_eq!(ontology.class_count(), uor_ontology::counts::CLASSES);
        assert_eq!(ontology.property_count(), uor_ontology::counts::PROPERTIES);
        assert_eq!(
            ontology.individual_count(),
            uor_ontology::counts::INDIVIDUALS
        );
    }

    #[test]
    fn owl_dl_constraints_pass() {
        let report = validators::ontology::owl::validate();
        let failures: Vec<_> = report.results.iter().filter(|r| r.is_failure()).collect();
        assert!(
            failures.is_empty(),
            "OWL 2 DL constraint failures: {:#?}",
            failures
        );
    }

    #[test]
    fn shacl_instances_pass() {
        let report = validators::ontology::shacl::validate();
        let failures: Vec<_> = report.results.iter().filter(|r| r.is_failure()).collect();
        assert!(
            failures.is_empty(),
            "SHACL conformance failures: {:#?}",
            failures
        );
    }
}
