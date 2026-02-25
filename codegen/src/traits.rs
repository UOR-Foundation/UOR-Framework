//! Trait generation: OWL class → Rust trait, OWL property → trait method.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as FmtWrite;

use uor_ontology::model::iris::*;
use uor_ontology::model::{Class, Property, PropertyKind};
use uor_ontology::NamespaceModule;

use crate::emit::{normalize_comment, RustFile};
use crate::mapping::{
    class_trait_path, local_name, namespace_mappings, to_snake_case, xsd_is_unsized,
    xsd_to_primitives_type, NamespaceMapping,
};

/// Set of class local names that are represented as enums, not traits.
fn enum_class_names() -> HashSet<&'static str> {
    // These classes have their instances as enum variants, so we skip trait generation
    // MetricAxis is both a class and an enum — the enum takes priority
    ["MetricAxis"].into_iter().collect()
}

/// Maps an enum class local name to its enum type name.
/// When an ObjectProperty's range is one of these, we return the enum directly
/// instead of generating an associated type with a trait bound.
fn object_property_enum_override(range_local: &str) -> Option<&'static str> {
    match range_local {
        "MetricAxis" => Some("MetricAxis"),
        _ => None,
    }
}

/// Generates a single namespace module file.
///
/// Returns the Rust source code for the module.
pub fn generate_namespace_module(
    module: &NamespaceModule,
    ns_map: &HashMap<&str, NamespaceMapping>,
) -> String {
    let ns = &module.namespace;
    let space_str = format!("{:?}", ns.space);

    let mut f = RustFile::new(&format!(
        "`{}/` namespace — {}.\n//!\n//! Space: {space_str}",
        ns.prefix,
        normalize_comment(ns.comment)
    ));

    let skip_classes = enum_class_names();

    // Determine imports needed
    let mut needs_primitives = false;
    for prop in &module.properties {
        if prop.domain.is_some() && prop.kind != PropertyKind::Annotation {
            needs_primitives = true;
            break;
        }
    }
    // Also check if any class has supertrait that needs P
    for class in &module.classes {
        if skip_classes.contains(local_name(class.id)) {
            continue;
        }
        for _parent in class.subclass_of {
            if *_parent != OWL_THING {
                needs_primitives = true;
            }
        }
    }

    // Collect enum imports needed
    let mut enum_imports: Vec<&str> = Vec::new();
    for prop in &module.properties {
        if let Some(override_name) = datatype_enum_override(prop) {
            if !enum_imports.contains(&override_name) {
                enum_imports.push(override_name);
            }
        }
        // Also check object property ranges that are enum classes
        if prop.kind == PropertyKind::Object {
            let range_local = local_name(prop.range);
            if let Some(enum_name) = object_property_enum_override(range_local) {
                if !enum_imports.contains(&enum_name) {
                    enum_imports.push(enum_name);
                }
            }
        }
    }

    // Emit imports in alphabetical order (enum imports before Primitives)
    for imp in &enum_imports {
        let _ = writeln!(f.buf, "use crate::enums::{imp};");
    }
    if needs_primitives {
        f.line("use crate::Primitives;");
    }
    f.blank();

    // Build property-to-domain lookup
    let props_by_domain = build_props_by_domain(&module.properties);

    // Generate traits for each class (skip enum-represented classes)
    for class in &module.classes {
        if skip_classes.contains(local_name(class.id)) {
            continue;
        }
        generate_trait(&mut f, class, &props_by_domain, ns_map, ns.iri);
    }

    // Generate individual constants
    generate_individuals(&mut f, module);

    f.finish()
}

/// Builds a map from domain class IRI → list of properties.
fn build_props_by_domain(properties: &[Property]) -> HashMap<&str, Vec<&Property>> {
    let mut map: HashMap<&str, Vec<&Property>> = HashMap::new();
    for prop in properties {
        if let Some(domain) = prop.domain {
            map.entry(domain).or_default().push(prop);
        }
    }
    map
}

