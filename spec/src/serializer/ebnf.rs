//! EBNF serializer for the UOR Term Language grammar (Amendment 42).
//!
//! Generates an ISO/IEC 14977 EBNF grammar from the ontology, output to
//! `public/uor.term.ebnf`. Operations, quantum levels, and rewrite rules
//! are derived from `Ontology::full()` — all other sections are static.

use std::fmt::Write;

use crate::model::{Individual, IndividualValue, NamespaceModule, Ontology};

/// Canonical ordering of unary operations in the grammar.
const UNARY_ORDER: &[&str] = &["neg", "bnot", "succ", "pred"];

/// Canonical ordering of binary operations in the grammar.
const BINARY_ORDER: &[&str] = &["add", "sub", "mul", "xor", "and", "or"];

/// IRI prefix for op/ namespace properties.
const OP: &str = "https://uor.foundation/op/";

/// IRI prefix for schema/ namespace properties.
const SCHEMA: &str = "https://uor.foundation/schema/";

/// IRI suffix for the `RewriteRule` class.
const REWRITE_RULE_TYPE: &str = "https://uor.foundation/derivation/RewriteRule";

/// IRI suffix for the `QuantumLevel` class.
const QUANTUM_LEVEL_TYPE: &str = "https://uor.foundation/schema/QuantumLevel";

/// Serializes the UOR Term Language grammar as ISO/IEC 14977 EBNF.
///
/// The grammar is generated from the ontology: operations, quantum levels,
/// and rewrite rules are derived from named individuals. All other sections
/// (top-level syntax, type declarations, bindings, assertions) are static.
///
/// # Errors
///
/// This function is infallible; it always returns a valid EBNF string.
#[must_use]
pub fn to_ebnf(ontology: &Ontology) -> String {
    let mut out = String::with_capacity(8 * 1024);

    emit_header(&mut out, ontology);
    emit_top_level(&mut out);
    emit_terms(&mut out);
    emit_quantum_level(&mut out, ontology);
    emit_applications(&mut out);
    emit_unary_op(&mut out, ontology);
    emit_binary_op(&mut out, ontology);
    emit_variables(&mut out);
    emit_type_decl(&mut out);
    emit_bindings(&mut out);
    emit_assertions(&mut out);
    emit_rewrite_rules(&mut out, ontology);
    emit_quantum_generalisation(&mut out, ontology);
    emit_whitespace(&mut out);
    emit_end(&mut out);

    out
}

// ── Section emitters ────────────────────────────────────────────────────────

/// Emits the header block with version and ISO/IEC 14977 notation guide.
fn emit_header(out: &mut String, ontology: &Ontology) {
    let _ = write!(
        out,
        "\
(* ============================================================================
   UOR Term Language — Extended Backus-Naur Form Grammar
   Generated from: uor_ontology::Ontology::full()
   Authoritative source: https://uor.foundation/
   Specification version: v{version} (Amendment 42+)

   Notation: ISO/IEC 14977 EBNF.
     ::=   definition
     |     alternation
     ,     concatenation
     {{}}    zero or more
     []    optional
     ()    grouping
     \"\"    terminal string
     (**)  comment
   ============================================================================ *)\n\n",
        version = ontology.version
    );
}

/// Emits the top-level entry point rules.
fn emit_top_level(out: &mut String) {
    out.push_str(
        "\
(* ── Top-level entry point ─────────────────────────────────────────────────── *)

program
    ::= { statement } ;

statement
    ::= type-decl
      | binding
      | assertion
      | expression \";\" ;

expression
    ::= term ;

",
    );
}

