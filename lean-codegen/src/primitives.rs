//! Lean 4 `Primitives` class generator.
//!
//! Generates `UOR/Primitives.lean` containing the `Primitives` typeclass.

use crate::emit::LeanFile;

/// Generates the content of `UOR/Primitives.lean`.
pub fn generate_primitives() -> String {
    let mut f = LeanFile::new("Primitives typeclass \u{2014} XSD primitive type family.");
    f.doc_comment(
        "XSD primitive type family. Implementations choose concrete representations \
         for each XSD type. All generated structures are parametric over this class.",
    );
    f.line("class Primitives where");
    f.indented_doc_comment("String type (xsd:string).");
    f.line("  String : Type");
    f.indented_doc_comment("Integer type (xsd:integer).");
    f.line("  Integer : Type");
    f.indented_doc_comment("Non-negative integer type (xsd:nonNegativeInteger).");
    f.line("  NonNegativeInteger : Type");
    f.indented_doc_comment("Positive integer type (xsd:positiveInteger).");
    f.line("  PositiveInteger : Type");
    f.indented_doc_comment("Decimal type (xsd:decimal).");
    f.line("  Decimal : Type");
    f.indented_doc_comment("Boolean type (xsd:boolean).");
    f.line("  Boolean : Type");
    f.finish()
}
