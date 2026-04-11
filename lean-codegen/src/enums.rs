//! Lean 4 inductive type generator for enum classes.
//!
//! Generates `UOR/Enums.lean` containing all 18 enum classes as inductives
//! (or structures for open-world types like WittLevel), plus hardcoded
//! codegen enums (Space, PrimitiveOp, SiteState, ProofModality).

use std::fmt::Write as FmtWrite;

use uor_ontology::model::{IndividualValue, Ontology};

use crate::emit::{normalize_lean_comment, LeanFile};
use crate::mapping::local_name;

/// A detected enum with its variants.
struct DetectedEnum {
    /// Lean inductive name.
    name: String,
    /// Doc comment.
    comment: String,
    /// (variant_name, variant_comment) pairs.
    variants: Vec<(String, String)>,
}

/// Generates the content of `UOR/Enums.lean`.
pub fn generate_enums(ontology: &Ontology) -> String {
    let mut f = LeanFile::new("Controlled vocabulary types (enum classes).");
    f.line("import UOR.Primitives");
    f.blank();
    f.line("open UOR.Primitives");
    f.blank();

    let enums = detect_enums(ontology);

    for e in &enums {
        f.doc_comment(&e.comment);
        let _ = writeln!(f.buf, "inductive {} where", e.name);
        for (variant, comment) in &e.variants {
            f.indented_doc_comment(comment);
            let _ = writeln!(f.buf, "  | {variant} : {}", e.name);
        }
        f.line("  deriving DecidableEq, Repr, BEq, Hashable, Inhabited");
        f.blank();
    }

    // WittLevel — open-world structure (not inductive)
    generate_witt_level(&mut f);

    f.finish()
}

/// Detects all enum classes from the ontology and hardcoded codegen enums.
fn detect_enums(ontology: &Ontology) -> Vec<DetectedEnum> {
    let mut enums = Vec::new();

    // Hardcoded: Space
    enums.push(DetectedEnum {
        name: "Space".into(),
        comment: "Ontology space classification.".into(),
        variants: vec![
            ("kernel".into(), "Immutable foundation layer.".into()),
            ("user".into(), "Runtime-parameterizable layer.".into()),
            (
                "bridge".into(),
                "Kernel-computed, user-consumed layer.".into(),
            ),
        ],
    });

    // Hardcoded: SiteState
    enums.push(DetectedEnum {
        name: "SiteState".into(),
        comment: "Site occupancy state within a partition.".into(),
        variants: vec![
            ("pinned".into(), "Site is occupied and immutable.".into()),
            ("free".into(), "Site is available for allocation.".into()),
        ],
    });

    // PrimitiveOp — from individuals
    detect_primitive_op(ontology, &mut enums);

    // Vocabulary enums — from ontology individuals
    // MetricAxis — detected from type/ namespace (not op/)
    detect_vocabulary_enum(
        ontology,
        "type",
        "MetricAxis",
        "Metric axis for measurement.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "op",
        "GeometricCharacter",
        "Geometric character of an operation.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "op",
        "VerificationDomain",
        "Domain of verification for identities.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "op",
        "ValidityScopeKind",
        "Validity scope classification.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "resolver",
        "ExecutionPolicyKind",
        "Execution policy for composed operations.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "resolver",
        "ComplexityClass",
        "Computational complexity classification.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "derivation",
        "RewriteRule",
        "Rewrite rule classification.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "type",
        "VarianceAnnotation",
        "Type variance annotation.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "observable",
        "MeasurementUnit",
        "Unit of measurement.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "query",
        "TriadProjection",
        "Triad projection axis.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "observable",
        "PhaseBoundaryType",
        "Phase boundary classification.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "state",
        "GroundingPhase",
        "Grounding phase of resolution.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "observable",
        "AchievabilityStatus",
        "Achievability status of a morphospace target.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "state",
        "SessionBoundaryType",
        "Session boundary classification.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "proof",
        "ProofStrategy",
        "Strategy for proof compilation.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "schema",
        "QuantifierKind",
        "Quantifier classification.",
        &mut enums,
    );
    detect_vocabulary_enum(
        ontology,
        "conformance",
        "ViolationKind",
        "SHACL violation classification.",
        &mut enums,
    );

    // Hardcoded: ProofModality
    enums.push(DetectedEnum {
        name: "ProofModality".into(),
        comment: "Proof modality classification.".into(),
        variants: vec![
            (
                "computation".into(),
                "Exhaustive computation at a quantum level.".into(),
            ),
            (
                "axiomatic".into(),
                "Derivation from axioms and definitions.".into(),
            ),
            (
                "empirical".into(),
                "Empirical verification with measurement data.".into(),
            ),
            (
                "inductive".into(),
                "Structural induction on quantum level.".into(),
            ),
        ],
    });

    enums
}