/// Generates a single trait for a class.
fn generate_trait(
    f: &mut RustFile,
    class: &Class,
    props_by_domain: &HashMap<&str, Vec<&Property>>,
    ns_map: &HashMap<&str, NamespaceMapping>,
    current_ns_iri: &str,
) {
    let trait_name = local_name(class.id);
    let comment = normalize_comment(class.comment);

    // Doc comment
    f.doc_comment(&comment);

    // Disjoint-with note
    if !class.disjoint_with.is_empty() {
        f.doc_comment("");
        let disjoints: Vec<&str> = class.disjoint_with.iter().map(|d| local_name(d)).collect();
        let _ = writeln!(f.buf, "/// Disjoint with: {}.", disjoints.join(", "));
    }

    // All traits in this crate take P: Primitives for consistency
    let p_param = "<P: Primitives>";

    // Supertrait bounds
    let supertraits = build_supertrait_bounds(class, ns_map, current_ns_iri);

    if supertraits.is_empty() {
        let _ = writeln!(f.buf, "pub trait {trait_name}{p_param} {{");
    } else {
        let bounds = supertraits.join(" + ");
        let _ = writeln!(f.buf, "pub trait {trait_name}{p_param}: {bounds} {{");
    }

    // Associated types and methods from properties
    let props = props_by_domain.get(class.id).cloned().unwrap_or_default();
    let non_annotation_props: Vec<&&Property> = props
        .iter()
        .filter(|p| p.kind != PropertyKind::Annotation)
        .collect();

    if non_annotation_props.is_empty() {
        // Empty trait body — emit `{}` on the same line
        // Remove the trailing `{\n` and replace with `{}\n`
        if f.buf.ends_with("{\n") {
            f.buf.truncate(f.buf.len() - 2);
            f.buf.push_str("{}\n");
        }
    } else {
        let mut associated_types: HashSet<String> = HashSet::new();
        for prop in &non_annotation_props {
            generate_property_method(
                f,
                prop,
                ns_map,
                current_ns_iri,
                trait_name,
                &mut associated_types,
            );
        }
        f.line("}");
    }
    f.blank();
}