/// Emits the term, literal, and related rules.
fn emit_terms(out: &mut String) {
    out.push_str(
        "\
(* ── Terms ──────────────────────────────────────────────────────────────────
   schema:Term — syntactic expressions that evaluate to Datums.
   Disjoint from schema:Datum by the OWL disjointness axiom.
   Subclasses: schema:Literal, schema:Application.
   ─────────────────────────────────────────────────────────────────────────── *)

term
    ::= literal
      | application
      | variable ;

(* schema:Literal — a leaf term that directly denotes a Datum via schema:denotes *)
literal
    ::= integer-literal
      | braille-literal
      | quantum-literal ;

integer-literal
    ::= digit , { digit } ;

(* Braille address literal — U+2800..U+28FF glyph sequence (schema:Triad glyph) *)
braille-literal
    ::= braille-glyph , { braille-glyph } ;

braille-glyph
    ::= \"\\u2800\" .. \"\\u28FF\" ;   (* Unicode Braille Patterns block *)

(* Quantum-tagged literal: value at an explicit quantum level Q_k *)
quantum-literal
    ::= integer-literal , \"@\" , quantum-level ;

",
    );
}

/// Emits the quantum-level rule from QuantumLevel individuals.
fn emit_quantum_level(out: &mut String, ontology: &Ontology) {
    let levels = sorted_quantum_levels(ontology);

    out.push_str("quantum-level\n    ::= ");
    for (i, (label, _, _, _)) in levels.iter().enumerate() {
        if i > 0 {
            out.push_str(" | ");
        }
        let _ = write!(out, "\"{}\"", label);
    }
    out.push_str(
        "\n      | \"Q\" , digit , { digit } ;   \
         (* open: implementations declare higher levels *)\n\n",
    );
}

/// Emits the application structure rules.
fn emit_applications(out: &mut String) {
    out.push_str(
        "\
(* schema:Application — a term formed by applying an operation to argument terms *)
application
    ::= unary-application
      | binary-application ;

",
    );
}

/// Emits the Operations section header and the unary-op rule.
fn emit_unary_op(out: &mut String, ontology: &Ontology) {
    out.push_str(
        "\
(* ── Operations ──────────────────────────────────────────────────────────────
   Derived from op:PrimitiveOp enumeration (foundation/src/enums.rs).
   Arity and commutativity/associativity flags match spec/src/namespaces/op.rs.
   ─────────────────────────────────────────────────────────────────────────── *)

unary-application
    ::= unary-op , \"(\" , term , \")\" ;

unary-op\n",
    );

    let op_ns = find_namespace(ontology, "op");

    for (i, &op_label) in UNARY_ORDER.iter().enumerate() {
        let prefix = if i == 0 { "    ::= " } else { "      | " };
        let padded = format!("\"{}\"", op_label);
        // Pad to 8 chars for alignment.
        let pad = if padded.len() < 8 {
            " ".repeat(8 - padded.len())
        } else {
            " ".to_string()
        };

        if let Some(ns) = op_ns {
            if let Some(ind) = find_by_label(ns, op_label) {
                let comment = format_unary_comment(ind);
                let _ = write!(out, "{prefix}{padded}{pad}(* {comment} *)");
            } else {
                let _ = write!(out, "{prefix}{padded}");
            }
        } else {
            let _ = write!(out, "{prefix}{padded}");
        }

        if i == UNARY_ORDER.len() - 1 {
            out.push_str(" ;\n");
        }
        out.push('\n');
    }
}

/// Emits the binary-op rule.
fn emit_binary_op(out: &mut String, ontology: &Ontology) {
    out.push_str(
        "\
binary-application
    ::= binary-op , \"(\" , term , \",\" , term , \")\" ;

binary-op\n",
    );

    let op_ns = find_namespace(ontology, "op");

    for (i, &op_label) in BINARY_ORDER.iter().enumerate() {
        let prefix = if i == 0 { "    ::= " } else { "      | " };
        let padded = format!("\"{}\"", op_label);
        let pad = if padded.len() < 8 {
            " ".repeat(8 - padded.len())
        } else {
            " ".to_string()
        };

        if let Some(ns) = op_ns {
            if let Some(ind) = find_by_label(ns, op_label) {
                let comment = format_binary_comment(ind);
                let _ = write!(out, "{prefix}{padded}{pad}(* {comment} *)");
            } else {
                let _ = write!(out, "{prefix}{padded}");
            }
        } else {
            let _ = write!(out, "{prefix}{padded}");
        }

        if i == BINARY_ORDER.len() - 1 {
            out.push_str(" ;\n");
        }
        out.push('\n');
    }
}

