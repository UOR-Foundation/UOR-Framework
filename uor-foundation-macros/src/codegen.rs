//! Converts `ParsedNode` into Rust token streams.
//!
//! For each `uor!` invocation, emits code that constructs typed AST values
//! from the `uor_foundation::enforcement` module. Term expressions are
//! allocated bottom-up into a `TermArena` with correct indices.

use proc_macro::TokenStream;
use quote::quote;

use crate::address;
use crate::parser::ParsedNode;
use crate::surface;

/// Emit Rust code for a parsed UOR node.
pub fn emit(node: &ParsedNode) -> TokenStream {
    match node {
        ParsedNode::Literal { .. }
        | ParsedNode::StringLiteral(_)
        | ParsedNode::Variable(_)
        | ParsedNode::Application { .. }
        | ParsedNode::Lift { .. }
        | ParsedNode::Project { .. } => emit_term_arena(node),
        ParsedNode::TypeDecl {
            name,
            constraints,
            type_params: _,
        } => emit_type_decl(name, constraints),
        ParsedNode::Binding {
            name,
            type_name,
            value,
        } => emit_binding(name, type_name, value),
        ParsedNode::Assertion { lhs, rhs } => emit_assertion(lhs, rhs),
        ParsedNode::EffectDecl { name, props } => emit_effect_decl(name, props),
        ParsedNode::SourceDecl {
            name,
            type_name,
            via,
        } => emit_source_decl(name, type_name, via),
        ParsedNode::SinkDecl {
            name,
            type_name,
            via,
        } => emit_sink_decl(name, type_name, via),
        ParsedNode::Match { .. }
        | ParsedNode::TryExpr { .. }
        | ParsedNode::Recurse { .. }
        | ParsedNode::Unfold { .. } => emit_term_arena(node),
    }
}

// ── Arena-aware term emission ──

/// Count the total number of `Term` nodes in a `ParsedNode` tree.
fn count_nodes(node: &ParsedNode) -> usize {
    match node {
        ParsedNode::Literal { .. } | ParsedNode::StringLiteral(_) | ParsedNode::Variable(_) => 1,
        ParsedNode::Application { args, .. } => 1 + args.iter().map(count_nodes).sum::<usize>(),
        ParsedNode::Lift { operand, .. } | ParsedNode::Project { operand, .. } => {
            1 + count_nodes(operand)
        }
        ParsedNode::Match {
            arms, otherwise, ..
        } => 1 + arms.iter().map(|(_, e)| count_nodes(e)).sum::<usize>() + count_nodes(otherwise),
        ParsedNode::TryExpr { body, recover_arms } => {
            1 + count_nodes(body)
                + recover_arms
                    .iter()
                    .map(|(_, e)| count_nodes(e))
                    .sum::<usize>()
        }
        ParsedNode::Recurse {
            base_arms,
            step_expr,
            ..
        } => {
            1 + base_arms.iter().map(|(_, e)| count_nodes(e)).sum::<usize>()
                + count_nodes(step_expr)
        }
        ParsedNode::Unfold { seed, .. } => 1 + count_nodes(seed),
        _ => 1,
    }
}

