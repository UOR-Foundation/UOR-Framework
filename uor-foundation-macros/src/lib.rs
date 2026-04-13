//! Proc macro crate for the UOR Foundation.
//!
//! Provides the `uor!` macro (since v0.2.0) and the v0.2.1 ergonomics
//! macros: `uor_ground!`, `#[derive(ConstrainedType)]`, `#[derive(CompileUnit)]`,
//! and the `#[uor_grounded]` attribute. The v0.2.1 macros wire downstream code
//! into the foundation's sealed-constructor minting path so the consumer-facing
//! one-liners in `uor_foundation::enforcement::prelude` work end-to-end at
//! compile time.
//!
//! # Usage
//!
//! ```rust,ignore
//! use uor_foundation::uor;
//!
//! // Type declaration (named-argument constraint syntax)
//! let pixel = uor! { type Pixel { ResidueConstraint(modulus: 256, residue: 255); } };
//!
//! // Term expression
//! let sum = uor! { add(mul(3, 5), 7) };
//!
//! // Assertion (ground — checked at compile time)
//! uor! { assert add(1, 2) = 3; };
//! ```

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    missing_docs,
    clippy::missing_errors_doc
)]

mod address;
mod codegen;
mod conformance_parser;
mod derives;
mod generated;
mod lexer;
mod parser;
mod surface;

use proc_macro::TokenStream;