/// Generates a method (and possibly an associated type) for a property.
fn generate_property_method(
    f: &mut RustFile,
    prop: &Property,
    ns_map: &HashMap<&str, NamespaceMapping>,
    current_ns_iri: &str,
    owner_trait_name: &str,
    associated_types: &mut HashSet<String>,
) {
    let method_name = to_snake_case(local_name(prop.id));
    let comment = normalize_comment(prop.comment);

    match prop.kind {
        PropertyKind::Datatype => {
            // Check if the range maps to a known enum type
            let enum_type = datatype_enum_override(prop);
            if let Some(enum_t) = enum_type {
                f.indented_doc_comment(&comment);
                let _ = writeln!(f.buf, "    fn {method_name}(&self) -> {enum_t};");
                return;
            }

            let prim_type = xsd_to_primitives_type(prop.range);
            match prim_type {
                Some(t) => {
                    f.indented_doc_comment(&comment);
                    if prop.functional {
                        if xsd_is_unsized(prop.range) {
                            let _ = writeln!(f.buf, "    fn {method_name}(&self) -> &{t};");
                        } else {
                            let _ = writeln!(f.buf, "    fn {method_name}(&self) -> {t};");
                        }
                    } else if xsd_is_unsized(prop.range) {
                        // Non-functional unsized: can't have &[str], use iterator count instead
                        // Emit a count method and an indexed getter
                        let _ = writeln!(f.buf, "    fn {method_name}_count(&self) -> usize;");
                    } else {
                        // Non-functional sized: return slice
                        let _ = writeln!(f.buf, "    fn {method_name}(&self) -> &[{t}];");
                    }
                }
                None => {
                    // Unknown XSD type — fall back to String ref
                    f.indented_doc_comment(&comment);
                    let _ = writeln!(f.buf, "    fn {method_name}(&self) -> &P::String;");
                }
            }
        }
        PropertyKind::Object => {
            let range_local = local_name(prop.range);
            let is_owl_thing = prop.range == OWL_THING;
            let is_owl_class = prop.range == OWL_CLASS;
            let is_rdf_list = prop.range == RDF_LIST;

            // Check if range is an enum class — return enum type directly
            if let Some(enum_path) = object_property_enum_override(range_local) {
                f.indented_doc_comment(&comment);
                if prop.functional {
                    let _ = writeln!(f.buf, "    fn {method_name}(&self) -> {enum_path};");
                } else {
                    let _ = writeln!(f.buf, "    fn {method_name}(&self) -> &[{enum_path}];");
                }
            } else if is_owl_thing || is_owl_class || is_rdf_list {
                // Generic object — use a string IRI reference
                f.indented_doc_comment(&comment);
                if prop.functional {
                    let _ = writeln!(f.buf, "    fn {method_name}(&self) -> &P::String;");
                } else {
                    // Non-functional unsized: emit count method
                    let _ = writeln!(f.buf, "    fn {method_name}_count(&self) -> usize;");
                }
            } else {
                // Generate associated type + method
                // Disambiguate if the associated type name matches the owning trait
                let assoc_name = if range_local == owner_trait_name {
                    format!("{range_local}Target")
                } else {
                    range_local.to_string()
                };

                // Avoid duplicate associated types
                if !associated_types.contains(&assoc_name) {
                    // Determine the trait bound path
                    let is_cross_ns = !prop.range.starts_with(current_ns_iri);
                    let trait_bound = if is_cross_ns {
                        class_trait_path(prop.range, ns_map)
                            .map(|p| format!("{p}<P>"))
                            .unwrap_or_else(|| format!("{range_local}<P>"))
                    } else {
                        format!("{range_local}<P>")
                    };

                    let _ = writeln!(f.buf, "    /// Associated type for `{range_local}`.");
                    let _ = writeln!(f.buf, "    type {assoc_name}: {trait_bound};");
                    associated_types.insert(assoc_name.clone());
                }

                f.indented_doc_comment(&comment);
                if prop.functional {
                    let _ = writeln!(f.buf, "    fn {method_name}(&self) -> &Self::{assoc_name};");
                } else {
                    let _ = writeln!(
                        f.buf,
                        "    fn {method_name}(&self) -> &[Self::{assoc_name}];"
                    );
                }
            }
        }
        PropertyKind::Annotation => {
            // Skip annotation properties in trait generation
        }
    }
}

/// Returns an enum type override for special datatype properties.
fn datatype_enum_override(prop: &Property) -> Option<&'static str> {
    let local = local_name(prop.id);
    match local {
        "fiberState" => Some("FiberState"),
        "geometricCharacter" => Some("GeometricCharacter"),
        _ => None,
    }
}

/// Builds supertrait bounds for a class.
fn build_supertrait_bounds(
    class: &Class,
    ns_map: &HashMap<&str, NamespaceMapping>,
    current_ns_iri: &str,
) -> Vec<String> {
    let mut bounds = Vec::new();
    let skip = enum_class_names();

    for parent_iri in class.subclass_of {
        // Skip owl:Thing — it's the universal superclass
        if *parent_iri == OWL_THING {
            continue;
        }

        let parent_local = local_name(parent_iri);

        // Skip if the parent is an enum class
        if skip.contains(parent_local) {
            continue;
        }

        let is_cross_ns = !parent_iri.starts_with(current_ns_iri);

        if is_cross_ns {
            if let Some(path) = class_trait_path(parent_iri, ns_map) {
                bounds.push(format!("{path}<P>"));
            } else {
                bounds.push(format!("{parent_local}<P>"));
            }
        } else {
            bounds.push(format!("{parent_local}<P>"));
        }
    }

    bounds
}

