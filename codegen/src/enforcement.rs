//! Generates the `enforcement.rs` module for the `uor-foundation` crate.
//!
//! This module emits Layer 1 (opaque witnesses), Layer 2 (declarative builders),
//! and the Term AST types that form the declarative enforcement surface.

use crate::emit::RustFile;

/// Generates the complete `enforcement.rs` module content.
///
/// # Errors
///
/// This function is infallible but returns `String` for consistency.
#[must_use]
pub fn generate_enforcement_module() -> String {
    let mut f = RustFile::new(
        "Declarative enforcement types.\n\
         //!\n\
         //! This module contains the opaque witness types, declarative builders,\n\
         //! and Term AST that form the declarative enforcement surface of the\n\
         //! UOR Foundation crate. Prism code consumes these types but cannot\n\
         //! construct the sealed ones directly.\n\
         //!\n\
         //! # Layers\n\
         //!\n\
         //! - **Layer 1** \\[Opaque Witnesses\\]: `Datum`, `Validated<T>`, `Derivation`,\n\
         //!   `FreeRank` \\[private fields, no public constructors\\]\n\
         //! - **Layer 2** \\[Declarative Builders\\]: `CompileUnitBuilder`,\n\
         //!   `EffectDeclarationBuilder`, etc. \\[produce `Validated<T>` on success\\]\n\
         //! - **Term AST**: `Term`, `TermArena`, `Binding`, `Assertion`, etc.",
    );

    f.line("use crate::{PrimitiveOp, WittLevel, VerificationDomain, ViolationKind};");
    f.blank();

    generate_sealed_module(&mut f);
    generate_datum_types(&mut f);
    generate_grounding_types(&mut f);
    generate_witness_types(&mut f);
    generate_term_ast(&mut f);
    generate_shape_violation(&mut f);
    generate_builders(&mut f);
    generate_minting_session(&mut f);
    generate_const_ring_eval(&mut f);

    f.finish()
}

fn generate_sealed_module(f: &mut RustFile) {
    f.doc_comment("Private sealed module preventing downstream implementations.");
    f.doc_comment("Only `GroundedCoord` and `GroundedTuple<N>` implement `Sealed`.");
    f.line("mod sealed {");
    f.indented_doc_comment(
        "Sealed trait. Not publicly implementable because this module is private.",
    );
    f.line("    pub trait Sealed {}");
    f.line("    impl Sealed for super::GroundedCoord {}");
    f.line("    impl<const N: usize> Sealed for super::GroundedTuple<N> {}");
    f.line("}");
    f.blank();
}

