//! Lean 4 individual constant generator.
//!
//! Generates `namespace` blocks with `def` constants for each named individual
//! in the ontology.

use std::collections::{BTreeMap, HashSet};
use std::fmt::Write as FmtWrite;

use uor_ontology::model::{IndividualValue, NamespaceModule};

use crate::emit::normalize_lean_comment;
use crate::mapping::local_name;

/// Generates individual constant namespaces for a single ontology module.
///
/// Returns the generated Lean source as a string.
pub fn generate_individuals(module: &NamespaceModule, skip_types: &HashSet<&str>) -> String {
    let mut buf = String::new();

    for ind in &module.individuals {
        let type_local = local_name(ind.type_);

        // Skip enum-type individuals (they become inductive variants)
        if skip_types.contains(type_local)
            || type_local == "UnaryOp"
            || type_local == "BinaryOp"
            || type_local == "Involution"
        {
            continue;
        }

        let mod_name = to_individual_name(local_name(ind.id));
        let comment = normalize_lean_comment(ind.comment);

        if ind.properties.is_empty() {
            let _ = writeln!(buf, "/-- {comment} -/");
            let _ = writeln!(buf, "namespace {mod_name} end {mod_name}");
            buf.push('\n');
            continue;
        }

        let _ = writeln!(buf, "/-- {comment} -/");
        let _ = writeln!(buf, "namespace {mod_name}");

        // Group properties by IRI
        let mut grouped: BTreeMap<&str, Vec<&IndividualValue>> = BTreeMap::new();
        for (prop_iri, value) in ind.properties {
            grouped.entry(prop_iri).or_default().push(value);
        }

        for (prop_iri, values) in &grouped {
            let prop_local = local_name(prop_iri);
            let const_name = to_screaming_snake(prop_local);

            // Check for List values first (highest priority)
            if values.iter().any(|v| matches!(v, IndividualValue::List(_))) {
                if let Some(IndividualValue::List(items)) = values
                    .iter()
                    .find(|v| matches!(v, IndividualValue::List(_)))
                {
                    let _ = write!(buf, "def {const_name} : Array String := #[");
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 {
                            buf.push_str(", ");
                        }
                        let _ = write!(buf, "\"{item}\"");
                    }
                    buf.push_str("]\n");
                }
                continue;
            }

            // Multiple IriRef values → Array
            if values.len() > 1
                && values
                    .iter()
                    .all(|v| matches!(v, IndividualValue::IriRef(_)))
            {
                let _ = write!(buf, "def {const_name} : Array String := #[");
                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        buf.push_str(", ");
                    }
                    if let IndividualValue::IriRef(iri) = v {
                        let _ = write!(buf, "\"{iri}\"");
                    }
                }
                buf.push_str("]\n");
                continue;
            }

            // Multiple Str values → Array
            if values.len() > 1 && values.iter().all(|v| matches!(v, IndividualValue::Str(_))) {
                let _ = write!(buf, "def {const_name} : Array String := #[");
                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        buf.push_str(", ");
                    }
                    if let IndividualValue::Str(s) = v {
                        let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
                        let _ = write!(buf, "\"{escaped}\"");
                    }
                }
                buf.push_str("]\n");
                continue;
            }

            // Single value
            match values[0] {
                IndividualValue::Str(s) => {
                    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
                    let _ = writeln!(buf, "def {const_name} : String := \"{escaped}\"");
                }
                IndividualValue::Int(n) => {
                    let _ = writeln!(buf, "def {const_name} : Int := {n}");
                }
                IndividualValue::Bool(b) => {
                    let _ = writeln!(buf, "def {const_name} : Bool := {b}");
                }
                IndividualValue::IriRef(iri) => {
                    let _ = writeln!(buf, "def {const_name} : String := \"{iri}\"");
                }
                IndividualValue::List(_) => {
                    // Already handled above
                }
            }
        }

        let _ = writeln!(buf, "end {mod_name}");
        buf.push('\n');
    }

    buf
}

/// Converts a local name to a Lean namespace name for an individual.
///
/// Uses camelCase (lowercasing the first character) per Lean convention
/// for non-type namespaces.
fn to_individual_name(s: &str) -> String {
    // Lean namespaces are scoped to their parent module, so no collision
    // with structure names (different declaration kinds in Lean 4).
    if is_lean_keyword(s) {
        format!("\u{ab}{s}\u{bb}")
    } else {
        s.to_string()
    }
}

/// Converts a camelCase property name to SCREAMING_SNAKE_CASE.
fn to_screaming_snake(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                let prev = s.as_bytes().get(i - 1).copied().unwrap_or(b'_');
                if (prev as char).is_lowercase() || (prev as char).is_ascii_digit() {
                    result.push('_');
                }
            }
            result.push(ch.to_uppercase().next().unwrap_or(ch));
        } else {
            result.push(ch.to_uppercase().next().unwrap_or(ch));
        }
    }
    result
}

/// Checks if a name is a Lean 4 keyword.
fn is_lean_keyword(s: &str) -> bool {
    matches!(
        s,
        "type"
            | "def"
            | "where"
            | "structure"
            | "class"
            | "theorem"
            | "let"
            | "do"
            | "return"
            | "match"
            | "if"
            | "else"
            | "for"
            | "in"
            | "open"
            | "namespace"
            | "end"
            | "import"
            | "mutual"
            | "variable"
            | "instance"
            | "deriving"
            | "extends"
            | "with"
            | "fun"
            | "have"
            | "show"
            | "calc"
            | "by"
            | "sorry"
            | "set_option"
            | "section"
            | "true"
            | "false"
    )
}

/// Returns the count of non-enum individuals in a module.
pub fn count_individuals(module: &NamespaceModule, skip_types: &HashSet<&str>) -> usize {
    module
        .individuals
        .iter()
        .filter(|ind| {
            let type_local = local_name(ind.type_);
            !skip_types.contains(type_local)
                && type_local != "UnaryOp"
                && type_local != "BinaryOp"
                && type_local != "Involution"
        })
        .count()
}
