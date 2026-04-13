//! v0.2.1 parser and code-emitter for `uor.conformance.ebnf` declarations.
//!
//! Recognises the seven productions (compile-unit, dispatch-rule, witt-level,
//! predicate, parallel, stream, lease), validates the brace structure, parses
//! a trailing `as Grounded<T>` clause when present to recover the type
//! parameter, and dispatches to a per-form emitter that produces the correct
//! Rust output for each declaration.
//!
//! v0.2.1 Phase 7c.2 + 8e:
//!
//! - `KNOWN_KEYWORDS` and `SHAPE_REQUIRED_KEYS` are generated from the
//!   ontology by `uor-crate` and checked in under `src/generated/`.
//!   Regenerate with `cargo run --bin uor-crate`.
//! - `compile_unit` expands to a real `Grounded<T>` via the foundation's
//!   back-door minting API.
//! - The other six forms (`dispatch_rule`, `witt_level`, `predicate`,
//!   `parallel`, `stream`, `lease`) expand to `pub const` slots or zero-sized
//!   marker structs carrying the declared IRIs / parameters.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Ident, Token, Type};

// v0.2.1 Phase 8e: ontology-derived keyword + shape-requirement tables.
// Written by `uor-crate` into `src/generated/`.
use crate::generated::keywords::KNOWN_KEYWORDS;
use crate::generated::shape_requirements::SHAPE_REQUIRED_KEYS;

/// A parsed conformance declaration. Carries the keyword, the identifier
/// introduced by the declaration, the raw body text between braces, and the
/// optional `as Grounded<T>` type ascription.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ConformanceDecl {
    /// The leading keyword (e.g., `compile_unit`, `dispatch_rule`).
    pub keyword: String,
    /// The fresh identifier introduced by the declaration.
    pub identifier: String,
    /// Raw body text between the outermost braces, trimmed.
    pub body: String,
    /// The `T` in `as Grounded<T>`. Required for `compile_unit`; ignored
    /// by the other six declaration forms.
    pub grounded_type: Option<Type>,
}

/// A single `key: value;` entry parsed from a declaration body.
#[derive(Debug, Clone)]
struct BodyEntry {
    key: String,
    value: String,
}

/// Parse a declaration body (already brace-stripped) into (key, value) pairs.
/// Nested `{ ... }` values are handled by counting brace depth.
fn parse_body(body: &str) -> Vec<BodyEntry> {
    let mut out = Vec::new();
    let bytes = body.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let key_start = i;
        while i < bytes.len() && bytes[i] != b':' && bytes[i] != b';' {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] == b';' {
            break;
        }
        let key = body[key_start..i].trim().to_string();
        i += 1; // skip ':'
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let value_start = i;
        let mut depth = 0i32;
        while i < bytes.len() {
            match bytes[i] {
                b'{' => depth += 1,
                b'}' => depth -= 1,
                b';' if depth == 0 => break,
                _ => {}
            }
            i += 1;
        }
        let value = body[value_start..i].trim().to_string();
        if !key.is_empty() {
            out.push(BodyEntry { key, value });
        }
        if i < bytes.len() {
            i += 1; // skip ';'
        }
    }
    out
}

/// Look up a body entry by key.
fn body_lookup<'a>(entries: &'a [BodyEntry], key: &str) -> Option<&'a str> {
    entries
        .iter()
        .find(|e| e.key == key)
        .map(|e| e.value.as_str())
}

