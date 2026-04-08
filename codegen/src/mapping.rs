//! Ontology → Rust mapping tables.
//!
//! Deterministic mappings from OWL constructs to Rust identifiers, modules,
//! and types.

use std::collections::HashMap;

use uor_ontology::model::iris::*;
use uor_ontology::model::Space;

/// Mapping from a namespace IRI to its Rust module path.
pub struct NamespaceMapping {
    /// The space classification (Kernel, Bridge, User).
    pub space: Space,
    /// e.g. "kernel", "bridge", "user"
    pub space_module: &'static str,
    /// e.g. "address", "schema", "op"
    pub file_module: &'static str,
}

/// Returns the namespace → module mapping.
pub fn namespace_mappings() -> HashMap<&'static str, NamespaceMapping> {
    let mut m = HashMap::new();
    m.insert(
        NS_U,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "address",
        },
    );
    m.insert(
        NS_SCHEMA,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "schema",
        },
    );
    m.insert(
        NS_OP,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "op",
        },
    );
    m.insert(
        NS_QUERY,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "query",
        },
    );
    m.insert(
        NS_RESOLVER,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "resolver",
        },
    );
    m.insert(
        NS_PARTITION,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "partition",
        },
    );
    m.insert(
        NS_OBSERVABLE,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "observable",
        },
    );
    m.insert(
        NS_PROOF,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "proof",
        },
    );
    m.insert(
        NS_DERIVATION,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "derivation",
        },
    );
    m.insert(
        NS_TRACE,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "trace",
        },
    );
    m.insert(
        NS_CERT,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "cert",
        },
    );
    m.insert(
        NS_HOMOLOGY,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "homology",
        },
    );
    m.insert(
        NS_COHOMOLOGY,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "cohomology",
        },
    );
    m.insert(
        NS_CARRY,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "carry",
        },
    );
    m.insert(
        NS_REDUCTION,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "reduction",
        },
    );
    m.insert(
        NS_CONVERGENCE,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "convergence",
        },
    );
    m.insert(
        NS_DIVISION,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "division",
        },
    );
    m.insert(
        NS_INTERACTION,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "interaction",
        },
    );
    m.insert(
        NS_MONOIDAL,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "monoidal",
        },
    );
    m.insert(
        NS_OPERAD,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "operad",
        },
    );
    m.insert(
        NS_EFFECT,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "effect",
        },
    );
    m.insert(
        NS_PARALLEL,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "parallel",
        },
    );
    m.insert(
        NS_CONFORMANCE,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "conformance_",
        },
    );
    m.insert(
        NS_BOUNDARY,
        NamespaceMapping {
            space: Space::Bridge,
            space_module: "bridge",
            file_module: "boundary",
        },
    );
    m.insert(
        NS_REGION,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "region",
        },
    );
    m.insert(
        NS_RECURSION,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "recursion",
        },
    );
    m.insert(
        NS_LINEAR,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "linear",
        },
    );
    m.insert(
        NS_FAILURE,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "failure",
        },
    );
    m.insert(
        NS_STREAM,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "stream",
        },
    );
    m.insert(
        NS_PREDICATE,
        NamespaceMapping {
            space: Space::Kernel,
            space_module: "kernel",
            file_module: "predicate",
        },
    );
    m.insert(
        NS_TYPE,
        NamespaceMapping {
            space: Space::User,
            space_module: "user",
            file_module: "type_",
        },
    );
    m.insert(
        NS_MORPHISM,
        NamespaceMapping {
            space: Space::User,
            space_module: "user",
            file_module: "morphism",
        },
    );
    m.insert(
        NS_STATE,
        NamespaceMapping {
            space: Space::User,
            space_module: "user",
            file_module: "state",
        },
    );
    m
}