/// Detects PrimitiveOp variants from op namespace individuals.
fn detect_primitive_op(ontology: &Ontology, enums: &mut Vec<DetectedEnum>) {
    let op_module = match ontology.find_namespace("op") {
        Some(m) => m,
        None => return,
    };

    let mut variants = Vec::new();
    for ind in &op_module.individuals {
        let type_local = local_name(ind.type_);
        if type_local != "UnaryOp" && type_local != "BinaryOp" && type_local != "Involution" {
            continue;
        }
        let name = to_camel_case_variant(local_name(ind.id));
        let comment = normalize_lean_comment(ind.comment);
        variants.push((name, comment));
    }

    if !variants.is_empty() {
        enums.push(DetectedEnum {
            name: "PrimitiveOp".into(),
            comment: "Primitive algebraic operations.".into(),
            variants,
        });
    }
}

/// Detects a vocabulary enum from individuals of a specific class in a namespace.
fn detect_vocabulary_enum(
    ontology: &Ontology,
    ns_prefix: &str,
    class_name: &str,
    comment: &str,
    enums: &mut Vec<DetectedEnum>,
) {
    let module = match ontology.find_namespace(ns_prefix) {
        Some(m) => m,
        None => return,
    };

    let suffix = format!("/{class_name}");
    let mut variants: Vec<(String, String)> = module
        .individuals
        .iter()
        .filter(|ind| ind.type_.ends_with(&suffix))
        .map(|ind| {
            let name = to_camel_case_variant(local_name(ind.id));
            let c = normalize_lean_comment(ind.comment);
            (name, c)
        })
        .collect();

    // Strip common PascalCase suffix to avoid redundancy
    if let Some(sfx) = common_variant_suffix(&variants) {
        for (name, _) in &mut variants {
            if name.len() > sfx.len() && name.ends_with(&sfx) {
                name.truncate(name.len() - sfx.len());
            }
        }
    }

    if !variants.is_empty() {
        enums.push(DetectedEnum {
            name: class_name.to_string(),
            comment: comment.to_string(),
            variants,
        });
    }
}

/// Finds the common PascalCase-word suffix shared by all variant names.
fn common_variant_suffix(variants: &[(String, String)]) -> Option<String> {
    if variants.len() < 2 {
        return None;
    }
    let first = &variants[0].0;
    // Find the last uppercase boundary in the first variant
    let boundary = first
        .char_indices()
        .rev()
        .find(|(i, c)| *i > 0 && c.is_uppercase())
        .map(|(i, _)| i)?;
    let candidate = &first[boundary..];
    // Check all variants share this suffix and stripping leaves non-empty
    for (name, _) in variants {
        if !name.ends_with(candidate) || name.len() <= candidate.len() {
            return None;
        }
    }
    Some(candidate.to_string())
}

/// Converts a local name to a camelCase Lean variant name.
fn to_camel_case_variant(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => {
            let mut result: String = c.to_lowercase().collect();
            result.extend(chars);
            result
        }
    }
}

/// Generates the WittLevel structure (open-world, not inductive).
fn generate_witt_level(f: &mut LeanFile) {
    f.doc_comment("Witt vector length (multiples of 8). Open-world: any W_n is valid.");
    f.line("structure WittLevel where");
    f.indented_doc_comment("The Witt vector length in bits.");
    f.line("  wittLength : Nat");
    f.line("  deriving DecidableEq, Repr, BEq, Hashable");
    f.blank();
    f.line("namespace WittLevel");
    f.blank();
    f.doc_comment("Standard Witt level W8 (8-bit ring).");
    f.line("def W8  : WittLevel := \u{27e8}8\u{27e9}");
    f.doc_comment("Standard Witt level W16 (16-bit ring).");
    f.line("def W16 : WittLevel := \u{27e8}16\u{27e9}");
    f.doc_comment("Standard Witt level W24 (24-bit ring).");
    f.line("def W24 : WittLevel := \u{27e8}24\u{27e9}");
    f.doc_comment("Standard Witt level W32 (32-bit ring).");
    f.line("def W32 : WittLevel := \u{27e8}32\u{27e9}");
    f.blank();
    f.doc_comment("Construct an arbitrary Witt level.");
    f.line("def new (n : Nat) : WittLevel := \u{27e8}n\u{27e9}");
    f.blank();
    f.doc_comment("The bit width (identity with wittLength).");
    f.line("def bitsWidth (w : WittLevel) : Nat := w.wittLength");
    f.blank();
    f.line("end WittLevel");
}

