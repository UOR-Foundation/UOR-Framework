//! Lean 4 code generator for the UOR Foundation ontology.
//!
//! Generates `.lean` files from `Ontology::full()`, producing structures for
//! OWL classes, inductives for enum classes, and constant namespaces for
//! named individuals.

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    missing_docs,
    clippy::missing_errors_doc
)]

pub mod emit;
pub mod enums;
pub mod individuals;
pub mod mapping;
pub mod primitives;
pub mod structures;

use std::collections::{HashMap, HashSet};
use std::fmt::Write as FmtWrite;
use std::path::Path;

use anyhow::Result;
use uor_ontology::model::{Class, Ontology, Property, PropertyKind};

use crate::emit::write_file;
use crate::mapping::lean_namespace_mappings;

/// Report of what the generator produced.
pub struct LeanGenerationReport {
    /// Number of `structure` declarations generated.
    pub structure_count: usize,
    /// Number of structure fields generated.
    pub field_count: usize,
    /// Number of `inductive` + struct enum types generated.
    pub enum_count: usize,
    /// Number of individual constant namespaces generated.
    pub def_count: usize,
    /// Absolute paths of files written.
    pub files: Vec<String>,
}

/// Generates the complete Lean 4 formalization from the ontology.
///
/// Writes all `.lean` files to `out_dir/` and returns a generation report.
///
/// # Errors
///
/// Returns an error if any file cannot be written.
pub fn generate(ontology: &Ontology, out_dir: &Path) -> Result<LeanGenerationReport> {
    let ns_map = lean_namespace_mappings();
    let mut files = Vec::new();
    let mut total_structures = 0usize;
    let mut total_fields = 0usize;
    let mut total_defs = 0usize;

    // Build cross-namespace maps
    let all_props_by_domain = build_all_props_by_domain(ontology);
    let all_classes_by_iri = build_all_classes_by_iri(ontology);

    // 1. Generate Primitives
    let primitives_content = primitives::generate_primitives();
    let primitives_path = out_dir.join("UOR").join("Primitives.lean");
    write_file(&primitives_path, &primitives_content)?;
    files.push(primitives_path.display().to_string());

    // 2. Generate Enums
    let mut enums_content = enums::generate_enums(ontology);
    let op_methods = enums::generate_primitive_op_methods(ontology);
    if !op_methods.is_empty() {
        enums_content.push('\n');
        enums_content.push_str(&op_methods);
        enums_content.push('\n');
    }
    let enums_path = out_dir.join("UOR").join("Enums.lean");
    write_file(&enums_path, &enums_content)?;
    files.push(enums_path.display().to_string());
    let enum_count = enums::count_enums(ontology);

    // 3. Generate per-namespace modules
    let skip_types: HashSet<&str> = enums::enum_individual_types().into_iter().collect();

    // Track which modules belong to which space
    let mut kernel_modules = Vec::new();
    let mut bridge_modules = Vec::new();
    let mut user_modules = Vec::new();

    for module in &ontology.namespaces {
        let ns_iri = module.namespace.iri;
        let mapping = match ns_map.get(ns_iri) {
            Some(m) => m,
            None => continue,
        };

        let (content, sc, fc) = structures::generate_namespace_module(
            module,
            &ns_map,
            &all_props_by_domain,
            &all_classes_by_iri,
        );

        total_structures += sc;
        total_fields += fc;

        // Count individuals
        total_defs += individuals::count_individuals(module, &skip_types);

        let file_path = out_dir
            .join("UOR")
            .join(mapping.space_module)
            .join(format!("{}.lean", mapping.file_module));
        write_file(&file_path, &content)?;
        files.push(file_path.display().to_string());

        let module_import = format!("UOR.{}.{}", mapping.space_module, mapping.file_module);
        match mapping.space_module {
            "Kernel" => kernel_modules.push(module_import),
            "Bridge" => bridge_modules.push(module_import),
            "User" => user_modules.push(module_import),
            _ => {}
        }
    }

    // 4. Generate space-level import files
    kernel_modules.sort();
    bridge_modules.sort();
    user_modules.sort();

    let kernel_content = generate_space_import("Kernel-space modules.", &kernel_modules);
    let kernel_path = out_dir.join("UOR").join("Kernel.lean");
    write_file(&kernel_path, &kernel_content)?;
    files.push(kernel_path.display().to_string());

    let bridge_content = generate_space_import("Bridge-space modules.", &bridge_modules);
    let bridge_path = out_dir.join("UOR").join("Bridge.lean");
    write_file(&bridge_path, &bridge_content)?;
    files.push(bridge_path.display().to_string());

    let user_content = generate_space_import("User-space modules.", &user_modules);
    let user_path = out_dir.join("UOR").join("User.lean");
    write_file(&user_path, &user_content)?;
    files.push(user_path.display().to_string());

    // 5. Generate root UOR.lean
    let root_content = generate_root_import();
    let root_path = out_dir.join("UOR.lean");
    write_file(&root_path, &root_content)?;
    files.push(root_path.display().to_string());

    // 6. Generate LICENSE (required for Lean Reservoir indexing)
    let license_content = include_str!("../../LICENSE");
    let license_path = out_dir.join("LICENSE");
    write_file(&license_path, license_content)?;
    files.push(license_path.display().to_string());

    // 7. Generate README.md
    let readme = format!(
        "# UOR Foundation \u{2014} Lean 4 Formalization\n\n\
         Machine-generated Lean 4 structures, enums, and constants for the\n\
         [UOR Foundation](https://uor.foundation/) ontology (v{}).\n\n\
         **Do not edit manually** \u{2014} regenerated by \
         [UOR-Framework](https://github.com/UOR-Foundation/UOR-Framework).\n\n\
         ## Usage\n\n\
         Add to your `lakefile.lean`:\n\n\
         ```lean\n\
         require uor from git\n\
         \x20 \"https://github.com/UOR-Foundation/UOR-Framework\"\n\
         ```\n\n\
         Then `import UOR` in your Lean files.\n",
        ontology.version
    );
    let readme_path = out_dir.join("README.md");
    write_file(&readme_path, &readme)?;
    files.push(readme_path.display().to_string());

    Ok(LeanGenerationReport {
        structure_count: total_structures,
        field_count: total_fields,
        enum_count,
        def_count: total_defs,
        files,
    })
}

