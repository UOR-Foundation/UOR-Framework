//! Lean 4 structure generator.
//!
//! Generates per-namespace module files containing `structure` declarations
//! for each OWL class, with fields for each property.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Write as FmtWrite;

use uor_ontology::model::iris::*;
use uor_ontology::model::{Class, NamespaceModule, Property, PropertyKind};

use crate::emit::{normalize_lean_comment, LeanFile};
use crate::enums::enum_class_names;
use crate::mapping::{
    lean_qualified_name, local_name, to_lean_field_name, xsd_to_lean_type, LeanNamespaceMapping,
};

/// Generates a complete Lean 4 module file for a single namespace.
///
/// Returns `(content, structure_count, field_count)`.
pub fn generate_namespace_module(
    module: &NamespaceModule,
    ns_map: &HashMap<&str, LeanNamespaceMapping>,
    all_props_by_domain: &HashMap<&str, Vec<&Property>>,
    all_classes_by_iri: &HashMap<&str, &Class>,
) -> (String, usize, usize) {
    let ns = &module.namespace;
    let skip_classes: HashSet<&str> = enum_class_names().iter().copied().collect();

    // Build property-by-domain map for this namespace
    let props_by_domain = build_props_by_domain(&module.properties, ns.iri);

    // Collect required imports
    let imports = collect_imports(module, ns_map, ns.iri, &skip_classes);

    let mut f = LeanFile::new(&format!(
        "{} namespace structures.",
        normalize_lean_comment(ns.label)
    ));

    // Emit imports
    f.line("import UOR.Primitives");
    f.line("import UOR.Enums");
    for imp in &imports {
        let _ = writeln!(f.buf, "import {imp}");
    }
    f.blank();
    f.line("open UOR.Primitives");
    f.blank();

    let mut structure_count = 0usize;
    let mut field_count = 0usize;

    for class in &module.classes {
        let class_local = local_name(class.id);
        if skip_classes.contains(class_local) {
            continue;
        }

        let props = props_by_domain.get(class.id).cloned().unwrap_or_default();

        // Filter to non-annotation properties
        let non_annotation_props: Vec<&&Property> = props
            .iter()
            .filter(|p| p.kind != PropertyKind::Annotation)
            .collect();

        // Collect inherited fields to avoid re-declaring
        let inherited = collect_inherited_fields(
            class,
            all_props_by_domain,
            all_classes_by_iri,
            &skip_classes,
        );

        let fc = generate_structure(
            &mut f,
            class,
            &non_annotation_props,
            &inherited,
            ns_map,
            ns.iri,
            &skip_classes,
        );

        structure_count += 1;
        field_count += fc;
    }

    // Generate individual constants
    let ind_content = crate::individuals::generate_individuals(module, &skip_classes);
    if !ind_content.is_empty() {
        f.blank();
        f.buf.push_str(&ind_content);
    }

    (f.finish(), structure_count, field_count)
}

/// Generates a single `structure` declaration. Returns the number of fields emitted.
fn generate_structure(
    f: &mut LeanFile,
    class: &Class,
    props: &[&&Property],
    inherited_fields: &HashSet<String>,
    ns_map: &HashMap<&str, LeanNamespaceMapping>,
    current_ns_iri: &str,
    skip_classes: &HashSet<&str>,
) -> usize {
    let class_local = local_name(class.id);
    let comment = normalize_lean_comment(class.comment);

    f.doc_comment(&comment);

    // Build extends clause
    let extends = build_extends_clause(class, ns_map, current_ns_iri, skip_classes);

    // Start the structure declaration
    let extends_str = if extends.is_empty() {
        String::new()
    } else {
        format!(" extends {}", extends.join(", "))
    };

    // Filter out inherited fields
    let own_props: Vec<&&Property> = props
        .iter()
        .filter(|p| {
            let field_name = to_lean_field_name(local_name(p.id));
            !inherited_fields.contains(&field_name)
        })
        .copied()
        .collect();

    if own_props.is_empty() {
        let _ = writeln!(
            f.buf,
            "structure {class_local} (P : Primitives){extends_str}"
        );
    } else {
        let _ = writeln!(
            f.buf,
            "structure {class_local} (P : Primitives){extends_str} where"
        );
        for prop in &own_props {
            generate_field(f, prop, class_local, ns_map, current_ns_iri, skip_classes);
        }
    }
    f.blank();

    own_props.len()
}