/// Generates named individual constant modules.
fn generate_individuals(f: &mut RustFile, module: &NamespaceModule) {
    use uor_ontology::IndividualValue;

    for ind in &module.individuals {
        let type_local = local_name(ind.type_);

        // Skip individuals that are part of enums (operations, metric axes)
        if type_local == "UnaryOp"
            || type_local == "BinaryOp"
            || type_local == "Involution"
            || type_local == "MetricAxis"
        {
            continue;
        }

        let mod_name = to_snake_case(local_name(ind.id));
        let comment = normalize_comment(ind.comment);

        f.doc_comment(&comment);
        let _ = writeln!(f.buf, "pub mod {mod_name} {{");

        // Group property assertions by IRI (preserving insertion order)
        let mut grouped: BTreeMap<&str, Vec<&IndividualValue>> = BTreeMap::new();
        for (prop_iri, value) in ind.properties {
            grouped.entry(prop_iri).or_default().push(value);
        }

        for (prop_iri, values) in &grouped {
            let prop_local = local_name(prop_iri);
            let base_const = to_snake_case(prop_local).to_uppercase();

            // If any value is a List, emit from the List (subsumes IriRef entries)
            if let Some(list_val) = values.iter().find_map(|v| match v {
                IndividualValue::List(items) => Some(items),
                _ => None,
            }) {
                let _ = writeln!(f.buf, "    /// `{prop_local}`");
                emit_str_slice(&mut f.buf, &base_const, list_val);
                continue;
            }

            // Multiple IriRef values → emit as slice
            if values.len() > 1 {
                if values
                    .iter()
                    .all(|v| matches!(v, IndividualValue::IriRef(_)))
                {
                    let items: Vec<&str> = values
                        .iter()
                        .filter_map(|v| match v {
                            IndividualValue::IriRef(iri) => Some(*iri),
                            _ => None,
                        })
                        .collect();
                    let _ = writeln!(f.buf, "    /// `{prop_local}`");
                    emit_str_slice(&mut f.buf, &base_const, &items);
                    continue;
                }
                if values.iter().all(|v| matches!(v, IndividualValue::Str(_))) {
                    let items: Vec<&str> = values
                        .iter()
                        .filter_map(|v| match v {
                            IndividualValue::Str(s) => Some(*s),
                            _ => None,
                        })
                        .collect();
                    let _ = writeln!(f.buf, "    /// `{prop_local}`");
                    emit_str_slice(&mut f.buf, &base_const, &items);
                    continue;
                }
            }

            // Single value — emit scalar const
            match values[0] {
                IndividualValue::Str(s) => {
                    let _ = writeln!(f.buf, "    /// `{prop_local}`");
                    let _ = writeln!(f.buf, "    pub const {base_const}: &str = \"{s}\";");
                }
                IndividualValue::Int(n) => {
                    let _ = writeln!(f.buf, "    /// `{prop_local}`");
                    let _ = writeln!(f.buf, "    pub const {base_const}: i64 = {n};");
                }
                IndividualValue::Bool(b) => {
                    let _ = writeln!(f.buf, "    /// `{prop_local}`");
                    let _ = writeln!(f.buf, "    pub const {base_const}: bool = {b};");
                }
                IndividualValue::IriRef(iri) => {
                    let ref_local = local_name(iri);
                    let _ = writeln!(f.buf, "    /// `{prop_local}` -> `{ref_local}`");
                    let _ = writeln!(f.buf, "    pub const {base_const}: &str = \"{iri}\";");
                }
                IndividualValue::List(_) => unreachable!(),
            }
        }

        f.line("}");
        f.blank();
    }
}

/// Emits a `pub const NAME: &[&str] = &[...];` with multi-line formatting for long items.
fn emit_str_slice(buf: &mut String, const_name: &str, items: &[&str]) {
    use std::fmt::Write as _;
    // Format each item on its own line for readability
    let _ = writeln!(buf, "    pub const {const_name}: &[&str] = &[");
    for item in items {
        let _ = writeln!(buf, "        \"{item}\",");
    }
    let _ = writeln!(buf, "    ];");
}

/// Returns the set of all namespace IRIs used by the ontology.
pub fn all_namespace_iris() -> HashMap<&'static str, NamespaceMapping> {
    namespace_mappings()
}