fn generate_datum_types(f: &mut RustFile) {
    // DatumInner
    f.doc_comment("Internal level-tagged ring value. Width determined by quantum level.");
    f.doc_comment("Not publicly constructible \\[sealed within the crate\\].");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("#[allow(clippy::large_enum_variant, dead_code)]");
    f.line("pub(crate) enum DatumInner {");
    f.indented_doc_comment("Q0: 8-bit ring Z/256Z.");
    f.line("    Q0([u8; 1]),");
    f.indented_doc_comment("Q1: 16-bit ring Z/65536Z.");
    f.line("    Q1([u8; 2]),");
    f.indented_doc_comment("Q3: 32-bit ring Z/2^32Z.");
    f.line("    Q3([u8; 4]),");
    f.indented_doc_comment("Q7: 64-bit ring Z/2^64Z.");
    f.line("    Q7([u8; 8]),");
    f.indented_doc_comment("Q511: 4096-bit ring Z/2^4096Z.");
    f.line("    Q511([u8; 512]),");
    f.line("}");
    f.blank();

    // Datum
    f.doc_comment("A ring element at its minting quantum level.");
    f.doc_comment("");
    f.doc_comment("Cannot be constructed outside the `uor_foundation` crate.");
    f.doc_comment("The only way to obtain a `Datum` is through reduction evaluation");
    f.doc_comment("or the two-phase minting boundary (`validate_and_mint_coord` /");
    f.doc_comment("`validate_and_mint_tuple`).");
    f.doc_example(
        "// A Datum is produced by reduction evaluation or the minting boundary —\n\
         // you never construct one directly.\n\
         fn inspect_datum(d: &uor_foundation::enforcement::Datum) {\n\
         \x20   // Query its quantum level (Q0 = 8-bit, Q7 = 64-bit, etc.)\n\
         \x20   let level = d.level();\n\
         \n\
         \x20   // Datum width is determined by its level:\n\
         \x20   //   Q0 → 1 byte,  Q1 → 2 bytes,  Q3 → 4 bytes,\n\
         \x20   //   Q7 → 8 bytes, Q511 → 512 bytes\n\
         \x20   let bytes = d.as_bytes();\n\
         }",
        "rust,ignore",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct Datum {");
    f.indented_doc_comment("Level-tagged ring value \\[sealed\\].");
    f.line("    inner: DatumInner,");
    f.line("}");
    f.blank();
    f.line("impl Datum {");
    f.indented_doc_comment("Returns the Witt level at which this datum was minted.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn level(&self) -> WittLevel {");
    f.line("        match self.inner {");
    f.line("            DatumInner::Q0(_) => WittLevel::W8,");
    f.line("            DatumInner::Q1(_) => WittLevel::W16,");
    f.line("            DatumInner::Q3(_) => WittLevel::new(32),");
    f.line("            DatumInner::Q7(_) => WittLevel::new(64),");
    f.line("            DatumInner::Q511(_) => WittLevel::new(4096),");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the raw byte representation of this datum.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn as_bytes(&self) -> &[u8] {");
    f.line("        match &self.inner {");
    f.line("            DatumInner::Q0(b) => b,");
    f.line("            DatumInner::Q1(b) => b,");
    f.line("            DatumInner::Q3(b) => b,");
    f.line("            DatumInner::Q7(b) => b,");
    f.line("            DatumInner::Q511(b) => b,");
    f.line("        }");
    f.line("    }");
    f.line("}");
    f.blank();
}

fn generate_grounding_types(f: &mut RustFile) {
    // GroundedCoordInner
    f.doc_comment("Internal level-tagged coordinate value for grounding intermediates.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("#[allow(clippy::large_enum_variant, dead_code)]");
    f.line("pub(crate) enum GroundedCoordInner {");
    f.indented_doc_comment("Q0: 8-bit coordinate.");
    f.line("    Q0([u8; 1]),");
    f.indented_doc_comment("Q1: 16-bit coordinate.");
    f.line("    Q1([u8; 2]),");
    f.indented_doc_comment("Q3: 32-bit coordinate.");
    f.line("    Q3([u8; 4]),");
    f.indented_doc_comment("Q7: 64-bit coordinate.");
    f.line("    Q7([u8; 8]),");
    f.indented_doc_comment("Q511: 4096-bit coordinate.");
    f.line("    Q511([u8; 512]),");
    f.line("}");
    f.blank();

    // GroundedCoord
    f.doc_comment("A single grounded coordinate value.");
    f.doc_comment("");
    f.doc_comment("Not a `Datum` \\[this is the narrow intermediate that a `Grounding`");
    f.doc_comment("impl produces\\]. The foundation validates and mints it into a `Datum`.");
    f.doc_comment("Uses the same closed level-tagged family as `Datum`, ensuring that");
    f.doc_comment("coordinate width matches the target quantum level.");
    f.doc_example(
        "use uor_foundation::enforcement::GroundedCoord;\n\
         \n\
         // Q0: 8-bit ring Z/256Z — lightweight, exhaustive-verification baseline\n\
         let byte_coord = GroundedCoord::q0(42);\n\
         \n\
         // Q1: 16-bit ring Z/65536Z — audio samples, small indices\n\
         let short_coord = GroundedCoord::q1(1000);\n\
         \n\
         // Q3: 32-bit ring Z/2^32Z — pixel data, general-purpose integers\n\
         let word_coord = GroundedCoord::q3(70_000);\n\
         \n\
         // Q7: 64-bit ring Z/2^64Z — cryptographic hashes, content addresses\n\
         let wide_coord = GroundedCoord::q7(0xDEAD_BEEF_CAFE_BABE);\n\
         \n\
         // Q511: 4096-bit ring Z/2^4096Z — cryptographic/verification workloads\n\
         let huge_coord = GroundedCoord::q511([0u8; 512]);",
        "rust",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct GroundedCoord {");
    f.indented_doc_comment("Level-tagged coordinate bytes.");
    f.line("    pub(crate) inner: GroundedCoordInner,");
    f.line("}");
    f.blank();
    f.line("impl GroundedCoord {");
    f.indented_doc_comment("Construct a Q0 coordinate from a `u8` value.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn q0(value: u8) -> Self {");
    f.line("        Self { inner: GroundedCoordInner::Q0([value]) }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Construct a Q1 coordinate from a `u16` value (little-endian).");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn q1(value: u16) -> Self {");
    f.line("        Self { inner: GroundedCoordInner::Q1(value.to_le_bytes()) }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Construct a Q3 coordinate from a `u32` value (little-endian).");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn q3(value: u32) -> Self {");
    f.line("        Self { inner: GroundedCoordInner::Q3(value.to_le_bytes()) }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Construct a Q7 coordinate from a `u64` value (little-endian).");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn q7(value: u64) -> Self {");
    f.line("        Self { inner: GroundedCoordInner::Q7(value.to_le_bytes()) }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Construct a Q511 coordinate from a 512-byte array (little-endian).");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn q511(bytes: [u8; 512]) -> Self {");
    f.line("        Self { inner: GroundedCoordInner::Q511(bytes) }");
    f.line("    }");
    f.line("}");
    f.blank();

    // GroundedTuple
    f.doc_comment("A grounded tuple: a fixed-size array of `GroundedCoord` values.");
    f.doc_comment("");
    f.doc_comment("Represents a structured type (e.g., the 8 coordinates of an E8");
    f.doc_comment("lattice point). Not a `Datum` until the foundation validates and");
    f.doc_comment("mints it. Stack-resident, no heap allocation.");
    f.doc_example(
        "use uor_foundation::enforcement::{GroundedCoord, GroundedTuple};\n\
         \n\
         // A 2D pixel: (red, green) at Q0 (8-bit per channel)\n\
         let pixel = GroundedTuple::new([\n\
         \x20   GroundedCoord::q0(255), // red channel\n\
         \x20   GroundedCoord::q0(128), // green channel\n\
         ]);\n\
         \n\
         // An E8 lattice point: 8 coordinates at Q0\n\
         let lattice_point = GroundedTuple::new([\n\
         \x20   GroundedCoord::q0(2), GroundedCoord::q0(0),\n\
         \x20   GroundedCoord::q0(0), GroundedCoord::q0(0),\n\
         \x20   GroundedCoord::q0(0), GroundedCoord::q0(0),\n\
         \x20   GroundedCoord::q0(0), GroundedCoord::q0(0),\n\
         ]);",
        "rust",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct GroundedTuple<const N: usize> {");
    f.indented_doc_comment("The coordinate array.");
    f.line("    pub(crate) coords: [GroundedCoord; N],");
    f.line("}");
    f.blank();
    f.line("impl<const N: usize> GroundedTuple<N> {");
    f.indented_doc_comment("Construct a tuple from a fixed-size array of coordinates.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(coords: [GroundedCoord; N]) -> Self {");
    f.line("        Self { coords }");
    f.line("    }");
    f.line("}");
    f.blank();

    // GroundedValue sealed trait
    f.doc_comment("Sealed marker trait for grounded intermediates.");
    f.doc_comment("");
    f.doc_comment("Implemented only for `GroundedCoord` and `GroundedTuple<N>`.");
    f.doc_comment("Prism code cannot implement this \\[the sealed module pattern");
    f.doc_comment("prevents it\\].");
    f.line("pub trait GroundedValue: sealed::Sealed {}");
    f.line("impl GroundedValue for GroundedCoord {}");
    f.line("impl<const N: usize> GroundedValue for GroundedTuple<N> {}");
    f.blank();

    // Grounding open trait
    f.doc_comment("Open trait for boundary crossing: external data to grounded intermediate.");
    f.doc_comment("");
    f.doc_comment("The foundation validates the returned value against the declared");
    f.doc_comment("`GroundingShape` and mints it into a `Datum` if conformant.");
    f.doc_example(
        "use uor_foundation::enforcement::{Grounding, GroundedCoord};\n\
         \n\
         /// Doubling grounding: maps each input byte b to 2b mod 256.\n\
         struct DoublingGrounding;\n\
         \n\
         impl Grounding for DoublingGrounding {\n\
         \x20   type Output = GroundedCoord;\n\
         \n\
         \x20   fn ground(&self, external: &[u8]) -> Option<GroundedCoord> {\n\
         \x20       // Reject empty input at the boundary\n\
         \x20       let &byte = external.first()?;\n\
         \x20       // Apply the doubling map: b -> 2b mod 256\n\
         \x20       Some(GroundedCoord::q0(byte.wrapping_mul(2)))\n\
         \x20   }\n\
         }",
        "rust,ignore",
    );
    f.line("pub trait Grounding {");
    f.indented_doc_comment(
        "The grounded intermediate type. Bounded by `GroundedValue`,\n\
         which is sealed \\[only `GroundedCoord` and `GroundedTuple<N>`\n\
         are permitted\\].",
    );
    f.line("    type Output: GroundedValue;");
    f.blank();
    f.indented_doc_comment(
        "Map external bytes into a grounded intermediate.\n\
         The foundation handles validation and minting.\n\
         Returns `None` if the input is malformed or undersized.",
    );
    f.line("    fn ground(&self, external: &[u8]) -> Option<Self::Output>;");
    f.line("}");
    f.blank();
}

fn generate_witness_types(f: &mut RustFile) {
    // Validated<T>
    f.doc_comment("Proof that a value was produced by the conformance checker,");
    f.doc_comment("not fabricated by Prism code.");
    f.doc_comment("");
    f.doc_comment("The inner value and `_sealed` field are private, so `Validated<T>`");
    f.doc_comment("can only be constructed within this crate.");
    f.doc_example(
        "use uor_foundation::enforcement::{CompileUnitBuilder, Term};\n\
         use uor_foundation::{WittLevel, VerificationDomain};\n\
         \n\
         // Validated<T> proves that a value passed conformance checking.\n\
         // You cannot construct one directly — only builder validate() methods\n\
         // and the minting boundary produce them.\n\
         let terms = [Term::Literal { value: 1, level: WittLevel::W8 }];\n\
         let domains = [VerificationDomain::Enumerative];\n\
         \n\
         let validated = CompileUnitBuilder::new()\n\
         \x20   .root_term(&terms)\n\
         \x20   .witt_level_ceiling(WittLevel::W8)\n\
         \x20   .thermodynamic_budget(1024)\n\
         \x20   .target_domains(&domains)\n\
         \x20   .validate()\n\
         \x20   .expect(\"all fields set\");\n\
         \n\
         // Access the inner value through the proof wrapper:\n\
         let compile_unit = validated.inner();",
        "rust,ignore",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct Validated<T> {");
    f.indented_doc_comment("The validated inner value.");
    f.line("    inner: T,");
    f.indented_doc_comment("Prevents external construction.");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<T> Validated<T> {");
    f.indented_doc_comment("Returns a reference to the validated inner value.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn inner(&self) -> &T {");
    f.line("        &self.inner");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Creates a new `Validated<T>` wrapper. Only callable within the crate.");
    f.line("    #[inline]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(inner: T) -> Self {");
    f.line("        Self { inner, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();

    // Derivation
    f.doc_comment("An opaque derivation trace that can only be extended by the rewrite engine.");
    f.doc_comment("");
    f.doc_comment("Records the number of rewrite steps and the content address of the");
    f.doc_comment("root term. Private fields prevent external construction.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct Derivation {");
    f.indented_doc_comment("Number of rewrite steps in this derivation.");
    f.line("    step_count: u32,");
    f.indented_doc_comment("Content address of the root term.");
    f.line("    root_address: u64,");
    f.line("}");
    f.blank();
    f.line("impl Derivation {");
    f.indented_doc_comment("Returns the number of rewrite steps.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn step_count(&self) -> u32 {");
    f.line("        self.step_count");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the content address of the root term.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn root_address(&self) -> u64 {");
    f.line("        self.root_address");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Creates a new derivation. Only callable within the crate.");
    f.line("    #[inline]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(step_count: u32, root_address: u64) -> Self {");
    f.line("        Self { step_count, root_address }");
    f.line("    }");
    f.line("}");
    f.blank();

    // FreeRank
    f.doc_comment("An opaque free rank that can only be decremented by `PinningEffect`");
    f.doc_comment("and incremented by `UnbindingEffect` \\[never by direct mutation\\].");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct FreeRank {");
    f.indented_doc_comment("Total site capacity at the Witt level.");
    f.line("    total: u32,");
    f.indented_doc_comment("Currently pinned sites.");
    f.line("    pinned: u32,");
    f.line("}");
    f.blank();
    f.line("impl FreeRank {");
    f.indented_doc_comment("Returns the total site capacity.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn total(&self) -> u32 {");
    f.line("        self.total");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the number of currently pinned sites.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn pinned(&self) -> u32 {");
    f.line("        self.pinned");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the number of remaining (unpinned) sites.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn remaining(&self) -> u32 {");
    f.line("        self.total - self.pinned");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Creates a new free rank. Only callable within the crate.");
    f.line("    #[inline]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(total: u32, pinned: u32) -> Self {");
    f.line("        Self { total, pinned }");
    f.line("    }");
    f.line("}");
    f.blank();
}

fn generate_term_ast(f: &mut RustFile) {
    // TermList
    f.doc_comment("Fixed-capacity term list for `#![no_std]`. Indices into a `TermArena`.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
    f.line("pub struct TermList {");
    f.indented_doc_comment("Start index in the arena.");
    f.line("    pub start: u32,");
    f.indented_doc_comment("Number of terms in this list.");
    f.line("    pub len: u32,");
    f.line("}");
    f.blank();

    // TermArena
    f.doc_comment("Stack-resident arena for `Term` trees.");
    f.doc_comment("");
    f.doc_comment("Fixed capacity determined by the const generic `CAP`.");
    f.doc_comment("All `Term` child references are `u32` indices into this arena.");
    f.doc_comment("`#![no_std]`-safe: no heap allocation.");
    f.doc_example(
        "use uor_foundation::enforcement::{TermArena, Term, TermList};\n\
         use uor_foundation::{WittLevel, PrimitiveOp};\n\
         \n\
         // Build the expression `add(3, 5)` bottom-up in an arena.\n\
         let mut arena = TermArena::<4>::new();\n\
         \n\
         // Push leaves first:\n\
         let idx_3 = arena.push(Term::Literal { value: 3, level: WittLevel::W8 });\n\
         let idx_5 = arena.push(Term::Literal { value: 5, level: WittLevel::W8 });\n\
         \n\
         // Push the application node, referencing the leaves by index:\n\
         let idx_add = arena.push(Term::Application {\n\
         \x20   operator: PrimitiveOp::Add,\n\
         \x20   args: TermList { start: idx_3.unwrap_or(0), len: 2 },\n\
         });\n\
         \n\
         assert_eq!(arena.len(), 3);\n\
         // Retrieve a node by index:\n\
         let node = arena.get(idx_add.unwrap_or(0));\n\
         assert!(node.is_some());",
        "rust",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct TermArena<const CAP: usize> {");
    f.indented_doc_comment("Node storage. `None` slots are unused.");
    f.line("    nodes: [Option<Term>; CAP],");
    f.indented_doc_comment("Number of allocated nodes.");
    f.line("    len: u32,");
    f.line("}");
    f.blank();
    f.line("impl<const CAP: usize> TermArena<CAP> {");
    f.indented_doc_comment("Creates an empty arena.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn new() -> Self {");
    f.line("        Self {");
    f.line("            nodes: core::array::from_fn(|_| None),");
    f.line("            len: 0,");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Push a term into the arena and return its index.");
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `None` if the arena is full.");
    f.line("    #[must_use]");
    f.line("    pub fn push(&mut self, term: Term) -> Option<u32> {");
    f.line("        let idx = self.len;");
    f.line("        if (idx as usize) >= CAP {");
    f.line("            return None;");
    f.line("        }");
    f.line("        self.nodes[idx as usize] = Some(term);");
    f.line("        self.len = idx + 1;");
    f.line("        Some(idx)");
    f.line("    }");
    f.blank();
    f.indented_doc_comment(
        "Returns a reference to the term at `index`, or `None` if out of bounds.",
    );
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn get(&self, index: u32) -> Option<&Term> {");
    f.line("        self.nodes.get(index as usize).and_then(|slot| slot.as_ref())");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the number of allocated nodes.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn len(&self) -> u32 {");
    f.line("        self.len");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns `true` if the arena has no allocated nodes.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn is_empty(&self) -> bool {");
    f.line("        self.len == 0");
    f.line("    }");
    f.line("}");
    f.blank();
    f.line("impl<const CAP: usize> Default for TermArena<CAP> {");
    f.line("    fn default() -> Self {");
    f.line("        Self::new()");
    f.line("    }");
    f.line("}");
    f.blank();

    // Term
    f.doc_comment("Concrete AST node for the UOR term language.");
    f.doc_comment("");
    f.doc_comment("Mirrors the EBNF grammar productions. All child references are");
    f.doc_comment("indices into a `TermArena`, keeping the AST stack-resident and");
    f.doc_comment("`#![no_std]`-safe.");
    f.doc_example(
        "use uor_foundation::enforcement::{Term, TermList};\n\
         use uor_foundation::{WittLevel, PrimitiveOp};\n\
         \n\
         // Literal: an integer value tagged with a Witt level.\n\
         let lit = Term::Literal { value: 42, level: WittLevel::W8 };\n\
         \n\
         // Application: an operation applied to arguments.\n\
         // `args` is a TermList { start, len } pointing into a TermArena.\n\
         let app = Term::Application {\n\
         \x20   operator: PrimitiveOp::Mul,\n\
         \x20   args: TermList { start: 0, len: 2 },\n\
         };\n\
         \n\
         // Lift: canonical injection from a lower to a higher Witt level.\n\
         let lift = Term::Lift { operand_index: 0, target: WittLevel::new(32) };\n\
         \n\
         // Project: canonical surjection from a higher to a lower level.\n\
         let proj = Term::Project { operand_index: 0, target: WittLevel::W8 };",
        "rust",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub enum Term {");
    f.indented_doc_comment("Integer literal with Witt level annotation.");
    f.line("    Literal {");
    f.line("        /// The literal integer value.");
    f.line("        value: u64,");
    f.line("        /// The Witt level of this literal.");
    f.line("        level: WittLevel,");
    f.line("    },");
    f.indented_doc_comment("Variable reference by name index.");
    f.line("    Variable {");
    f.line("        /// Index into the name table.");
    f.line("        name_index: u32,");
    f.line("    },");
    f.indented_doc_comment("Operation application: operator applied to arguments.");
    f.line("    Application {");
    f.line("        /// The primitive operation to apply.");
    f.line("        operator: PrimitiveOp,");
    f.line("        /// Argument list (indices into arena).");
    f.line("        args: TermList,");
    f.line("    },");
    f.indented_doc_comment("Lift: canonical injection W_n to W_m (n < m, lossless).");
    f.line("    Lift {");
    f.line("        /// Index of the operand term in the arena.");
    f.line("        operand_index: u32,");
    f.line("        /// Target Witt level.");
    f.line("        target: WittLevel,");
    f.line("    },");
    f.indented_doc_comment("Project: canonical surjection W_m to W_n (m > n, lossy).");
    f.line("    Project {");
    f.line("        /// Index of the operand term in the arena.");
    f.line("        operand_index: u32,");
    f.line("        /// Target Witt level.");
    f.line("        target: WittLevel,");
    f.line("    },");
    f.indented_doc_comment("Match expression with pattern-result pairs.");
    f.line("    Match {");
    f.line("        /// Index of the scrutinee term in the arena.");
    f.line("        scrutinee_index: u32,");
    f.line("        /// Match arms (indices into arena).");
    f.line("        arms: TermList,");
    f.line("    },");
    f.indented_doc_comment("Bounded recursion with descent measure.");
    f.line("    Recurse {");
    f.line("        /// Index of the descent measure term.");
    f.line("        measure_index: u32,");
    f.line("        /// Index of the base case term.");
    f.line("        base_index: u32,");
    f.line("        /// Index of the recursive step term.");
    f.line("        step_index: u32,");
    f.line("    },");
    f.indented_doc_comment("Stream construction via unfold.");
    f.line("    Unfold {");
    f.line("        /// Index of the seed term.");
    f.line("        seed_index: u32,");
    f.line("        /// Index of the step function term.");
    f.line("        step_index: u32,");
    f.line("    },");
    f.indented_doc_comment("Try expression with failure recovery.");
    f.line("    Try {");
    f.line("        /// Index of the body term.");
    f.line("        body_index: u32,");
    f.line("        /// Index of the handler term.");
    f.line("        handler_index: u32,");
    f.line("    },");
    f.line("}");
    f.blank();

    // TypeDeclaration
    f.doc_comment("A type declaration with constraint kinds.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct TypeDeclaration {");
    f.indented_doc_comment("Name index for this type.");
    f.line("    pub name_index: u32,");
    f.indented_doc_comment("Constraint terms (indices into arena).");
    f.line("    pub constraints: TermList,");
    f.line("}");
    f.blank();

    // Binding
    f.doc_comment("A named binding: `let name : Type = term`.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct Binding {");
    f.indented_doc_comment("Name index for this binding.");
    f.line("    pub name_index: u32,");
    f.indented_doc_comment("Index of the type declaration.");
    f.line("    pub type_index: u32,");
    f.indented_doc_comment("Index of the value term in the arena.");
    f.line("    pub value_index: u32,");
    f.indented_doc_comment("EBNF surface syntax (compile-time constant).");
    f.line("    pub surface: &'static str,");
    f.indented_doc_comment("FNV-1a content address (compile-time constant).");
    f.line("    pub content_address: u64,");
    f.line("}");
    f.blank();

    // Assertion
    f.doc_comment("An assertion: `assert lhs = rhs`.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct Assertion {");
    f.indented_doc_comment("Index of the left-hand side term.");
    f.line("    pub lhs_index: u32,");
    f.indented_doc_comment("Index of the right-hand side term.");
    f.line("    pub rhs_index: u32,");
    f.indented_doc_comment("EBNF surface syntax (compile-time constant).");
    f.line("    pub surface: &'static str,");
    f.line("}");
    f.blank();

    // SourceDeclaration
    f.doc_comment("Boundary source declaration: `source name : Type via grounding`.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct SourceDeclaration {");
    f.indented_doc_comment("Name index for the source.");
    f.line("    pub name_index: u32,");
    f.indented_doc_comment("Index of the type declaration.");
    f.line("    pub type_index: u32,");
    f.indented_doc_comment("Name index of the grounding map.");
    f.line("    pub grounding_name_index: u32,");
    f.line("}");
    f.blank();

    // SinkDeclaration
    f.doc_comment("Boundary sink declaration: `sink name : Type via projection`.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct SinkDeclaration {");
    f.indented_doc_comment("Name index for the sink.");
    f.line("    pub name_index: u32,");
    f.indented_doc_comment("Index of the type declaration.");
    f.line("    pub type_index: u32,");
    f.indented_doc_comment("Name index of the projection map.");
    f.line("    pub projection_name_index: u32,");
    f.line("}");
    f.blank();
}

fn generate_shape_violation(f: &mut RustFile) {
    f.doc_comment("Structured violation diagnostic carrying metadata from the");
    f.doc_comment("conformance namespace. Every field is machine-readable.");
    f.doc_example(
        "use uor_foundation::enforcement::ShapeViolation;\n\
         use uor_foundation::ViolationKind;\n\
         \n\
         // ShapeViolation carries structured metadata from the ontology.\n\
         // Every field is machine-readable — IRIs, counts, and a typed kind.\n\
         let violation = ShapeViolation {\n\
         \x20   shape_iri: \"https://uor.foundation/conformance/CompileUnitShape\",\n\
         \x20   constraint_iri: \"https://uor.foundation/conformance/compileUnit_rootTerm_constraint\",\n\
         \x20   property_iri: \"https://uor.foundation/reduction/rootTerm\",\n\
         \x20   expected_range: \"https://uor.foundation/schema/Term\",\n\
         \x20   min_count: 1,\n\
         \x20   max_count: 1,\n\
         \x20   kind: ViolationKind::Missing,\n\
         };\n\
         \n\
         // Machine-readable for tooling (IDE plugins, CI pipelines):\n\
         assert_eq!(violation.kind, ViolationKind::Missing);\n\
         assert!(violation.shape_iri.ends_with(\"CompileUnitShape\"));\n\
         assert_eq!(violation.min_count, 1);",
        "rust",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct ShapeViolation {");
    f.indented_doc_comment("IRI of the `conformance:Shape` that was validated against.");
    f.line("    pub shape_iri: &'static str,");
    f.indented_doc_comment("IRI of the specific `conformance:PropertyConstraint` that failed.");
    f.line("    pub constraint_iri: &'static str,");
    f.indented_doc_comment("IRI of the property that was missing or invalid.");
    f.line("    pub property_iri: &'static str,");
    f.indented_doc_comment("The expected range class IRI.");
    f.line("    pub expected_range: &'static str,");
    f.indented_doc_comment("Minimum cardinality from the constraint.");
    f.line("    pub min_count: u32,");
    f.indented_doc_comment("Maximum cardinality (0 = unbounded).");
    f.line("    pub max_count: u32,");
    f.indented_doc_comment("What went wrong.");
    f.line("    pub kind: ViolationKind,");
    f.line("}");
    f.blank();
}

fn generate_builders(f: &mut RustFile) {
    // CompileUnitBuilder
    f.doc_comment("Builder for `CompileUnit` admission into the reduction pipeline.");
    f.doc_comment("");
    f.doc_comment("Collects `rootTerm`, `wittLevelCeiling`, `thermodynamicBudget`,");
    f.doc_comment("and `targetDomains`. The `validate()` method checks structural");
    f.doc_comment("constraints (Tier 1) and value-dependent constraints (Tier 2).");
    f.doc_example(
        "use uor_foundation::enforcement::{CompileUnitBuilder, Term};\n\
         use uor_foundation::{WittLevel, VerificationDomain, ViolationKind};\n\
         \n\
         // A CompileUnit packages a term graph for reduction admission.\n\
         // The builder enforces that all required fields are present.\n\
         let terms = [Term::Literal { value: 1, level: WittLevel::W8 }];\n\
         let domains = [VerificationDomain::Enumerative];\n\
         \n\
         let unit = CompileUnitBuilder::new()\n\
         \x20   .root_term(&terms)\n\
         \x20   .witt_level_ceiling(WittLevel::W8)\n\
         \x20   .thermodynamic_budget(1024)\n\
         \x20   .target_domains(&domains)\n\
         \x20   .validate();\n\
         assert!(unit.is_ok());\n\
         \n\
         // Omitting a required field produces a ShapeViolation\n\
         // with the exact conformance IRI that failed:\n\
         let err = CompileUnitBuilder::new()\n\
         \x20   .witt_level_ceiling(WittLevel::W8)\n\
         \x20   .thermodynamic_budget(1024)\n\
         \x20   .target_domains(&domains)\n\
         \x20   .validate();\n\
         assert!(err.is_err());\n\
         if let Err(violation) = err {\n\
         \x20   assert_eq!(violation.kind, ViolationKind::Missing);\n\
         \x20   assert!(violation.property_iri.contains(\"rootTerm\"));\n\
         }",
        "rust",
    );
    f.line("#[derive(Debug, Clone)]");
    f.line("pub struct CompileUnitBuilder<'a> {");
    f.indented_doc_comment("The root term expression.");
    f.line("    root_term: Option<&'a [Term]>,");
    f.indented_doc_comment("The widest Witt level the computation may reference.");
    f.line("    witt_level_ceiling: Option<WittLevel>,");
    f.indented_doc_comment("Landauer-bounded energy budget.");
    f.line("    thermodynamic_budget: Option<u64>,");
    f.indented_doc_comment("Verification domains targeted.");
    f.line("    target_domains: Option<&'a [VerificationDomain]>,");
    f.line("}");
    f.blank();

    // CompileUnit (validated result type)
    f.doc_comment("A validated compile unit ready for reduction admission.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct CompileUnit {");
    f.indented_doc_comment("The Witt level ceiling.");
    f.line("    level: WittLevel,");
    f.indented_doc_comment("The thermodynamic budget.");
    f.line("    budget: u64,");
    f.line("}");
    f.blank();

    f.line("impl<'a> CompileUnitBuilder<'a> {");
    f.indented_doc_comment("Creates a new empty builder.");
    f.line("    #[must_use]");
    f.line("    pub const fn new() -> Self {");
    f.line("        Self {");
    f.line("            root_term: None,");
    f.line("            witt_level_ceiling: None,");
    f.line("            thermodynamic_budget: None,");
    f.line("            target_domains: None,");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the root term expression.");
    f.line("    #[must_use]");
    f.line("    pub const fn root_term(mut self, terms: &'a [Term]) -> Self {");
    f.line("        self.root_term = Some(terms);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the Witt level ceiling.");
    f.line("    #[must_use]");
    f.line("    pub const fn witt_level_ceiling(mut self, level: WittLevel) -> Self {");
    f.line("        self.witt_level_ceiling = Some(level);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the thermodynamic budget.");
    f.line("    #[must_use]");
    f.line("    pub const fn thermodynamic_budget(mut self, budget: u64) -> Self {");
    f.line("        self.thermodynamic_budget = Some(budget);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the target verification domains.");
    f.line("    #[must_use]");
    f.line(
        "    pub const fn target_domains(mut self, domains: &'a [VerificationDomain]) -> Self {",
    );
    f.line("        self.target_domains = Some(domains);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Validate against `CompileUnitShape`.");
    f.indented_doc_comment("");
    f.indented_doc_comment("Tier 1: checks presence and cardinality of all required fields.");
    f.indented_doc_comment("Tier 2: checks budget solvency and level coherence.");
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `ShapeViolation` if any constraint is not satisfied.");
    f.line("    pub fn validate(self) -> Result<Validated<CompileUnit>, ShapeViolation> {");
    f.line("        if self.root_term.is_none() {");
    f.line("            return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/CompileUnitShape\",");
    f.line("                constraint_iri: \"https://uor.foundation/conformance/compileUnit_rootTerm_constraint\",");
    f.line("                property_iri: \"https://uor.foundation/reduction/rootTerm\",");
    f.line("                expected_range: \"https://uor.foundation/schema/Term\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            });");
    f.line("        }");
    f.line("        let level = match self.witt_level_ceiling {");
    f.line("            Some(l) => l,");
    f.line("            None => return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/CompileUnitShape\",");
    f.line("                constraint_iri: \"https://uor.foundation/conformance/compileUnit_unitWittLevel_constraint\",");
    f.line("                property_iri: \"https://uor.foundation/reduction/unitWittLevel\",");
    f.line("                expected_range: \"https://uor.foundation/schema/WittLevel\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            }),");
    f.line("        };");
    f.line("        let budget = match self.thermodynamic_budget {");
    f.line("            Some(b) => b,");
    f.line("            None => return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/CompileUnitShape\",");
    f.line("                constraint_iri: \"https://uor.foundation/conformance/compileUnit_thermodynamicBudget_constraint\",");
    f.line(
        "                property_iri: \"https://uor.foundation/reduction/thermodynamicBudget\",",
    );
    f.line("                expected_range: \"http://www.w3.org/2001/XMLSchema#decimal\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            }),");
    f.line("        };");
    f.line("        match self.target_domains {");
    f.line("            Some(d) if !d.is_empty() => {},");
    f.line("            _ => return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/CompileUnitShape\",");
    f.line("                constraint_iri: \"https://uor.foundation/conformance/compileUnit_targetDomains_constraint\",");
    f.line("                property_iri: \"https://uor.foundation/reduction/targetDomains\",");
    f.line("                expected_range: \"https://uor.foundation/op/VerificationDomain\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 0,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            }),");
    f.line("        }");
    f.line("        Ok(Validated::new(CompileUnit { level, budget }))");
    f.line("    }");
    f.line("}");
    f.blank();

    // Default impl for CompileUnitBuilder
    f.line("impl<'a> Default for CompileUnitBuilder<'a> {");
    f.line("    fn default() -> Self {");
    f.line("        Self::new()");
    f.line("    }");
    f.line("}");
    f.blank();

    // Generate builders for the remaining 8 shapes
    generate_simple_builder(
        f,
        "EffectDeclarationBuilder",
        "Declared effect validated against `EffectShape`.",
        &[
            ("name", "&'a str"),
            ("target_sites", "&'a [u32]"),
            ("budget_delta", "i64"),
            ("commutes", "bool"),
        ],
        "EffectDeclaration",
        "https://uor.foundation/conformance/EffectShape",
    );
    generate_simple_builder(
        f,
        "GroundingDeclarationBuilder",
        "Declared grounding validated against `GroundingShape`.",
        &[
            ("source_type", "&'a str"),
            ("ring_mapping", "&'a str"),
            ("invertibility", "bool"),
        ],
        "GroundingDeclaration",
        "https://uor.foundation/conformance/GroundingShape",
    );
    generate_simple_builder(
        f,
        "DispatchDeclarationBuilder",
        "Declared dispatch rule validated against `DispatchShape`.",
        &[
            ("predicate", "&'a [Term]"),
            ("target_resolver", "&'a str"),
            ("priority", "u32"),
        ],
        "DispatchDeclaration",
        "https://uor.foundation/conformance/DispatchShape",
    );
    generate_simple_builder(
        f,
        "LeaseDeclarationBuilder",
        "Declared lease validated against `LeaseShape`.",
        &[("linear_site", "u32"), ("scope", "&'a str")],
        "LeaseDeclaration",
        "https://uor.foundation/conformance/LeaseShape",
    );
    generate_simple_builder(
        f,
        "StreamDeclarationBuilder",
        "Declared stream validated against `StreamShape`.",
        &[
            ("seed", "&'a [Term]"),
            ("step", "&'a [Term]"),
            ("productivity_witness", "&'a str"),
        ],
        "StreamDeclaration",
        "https://uor.foundation/conformance/StreamShape",
    );
    generate_simple_builder(
        f,
        "PredicateDeclarationBuilder",
        "Declared predicate validated against `PredicateShape`.",
        &[
            ("input_type", "&'a str"),
            ("evaluator", "&'a [Term]"),
            ("termination_witness", "&'a str"),
        ],
        "PredicateDeclaration",
        "https://uor.foundation/conformance/PredicateShape",
    );
    generate_simple_builder(
        f,
        "ParallelDeclarationBuilder",
        "Declared parallel composition validated against `ParallelShape`.",
        &[
            ("site_partition", "&'a [u32]"),
            ("disjointness_witness", "&'a str"),
        ],
        "ParallelDeclaration",
        "https://uor.foundation/conformance/ParallelShape",
    );

    // WittLevelDeclarationBuilder (no lifetime needed)
    f.doc_comment("Builder for declaring a new Witt level beyond W32.");
    f.doc_comment("");
    f.doc_comment("Validates against `WittLevelShape`.");
    f.line("#[derive(Debug, Clone)]");
    f.line("pub struct WittLevelDeclarationBuilder {");
    f.indented_doc_comment("The declared bit width.");
    f.line("    bit_width: Option<u32>,");
    f.indented_doc_comment("The declared cycle size.");
    f.line("    cycle_size: Option<u128>,");
    f.indented_doc_comment("The predecessor level.");
    f.line("    predecessor: Option<WittLevel>,");
    f.line("}");
    f.blank();

    f.doc_comment("Validated Witt level declaration.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct WittLevelDeclaration {");
    f.indented_doc_comment("The declared bit width.");
    f.line("    pub bit_width: u32,");
    f.indented_doc_comment("The predecessor level.");
    f.line("    pub predecessor: WittLevel,");
    f.line("}");
    f.blank();

    f.line("impl WittLevelDeclarationBuilder {");
    f.indented_doc_comment("Creates a new empty builder.");
    f.line("    #[must_use]");
    f.line("    pub const fn new() -> Self {");
    f.line("        Self { bit_width: None, cycle_size: None, predecessor: None }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the declared bit width.");
    f.line("    #[must_use]");
    f.line("    pub const fn bit_width(mut self, w: u32) -> Self {");
    f.line("        self.bit_width = Some(w);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the declared cycle size.");
    f.line("    #[must_use]");
    f.line("    pub const fn cycle_size(mut self, s: u128) -> Self {");
    f.line("        self.cycle_size = Some(s);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the predecessor Witt level.");
    f.line("    #[must_use]");
    f.line("    pub const fn predecessor(mut self, level: WittLevel) -> Self {");
    f.line("        self.predecessor = Some(level);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Validate against `WittLevelShape`.");
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `ShapeViolation` if any required field is missing.");
    f.line(
        "    pub fn validate(self) -> Result<Validated<WittLevelDeclaration>, ShapeViolation> {",
    );
    f.line("        let bw = match self.bit_width {");
    f.line("            Some(w) => w,");
    f.line("            None => return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/WittLevelShape\",");
    f.line(
        "                constraint_iri: \"https://uor.foundation/conformance/WittLevelShape\",",
    );
    f.line(
        "                property_iri: \"https://uor.foundation/conformance/declaredBitWidth\",",
    );
    f.line("                expected_range: \"http://www.w3.org/2001/XMLSchema#positiveInteger\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            }),");
    f.line("        };");
    f.line("        let pred = match self.predecessor {");
    f.line("            Some(p) => p,");
    f.line("            None => return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/WittLevelShape\",");
    f.line(
        "                constraint_iri: \"https://uor.foundation/conformance/WittLevelShape\",",
    );
    f.line(
        "                property_iri: \"https://uor.foundation/conformance/predecessorLevel\",",
    );
    f.line("                expected_range: \"https://uor.foundation/schema/WittLevel\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            }),");
    f.line("        };");
    f.line("        Ok(Validated::new(WittLevelDeclaration { bit_width: bw, predecessor: pred }))");
    f.line("    }");
    f.line("}");
    f.blank();

    f.line("impl Default for WittLevelDeclarationBuilder {");
    f.line("    fn default() -> Self {");
    f.line("        Self::new()");
    f.line("    }");
    f.line("}");
    f.blank();
}

/// Generates a simple builder struct with `Option` fields and a `validate()` method
/// that checks all fields are present.
fn generate_simple_builder(
    f: &mut RustFile,
    builder_name: &str,
    result_doc: &str,
    fields: &[(&str, &str)],
    result_name: &str,
    shape_iri: &str,
) {
    let needs_lifetime = fields.iter().any(|(_, ty)| ty.starts_with('&'));
    let lt = if needs_lifetime { "<'a>" } else { "" };

    // Builder struct
    f.doc_comment(&format!(
        "Builder for `{result_name}`. Validates against `{}`.",
        shape_iri.rsplit('/').next().unwrap_or(shape_iri),
    ));
    f.line("#[derive(Debug, Clone)]");
    f.line(&format!("pub struct {builder_name}{lt} {{"));
    for (name, ty) in fields {
        let opt_ty = format!("Option<{ty}>");
        f.indented_doc_comment(&format!("The `{name}` field."));
        f.line(&format!("    {name}: {opt_ty},"));
    }
    f.line("}");
    f.blank();

    // Validated result struct
    f.doc_comment(result_doc);
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line(&format!("pub struct {result_name} {{"));
    f.indented_doc_comment("Shape IRI this declaration was validated against.");
    f.line("    pub shape_iri: &'static str,");
    f.line("}");
    f.blank();

    // impl block
    f.line(&format!("impl{lt} {builder_name}{lt} {{"));
    f.indented_doc_comment("Creates a new empty builder.");
    f.line("    #[must_use]");
    f.line("    pub const fn new() -> Self {");
    f.line("        Self {");
    for (name, _) in fields {
        f.line(&format!("            {name}: None,"));
    }
    f.line("        }");
    f.line("    }");
    f.blank();

    // Setter methods
    for (name, ty) in fields {
        f.indented_doc_comment(&format!("Set the `{name}` field."));
        f.line("    #[must_use]");
        f.line(&format!(
            "    pub const fn {name}(mut self, value: {ty}) -> Self {{"
        ));
        f.line(&format!("        self.{name} = Some(value);"));
        f.line("        self");
        f.line("    }");
        f.blank();
    }

    // validate method
    f.indented_doc_comment(&format!(
        "Validate against `{}`.",
        shape_iri.rsplit('/').next().unwrap_or(shape_iri)
    ));
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `ShapeViolation` if any required field is missing.");
    f.line(&format!(
        "    pub fn validate(self) -> Result<Validated<{result_name}>, ShapeViolation> {{"
    ));
    // Check first field as representative
    let first = fields[0].0;
    f.line(&format!("        if self.{first}.is_none() {{"));
    f.line("            return Err(ShapeViolation {");
    f.line(&format!("                shape_iri: \"{shape_iri}\","));
    f.line(&format!("                constraint_iri: \"{shape_iri}\","));
    f.line(&format!(
        "                property_iri: \"https://uor.foundation/conformance/{first}\","
    ));
    f.line("                expected_range: \"http://www.w3.org/2002/07/owl#Thing\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            });");
    f.line("        }");
    // Check remaining fields
    for (name, _) in &fields[1..] {
        f.line(&format!("        if self.{name}.is_none() {{"));
        f.line("            return Err(ShapeViolation {");
        f.line(&format!("                shape_iri: \"{shape_iri}\","));
        f.line(&format!("                constraint_iri: \"{shape_iri}\","));
        f.line(&format!(
            "                property_iri: \"https://uor.foundation/conformance/{name}\","
        ));
        f.line("                expected_range: \"http://www.w3.org/2002/07/owl#Thing\",");
        f.line("                min_count: 1,");
        f.line("                max_count: 1,");
        f.line("                kind: ViolationKind::Missing,");
        f.line("            });");
        f.line("        }");
    }
    f.line(&format!(
        "        Ok(Validated::new({result_name} {{ shape_iri: \"{shape_iri}\" }}))"
    ));
    f.line("    }");
    f.line("}");
    f.blank();

    // Default impl
    f.line(&format!("impl{lt} Default for {builder_name}{lt} {{"));
    f.line("    fn default() -> Self {");
    f.line("        Self::new()");
    f.line("    }");
    f.line("}");
    f.blank();
}

fn generate_minting_session(f: &mut RustFile) {
    f.doc_comment("Boundary session state tracker for the two-phase minting boundary.");
    f.doc_comment("");
    f.doc_comment("Records crossing count and idempotency flag. Private fields");
    f.doc_comment("prevent external construction.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct BoundarySession {");
    f.indented_doc_comment("Total boundary crossings in this session.");
    f.line("    crossing_count: u32,");
    f.indented_doc_comment("Whether the boundary effect is idempotent.");
    f.line("    is_idempotent: bool,");
    f.line("}");
    f.blank();
    f.line("impl BoundarySession {");
    f.indented_doc_comment("Creates a new boundary session. Only callable within the crate.");
    f.line("    #[inline]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(is_idempotent: bool) -> Self {");
    f.line("        Self { crossing_count: 0, is_idempotent }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the total boundary crossings.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn crossing_count(&self) -> u32 {");
    f.line("        self.crossing_count");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns whether the boundary effect is idempotent.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn is_idempotent(&self) -> bool {");
    f.line("        self.is_idempotent");
    f.line("    }");
    f.line("}");
    f.blank();

    // validate_and_mint functions
    f.doc_comment("Validate a scalar grounding intermediate against a `GroundingShape`");
    f.doc_comment("and mint it into a `Datum`. Only callable within `uor-foundation`.");
    f.doc_comment("");
    f.doc_comment("# Errors");
    f.doc_comment("");
    f.doc_comment("Returns `ShapeViolation` if the coordinate fails validation.");
    f.line("#[allow(dead_code)]");
    f.line("pub(crate) fn validate_and_mint_coord(");
    f.line("    grounded: GroundedCoord,");
    f.line("    shape: &Validated<GroundingDeclaration>,");
    f.line("    session: &mut BoundarySession,");
    f.line(") -> Result<Datum, ShapeViolation> {");
    f.line("    // The Validated<GroundingDeclaration> proves the shape was already");
    f.line("    // validated at builder time. The coordinate's level is guaranteed");
    f.line("    // correct by the closed GroundedCoordInner enum — the type system");
    f.line("    // enforces that only supported levels can be constructed.");
    f.line("    let _ = shape; // shape validation passed at builder time");
    f.line("    session.crossing_count += 1;");
    f.line("    let inner = match grounded.inner {");
    f.line("        GroundedCoordInner::Q0(b) => DatumInner::Q0(b),");
    f.line("        GroundedCoordInner::Q1(b) => DatumInner::Q1(b),");
    f.line("        GroundedCoordInner::Q3(b) => DatumInner::Q3(b),");
    f.line("        GroundedCoordInner::Q7(b) => DatumInner::Q7(b),");
    f.line("        GroundedCoordInner::Q511(b) => DatumInner::Q511(b),");
    f.line("    };");
    f.line("    Ok(Datum { inner })");
    f.line("}");
    f.blank();

    f.doc_comment("Validate a tuple grounding intermediate and mint into a `Datum`.");
    f.doc_comment("Only callable within `uor-foundation`.");
    f.doc_comment("");
    f.doc_comment("Mints the first coordinate of the tuple as the representative `Datum`.");
    f.doc_comment("Composite multi-coordinate `Datum` construction depends on the target");
    f.doc_comment("type's site decomposition, which is resolved during reduction evaluation.");
    f.doc_comment("");
    f.doc_comment("# Errors");
    f.doc_comment("");
    f.doc_comment("Returns `ShapeViolation` if the tuple is empty or fails validation.");
    f.line("#[allow(dead_code)]");
    f.line("pub(crate) fn validate_and_mint_tuple<const N: usize>(");
    f.line("    grounded: GroundedTuple<N>,");
    f.line("    shape: &Validated<GroundingDeclaration>,");
    f.line("    session: &mut BoundarySession,");
    f.line(") -> Result<Datum, ShapeViolation> {");
    f.line("    if N == 0 {");
    f.line("        return Err(ShapeViolation {");
    f.line("            shape_iri: shape.inner().shape_iri,");
    f.line("            constraint_iri: shape.inner().shape_iri,");
    f.line("            property_iri: \"https://uor.foundation/conformance/groundingSourceType\",");
    f.line("            expected_range: \"https://uor.foundation/type/TypeDefinition\",");
    f.line("            min_count: 1,");
    f.line("            max_count: 0,");
    f.line("            kind: ViolationKind::CardinalityViolation,");
    f.line("        });");
    f.line("    }");
    f.line("    // Mint the first coordinate as the representative Datum.");
    f.line("    // The full tuple is decomposed during reduction evaluation,");
    f.line("    // where each coordinate maps to a site in the constrained type.");
    f.line("    validate_and_mint_coord(grounded.coords[0].clone(), shape, session)");
    f.line("}");
    f.blank();
}

fn generate_const_ring_eval(f: &mut RustFile) {
    f.doc_comment("Evaluate a binary ring operation on Q0 (8-bit, Z/256Z) at compile time.");
    f.doc_comment("");
    f.doc_comment("This is the sole evaluator for ground assertions. The `uor!` proc macro");
    f.doc_comment("delegates to this function; it never performs ring arithmetic itself.");
    f.doc_example(
        "use uor_foundation::enforcement::{const_ring_eval_q0, const_ring_eval_unary_q0};\n\
         use uor_foundation::PrimitiveOp;\n\
         \n\
         // Ring arithmetic in Z/256Z: all operations wrap modulo 256.\n\
         \n\
         // Addition wraps: 200 + 100 = 300 -> 300 - 256 = 44\n\
         assert_eq!(const_ring_eval_q0(PrimitiveOp::Add, 200, 100), 44);\n\
         \n\
         // Multiplication: 3 * 5 = 15 (no wrap needed)\n\
         assert_eq!(const_ring_eval_q0(PrimitiveOp::Mul, 3, 5), 15);\n\
         \n\
         // XOR: bitwise exclusive-or\n\
         assert_eq!(const_ring_eval_q0(PrimitiveOp::Xor, 0b1010, 0b1100), 0b0110);\n\
         \n\
         // Negation: neg(x) = 256 - x (additive inverse in Z/256Z)\n\
         assert_eq!(const_ring_eval_unary_q0(PrimitiveOp::Neg, 1), 255);\n\
         \n\
         // The critical identity: neg(bnot(x)) = succ(x) for all x\n\
         let x = 42u8;\n\
         let lhs = const_ring_eval_unary_q0(PrimitiveOp::Neg,\n\
         \x20   const_ring_eval_unary_q0(PrimitiveOp::Bnot, x));\n\
         let rhs = const_ring_eval_unary_q0(PrimitiveOp::Succ, x);\n\
         assert_eq!(lhs, rhs);",
        "rust",
    );
    f.line("#[inline]");
    f.line("#[must_use]");
    f.line("pub const fn const_ring_eval_q0(op: PrimitiveOp, a: u8, b: u8) -> u8 {");
    f.line("    match op {");
    f.line("        PrimitiveOp::Add => a.wrapping_add(b),");
    f.line("        PrimitiveOp::Sub => a.wrapping_sub(b),");
    f.line("        PrimitiveOp::Mul => a.wrapping_mul(b),");
    f.line("        PrimitiveOp::Xor => a ^ b,");
    f.line("        PrimitiveOp::And => a & b,");
    f.line("        PrimitiveOp::Or => a | b,");
    f.line("        _ => 0,");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("Evaluate a unary ring operation on Q0 (8-bit, Z/256Z) at compile time.");
    f.line("#[inline]");
    f.line("#[must_use]");
    f.line("pub const fn const_ring_eval_unary_q0(op: PrimitiveOp, a: u8) -> u8 {");
    f.line("    match op {");
    f.line("        PrimitiveOp::Neg => 0u8.wrapping_sub(a),");
    f.line("        PrimitiveOp::Bnot => !a,");
    f.line("        PrimitiveOp::Succ => a.wrapping_add(1),");
    f.line("        PrimitiveOp::Pred => a.wrapping_sub(1),");
    f.line("        _ => 0,");
    f.line("    }");
    f.line("}");
    f.blank();

    // Q1 (u16)
    f.doc_comment("Evaluate a binary ring operation on Q1 (16-bit, Z/65536Z) at compile time.");
    f.line("#[inline]");
    f.line("#[must_use]");
    f.line("pub const fn const_ring_eval_q1(op: PrimitiveOp, a: u16, b: u16) -> u16 {");
    f.line("    match op {");
    f.line("        PrimitiveOp::Add => a.wrapping_add(b),");
    f.line("        PrimitiveOp::Sub => a.wrapping_sub(b),");
    f.line("        PrimitiveOp::Mul => a.wrapping_mul(b),");
    f.line("        PrimitiveOp::Xor => a ^ b,");
    f.line("        PrimitiveOp::And => a & b,");
    f.line("        PrimitiveOp::Or => a | b,");
    f.line("        _ => 0,");
    f.line("    }");
    f.line("}");
    f.blank();
    f.doc_comment("Evaluate a unary ring operation on Q1 (16-bit, Z/65536Z) at compile time.");
    f.line("#[inline]");
    f.line("#[must_use]");
    f.line("pub const fn const_ring_eval_unary_q1(op: PrimitiveOp, a: u16) -> u16 {");
    f.line("    match op {");
    f.line("        PrimitiveOp::Neg => 0u16.wrapping_sub(a),");
    f.line("        PrimitiveOp::Bnot => !a,");
    f.line("        PrimitiveOp::Succ => a.wrapping_add(1),");
    f.line("        PrimitiveOp::Pred => a.wrapping_sub(1),");
    f.line("        _ => 0,");
    f.line("    }");
    f.line("}");
    f.blank();

    // Q3 (u32)
    f.doc_comment("Evaluate a binary ring operation on Q3 (32-bit, Z/2^32Z) at compile time.");
    f.line("#[inline]");
    f.line("#[must_use]");
    f.line("pub const fn const_ring_eval_q3(op: PrimitiveOp, a: u32, b: u32) -> u32 {");
    f.line("    match op {");
    f.line("        PrimitiveOp::Add => a.wrapping_add(b),");
    f.line("        PrimitiveOp::Sub => a.wrapping_sub(b),");
    f.line("        PrimitiveOp::Mul => a.wrapping_mul(b),");
    f.line("        PrimitiveOp::Xor => a ^ b,");
    f.line("        PrimitiveOp::And => a & b,");
    f.line("        PrimitiveOp::Or => a | b,");
    f.line("        _ => 0,");
    f.line("    }");
    f.line("}");
    f.blank();
    f.doc_comment("Evaluate a unary ring operation on Q3 (32-bit, Z/2^32Z) at compile time.");
    f.line("#[inline]");
    f.line("#[must_use]");
    f.line("pub const fn const_ring_eval_unary_q3(op: PrimitiveOp, a: u32) -> u32 {");
    f.line("    match op {");
    f.line("        PrimitiveOp::Neg => 0u32.wrapping_sub(a),");
    f.line("        PrimitiveOp::Bnot => !a,");
    f.line("        PrimitiveOp::Succ => a.wrapping_add(1),");
    f.line("        PrimitiveOp::Pred => a.wrapping_sub(1),");
    f.line("        _ => 0,");
    f.line("    }");
    f.line("}");
    f.blank();

    // Q7 (u64)
    f.doc_comment("Evaluate a binary ring operation on Q7 (64-bit, Z/2^64Z) at compile time.");
    f.line("#[inline]");
    f.line("#[must_use]");
    f.line("pub const fn const_ring_eval_q7(op: PrimitiveOp, a: u64, b: u64) -> u64 {");
    f.line("    match op {");
    f.line("        PrimitiveOp::Add => a.wrapping_add(b),");
    f.line("        PrimitiveOp::Sub => a.wrapping_sub(b),");
    f.line("        PrimitiveOp::Mul => a.wrapping_mul(b),");
    f.line("        PrimitiveOp::Xor => a ^ b,");
    f.line("        PrimitiveOp::And => a & b,");
    f.line("        PrimitiveOp::Or => a | b,");
    f.line("        _ => 0,");
    f.line("    }");
    f.line("}");
    f.blank();
    f.doc_comment("Evaluate a unary ring operation on Q7 (64-bit, Z/2^64Z) at compile time.");
    f.line("#[inline]");
    f.line("#[must_use]");
    f.line("pub const fn const_ring_eval_unary_q7(op: PrimitiveOp, a: u64) -> u64 {");
    f.line("    match op {");
    f.line("        PrimitiveOp::Neg => 0u64.wrapping_sub(a),");
    f.line("        PrimitiveOp::Bnot => !a,");
    f.line("        PrimitiveOp::Succ => a.wrapping_add(1),");
    f.line("        PrimitiveOp::Pred => a.wrapping_sub(1),");
    f.line("        _ => 0,");
    f.line("    }");
    f.line("}");
    f.blank();
}
