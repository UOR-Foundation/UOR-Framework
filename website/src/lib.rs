//! UOR Foundation static site generator.
//!
//! Generates the complete https://uor.foundation/ website as a directory of
//! static HTML, CSS, and JavaScript files. All namespace and class pages are
//! 100% auto-generated from `uor_spec::Ontology::full()`.
//!
//! # Entry Point
//!
//! ```no_run
//! use std::path::PathBuf;
//! use uor_website::generate;
//!
//! let out = PathBuf::from("public");
//! generate(&out).expect("Website generation failed");
//! ```
//!
//! # Output Structure
//!
//! ```text
//! public/
//!   index.html
//!   search.html
//!   search-index.json
//!   sitemap.xml
//!   namespaces/
//!     <prefix>/index.html   (14 pages, 100% auto-generated)
//!   css/style.css
//!   js/search.js
//! ```

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    missing_docs,
    clippy::missing_errors_doc
)]

pub mod extractor;
pub mod model;
pub mod nav;
pub mod renderer;
pub mod search;
pub mod writer;

use std::path::Path;

use anyhow::Result;
use uor_spec::Ontology;

use extractor::{home_breadcrumbs, namespace_breadcrumbs, namespace_summaries, namespaces_index_breadcrumbs};
use nav::{build_nav, render_nav};
use renderer::{
    render_homepage, render_namespace_page, render_namespaces_index, render_page,
    render_search_page, render_sitemap,
};

const BASE_URL: &str = "https://uor.foundation";

/// Generates the complete website into `out_dir`.
///
/// # Errors
///
/// Returns an error if any file cannot be written.
pub fn generate(out_dir: &Path) -> Result<()> {
    let nav = build_nav();
    let nav_html = render_nav(&nav, "/");

    // Track all pages for sitemap
    let mut sitemap_paths: Vec<String> = Vec::new();

    // Homepage
    let summaries = namespace_summaries();
    let home_body = render_homepage(&summaries);
    let home_html = render_page("UOR Foundation", &home_body, &nav_html, &home_breadcrumbs());
    writer::write(&out_dir.join("index.html"), &home_html)?;
    sitemap_paths.push("/".to_string());

    // Search page
    let search_body = render_search_page();
    let search_nav = render_nav(&nav, "/search.html");
    let search_crumbs = vec![
        model::BreadcrumbItem { label: "Home".to_string(), url: "/".to_string() },
        model::BreadcrumbItem { label: "Search".to_string(), url: String::new() },
    ];
    let search_html = render_page("Search", &search_body, &search_nav, &search_crumbs);
    writer::write(&out_dir.join("search.html"), &search_html)?;
    sitemap_paths.push("/search.html".to_string());

    // Namespaces index page
    let ns_index_nav = render_nav(&nav, "/namespaces/");
    let ns_index_body = render_namespaces_index(&summaries);
    let ns_index_html = render_page("Namespaces", &ns_index_body, &ns_index_nav, &namespaces_index_breadcrumbs());
    writer::write(&out_dir.join("namespaces").join("index.html"), &ns_index_html)?;
    sitemap_paths.push("/namespaces/".to_string());

    // Namespace pages (100% auto-generated from spec)
    let ontology = Ontology::full();
    for module in &ontology.namespaces {
        let prefix = module.namespace.prefix;
        let page_path = format!("/namespaces/{}/", prefix);
        let page_nav = render_nav(&nav, &page_path);
        let ns_breadcrumbs = namespace_breadcrumbs(module.namespace.label);
        let body = render_namespace_page(module);
        let html = render_page(module.namespace.label, &body, &page_nav, &ns_breadcrumbs);

        let out_path = out_dir.join("namespaces").join(prefix).join("index.html");
        writer::write(&out_path, &html)?;
        sitemap_paths.push(page_path);
    }

    // Search index
    let search_index_json = search::generate_search_index()?;
    writer::write(&out_dir.join("search-index.json"), &search_index_json)?;

    // Sitemap
    let sitemap_xml = render_sitemap(BASE_URL, &sitemap_paths);
    writer::write(&out_dir.join("sitemap.xml"), &sitemap_xml)?;

    // CSS
    writer::write(&out_dir.join("css").join("style.css"), style_css())?;

    // JavaScript
    writer::write(&out_dir.join("js").join("search.js"), search::search_js())?;

    Ok(())
}

/// Returns the complete CSS stylesheet.
fn style_css() -> &'static str {
    include_str!("../static/css/style.css")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_index_has_all_classes() {
        let entries = extractor::build_search_index();
        let class_count = entries.iter().filter(|e| e.kind == "class").count();
        assert_eq!(class_count, 82, "Expected 82 class entries in search index");
    }

    #[test]
    fn namespace_summaries_count() {
        let summaries = namespace_summaries();
        assert_eq!(summaries.len(), 14);
    }

    #[test]
    fn nav_renders_non_empty() {
        let nav = build_nav();
        let html = render_nav(&nav, "/");
        assert!(!html.is_empty());
        assert!(html.contains("UOR Foundation") || html.contains("Home"));
    }
}
