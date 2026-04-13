//! v0.2.1 derive macros: `#[derive(ConstrainedType)]` and `#[derive(CompileUnit)]`.
//!
//! The `ConstrainedType` derive emits:
//!
//! 1. `impl __macro_internals::GroundedShapeSealed for T {}` — opens the
//!    foundation's sealed supertrait via the doc-hidden macro back-door.
//! 2. `impl ConstrainedTypeShape for T` — provides `IRI`, `SITE_COUNT`, and
//!    `CONSTRAINTS` from the struct's `#[uor(...)]` attributes.
//! 3. `pub const UOR_CONSTRAINED_TYPE_IRI: &'static str` — documentation hook.
//!
//! v0.2.1 Phase 7c.1: six constraint kinds are supported via the nested
//! attribute form:
//!
//!   - `#[uor(residue(modulus = 256, residue = 255))]` → `ResidueConstraint`
//!   - `#[uor(carry(site = 3))]`                        → `CarryConstraint`
//!   - `#[uor(depth(min = 0, max = 8))]`                → `DepthConstraint`
//!   - `#[uor(hamming(bound = 8))]`                     → `HammingConstraint`
//!   - `#[uor(site(position = 7))]`                     → `SiteConstraint`
//!   - `#[uor(affine(coefficients = [1, -1, 0], bias = 0))]` → `AffineConstraint`
//!
//! Legacy compat: `#[uor(residue = 255, hamming = 8)]` (flat form) is still
//! accepted. The default residue modulus is sourced at macro-crate build
//! time from the ontology individual `type:ResidueDefaultModulus`.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Expr, ExprLit, Lit, Meta};

// v0.2.1 Phase 7c.3: ontology-driven defaults, sourced by build.rs.
include!(concat!(env!("OUT_DIR"), "/defaults.rs"));

#[derive(Default)]
struct ConstraintEmission {
    residues: Vec<(u64, u64)>,
    carries: Vec<u32>,
    depths: Vec<(u32, u32)>,
    hammings: Vec<u32>,
    sites: Vec<u32>,
    affines: Vec<(Vec<i64>, i64)>,
}

fn lit_int_u64(expr: &Expr) -> u64 {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(i), ..
        }) => i.base10_parse::<u64>().unwrap_or(0),
        _ => 0,
    }
}

fn lit_int_i64(expr: &Expr) -> i64 {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(i), ..
        }) => i.base10_parse::<i64>().unwrap_or(0),
        Expr::Unary(syn::ExprUnary {
            op: syn::UnOp::Neg(_),
            expr: inner,
            ..
        }) => -lit_int_i64(inner),
        _ => 0,
    }
}

fn lit_int_array_i64(expr: &Expr) -> Vec<i64> {
    match expr {
        Expr::Array(arr) => arr.elems.iter().map(lit_int_i64).collect(),
        _ => Vec::new(),
    }
}