/// Emit push calls for a term tree, returning the index of the root node.
/// `stmts` accumulates the `let _idxN = arena.push(...)` statements.
fn emit_push(
    node: &ParsedNode,
    counter: &mut usize,
    stmts: &mut Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    match node {
        ParsedNode::Literal { value, level } => {
            let idx_name =
                proc_macro2::Ident::new(&format!("_idx{counter}"), proc_macro2::Span::call_site());
            *counter += 1;
            let level_expr = match level.as_deref() {
                Some(name) => {
                    let i = parse_quantum_level_name(name);
                    quote! { uor_foundation::WittLevel::new(#i) }
                }
                None => quote! { uor_foundation::WittLevel::W8 },
            };
            stmts.push(quote! {
                let #idx_name = arena.push(uor_foundation::enforcement::Term::Literal {
                    value: #value,
                    level: #level_expr,
                });
            });
            quote! { #idx_name }
        }
        ParsedNode::StringLiteral(_) => {
            // String literals emit as Variable placeholders (host values
            // are resolved at the enforcement layer, not in the term arena)
            let idx_name =
                proc_macro2::Ident::new(&format!("_idx{counter}"), proc_macro2::Span::call_site());
            *counter += 1;
            stmts.push(quote! {
                let #idx_name = arena.push(uor_foundation::enforcement::Term::Variable {
                    name_index: 0,
                });
            });
            quote! { #idx_name }
        }
        ParsedNode::Variable(_name) => {
            let idx_name =
                proc_macro2::Ident::new(&format!("_idx{counter}"), proc_macro2::Span::call_site());
            *counter += 1;
            stmts.push(quote! {
                let #idx_name = arena.push(uor_foundation::enforcement::Term::Variable {
                    name_index: 0,
                });
            });
            quote! { #idx_name }
        }
        ParsedNode::Application { op, args } => {
            // Push args first (bottom-up), recording start index
            let arg_indices: Vec<_> = args.iter().map(|a| emit_push(a, counter, stmts)).collect();
            let arg_count = arg_indices.len() as u32;
            let first_arg = if arg_indices.is_empty() {
                quote! { None }
            } else {
                let first = &arg_indices[0];
                quote! { #first }
            };
            let op_variant = op_to_primitive(op);
            let idx_name =
                proc_macro2::Ident::new(&format!("_idx{counter}"), proc_macro2::Span::call_site());
            *counter += 1;
            stmts.push(quote! {
                let #idx_name = arena.push(uor_foundation::enforcement::Term::Application {
                    operator: uor_foundation::PrimitiveOp::#op_variant,
                    args: uor_foundation::enforcement::TermList {
                        start: #first_arg.unwrap_or(0),
                        len: #arg_count,
                    },
                });
            });
            quote! { #idx_name }
        }
        ParsedNode::Lift { operand, target } => {
            let op_idx = emit_push(operand, counter, stmts);
            let target_index = parse_quantum_level_name(target);
            let idx_name =
                proc_macro2::Ident::new(&format!("_idx{counter}"), proc_macro2::Span::call_site());
            *counter += 1;
            stmts.push(quote! {
                let #idx_name = arena.push(uor_foundation::enforcement::Term::Lift {
                    operand_index: #op_idx.unwrap_or(0),
                    target: uor_foundation::WittLevel::new(#target_index),
                });
            });
            quote! { #idx_name }
        }
        ParsedNode::Project { operand, target } => {
            let op_idx = emit_push(operand, counter, stmts);
            let target_index = parse_quantum_level_name(target);
            let idx_name =
                proc_macro2::Ident::new(&format!("_idx{counter}"), proc_macro2::Span::call_site());
            *counter += 1;
            stmts.push(quote! {
                let #idx_name = arena.push(uor_foundation::enforcement::Term::Project {
                    operand_index: #op_idx.unwrap_or(0),
                    target: uor_foundation::WittLevel::new(#target_index),
                });
            });
            quote! { #idx_name }
        }
        // Match, Recurse, and Unfold nodes emit a Variable placeholder —
        // full arena support requires extending the Term enum with
        // named-reference variants.
        _ => {
            let idx_name =
                proc_macro2::Ident::new(&format!("_idx{counter}"), proc_macro2::Span::call_site());
            *counter += 1;
            stmts.push(quote! {
                let #idx_name = arena.push(uor_foundation::enforcement::Term::Variable {
                    name_index: 0,
                });
            });
            quote! { #idx_name }
        }
    }
}