/// Emits variable and identifier rules.
fn emit_variables(out: &mut String) {
    out.push_str(
        "\
(* ── Variables ───────────────────────────────────────────────────────────────
   Variables bind inside type-decl or binding forms. They denote
   schema:Datum values and are resolved by the PRISM pipeline. *)

variable
    ::= identifier ;

identifier
    ::= alpha , { alpha | digit | \"_\" } ;

alpha
    ::= \"a\" .. \"z\" | \"A\" .. \"Z\" ;

digit
    ::= \"0\" .. \"9\" ;

",
    );
}

/// Emits type declaration rules.
fn emit_type_decl(out: &mut String) {
    out.push_str(
        "\
(* ── Type declarations ───────────────────────────────────────────────────────
   A TypeDefinition (type:TypeDefinition) declares constraints that pin
   fibers of the Z/2Z fibration, contributing to the fiber budget.
   Constraints are applied via the ψ-pipeline during resolution. *)

type-decl
    ::= \"type\" , identifier , \"{\" , { constraint-decl } , \"}\" ;

constraint-decl
    ::= constraint-kind , \":\" , term , \";\" ;

constraint-kind
    ::= \"residue\"    (* vertical / ring-arithmetic axis *)
      | \"carry\"      (* carry-pattern constraint *)
      | \"hamming\"    (* horizontal / Hamming-metric axis *)
      | \"depth\"      (* diagonal / fiber-depth axis *)
      | \"fiber\"      (* explicit fiber assignment *)
      | \"affine\"     (* affine subspace constraint *) ;

",
    );
}

/// Emits binding rules.
fn emit_bindings(out: &mut String) {
    out.push_str(
        "\
(* ── Bindings ────────────────────────────────────────────────────────────────
   A binding associates an identifier with a term under a given type. *)

binding
    ::= \"let\" , identifier , \":\" , identifier , \"=\" , term , \";\" ;

",
    );
}

/// Emits assertion rules.
fn emit_assertions(out: &mut String) {
    out.push_str(
        "\
(* ── Assertions ──────────────────────────────────────────────────────────────
   Ground assertions checked by the conformance suite. *)

assertion
    ::= \"assert\" , term , equality-op , term , \";\" ;

equality-op
    ::= \"=\"    (* strict ring equality *)
      | \"\u{2261}\"    (* canonical-form equivalence *) ;

",
    );
}

/// Emits the rewrite rules comment section from RewriteRule individuals.
fn emit_rewrite_rules(out: &mut String, ontology: &Ontology) {
    out.push_str(
        "(* ── Rewrite rules ───────────────────────────────────────────────────────────\n\
         \x20\x20\x20The six rewrite rules from foundation/src/enums.rs::RewriteRule.\n\
         \x20\x20\x20Applied by the canonical-form resolver \
         (resolver:CanonicalFormResolver). *)\n\n",
    );

    if let Some(ns) = find_namespace(ontology, "derivation") {
        let rules: Vec<&Individual> = ns
            .individuals
            .iter()
            .filter(|ind| ind.type_ == REWRITE_RULE_TYPE)
            .collect();

        for ind in &rules {
            let patterns = rewrite_patterns(ind.label);
            for pattern in patterns {
                let _ = writeln!(out, "(* {pattern} *)");
            }
        }
    }

    out.push('\n');
}

/// Emits the quantum-level generalisation comment section.
fn emit_quantum_generalisation(out: &mut String, ontology: &Ontology) {
    out.push_str(
        "(* ── Quantum-level generalisation ────────────────────────────────────────────\n\
         \x20\x20\x20At quantum level k the ring is R_k = Z/(2^(8*(k+1)))Z.\n",
    );

    let levels = sorted_quantum_levels(ontology);
    for (label, _index, bits, cycle) in &levels {
        let _ = writeln!(out, "   {label}: {bits}-bit, {cycle} states.");
    }

    out.push_str("   All grammar constructs are parametric in the quantum level. *)\n\n");
}

