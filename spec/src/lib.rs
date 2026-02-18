//! UOR Foundation ontology encoded as typed Rust data.
//!
//! The `uor-spec` crate provides the complete UOR Foundation ontology —
//! 14 namespaces, 82 classes, 119 properties, and 14 named individuals —
//! as static Rust data structures, along with serializers that produce
//! JSON-LD, Turtle, and N-Triples output.
//!
//! # Entry Point
//!
//! ```
//! let ontology = uor_spec::Ontology::full();
//! assert_eq!(ontology.namespaces.len(), 14);
//! ```
//!
//! # Serialization
//!
//! ```
//! let ontology = uor_spec::Ontology::full();
//! let json_ld = uor_spec::serializer::jsonld::to_json_ld(&ontology);
//! let turtle  = uor_spec::serializer::turtle::to_turtle(&ontology);
//! ```

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    missing_docs,
    clippy::missing_errors_doc
)]

pub mod model;
pub mod namespaces;
pub mod serializer;

pub use model::{
    Class, Individual, IndividualValue, Namespace, NamespaceModule, Ontology, Property,
    PropertyKind, Space,
};

impl Ontology {
    /// Returns the complete UOR Foundation ontology with all 14 namespaces
    /// and all 8 amendments applied.
    ///
    /// Assembly order follows the dependency graph specified in the UOR Foundation
    /// completion plan:
    /// `u → schema → op → query → resolver → type → partition →
    ///  observable → proof → derivation → trace → cert → morphism → state`
    #[must_use]
    pub fn full() -> &'static Ontology {
        static ONTOLOGY: std::sync::OnceLock<Ontology> = std::sync::OnceLock::new();
        ONTOLOGY.get_or_init(|| Ontology {
            version: "1.0.0",
            base_iri: "https://uor.foundation/",
            namespaces: vec![
                namespaces::u::module(),
                namespaces::schema::module(),
                namespaces::op::module(),
                namespaces::query::module(),
                namespaces::resolver::module(),
                namespaces::type_::module(),
                namespaces::partition::module(),
                namespaces::observable::module(),
                namespaces::proof::module(),
                namespaces::derivation::module(),
                namespaces::trace::module(),
                namespaces::cert::module(),
                namespaces::morphism::module(),
                namespaces::state::module(),
            ],
            annotation_properties: vec![model::annotation_space_property()],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_count() {
        assert_eq!(Ontology::full().namespaces.len(), 14);
    }

    #[test]
    fn class_count() {
        let total: usize = Ontology::full()
            .namespaces
            .iter()
            .map(|m| m.classes.len())
            .sum();
        // 82 classes across 14 namespaces per the UOR Foundation ontology spec.
        assert_eq!(total, 82);
    }

    #[test]
    fn property_count() {
        // 120 = 119 namespace-level properties + 1 global uor:space annotation (Amendment 8).
        assert_eq!(Ontology::full().property_count(), 120);
    }

    #[test]
    fn individual_count() {
        let total: usize = Ontology::full()
            .namespaces
            .iter()
            .map(|m| m.individuals.len())
            .sum();
        // 14 individuals: 10 operations (Amendment 1) + pi1, zero (Amendment 2)
        // + criticalIdentity (Amendment 3) + D2n (Amendment 4).
        assert_eq!(total, 14);
    }

    #[test]
    fn all_class_iris_unique() {
        let mut iris = std::collections::HashSet::new();
        for module in &Ontology::full().namespaces {
            for class in &module.classes {
                assert!(iris.insert(class.id), "Duplicate class IRI: {}", class.id);
            }
        }
    }

    #[test]
    fn all_property_iris_unique() {
        let mut iris = std::collections::HashSet::new();
        for module in &Ontology::full().namespaces {
            for prop in &module.properties {
                assert!(iris.insert(prop.id), "Duplicate property IRI: {}", prop.id);
            }
        }
    }

    #[test]
    fn all_individual_iris_unique() {
        let mut iris = std::collections::HashSet::new();
        for module in &Ontology::full().namespaces {
            for ind in &module.individuals {
                assert!(iris.insert(ind.id), "Duplicate individual IRI: {}", ind.id);
            }
        }
    }

    #[test]
    fn space_annotations_on_all_namespaces() {
        for module in &Ontology::full().namespaces {
            // Every namespace must have a space classification.
            let _ = &module.namespace.space; // Space is non-optional; this compiles only if present.
        }
    }
}