/// Generates a single field declaration inside a structure.
fn generate_field(
    f: &mut LeanFile,
    prop: &Property,
    owner_class: &str,
    ns_map: &HashMap<&str, LeanNamespaceMapping>,
    current_ns_iri: &str,
    skip_classes: &HashSet<&str>,
) {
    let field_name = to_lean_field_name(local_name(prop.id));
    let comment = normalize_lean_comment(prop.comment);
    f.indented_doc_comment(&comment);

    let type_str = resolve_lean_type(prop, owner_class, ns_map, current_ns_iri, skip_classes);

    let _ = writeln!(f.buf, "  {field_name} : {type_str}");
}

/// Resolves the Lean type expression for a property.
fn resolve_lean_type(
    prop: &Property,
    owner_class: &str,
    ns_map: &HashMap<&str, LeanNamespaceMapping>,
    current_ns_iri: &str,
    skip_classes: &HashSet<&str>,
) -> String {
    match prop.kind {
        PropertyKind::Datatype => resolve_datatype(prop),
        PropertyKind::Object => {
            resolve_object_type(prop, owner_class, ns_map, current_ns_iri, skip_classes)
        }
        PropertyKind::Annotation => "P.String".to_string(),
    }
}

/// Resolves a datatype property to its Lean type.
fn resolve_datatype(prop: &Property) -> String {
    let base = xsd_to_lean_type(prop.range).unwrap_or("P.String");
    if prop.functional {
        base.to_string()
    } else {
        format!("Array {base}")
    }
}

/// Resolves an object property to its Lean type.
fn resolve_object_type(
    prop: &Property,
    owner_class: &str,
    ns_map: &HashMap<&str, LeanNamespaceMapping>,
    current_ns_iri: &str,
    skip_classes: &HashSet<&str>,
) -> String {
    let range_local = local_name(prop.range);

    // Check if range is an enum class → return enum type directly
    if skip_classes.contains(range_local) {
        return if prop.functional {
            range_local.to_string()
        } else {
            format!("Array {range_local}")
        };
    }

    // Check for generic OWL types
    if prop.range == OWL_THING || prop.range == OWL_CLASS || prop.range == RDF_LIST {
        return if prop.functional {
            "P.String".to_string()
        } else {
            "Array P.String".to_string()
        };
    }

    // Self-referential check
    let is_self_ref = range_local == owner_class;

    // Resolve the structure type
    let struct_type = if prop.range.starts_with(current_ns_iri) {
        // Same namespace — use bare name
        format!("{range_local} P")
    } else {
        // Cross-namespace — use fully qualified name
        match lean_qualified_name(prop.range, ns_map) {
            Some(qualified) => format!("{qualified} P"),
            None => format!("{range_local} P"),
        }
    };

    if is_self_ref && prop.functional {
        // Self-referential functional → Option to break recursion
        format!("Option ({struct_type})")
    } else if prop.functional {
        struct_type
    } else {
        format!("Array ({struct_type})")
    }
}

/// Builds the `extends` clause for a class.
fn build_extends_clause(
    class: &Class,
    ns_map: &HashMap<&str, LeanNamespaceMapping>,
    current_ns_iri: &str,
    skip_classes: &HashSet<&str>,
) -> Vec<String> {
    let mut parents = Vec::new();

    for parent_iri in class.subclass_of {
        // Skip owl:Thing
        if *parent_iri == OWL_THING {
            continue;
        }
        let parent_local = local_name(parent_iri);
        // Skip enum classes
        if skip_classes.contains(parent_local) {
            continue;
        }

        if parent_iri.starts_with(current_ns_iri) {
            // Same namespace
            parents.push(format!("{parent_local} P"));
        } else {
            // Cross-namespace
            match lean_qualified_name(parent_iri, ns_map) {
                Some(qualified) => parents.push(format!("{qualified} P")),
                None => parents.push(format!("{parent_local} P")),
            }
        }
    }

    parents
}

/// Collects field names inherited from parent classes (transitive).
fn collect_inherited_fields(
    class: &Class,
    all_props_by_domain: &HashMap<&str, Vec<&Property>>,
    all_classes_by_iri: &HashMap<&str, &Class>,
    skip_classes: &HashSet<&str>,
) -> HashSet<String> {
    let mut result = HashSet::new();
    let mut visited = HashSet::new();
    collect_inherited_fields_recursive(
        class.subclass_of,
        all_props_by_domain,
        all_classes_by_iri,
        skip_classes,
        &mut result,
        &mut visited,
    );
    result
}

