//! Builds the site model from `uor_spec::Ontology` and `uor_docs` content.

use uor_spec::{NamespaceModule, Ontology};

use crate::model::{BreadcrumbItem, NamespaceSummary, SearchEntry};

/// Builds the list of namespace summaries for the homepage grid.
pub fn namespace_summaries() -> Vec<NamespaceSummary> {
    let ontology = Ontology::full();
    ontology
        .namespaces
        .iter()
        .map(namespace_summary_from_module)
        .collect()
}

/// Converts a namespace module to a `NamespaceSummary`.
fn namespace_summary_from_module(module: &NamespaceModule) -> NamespaceSummary {
    let ns = &module.namespace;
    NamespaceSummary {
        prefix: ns.prefix.to_string(),
        iri: ns.iri.to_string(),
        label: ns.label.to_string(),
        comment: ns.comment.to_string(),
        space: format!("{:?}", ns.space).to_lowercase(),
        url: format!("/namespaces/{}/", ns.prefix),
        class_count: module.classes.len(),
        property_count: module.properties.len(),
        individual_count: module.individuals.len(),
    }
}

/// Builds the full search index from all ontology terms.
pub fn build_search_index() -> Vec<SearchEntry> {
    let ontology = Ontology::full();
    let mut entries = Vec::new();

    for module in &ontology.namespaces {
        let prefix = module.namespace.prefix;

        // Namespace itself
        entries.push(SearchEntry {
            label: module.namespace.label.to_string(),
            description: module.namespace.comment.to_string(),
            url: format!("/namespaces/{}/", prefix),
            kind: "namespace".to_string(),
        });

        // Classes
        for class in &module.classes {
            entries.push(SearchEntry {
                label: class.label.to_string(),
                description: class.comment.to_string(),
                url: format!("/namespaces/{}/#class-{}", prefix, local_name(class.id)),
                kind: "class".to_string(),
            });
        }

        // Properties
        for prop in &module.properties {
            entries.push(SearchEntry {
                label: prop.label.to_string(),
                description: prop.comment.to_string(),
                url: format!("/namespaces/{}/#prop-{}", prefix, local_name(prop.id)),
                kind: "property".to_string(),
            });
        }

        // Individuals
        for ind in &module.individuals {
            entries.push(SearchEntry {
                label: ind.label.to_string(),
                description: ind.comment.to_string(),
                url: format!("/namespaces/{}/#ind-{}", prefix, local_name(ind.id)),
                kind: "individual".to_string(),
            });
        }
    }

    entries
}

/// Extracts the local name from an IRI.
fn local_name(iri: &str) -> &str {
    iri.rsplit('/').next().unwrap_or(iri)
}

/// Builds standard breadcrumbs for a namespace page.
pub fn namespace_breadcrumbs(label: &str) -> Vec<BreadcrumbItem> {
    vec![
        BreadcrumbItem {
            label: "Home".to_string(),
            url: "/".to_string(),
        },
        BreadcrumbItem {
            label: "Namespaces".to_string(),
            url: "/namespaces/".to_string(),
        },
        BreadcrumbItem {
            label: label.to_string(),
            url: String::new(),
        },
    ]
}

/// Builds standard breadcrumbs for the homepage.
pub fn home_breadcrumbs() -> Vec<BreadcrumbItem> {
    vec![BreadcrumbItem {
        label: "Home".to_string(),
        url: "/".to_string(),
    }]
}

/// Builds breadcrumbs for the namespaces index page.
pub fn namespaces_index_breadcrumbs() -> Vec<BreadcrumbItem> {
    vec![
        BreadcrumbItem {
            label: "Home".to_string(),
            url: "/".to_string(),
        },
        BreadcrumbItem {
            label: "Namespaces".to_string(),
            url: String::new(),
        },
    ]
}