/// Emits whitespace and comment rules.
fn emit_whitespace(out: &mut String) {
    out.push_str(
        "\
(* ── Whitespace and comments ─────────────────────────────────────────────────
   Whitespace is insignificant outside of string literals and braille sequences.
   Line comments begin with \"--\".
   Block comments are delimited by \"(*\" and \"*)\". *)

whitespace
    ::= \" \" | \"\\t\" | \"\\n\" | \"\\r\" ;

line-comment
    ::= \"--\" , { any-char-except-newline } , \"\\n\" ;

block-comment
    ::= \"(*\" , { any-char } , \"*)\" ;

",
    );
}

/// Emits the end-of-grammar marker.
fn emit_end(out: &mut String) {
    out.push_str(
        "(* ── End of grammar ──────────────────────────────────────────────────────── *)\n",
    );
}

// ── Helper functions ────────────────────────────────────────────────────────

/// Finds a namespace module by prefix.
fn find_namespace<'a>(ontology: &'a Ontology, prefix: &str) -> Option<&'a NamespaceModule> {
    ontology
        .namespaces
        .iter()
        .find(|m| m.namespace.prefix == prefix)
}

/// Finds an individual by label within a namespace module.
fn find_by_label<'a>(ns: &'a NamespaceModule, label: &str) -> Option<&'a Individual> {
    ns.individuals.iter().find(|ind| ind.label == label)
}

/// Extracts an `IriRef` property value from an individual.
fn get_iri_prop<'a>(ind: &'a Individual, prop_iri: &str) -> Option<&'a str> {
    ind.properties.iter().find_map(|(k, v)| {
        if *k == prop_iri {
            if let IndividualValue::IriRef(iri) = v {
                Some(*iri)
            } else {
                None
            }
        } else {
            None
        }
    })
}

/// Extracts an `Int` property value from an individual.
fn get_int_prop(ind: &Individual, prop_iri: &str) -> Option<i64> {
    ind.properties.iter().find_map(|(k, v)| {
        if *k == prop_iri {
            if let IndividualValue::Int(n) = v {
                Some(*n)
            } else {
                None
            }
        } else {
            None
        }
    })
}

/// Extracts a `Bool` property value from an individual.
fn get_bool_prop(ind: &Individual, prop_iri: &str) -> Option<bool> {
    ind.properties.iter().find_map(|(k, v)| {
        if *k == prop_iri {
            if let IndividualValue::Bool(b) = v {
                Some(*b)
            } else {
                None
            }
        } else {
            None
        }
    })
}

/// Extracts a `List` property value from an individual.
fn get_list_prop<'a>(ind: &'a Individual, prop_iri: &str) -> Option<&'a [&'a str]> {
    ind.properties.iter().find_map(|(k, v)| {
        if *k == prop_iri {
            if let IndividualValue::List(items) = v {
                Some(*items)
            } else {
                None
            }
        } else {
            None
        }
    })
}

/// Extracts the local name from an IRI (after the last `/`).
fn local_name(iri: &str) -> &str {
    iri.rsplit('/').next().unwrap_or(iri)
}