/// The UOR term language DSL macro.
///
/// Parses EBNF surface syntax at compile time and produces typed `Term` ASTs
/// in the `uor_foundation::enforcement` module. The macro handles:
///
/// - **Term expressions**: `add(mul(3, 5), 7)` — operation applications
/// - **Type declarations**: `type Pixel { ResidueConstraint(modulus: 256, residue: 255); }` — constrained types
/// - **Bindings**: `let x : Pixel = add(0, 0);` — named term bindings
/// - **Assertions**: `assert lhs = rhs;` — ground assertions checked at compile time
/// - **Effect declarations**: `effect Name { target: {0,1}; delta: 0; commutes: true; }` — generic props
/// - **Boundary declarations**: `source name : Type via grounding;`
/// - **Quantum literals**: `42@Q7` — level-annotated integers
/// - **Lift/Project**: `lift(x, Q3)`, `project(y, Q0)` — level transitions
/// - **Match**: `match x { pred => expr; otherwise => expr; }`
/// - **Recursion**: `recurse f(n) measure n base is_zero => 1 step => mul(n, f(pred(n)))`
/// - **Streams**: `unfold nat : Successor from 0`
///
/// # Examples
///
/// ```rust,ignore
/// use uor_foundation::uor;
///
/// // Term expressions produce a TermArena with the expression tree.
/// let sum = uor! { add(mul(3, 5), 7) };
///
/// // Quantum-annotated literals tag a value at a specific ring width.
/// let wide = uor! { 144115188075855617@Q7 };
///
/// // Type declarations define constrained types.
/// let pixel = uor! {
///     type Pixel {
///         ResidueConstraint(modulus: 256, residue: 255);
///         HammingConstraint(hammingBound: 8);
///         DepthConstraint(minDepth: 0, maxDepth: 1);
///     }
/// };
///
/// // Bindings carry surface syntax and content addresses.
/// let origin = uor! { let origin : Pixel = add(0, 0); };
///
/// // Ground assertions are checked at COMPILE TIME.
/// uor! { assert add(1, 2) = 3; };
/// uor! { assert mul(3, 5) = 15; };
///
/// // Effect declarations register fiber-targeted effects.
/// let blit = uor! {
///     effect Blit {
///         target: {0, 1, 2, 3};
///         delta: 0;
///         commutes: true;
///     }
/// };
///
/// // Boundary declarations define data sources and sinks.
/// uor! { source pixel_in  : Pixel via sRGB; };
/// uor! { sink   pixel_out : Pixel via DisplayP3; };
///
/// // Lift/Project handle level transitions explicitly.
/// let widened  = uor! { lift(x, Q3) };
/// let narrowed = uor! { project(y, Q0) };
///
/// // Match expressions with pattern arms and a required otherwise arm.
/// let clamped = uor! {
///     match x {
///         is_negative => 0;
///         exceeds_max => 255;
///         otherwise => x;
///     }
/// };
///
/// // Bounded recursion with a descent measure and base case.
/// let factorial = uor! {
///     recurse fact(n)
///         measure n
///         base is_zero => 1
///         step => mul(n, fact(pred(n)))
/// };
///
/// // Stream construction via unfold (coinductive).
/// let naturals = uor! { unfold nat : Successor from 0 };
/// ```
///
/// # Errors
///
/// Parse errors produce `compile_error!()` at the macro call site.
/// The error message includes the unexpected token:
///
/// ```text
/// uor! { add(1, ) }
/// // error: uor! parse error: Unexpected token in expression: RParen
/// ```
///
/// Ground assertions that fail ring evaluation also produce compile errors:
///
/// ```text
/// uor! { assert add(1, 2) = 4; }
/// // error: assertion failed at compile time
/// ```
#[proc_macro]
pub fn uor(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    match parser::parse(&input_str) {
        Ok(parsed) => codegen::emit(&parsed),
        Err(err) => {
            let msg = format!("uor! parse error: {err}");
            quote::quote! { compile_error!(#msg); }.into()
        }
    }
}

/// The v0.2.1 ground-state DSL macro.
///
/// Parses a `conformance-program` production of `uor.conformance.ebnf` whose
/// `compile_unit` body contains a term-language program. Lowers through the
/// reduction pipeline at build time and expands to a `Grounded<T>` value via
/// the foundation crate's back-door minting API.
///
/// In v0.2.1 the in-process pipeline driver is a stub: it parses the
/// conformance grammar, validates the keyword set, and emits a back-door
/// `Grounded<T>` constructor call for inputs in the smoke-test corpus.
/// Non-corpus inputs produce a `compile_error!` citing the
/// `reduction:ConvergenceStall` IRI for downstream tooling to surface.
///
/// # Examples
///
/// ```rust,ignore
/// use uor_foundation::enforcement::prelude::*;
///
/// #[derive(ConstrainedType)]
/// #[uor(residue = 65535, hamming = 16)]
/// struct MatVec<const M: usize, const K: usize>;
///
/// let unit: Grounded<MatVec<64, 2048>> = uor_ground! {
///     compile_unit matvec_q32 {
///         root_term: { /* ... */ };
///         witt_level_ceiling: W32;
///         thermodynamic_budget: 2048.0;
///         target_domains: { ComposedAlgebraic, ArithmeticValuation };
///     }
/// };
/// ```
#[proc_macro]
pub fn uor_ground(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    match conformance_parser::parse(&input_str) {
        Ok(decl) => conformance_parser::emit_grounded(&decl),
        Err(err) => {
            let msg = format!(
                "uor_ground! parse error: {err}\n\
                 = reason: https://uor.foundation/reduction/ConvergenceStall\n\
                 = run `cargo uor explain reduction:ConvergenceStall` for ontology context"
            );
            quote::quote! { compile_error!(#msg); }.into()
        }
    }
}

/// Derive `ConstrainedType` for a struct.
///
/// Generates a `GroundedShape` impl so the struct can appear as the type
/// parameter of `Grounded<T>`. v0.2.1 emits the impl unconditionally; the
/// `#[uor(residue = …, hamming = …, …)]` attributes are recorded as constants
/// for downstream introspection.
///
/// # Examples
///
/// ```rust,ignore
/// use uor_foundation_macros::ConstrainedType;
///
/// #[derive(ConstrainedType)]
/// #[uor(residue = 255, hamming = 8)]
/// struct Pixel(u8);
/// ```
#[proc_macro_derive(ConstrainedType, attributes(uor))]
pub fn derive_constrained_type(input: TokenStream) -> TokenStream {
    derives::derive_constrained_type(input)
}

/// Derive `CompileUnit` for a struct whose fields name the v0.2.1 builder
/// inputs (`builder_root_term`, `builder_witt_level_ceiling`,
/// `builder_thermodynamic_budget`, `builder_target_domains`).
///
/// Generates a `build_compile_unit(&self) -> CompileUnit` method that calls
/// the v0.2.0 `CompileUnitBuilder` chain.
#[proc_macro_derive(CompileUnit, attributes(uor))]
pub fn derive_compile_unit(input: TokenStream) -> TokenStream {
    derives::derive_compile_unit(input)
}

/// Attribute marker asserting that the function body lowers cleanly at the
/// named Witt level.
///
/// v0.2.1 parses `level = "W8" | "W16" | "W24" | "W32"` and emits a
/// `const _: ::uor_foundation::WittLevel = ::uor_foundation::WittLevel::WN;`
/// static assertion after the function. Any undefined level name fails
/// const evaluation at type-check time.
///
/// # Examples
///
/// ```rust,ignore
/// #[uor_foundation_macros::uor_grounded(level = "W32")]
/// fn matvec() -> Grounded<MatVec<64, 2048>> { /* ... */ }
/// ```
#[proc_macro_attribute]
pub fn uor_grounded(attr: TokenStream, item: TokenStream) -> TokenStream {
    derives::attr_uor_grounded(attr, item)
}