/// Validate that every required body key for the given declaration keyword
/// is present. Returns a `compile_error!` TokenStream on missing keys.
fn validate_required_keys(keyword: &str, entries: &[BodyEntry]) -> Option<TokenStream> {
    let required = SHAPE_REQUIRED_KEYS
        .iter()
        .find(|(k, _)| *k == keyword)
        .map(|(_, reqs)| *reqs)?;
    for req in required {
        if !entries.iter().any(|e| e.key == *req) {
            let msg = format!("uor_ground! {keyword} declaration missing required key `{req}`");
            return Some(quote! { compile_error!(#msg); }.into());
        }
    }
    None
}

/// `syn`-based parser for the macro input shape:
///
/// ```text
/// <keyword> <ident> { ... } [as Grounded<T>]
/// ```
struct MacroInput {
    keyword: Ident,
    ident: Ident,
    brace_body: proc_macro2::TokenStream,
    grounded_type: Option<Type>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keyword: Ident = input.parse()?;
        let ident: Ident = input.parse()?;
        let brace_content;
        let _ = braced!(brace_content in input);
        let body_tokens: proc_macro2::TokenStream = brace_content.parse()?;
        let grounded_type = if input.peek(Token![as]) {
            let _: Token![as] = input.parse()?;
            let ty: Type = input.parse()?;
            Some(ty)
        } else {
            None
        };
        Ok(MacroInput {
            keyword,
            ident,
            brace_body: body_tokens,
            grounded_type,
        })
    }
}

/// Parse the macro input string into a `ConformanceDecl`.
///
/// # Errors
///
/// Returns `Err` when the input does not begin with a known declaration
/// keyword, lacks an identifier after the keyword, or has unbalanced braces.
pub fn parse(input: &str) -> Result<ConformanceDecl, String> {
    let tokens: proc_macro2::TokenStream =
        input.parse().map_err(|e| format!("lexer error: {e}"))?;
    let macro_input: MacroInput = syn::parse2(tokens).map_err(|e| format!("parse error: {e}"))?;

    let keyword = macro_input.keyword.to_string();
    if !KNOWN_KEYWORDS.contains(&keyword.as_str()) {
        return Err(format!(
            "unknown declaration keyword `{keyword}`; expected one of {KNOWN_KEYWORDS:?}"
        ));
    }

    Ok(ConformanceDecl {
        keyword,
        identifier: macro_input.ident.to_string(),
        body: macro_input.brace_body.to_string(),
        grounded_type: macro_input.grounded_type,
    })
}

/// Emit the Rust output for a parsed declaration. Dispatches on the keyword.
#[must_use]
pub fn emit_grounded(decl: &ConformanceDecl) -> TokenStream {
    match decl.keyword.as_str() {
        "compile_unit" => emit_compile_unit(decl),
        "dispatch_rule" => emit_dispatch_rule(decl),
        "witt_level" => emit_witt_level(decl),
        "predicate" => emit_predicate(decl),
        "parallel" => emit_parallel(decl),
        "stream" => emit_stream(decl),
        "lease" => emit_lease(decl),
        other => {
            let msg = format!("unknown uor_ground! keyword `{other}`");
            quote! { compile_error!(#msg); }.into()
        }
    }
}

// ---------- compile_unit ----------

/// Emit a real `Grounded<T>` construction via the foundation's back-door
/// minting API.
fn emit_compile_unit(decl: &ConformanceDecl) -> TokenStream {
    let entries = parse_body(&decl.body);
    if let Some(err) = validate_required_keys(&decl.keyword, &entries) {
        return err;
    }
    let grounded_type = match &decl.grounded_type {
        Some(t) => t,
        None => {
            let msg = "uor_ground! compile_unit requires a trailing \
                       `as Grounded<T>` type ascription so the macro can \
                       recover the type parameter at expansion time.";
            return quote! { compile_error!(#msg); }.into();
        }
    };
    let inner_type: Type =
        extract_grounded_inner(grounded_type).unwrap_or_else(|| grounded_type.clone());

    // Parse witt_level_ceiling if provided; fall back to the canonical
    // default W32 (matching `Certify::DEFAULT_LEVEL`) so the Witt level
    // propagates through the pipeline per Phase 7b.1.
    let witt_bits: u16 = body_lookup(&entries, "witt_level_ceiling")
        .and_then(parse_witt_level_name_to_bits)
        .unwrap_or(32);

    let expanded = quote! {
        {
            let __uor_input = <#inner_type as ::core::default::Default>::default();
            match ::uor_foundation::pipeline::run_pipeline::<#inner_type>(
                &__uor_input,
                #witt_bits,
            ) {
                Ok(g) => g,
                Err(_) => {
                    ::core::panic!(
                        "uor_ground! pipeline failure: reduction:ConvergenceStall"
                    )
                }
            }
        }
    };
    expanded.into()
}

/// Convert a bare `W8` / `W16` / `W24` / `W32` identifier in the body value
/// to its bit-width.
fn parse_witt_level_name_to_bits(value: &str) -> Option<u16> {
    let v = value.trim();
    if let Some(rest) = v.strip_prefix('W') {
        return rest.parse::<u16>().ok();
    }
    None
}

// ---------- dispatch_rule ----------

fn emit_dispatch_rule(decl: &ConformanceDecl) -> TokenStream {
    let entries = parse_body(&decl.body);
    if let Some(err) = validate_required_keys(&decl.keyword, &entries) {
        return err;
    }
    let predicate_iri = body_lookup(&entries, "predicate")
        .unwrap_or("https://uor.foundation/predicate/always")
        .trim_matches('"')
        .to_string();
    let target_raw = body_lookup(&entries, "target_resolver")
        .unwrap_or("ResidualVerdictResolver")
        .to_string();
    let target_iri = if target_raw.starts_with('"') {
        target_raw.trim_matches('"').to_string()
    } else {
        format!("https://uor.foundation/resolver/{target_raw}")
    };
    let priority: u32 = body_lookup(&entries, "priority")
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    let ident = syn::Ident::new(&decl.identifier, proc_macro2::Span::call_site());
    // Emit a zero-sized marker type carrying the IRIs as associated consts.
    // The full `DispatchRule` struct is an internal pipeline type — exposing
    // it would leak sealed state, so the macro expands into a user-visible
    // marker type with the same payload.
    let expanded = quote! {
        #[doc = "Generated dispatch rule marker. Carries the predicate IRI, target resolver IRI, and priority as associated constants."]
        #[derive(Debug, Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct #ident;
        impl #ident {
            /// Ontology IRI of the predicate this rule dispatches on.
            pub const PREDICATE_IRI: &'static str = #predicate_iri;
            /// Ontology IRI of the resolver this rule routes to.
            pub const TARGET_RESOLVER_IRI: &'static str = #target_iri;
            /// Priority of this rule within its dispatch table (lower = earlier).
            pub const PRIORITY: u32 = #priority;
        }
    };
    expanded.into()
}

// ---------- witt_level ----------

fn emit_witt_level(decl: &ConformanceDecl) -> TokenStream {
    let entries = parse_body(&decl.body);
    if let Some(err) = validate_required_keys(&decl.keyword, &entries) {
        return err;
    }
    let bit_width: u32 = body_lookup(&entries, "bit_width")
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(8);
    if bit_width == 0 || bit_width % 8 != 0 {
        let msg = format!("witt_level bit_width must be 8·(k+1); got {bit_width}");
        return quote! { compile_error!(#msg); }.into();
    }
    let ident = syn::Ident::new(&decl.identifier, proc_macro2::Span::call_site());
    let expanded = quote! {
        pub const #ident: ::uor_foundation::WittLevel =
            ::uor_foundation::WittLevel::new(#bit_width);
    };
    expanded.into()
}

// ---------- predicate ----------

fn emit_predicate(decl: &ConformanceDecl) -> TokenStream {
    let entries = parse_body(&decl.body);
    if let Some(err) = validate_required_keys(&decl.keyword, &entries) {
        return err;
    }
    let termination_witness = body_lookup(&entries, "termination_witness")
        .unwrap_or("https://uor.foundation/proof/AxiomaticDerivation")
        .trim_matches('"')
        .to_string();
    let ident = syn::Ident::new(&decl.identifier, proc_macro2::Span::call_site());
    let expanded = quote! {
        #[doc = "Generated predicate marker. Carries the termination-witness IRI as an associated constant."]
        #[derive(Debug, Default, Clone, Copy)]
        pub struct #ident;
        impl #ident {
            /// Ontology IRI of the termination witness proving this predicate halts.
            pub const TERMINATION_WITNESS_IRI: &'static str = #termination_witness;
        }
    };
    expanded.into()
}

// ---------- parallel ----------

fn emit_parallel(decl: &ConformanceDecl) -> TokenStream {
    let entries = parse_body(&decl.body);
    if let Some(err) = validate_required_keys(&decl.keyword, &entries) {
        return err;
    }
    let site_partition = body_lookup(&entries, "site_partition")
        .unwrap_or("https://uor.foundation/partition/TrivialPartition")
        .trim_matches('"')
        .to_string();
    let disjointness = body_lookup(&entries, "disjointness_witness")
        .unwrap_or("")
        .trim_matches('"')
        .to_string();
    let ident = syn::Ident::new(&decl.identifier, proc_macro2::Span::call_site());
    let expanded = quote! {
        #[doc = "Generated parallel marker. Carries the site-partition and disjointness-witness IRIs as associated constants."]
        #[derive(Debug, Default, Clone, Copy)]
        pub struct #ident;
        impl #ident {
            /// Ontology IRI of the site partition this parallel block operates over.
            pub const SITE_PARTITION_IRI: &'static str = #site_partition;
            /// Ontology IRI of the disjointness witness proving the partition is non-overlapping.
            pub const DISJOINTNESS_WITNESS_IRI: &'static str = #disjointness;
        }
    };
    expanded.into()
}

// ---------- stream ----------

fn emit_stream(decl: &ConformanceDecl) -> TokenStream {
    let entries = parse_body(&decl.body);
    if let Some(err) = validate_required_keys(&decl.keyword, &entries) {
        return err;
    }
    let productivity = body_lookup(&entries, "productivity_witness")
        .unwrap_or("")
        .trim_matches('"')
        .to_string();
    let seed = body_lookup(&entries, "unfold_seed")
        .unwrap_or("0")
        .to_string();
    let step = body_lookup(&entries, "step").unwrap_or("").to_string();
    let ident = syn::Ident::new(&decl.identifier, proc_macro2::Span::call_site());
    let expanded = quote! {
        #[doc = "Generated stream marker. Carries the unfold seed, step term source, and productivity-witness IRI as associated constants."]
        #[derive(Debug, Default, Clone, Copy)]
        pub struct #ident;
        impl #ident {
            /// Source text of the stream's unfold seed expression.
            pub const UNFOLD_SEED_SRC: &'static str = #seed;
            /// Source text of the stream's step term expression.
            pub const STEP_SRC: &'static str = #step;
            /// Ontology IRI of the productivity witness proving the stream is well-defined.
            pub const PRODUCTIVITY_WITNESS_IRI: &'static str = #productivity;
        }
    };
    expanded.into()
}

// ---------- lease ----------

fn emit_lease(decl: &ConformanceDecl) -> TokenStream {
    let entries = parse_body(&decl.body);
    if let Some(err) = validate_required_keys(&decl.keyword, &entries) {
        return err;
    }
    let linear_site: u32 = body_lookup(&entries, "linear_site")
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    let lease_scope = body_lookup(&entries, "lease_scope")
        .unwrap_or("")
        .trim_matches('"')
        .to_string();
    let ident = syn::Ident::new(&decl.identifier, proc_macro2::Span::call_site());
    let expanded = quote! {
        #[doc = "Generated lease marker. Carries the linear-site index and lease-scope IRI as associated constants."]
        #[derive(Debug, Default, Clone, Copy)]
        pub struct #ident;
        impl #ident {
            /// Index of the linear site this lease guards.
            pub const LINEAR_SITE: u32 = #linear_site;
            /// Ontology IRI of the lease's scope definition.
            pub const LEASE_SCOPE: &'static str = #lease_scope;
        }
    };
    expanded.into()
}

/// Extract `T` from `Grounded<T>`. Returns `None` if the input isn't a
/// path type of the form `X::Grounded<T>` or `Grounded<T>`.
fn extract_grounded_inner(ty: &Type) -> Option<Type> {
    use syn::{GenericArgument, PathArguments};
    if let Type::Path(tp) = ty {
        let last = tp.path.segments.last()?;
        if last.ident != "Grounded" {
            return None;
        }
        if let PathArguments::AngleBracketed(args) = &last.arguments {
            for arg in &args.args {
                if let GenericArgument::Type(inner) = arg {
                    return Some(inner.clone());
                }
            }
        }
    }
    None
}