/// Recursively walks parent classes to collect inherited field names.
fn collect_inherited_fields_recursive(
    parents: &[&str],
    all_props_by_domain: &HashMap<&str, Vec<&Property>>,
    all_classes_by_iri: &HashMap<&str, &Class>,
    skip_classes: &HashSet<&str>,
    result: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) {
    for parent_iri in parents {
        if *parent_iri == OWL_THING {
            continue;
        }
        let parent_local = local_name(parent_iri);
        if skip_classes.contains(parent_local) {
            continue;
        }
        if !visited.insert(parent_iri.to_string()) {
            continue; // Already visited (handles diamond)
        }

        // Add this parent's fields
        if let Some(props) = all_props_by_domain.get(parent_iri) {
            for prop in props {
                if prop.kind != PropertyKind::Annotation {
                    result.insert(to_lean_field_name(local_name(prop.id)));
                }
            }
        }

        // Recurse into grandparents
        if let Some(parent_class) = all_classes_by_iri.get(parent_iri) {
            collect_inherited_fields_recursive(
                parent_class.subclass_of,
                all_props_by_domain,
                all_classes_by_iri,
                skip_classes,
                result,
                visited,
            );
        }
    }
}

/// Builds a property-by-domain map for properties in this namespace.
///
/// Only includes properties whose domain is in the current namespace
/// (cross-namespace domain properties are not generated).
fn build_props_by_domain<'a>(
    properties: &'a [Property],
    ns_iri: &str,
) -> HashMap<&'a str, Vec<&'a Property>> {
    let mut map: HashMap<&str, Vec<&Property>> = HashMap::new();
    for prop in properties {
        if let Some(domain) = prop.domain {
            // Skip cross-namespace domain properties
            if !domain.starts_with(ns_iri) {
                continue;
            }
            map.entry(domain).or_default().push(prop);
        }
    }
    map
}

/// Collects cross-namespace import paths needed by this module.
fn collect_imports(
    module: &NamespaceModule,
    ns_map: &HashMap<&str, LeanNamespaceMapping>,
    current_ns_iri: &str,
    skip_classes: &HashSet<&str>,
) -> BTreeSet<String> {
    let mut imports = BTreeSet::new();

    for class in &module.classes {
        let class_local = local_name(class.id);
        if skip_classes.contains(class_local) {
            continue;
        }

        // Parents from other namespaces
        for parent_iri in class.subclass_of {
            if *parent_iri == OWL_THING {
                continue;
            }
            if !parent_iri.starts_with(current_ns_iri) {
                if let Some(imp) = find_import_for_iri(parent_iri, ns_map) {
                    imports.insert(imp);
                }
            }
        }
    }

    // Properties with cross-namespace ranges
    for prop in &module.properties {
        if prop.kind == PropertyKind::Annotation {
            continue;
        }
        if let Some(dom) = prop.domain {
            if !dom.starts_with(current_ns_iri) {
                continue; // Skip cross-namespace domain properties
            }
        }
        if prop.kind == PropertyKind::Object
            && !prop.range.starts_with(current_ns_iri)
            && prop.range != OWL_THING
            && prop.range != OWL_CLASS
            && prop.range != RDF_LIST
        {
            let range_local = local_name(prop.range);
            if !skip_classes.contains(range_local) {
                if let Some(imp) = find_import_for_iri(prop.range, ns_map) {
                    imports.insert(imp);
                }
            }
        }
    }

    imports
}

/// Finds the Lean module import path for a class IRI.
fn find_import_for_iri(
    class_iri: &str,
    ns_map: &HashMap<&str, LeanNamespaceMapping>,
) -> Option<String> {
    for (ns_iri, mapping) in ns_map {
        if class_iri.starts_with(ns_iri) {
            return Some(format!(
                "UOR.{}.{}",
                mapping.space_module, mapping.file_module
            ));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn self_referential_wraps_option() {
        let ontology = uor_ontology::Ontology::full();
        let ns_map = crate::mapping::lean_namespace_mappings();
        let all_props = crate::build_all_props_by_domain(ontology);
        let all_classes = crate::build_all_classes_by_iri(ontology);
        let op_module = ontology.find_namespace("op").unwrap();
        let (content, sc, _fc) =
            generate_namespace_module(op_module, &ns_map, &all_props, &all_classes);
        // Operation.inverse should be Option (self-referential)
        assert!(content.contains("Option (Operation P)"));
        assert!(sc > 0);
    }
}