/// Returns the number of enums that will be generated (for reporting).
pub fn count_enums(ontology: &Ontology) -> usize {
    // detect_enums length + 1 for WittLevel
    detect_enums(ontology).len() + 1
}

/// Returns the set of enum class names from the ontology (used for filtering).
///
/// This delegates to `Ontology::enum_class_names()` to maintain single source
/// of truth.
pub fn enum_class_names() -> &'static [&'static str] {
    Ontology::enum_class_names()
}

/// Returns the set of individual types that map to enum variants (not constant modules).
pub fn enum_individual_types() -> Vec<&'static str> {
    let mut types: Vec<&str> = vec!["UnaryOp", "BinaryOp", "Involution"];
    types.extend_from_slice(Ontology::enum_class_names());
    types
}

/// Generates PrimitiveOp method definitions from individual property data.
///
/// This produces `def arity`, `def isCommutative`, etc. in the PrimitiveOp
/// namespace, with match arms generated from individual properties.
pub fn generate_primitive_op_methods(ontology: &Ontology) -> String {
    let op_module = match ontology.find_namespace("op") {
        Some(m) => m,
        None => return String::new(),
    };

    struct OpData {
        variant: String,
        arity: Option<i64>,
        is_commutative: Option<bool>,
        geometric_character: Option<String>,
    }

    let mut ops: Vec<OpData> = Vec::new();
    for ind in &op_module.individuals {
        let type_local = local_name(ind.type_);
        if type_local != "UnaryOp" && type_local != "BinaryOp" && type_local != "Involution" {
            continue;
        }
        let variant = to_camel_case_variant(local_name(ind.id));
        let mut data = OpData {
            variant,
            arity: None,
            is_commutative: None,
            geometric_character: None,
        };
        for (prop_iri, value) in ind.properties {
            let prop = local_name(prop_iri);
            match prop {
                "arity" => {
                    if let IndividualValue::Int(n) = value {
                        data.arity = Some(*n);
                    }
                }
                "isCommutative" => {
                    if let IndividualValue::Bool(b) = value {
                        data.is_commutative = Some(*b);
                    }
                }
                "hasGeometricCharacter" => {
                    if let IndividualValue::IriRef(iri) = value {
                        data.geometric_character = Some(to_camel_case_variant(local_name(iri)));
                    }
                }
                _ => {}
            }
        }
        ops.push(data);
    }

    if ops.is_empty() {
        return String::new();
    }

    let mut buf = String::new();
    buf.push_str("namespace PrimitiveOp\n\n");

    // arity
    buf.push_str("/-- The arity of this operation. -/\n");
    buf.push_str("def arity : PrimitiveOp \u{2192} Int\n");
    for op in &ops {
        let a = op.arity.unwrap_or(0);
        let _ = writeln!(buf, "  | .{} => {a}", op.variant);
    }
    buf.push('\n');

    // isCommutative
    buf.push_str("/-- Whether this operation is commutative. -/\n");
    buf.push_str("def isCommutative : PrimitiveOp \u{2192} Bool\n");
    for op in &ops {
        let c = op.is_commutative.unwrap_or(false);
        let _ = writeln!(buf, "  | .{} => {c}", op.variant);
    }
    buf.push('\n');

    // hasGeometricCharacter
    buf.push_str("/-- The geometric character of this operation. -/\n");
    buf.push_str("def hasGeometricCharacter : PrimitiveOp \u{2192} GeometricCharacter\n");
    for op in &ops {
        let gc = op
            .geometric_character
            .as_deref()
            .unwrap_or("ringReflection");
        let _ = writeln!(buf, "  | .{} => .{gc}", op.variant);
    }
    buf.push('\n');

    buf.push_str("end PrimitiveOp\n");
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_all_enum_classes() {
        let ontology = uor_ontology::Ontology::full();
        let enums = detect_enums(ontology);
        // 17 vocabulary + 4 hardcoded (Space, SiteState, PrimitiveOp, ProofModality)
        assert_eq!(enums.len(), 21);
    }

    #[test]
    fn primitive_op_methods_include_geometric_character() {
        let ontology = uor_ontology::Ontology::full();
        let methods = generate_primitive_op_methods(ontology);
        assert!(methods.contains("def hasGeometricCharacter"));
        assert!(methods.contains("def arity"));
        assert!(methods.contains("def isCommutative"));
    }

    #[test]
    fn witt_level_counted_separately() {
        let ontology = uor_ontology::Ontology::full();
        // count_enums = detect_enums().len() + 1 for WittLevel
        assert_eq!(count_enums(ontology), 22);
    }
}
