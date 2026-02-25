//! Enum detection and generation.
//!
//! Identifies closed sets of named individuals that should be represented
//! as Rust enums, and generates the enum definitions with derives.

use std::fmt::Write as FmtWrite;

use uor_ontology::Ontology;

use crate::emit::{normalize_comment, RustFile};
use crate::mapping::local_name;

/// Detected enum type to generate.
pub struct DetectedEnum {
    /// Rust enum name.
    pub name: &'static str,
    /// Doc comment.
    pub comment: &'static str,
    /// Variants: (variant_name, doc_comment).
    pub variants: Vec<(String, String)>,
}

/// Detects all enums from the ontology.
pub fn detect_enums(ontology: &Ontology) -> Vec<DetectedEnum> {
    let mut enums = Vec::new();

    // 1. Space enum (already exists in the ontology model)
    enums.push(DetectedEnum {
        name: "Space",
        comment: "Kernel/user/bridge classification for each namespace module.",
        variants: vec![
            (
                "Kernel".to_string(),
                "Immutable kernel-space: compiled into ROM.".to_string(),
            ),
            (
                "User".to_string(),
                "Parameterizable user-space: runtime declarations.".to_string(),
            ),
            (
                "Bridge".to_string(),
                "Bridge: kernel-computed, user-consumed.".to_string(),
            ),
        ],
    });

    // 2. PrimitiveOp enum from the 10 operation individuals
    let op_ns = ontology.find_namespace("op");
    if let Some(op_module) = op_ns {
        let op_variants: Vec<(String, String)> = op_module
            .individuals
            .iter()
            .filter(|ind| {
                let t = local_name(ind.type_);
                t == "UnaryOp" || t == "BinaryOp" || t == "Involution"
            })
            .map(|ind| {
                let name = capitalize(local_name(ind.id));
                let comment = normalize_comment(ind.comment);
                (name, comment)
            })
            .collect();

        if !op_variants.is_empty() {
            enums.push(DetectedEnum {
                name: "PrimitiveOp",
                comment: "The 10 primitive operations defined in the UOR Foundation.",
                variants: op_variants,
            });
        }
    }

    // 3. MetricAxis enum from the 3 metric axis individuals
    let type_ns = ontology.find_namespace("type");
    if let Some(type_module) = type_ns {
        let axis_variants: Vec<(String, String)> = type_module
            .individuals
            .iter()
            .filter(|ind| local_name(ind.type_) == "MetricAxis")
            .map(|ind| {
                let mut name = capitalize(local_name(ind.id));
                // Strip "Axis" suffix to avoid clippy::enum_variant_names
                // (enum is already called MetricAxis)
                if name.ends_with("Axis") {
                    name.truncate(name.len() - 4);
                }
                let comment = normalize_comment(ind.comment);
                (name, comment)
            })
            .collect();

        if !axis_variants.is_empty() {
            enums.push(DetectedEnum {
                name: "MetricAxis",
                comment: "The three metric axes in the UOR tri-metric classification.",
                variants: axis_variants,
            });
        }
    }

    // 4. FiberState enum (from ontology comments)
    enums.push(DetectedEnum {
        name: "FiberState",
        comment: "The state of a fiber coordinate: pinned or free.",
        variants: vec![
            (
                "Pinned".to_string(),
                "Fiber is determined by a constraint.".to_string(),
            ),
            (
                "Free".to_string(),
                "Fiber is still available for refinement.".to_string(),
            ),
        ],
    });

    // 5. GeometricCharacter enum — collected from actual individual property values
    if let Some(op_module) = op_ns {
        let mut gc_values: Vec<String> = Vec::new();
        for ind in &op_module.individuals {
            for (prop_iri, value) in ind.properties {
                if local_name(prop_iri) == "geometricCharacter" {
                    if let uor_ontology::IndividualValue::Str(s) = value {
                        if !gc_values.contains(&s.to_string()) {
                            gc_values.push(s.to_string());
                        }
                    }
                }
            }
        }
        if !gc_values.is_empty() {
            let gc_variants: Vec<(String, String)> = gc_values
                .iter()
                .map(|v| {
                    let variant = snake_to_pascal(v);
                    let comment = format!("{v} geometric character.");
                    (variant, comment)
                })
                .collect();
            enums.push(DetectedEnum {
                name: "GeometricCharacter",
                comment: "The geometric character of an operation.",
                variants: gc_variants,
            });
        }
    }

    enums
}

/// Generates the `enums.rs` file content.
pub fn generate_enums_file(ontology: &Ontology) -> String {
    let enums = detect_enums(ontology);
    let mut f = RustFile::new("Shared enumerations derived from the UOR Foundation ontology.");

    f.line("use core::fmt;");
    f.blank();

    for e in &enums {
        f.doc_comment(e.comment);
        f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
        let _ = writeln!(f.buf, "pub enum {} {{", e.name);
        for (variant, comment) in &e.variants {
            f.indented_doc_comment(comment);
            let _ = writeln!(f.buf, "    {variant},");
        }
        f.line("}");
        f.blank();

        // Display impl
        let _ = writeln!(f.buf, "impl fmt::Display for {} {{", e.name);
        f.line("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {");
        f.line("        match self {");
        for (variant, _) in &e.variants {
            let display = to_display_str(variant);
            let _ = writeln!(
                f.buf,
                "            Self::{variant} => f.write_str(\"{display}\"),"
            );
        }
        f.line("        }");
        f.line("    }");
        f.line("}");
        f.blank();
    }

    f.finish()
}

/// Converts a snake_case string to PascalCase (e.g., "ring_reflection" → "RingReflection").
fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let mut result = c.to_uppercase().to_string();
                    result.push_str(chars.as_str());
                    result
                }
            }
        })
        .collect()
}

/// Capitalizes the first character of a string.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => {
            let mut result = c.to_uppercase().to_string();
            result.push_str(chars.as_str());
            result
        }
    }
}

/// Converts a PascalCase variant to a display string (e.g., "VerticalAxis" → "vertical_axis").
fn to_display_str(s: &str) -> String {
    crate::mapping::to_snake_case(s)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn enum_detection_finds_all() {
        let ontology = Ontology::full();
        let enums = detect_enums(ontology);
        assert!(
            enums.len() >= 5,
            "Expected at least 5 enums, got {}",
            enums.len()
        );

        let names: Vec<&str> = enums.iter().map(|e| e.name).collect();
        assert!(names.contains(&"Space"));
        assert!(names.contains(&"PrimitiveOp"));
        assert!(names.contains(&"MetricAxis"));
        assert!(names.contains(&"FiberState"));
        assert!(names.contains(&"GeometricCharacter"));
    }

    #[test]
    fn primitive_op_has_10_variants() {
        let ontology = Ontology::full();
        let enums = detect_enums(ontology);
        let prim_op = enums.iter().find(|e| e.name == "PrimitiveOp").unwrap();
        assert_eq!(prim_op.variants.len(), 10);
    }
}