/// Emit a term expression as a `TermArena` with correct indices.
fn emit_term_arena(node: &ParsedNode) -> TokenStream {
    let cap = count_nodes(node);
    let surface_str = surface::serialize(node);
    let addr = address::content_address(&surface_str);

    let mut stmts = Vec::new();
    let mut counter = 0usize;
    let _root_idx = emit_push(node, &mut counter, &mut stmts);

    let cap_lit = proc_macro2::Literal::usize_unsuffixed(cap);

    quote! {
        {
            /// Surface syntax for this term.
            const _SURFACE: &str = #surface_str;
            /// Content address for this term.
            const _ADDR: u64 = #addr;
            let mut arena = uor_foundation::enforcement::TermArena::<#cap_lit>::new();
            #(#stmts)*
            arena
        }
    }
    .into()
}

// ── Statement-level emission ──

/// Emit a type declaration.
fn emit_type_decl(name: &str, constraints: &[(String, Vec<(String, ParsedNode)>)]) -> TokenStream {
    let constraint_count = constraints.len() as u32;
    let surface_str = surface::serialize(&ParsedNode::TypeDecl {
        name: name.to_string(),
        type_params: Vec::new(),
        constraints: constraints.to_vec(),
    });
    let addr = address::content_address(&surface_str);
    quote! {
        {
            const _SURFACE: &str = #surface_str;
            const _ADDR: u64 = #addr;
            uor_foundation::enforcement::TypeDeclaration {
                name_index: 0,
                constraints: uor_foundation::enforcement::TermList {
                    start: 0,
                    len: #constraint_count,
                },
            }
        }
    }
    .into()
}

/// Emit a binding.
fn emit_binding(name: &str, type_name: &str, value: &ParsedNode) -> TokenStream {
    let surface_str = surface::serialize(&ParsedNode::Binding {
        name: name.to_string(),
        type_name: type_name.to_string(),
        value: Box::new(value.clone()),
    });
    let addr = address::content_address(&surface_str);

    // Emit the value term into an arena to get its root index
    let value_cap = count_nodes(value);
    let mut stmts = Vec::new();
    let mut counter = 0usize;
    let root_idx = emit_push(value, &mut counter, &mut stmts);
    let cap_lit = proc_macro2::Literal::usize_unsuffixed(value_cap);

    quote! {
        {
            let mut arena = uor_foundation::enforcement::TermArena::<#cap_lit>::new();
            #(#stmts)*
            uor_foundation::enforcement::Binding {
                name_index: 0,
                type_index: 0,
                value_index: #root_idx.unwrap_or(0),
                surface: #surface_str,
                content_address: #addr,
            }
        }
    }
    .into()
}

/// Emit an assertion. For ground assertions (no free variables), emits a
/// `const` block that calls `const_ring_eval_q0` to verify equality at
/// compile time.
fn emit_assertion(lhs: &ParsedNode, rhs: &ParsedNode) -> TokenStream {
    let surface_str = surface::serialize(&ParsedNode::Assertion {
        lhs: Box::new(lhs.clone()),
        rhs: Box::new(rhs.clone()),
    });

    if is_ground(lhs) && is_ground(rhs) {
        // Ground assertion: evaluate at compile time
        let lhs_eval = emit_const_eval(lhs);
        let rhs_eval = emit_const_eval(rhs);
        quote! {
            {
                const _SURFACE: &str = #surface_str;
                const _LHS: u8 = #lhs_eval;
                const _RHS: u8 = #rhs_eval;
                const _: () = assert!(_LHS == _RHS);
                uor_foundation::enforcement::Assertion {
                    lhs_index: 0,
                    rhs_index: 0,
                    surface: #surface_str,
                }
            }
        }
        .into()
    } else {
        // Non-ground: produce runtime Assertion struct
        quote! {
            uor_foundation::enforcement::Assertion {
                lhs_index: 0,
                rhs_index: 0,
                surface: #surface_str,
            }
        }
        .into()
    }
}

/// Check if a parsed node contains no free variables (all leaves are literals).
fn is_ground(node: &ParsedNode) -> bool {
    match node {
        ParsedNode::Literal { .. } | ParsedNode::StringLiteral(_) => true,
        ParsedNode::Variable(_) => false,
        ParsedNode::Application { args, .. } => args.iter().all(is_ground),
        ParsedNode::Lift { operand, .. } | ParsedNode::Project { operand, .. } => {
            is_ground(operand)
        }
        _ => false,
    }
}