/// Maps an XSD IRI to the corresponding `P::` associated type expression.
pub fn xsd_to_primitives_type(xsd_iri: &str) -> Option<&'static str> {
    match xsd_iri {
        XSD_STRING => Some("P::String"),
        XSD_INTEGER => Some("P::Integer"),
        XSD_NON_NEGATIVE_INTEGER => Some("P::NonNegativeInteger"),
        XSD_POSITIVE_INTEGER => Some("P::PositiveInteger"),
        XSD_BOOLEAN => Some("P::Boolean"),
        XSD_DECIMAL => Some("P::Decimal"),
        XSD_DATETIME => Some("P::String"), // DateTime mapped to String for flexibility
        XSD_HEX_BINARY => Some("P::String"), // HexBinary mapped to String
        _ => None,
    }
}

/// Returns true if the XSD type is `?Sized` (i.e., String which maps to `str`).
pub fn xsd_is_unsized(xsd_iri: &str) -> bool {
    xsd_iri == XSD_STRING || xsd_iri == XSD_DATETIME || xsd_iri == XSD_HEX_BINARY
}

/// Converts a camelCase or PascalCase label into a snake_case Rust identifier.
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                // Don't add underscore before consecutive uppercase (e.g., "D2n")
                let prev = s.as_bytes()[i - 1] as char;
                if prev.is_lowercase() || prev.is_ascii_digit() {
                    result.push('_');
                }
            }
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            result.push(ch);
        }
    }
    // Handle Rust keywords
    match result.as_str() {
        "type" | "self" | "super" | "crate" | "mod" | "fn" | "pub" | "use" | "let" | "mut"
        | "ref" | "as" | "in" | "for" | "if" | "else" | "match" | "return" | "struct" | "enum"
        | "trait" | "impl" | "where" | "loop" | "while" | "break" | "continue" | "move" | "box"
        | "dyn" | "true" | "false" => {
            result.push('_');
            result
        }
        _ => result,
    }
}

/// Converts a class label into a PascalCase Rust trait name.
pub fn to_trait_name(label: &str) -> String {
    // Already PascalCase in the ontology (e.g., "FreeRank", "IrreducibleSet")
    label.to_string()
}

/// Extracts the local name from a full IRI (after the last `/` or `#`).
pub fn local_name(iri: &str) -> &str {
    let after_slash = iri.rsplit('/').next().unwrap_or(iri);
    after_slash.rsplit('#').next().unwrap_or(after_slash)
}

/// Returns the crate-relative module path for a class IRI.
///
/// E.g. `"https://uor.foundation/partition/Partition"` → `"crate::bridge::partition"`
pub fn class_module_path(
    class_iri: &str,
    ns_map: &HashMap<&str, NamespaceMapping>,
) -> Option<String> {
    // Find which namespace this class belongs to
    for (ns_iri, mapping) in ns_map {
        if class_iri.starts_with(ns_iri) {
            return Some(format!(
                "crate::{}::{}",
                mapping.space_module, mapping.file_module
            ));
        }
    }
    None
}

/// Returns the fully-qualified trait path for a class IRI.
///
/// E.g. `"https://uor.foundation/partition/Partition"` → `"crate::bridge::partition::Partition"`
pub fn class_trait_path(
    class_iri: &str,
    ns_map: &HashMap<&str, NamespaceMapping>,
) -> Option<String> {
    let module = class_module_path(class_iri, ns_map)?;
    let name = local_name(class_iri);
    Some(format!("{module}::{name}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_case_conversion() {
        assert_eq!(to_snake_case("freeRank"), "free_rank");
        assert_eq!(to_snake_case("isClosed"), "is_closed");
        assert_eq!(to_snake_case("sourceType"), "source_type");
        assert_eq!(to_snake_case("type"), "type_");
    }

    #[test]
    fn local_name_extraction() {
        assert_eq!(
            local_name("https://uor.foundation/partition/Partition"),
            "Partition"
        );
        assert_eq!(
            local_name("http://www.w3.org/2001/XMLSchema#string"),
            "string"
        );
    }
}