/// Converts a PascalCase string to kebab-case.
///
/// `"RingReflection"` → `"ring-reflection"`,
/// `"HypercubeTranslation"` → `"hypercube-translation"`.
fn to_kebab_case(pascal: &str) -> String {
    let mut result = String::with_capacity(pascal.len() + 4);
    for (i, ch) in pascal.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('-');
            }
            for lower in ch.to_lowercase() {
                result.push(lower);
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Formats the EBNF comment for a unary operation individual.
///
/// Extracts the first sentence of the comment, involution status,
/// composition, and geometric character from the individual's properties.
fn format_unary_comment(ind: &Individual) -> String {
    let mut parts: Vec<String> = Vec::new();

    // First sentence from comment.
    let first = first_sentence(ind.comment);
    parts.push(first.to_string());

    // Involution note.
    if ind.type_.ends_with("Involution") {
        parts.push(format!("Involution: {l}({l}(x)) = x.", l = ind.label));
    }

    // Composition note (succ, pred).
    let composed_iri = format!("{OP}composedOf");
    if let Some(list) = get_list_prop(ind, &composed_iri) {
        let names: Vec<&str> = list.iter().map(|iri| local_name(iri)).collect();
        if names.len() >= 2 {
            parts.push(format!(
                "Critical identity: {} = {} \u{2218} {}.",
                ind.label, names[0], names[1]
            ));
        }
    }

    // Inverse note.
    let inverse_iri = format!("{OP}inverse");
    if let Some(inv) = get_iri_prop(ind, &inverse_iri) {
        parts.push(format!("Inverse of {}.", local_name(inv)));
    }

    // Geometric character.
    let gc_iri = format!("{OP}hasGeometricCharacter");
    if let Some(gc) = get_iri_prop(ind, &gc_iri) {
        parts.push(format!(
            "GeometricCharacter: {}",
            to_kebab_case(local_name(gc))
        ));
    }

    // Join with newline + padding for multi-line EBNF comments.
    if parts.len() <= 1 {
        parts.join("")
    } else {
        let first_part = parts.remove(0);
        let rest: Vec<String> = parts
            .iter()
            .map(|p| format!("\n                     {p}"))
            .collect();
        format!("{first_part}{}", rest.join(""))
    }
}

/// Formats the EBNF comment for a binary operation individual.
///
/// Extracts algebraic properties (commutativity, associativity, identity
/// element) and geometric character from the individual's properties.
fn format_binary_comment(ind: &Individual) -> String {
    let mut parts: Vec<String> = Vec::new();

    // First sentence from comment (the formula).
    let first = first_sentence(ind.comment);
    parts.push(first.to_string());

    // Commutativity / associativity.
    let comm_iri = format!("{OP}commutative");
    let assoc_iri = format!("{OP}associative");
    let comm = get_bool_prop(ind, &comm_iri);
    let assoc = get_bool_prop(ind, &assoc_iri);

    let mut algebra = String::new();
    if let Some(c) = comm {
        if c {
            algebra.push_str("Commutative");
        } else {
            algebra.push_str("Not commutative");
        }
    }
    if let Some(a) = assoc {
        if !algebra.is_empty() {
            algebra.push_str(", ");
        }
        if a {
            algebra.push_str("associative.");
        } else {
            algebra.push_str("not associative.");
        }
    }

    // Identity element.
    let id_iri = format!("{OP}identity");
    if let Some(id_val) = get_int_prop(ind, &id_iri) {
        algebra.push_str(&format!("  Identity: {id_val}."));
    }

    if !algebra.is_empty() {
        parts.push(algebra);
    }

    // Geometric character.
    let gc_iri = format!("{OP}hasGeometricCharacter");
    if let Some(gc) = get_iri_prop(ind, &gc_iri) {
        parts.push(format!(
            "GeometricCharacter: {}",
            to_kebab_case(local_name(gc))
        ));
    }

    // Join with newline + padding for multi-line EBNF comments.
    if parts.len() <= 1 {
        parts.join("")
    } else {
        let first_part = parts.remove(0);
        let rest: Vec<String> = parts
            .iter()
            .map(|p| format!("\n                     {p}"))
            .collect();
        format!("{first_part}{}", rest.join(""))
    }
}

/// Extracts the first sentence from a comment string.
///
/// Returns text up to the first `. ` (period-space) boundary, or the
/// entire comment if no sentence boundary is found.
fn first_sentence(comment: &str) -> &str {
    if let Some(pos) = comment.find(". ") {
        &comment[..pos + 1]
    } else {
        comment
    }
}

/// Returns the canonical rewrite pattern lines for a RewriteRule label.
///
/// Each rewrite rule has one or more well-known notation lines in the grammar.
/// Rules with multiple examples (Involution, IdentityElement) produce
/// multiple lines, each wrapped in its own `(* ... *)` comment.
fn rewrite_patterns(label: &str) -> Vec<&'static str> {
    match label {
        "CriticalIdentityRule" => vec!["CriticalIdentity:   neg(bnot(x))   \u{2192} succ(x)"],
        "InvolutionRule" => vec![
            "Involution:         neg(neg(x))    \u{2192} x",
            "                    bnot(bnot(x))  \u{2192} x",
        ],
        "AssociativityRule" => vec!["Associativity:      f(f(a,b),c)    \u{2192} f(a,f(b,c))"],
        "CommutativityRule" => {
            vec!["Commutativity:      f(a,b)         \u{2192} f(b,a)  (if f comm)"]
        }
        "IdentityElementRule" => vec![
            "IdentityElement:    add(x,0)       \u{2192} x",
            "                    mul(x,1)       \u{2192} x",
            "                    xor(x,0)       \u{2192} x",
        ],
        "NormalizationRule" => vec!["Normalization:      sort operands by content address"],
        _ => vec![],
    }
}