/// Emit a const expression that evaluates a ground term using Q0 ring arithmetic.
/// Returns a `proc_macro2::TokenStream` computing a `u8`.
fn emit_const_eval(node: &ParsedNode) -> proc_macro2::TokenStream {
    match node {
        ParsedNode::Literal { value, .. } => {
            let v = *value as u8;
            quote! { #v }
        }
        ParsedNode::Application { op, args } if args.len() == 2 => {
            let op_variant = op_to_primitive(op);
            let a = emit_const_eval(&args[0]);
            let b = emit_const_eval(&args[1]);
            quote! {
                uor_foundation::enforcement::const_ring_eval_q0(
                    uor_foundation::PrimitiveOp::#op_variant,
                    #a,
                    #b,
                )
            }
        }
        ParsedNode::Application { op, args } if args.len() == 1 => {
            let op_variant = op_to_primitive(op);
            let a = emit_const_eval(&args[0]);
            quote! {
                uor_foundation::enforcement::const_ring_eval_unary_q0(
                    uor_foundation::PrimitiveOp::#op_variant,
                    #a,
                )
            }
        }
        _ => quote! { 0u8 },
    }
}

/// Emit an effect declaration.
fn emit_effect_decl(name: &str, props: &[(String, ParsedNode)]) -> TokenStream {
    let surface_str = surface::serialize(&ParsedNode::EffectDecl {
        name: name.to_string(),
        props: props.to_vec(),
    });
    let addr = address::content_address(&surface_str);
    quote! {
        {
            const _SURFACE: &str = #surface_str;
            const _ADDR: u64 = #addr;
            _SURFACE
        }
    }
    .into()
}

/// Emit a source declaration.
fn emit_source_decl(name: &str, type_name: &str, via: &str) -> TokenStream {
    let surface_str = surface::serialize(&ParsedNode::SourceDecl {
        name: name.to_string(),
        type_name: type_name.to_string(),
        via: via.to_string(),
    });
    let _addr = address::content_address(&surface_str);
    quote! {
        uor_foundation::enforcement::SourceDeclaration {
            name_index: 0,
            type_index: 0,
            grounding_name_index: 0,
        }
    }
    .into()
}

/// Emit a sink declaration.
fn emit_sink_decl(name: &str, type_name: &str, via: &str) -> TokenStream {
    let surface_str = surface::serialize(&ParsedNode::SinkDecl {
        name: name.to_string(),
        type_name: type_name.to_string(),
        via: via.to_string(),
    });
    let _addr = address::content_address(&surface_str);
    quote! {
        uor_foundation::enforcement::SinkDeclaration {
            name_index: 0,
            type_index: 0,
            projection_name_index: 0,
        }
    }
    .into()
}

/// Map operation name to `PrimitiveOp` variant.
fn op_to_primitive(op: &str) -> proc_macro2::Ident {
    let variant = match op {
        "add" => "Add",
        "sub" => "Sub",
        "mul" => "Mul",
        "xor" => "Xor",
        "and" => "And",
        "or" => "Or",
        "neg" => "Neg",
        "bnot" => "Bnot",
        "succ" => "Succ",
        "pred" => "Pred",
        _ => "Add",
    };
    proc_macro2::Ident::new(variant, proc_macro2::Span::call_site())
}

/// Parse a Witt level name into a witt_length value.
///
/// Accepts `W<n>` (e.g. `W8`, `W32`) returning `n` directly,
/// or legacy `Q<k>` (e.g. `Q0`, `Q3`) returning `(k + 1) * 8`.
fn parse_quantum_level_name(name: &str) -> u32 {
    if let Some(num_str) = name.strip_prefix('W') {
        num_str.parse::<u32>().unwrap_or(8)
    } else if let Some(num_str) = name.strip_prefix('Q') {
        let k = num_str.parse::<u32>().unwrap_or(0);
        (k + 1) * 8
    } else {
        8
    }
}