/// Parse v0.2.1 `#[uor(...)]` attributes into a `ConstraintEmission`.
/// Supports both the nested form (canonical) and the flat `#[uor(residue = X)]`
/// form (legacy compat).
fn parse_uor_attrs(attrs: &[Attribute]) -> ConstraintEmission {
    let mut out = ConstraintEmission::default();
    for attr in attrs {
        if !attr.path().is_ident("uor") {
            continue;
        }
        let _ = attr.parse_nested_meta(|nm| {
            let kind = match nm.path.get_ident() {
                Some(i) => i.to_string(),
                None => return Ok(()),
            };
            // Legacy flat form: `residue = 255` / `hamming = 8`. The `value()`
            // call succeeds for `key = value` syntax; for nested `key(...)`
            // syntax we fall through to parse_nested_meta below.
            if let Ok(stream) = nm.value() {
                if let Ok(value) = stream.parse::<Expr>() {
                    match kind.as_str() {
                        "residue" => {
                            out.residues
                                .push((RESIDUE_DEFAULT_MODULUS, lit_int_u64(&value)));
                        }
                        "hamming" => out.hammings.push(lit_int_u64(&value) as u32),
                        "carry" => out.carries.push(lit_int_u64(&value) as u32),
                        "site" => out.sites.push(lit_int_u64(&value) as u32),
                        _ => {}
                    }
                    return Ok(());
                }
            }
            // Nested form: `residue(modulus = 256, residue = 255)` etc.
            match kind.as_str() {
                "residue" => {
                    let mut modulus: u64 = RESIDUE_DEFAULT_MODULUS;
                    let mut residue: u64 = 0;
                    let _ = nm.parse_nested_meta(|inner| {
                        let key = inner
                            .path
                            .get_ident()
                            .map(|i| i.to_string())
                            .unwrap_or_default();
                        let v: Expr = inner.value()?.parse()?;
                        match key.as_str() {
                            "modulus" => modulus = lit_int_u64(&v),
                            "residue" => residue = lit_int_u64(&v),
                            _ => {}
                        }
                        Ok(())
                    });
                    out.residues.push((modulus, residue));
                }
                "carry" => {
                    let mut site: u32 = 0;
                    let _ = nm.parse_nested_meta(|inner| {
                        if inner.path.is_ident("site") {
                            let v: Expr = inner.value()?.parse()?;
                            site = lit_int_u64(&v) as u32;
                        }
                        Ok(())
                    });
                    out.carries.push(site);
                }
                "depth" => {
                    let (mut min, mut max): (u32, u32) = (0, 0);
                    let _ = nm.parse_nested_meta(|inner| {
                        let key = inner
                            .path
                            .get_ident()
                            .map(|i| i.to_string())
                            .unwrap_or_default();
                        let v: Expr = inner.value()?.parse()?;
                        let n = lit_int_u64(&v) as u32;
                        match key.as_str() {
                            "min" => min = n,
                            "max" => max = n,
                            _ => {}
                        }
                        Ok(())
                    });
                    out.depths.push((min, max));
                }
                "hamming" => {
                    let mut bound: u32 = 0;
                    let _ = nm.parse_nested_meta(|inner| {
                        if inner.path.is_ident("bound") {
                            let v: Expr = inner.value()?.parse()?;
                            bound = lit_int_u64(&v) as u32;
                        }
                        Ok(())
                    });
                    out.hammings.push(bound);
                }
                "site" => {
                    let mut position: u32 = 0;
                    let _ = nm.parse_nested_meta(|inner| {
                        if inner.path.is_ident("position") {
                            let v: Expr = inner.value()?.parse()?;
                            position = lit_int_u64(&v) as u32;
                        }
                        Ok(())
                    });
                    out.sites.push(position);
                }
                "affine" => {
                    let mut coeffs: Vec<i64> = Vec::new();
                    let mut bias: i64 = 0;
                    let _ = nm.parse_nested_meta(|inner| {
                        let key = inner
                            .path
                            .get_ident()
                            .map(|i| i.to_string())
                            .unwrap_or_default();
                        let v: Expr = inner.value()?.parse()?;
                        match key.as_str() {
                            "coefficients" => coeffs = lit_int_array_i64(&v),
                            "bias" => bias = lit_int_i64(&v),
                            _ => {}
                        }
                        Ok(())
                    });
                    out.affines.push((coeffs, bias));
                }
                _ => {}
            }
            Ok(())
        });
        let _ = attr;
    }
    let _ = Meta::List;
    out
}