/// Returns sorted quantum levels as `(label, index, bits_width, cycle_size)`.
fn sorted_quantum_levels(ontology: &Ontology) -> Vec<(&str, i64, i64, i64)> {
    let mut levels: Vec<(&str, i64, i64, i64)> = Vec::new();

    if let Some(ns) = find_namespace(ontology, "schema") {
        for ind in &ns.individuals {
            if ind.type_ == QUANTUM_LEVEL_TYPE {
                let qi_iri = format!("{SCHEMA}quantumIndex");
                let bw_iri = format!("{SCHEMA}bitsWidth");
                let cs_iri = format!("{SCHEMA}cycleSize");

                let index = get_int_prop(ind, &qi_iri).unwrap_or(0);
                let bits = get_int_prop(ind, &bw_iri).unwrap_or(0);
                let cycle = get_int_prop(ind, &cs_iri).unwrap_or(0);

                levels.push((ind.label, index, bits, cycle));
            }
        }
    }

    levels.sort_by_key(|&(_, idx, _, _)| idx);
    levels
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ontology;

    #[test]
    fn produces_non_empty_ebnf() {
        let ontology = Ontology::full();
        let ebnf = to_ebnf(ontology);
        assert!(!ebnf.is_empty());
        assert!(ebnf.contains("::="));
    }

    #[test]
    fn contains_version() {
        let ontology = Ontology::full();
        let ebnf = to_ebnf(ontology);
        assert!(ebnf.contains(ontology.version));
    }

    #[test]
    fn contains_all_quantum_levels() {
        let ontology = Ontology::full();
        let ebnf = to_ebnf(ontology);
        for level in &["\"Q0\"", "\"Q1\"", "\"Q2\"", "\"Q3\""] {
            assert!(ebnf.contains(level), "Missing quantum level {level}");
        }
    }

    #[test]
    fn contains_all_operations() {
        let ontology = Ontology::full();
        let ebnf = to_ebnf(ontology);
        for op in &[
            "\"neg\"", "\"bnot\"", "\"succ\"", "\"pred\"", "\"add\"", "\"sub\"", "\"mul\"",
            "\"xor\"", "\"and\"", "\"or\"",
        ] {
            assert!(ebnf.contains(op), "Missing operation {op}");
        }
    }

    #[test]
    fn contains_rewrite_rules() {
        let ontology = Ontology::full();
        let ebnf = to_ebnf(ontology);
        for rule in &[
            "CriticalIdentity",
            "Involution",
            "Associativity",
            "Commutativity",
            "IdentityElement",
            "Normalization",
        ] {
            assert!(ebnf.contains(rule), "Missing rewrite rule {rule}");
        }
    }

    #[test]
    fn balanced_comments() {
        let ontology = Ontology::full();
        let ebnf = to_ebnf(ontology);
        let opens = ebnf.matches("(*").count();
        let closes = ebnf.matches("*)").count();
        assert_eq!(
            opens, closes,
            "Unbalanced EBNF comments: {opens} opens vs {closes} closes"
        );
    }
}