/// Builds the cross-namespace property-by-domain map.
pub fn build_all_props_by_domain(ontology: &Ontology) -> HashMap<&str, Vec<&Property>> {
    let mut map: HashMap<&str, Vec<&Property>> = HashMap::new();
    for module in &ontology.namespaces {
        for prop in &module.properties {
            if let Some(domain) = prop.domain {
                if prop.kind != PropertyKind::Annotation {
                    map.entry(domain).or_default().push(prop);
                }
            }
        }
    }
    map
}

/// Builds a map from class IRI to `Class` struct for transitive inheritance lookup.
pub fn build_all_classes_by_iri(ontology: &Ontology) -> HashMap<&str, &Class> {
    let mut map = HashMap::new();
    for module in &ontology.namespaces {
        for class in &module.classes {
            map.insert(class.id, class);
        }
    }
    map
}

/// Generates a space-level import file (e.g., `UOR/Kernel.lean`).
fn generate_space_import(doc: &str, modules: &[String]) -> String {
    let mut buf = String::new();
    let _ = writeln!(
        buf,
        "-- @generated by uor-lean from uor-ontology \u{2014} do not edit manually"
    );
    let _ = writeln!(buf, "--");
    let _ = writeln!(buf, "-- {doc}");
    buf.push('\n');
    for m in modules {
        let _ = writeln!(buf, "import {m}");
    }
    buf
}

/// Generates the root `UOR.lean` import file.
fn generate_root_import() -> String {
    let mut buf = String::new();
    let _ = writeln!(
        buf,
        "-- @generated by uor-lean from uor-ontology \u{2014} do not edit manually"
    );
    let _ = writeln!(buf, "--");
    let _ = writeln!(buf, "-- UOR Foundation \u{2014} Lean 4 formalization root.");
    buf.push('\n');
    buf.push_str("import UOR.Primitives\n");
    buf.push_str("import UOR.Enums\n");
    buf.push_str("import UOR.Kernel\n");
    buf.push_str("import UOR.Bridge\n");
    buf.push_str("import UOR.User\n");
    buf
}