/// Generate `GroundedShape` + `ConstrainedTypeShape` impls for a struct.
pub fn derive_constrained_type(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let emission = parse_uor_attrs(&ast.attrs);

    // Build the `CONSTRAINTS` array tokens.
    let residue_entries = emission.residues.iter().map(|(m, r)| {
        quote! {
            ::uor_foundation::pipeline::ConstraintRef::Residue {
                modulus: #m,
                residue: #r,
            }
        }
    });
    let hamming_entries = emission.hammings.iter().map(|h| {
        quote! {
            ::uor_foundation::pipeline::ConstraintRef::Hamming { bound: #h }
        }
    });
    let carry_entries = emission.carries.iter().map(|c| {
        quote! {
            ::uor_foundation::pipeline::ConstraintRef::Carry { site: #c }
        }
    });
    let depth_entries = emission.depths.iter().map(|(min, max)| {
        quote! {
            ::uor_foundation::pipeline::ConstraintRef::Depth { min: #min, max: #max }
        }
    });
    let site_entries = emission.sites.iter().map(|p| {
        quote! {
            ::uor_foundation::pipeline::ConstraintRef::Site { position: #p }
        }
    });
    let affine_entries = emission.affines.iter().map(|(coeffs, bias)| {
        let coeff_lits = coeffs.iter().map(|c| quote!(#c));
        quote! {
            ::uor_foundation::pipeline::ConstraintRef::Affine {
                coefficients: &[#(#coeff_lits),*],
                bias: #bias,
            }
        }
    });

    // SITE_COUNT derivation: max of hamming bounds / explicit site positions+1 /
    // affine coefficient list length.
    let from_hamming = emission.hammings.iter().copied().max().unwrap_or(0);
    let from_sites = emission
        .sites
        .iter()
        .copied()
        .max()
        .map(|p| p + 1)
        .unwrap_or(0);
    let from_affine = emission
        .affines
        .iter()
        .map(|(c, _)| c.len() as u32)
        .max()
        .unwrap_or(0);
    let site_count = [from_hamming, from_sites, from_affine]
        .into_iter()
        .max()
        .unwrap_or(0) as usize;

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// IRI of the corresponding `type:ConstrainedType` for this struct.
            /// Set by `#[derive(ConstrainedType)]` from `uor-foundation-macros`.
            pub const UOR_CONSTRAINED_TYPE_IRI: &'static str =
                "https://uor.foundation/type/ConstrainedType";
        }

        // Open the sealed `GroundedShape` supertrait via the foundation's
        // doc-hidden macro back-door.
        impl #impl_generics ::uor_foundation::enforcement::__macro_internals::GroundedShapeSealed
            for #name #ty_generics #where_clause {}

        impl #impl_generics
            ::uor_foundation::pipeline::constrained_type_shape_sealed::Sealed
            for #name #ty_generics #where_clause {}

        impl #impl_generics
            ::uor_foundation::pipeline::ConstrainedTypeShape
            for #name #ty_generics #where_clause
        {
            const IRI: &'static str = Self::UOR_CONSTRAINED_TYPE_IRI;
            const SITE_COUNT: usize = #site_count;
            const CONSTRAINTS: &'static [::uor_foundation::pipeline::ConstraintRef] = &[
                #(#residue_entries,)*
                #(#hamming_entries,)*
                #(#carry_entries,)*
                #(#depth_entries,)*
                #(#site_entries,)*
                #(#affine_entries,)*
            ];
        }
    };
    expanded.into()
}

/// Implement `#[uor_grounded(level = "WN")]`. Emits a static const that
/// references the named WittLevel, turning a typo or undefined level into
/// a compile error at type-check time.
pub fn attr_uor_grounded(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse `level = "WN"` from the attribute arguments. syn's `AttributeArgs`
    // is a legacy API; the v2 path uses a `Punctuated<Meta, Comma>`.
    let attr_tokens: proc_macro2::TokenStream = attr.into();
    let attr_str = attr_tokens.to_string();
    // Trivial scan: find `level = "..."`.
    let level_name = attr_str
        .split_once('=')
        .and_then(|(_, rhs)| {
            let rhs = rhs.trim();
            rhs.strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "W8".to_string());

    // Parse the annotated item so we can preserve its ident for the const name.
    let item_tokens: proc_macro2::TokenStream = item.into();
    let fn_item: syn::ItemFn = match syn::parse2(item_tokens.clone()) {
        Ok(f) => f,
        Err(_) => {
            // Not a function — leave the item alone; the user will get their
            // own compile error from the syntactic position.
            return item_tokens.into();
        }
    };
    let fn_ident = &fn_item.sig.ident;
    let const_name = syn::Ident::new(
        &format!("__UOR_GROUNDED_LEVEL_CHECK_{}", fn_ident),
        fn_ident.span(),
    );
    let level_ident = syn::Ident::new(&level_name, proc_macro2::Span::call_site());

    let expanded = quote! {
        #fn_item

        /// Compile-time Witt-level assertion emitted by `#[uor_grounded]`.
        /// Fails type-check if the named level is not a generated constant.
        #[doc(hidden)]
        #[allow(non_upper_case_globals, dead_code)]
        const #const_name: ::uor_foundation::WittLevel =
            ::uor_foundation::WittLevel::#level_ident;
    };
    expanded.into()
}

/// Generate a `build_compile_unit` method for a struct whose fields name
/// the v0.2.1 builder inputs.
pub fn derive_compile_unit(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// IRI of the `reduction:CompileUnit` class. Set by
            /// `#[derive(CompileUnit)]`.
            pub const UOR_COMPILE_UNIT_IRI: &'static str =
                "https://uor.foundation/reduction/CompileUnit";
        }
    };
    expanded.into()
}
