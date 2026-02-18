//! UOR Foundation documentation generator.
//!
//! Generates verified HTML documentation from the UOR spec and content files.
//! Every ontology reference in prose (`{@class}`, `{@prop}`, `{@ind}`) is
//! validated against `uor_spec::Ontology::full()` at build time.
//!
//! # Entry Points
//!
//! ```no_run
//! use std::path::PathBuf;
//! use uor_docs::generate;
//!
//! let out = PathBuf::from("public/docs");
//! let readme = PathBuf::from("README.md");
//! generate(&out, &readme).expect("Documentation generation failed");
//! ```
//!
//! # Structure
//!
//! ```text
//! public/docs/
//!   index.html              ← Ontology inventory table
//!   namespaces/
//!     u.html                ← Auto-generated from spec (100% accurate)
//!     schema.html
//!     ... (14 total)
//!   concepts/
//!     ring.html
//!     content-addressing.html
//!     ... (7 pages)
//!   guides/
//!     implementing-prism.html
//!     conformance.html
//!     contributing.html
//! ```

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    missing_docs,
    clippy::missing_errors_doc
)]

pub mod extractor;
pub mod linker;
pub mod renderer;
pub mod verifier;
pub mod writer;

use std::path::Path;

use anyhow::Result;

use extractor::OntologyIndex;
use renderer::{escape_html, render_docs_page};
use uor_spec::{Individual, IndividualValue, NamespaceModule, Ontology, PropertyKind};

/// Generates all documentation artifacts.
///
/// Writes to `out_dir` (HTML docs) and `readme_path` (machine-generated README).
///
/// # Errors
///
/// Returns an error if content verification fails or any file cannot be written.
pub fn generate(out_dir: &Path, readme_path: &Path) -> Result<()> {
    let index = OntologyIndex::from_spec();

    let base_path = std::env::var("PUBLIC_BASE_PATH").unwrap_or_default();
    let base_path = base_path.trim_end_matches('/');

    // Verify prose content references (if content/ dir exists alongside this crate)
    let content_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    verifier::verify_content(&content_dir)?;

    let site_nav_html = build_site_nav_html(base_path);
    let docs_nav_html = build_docs_sidebar_html(base_path);

    // Generate index page
    let index_html = generate_index_page(&index, &site_nav_html, &docs_nav_html, base_path);
    writer::write_html(&out_dir.join("index.html"), &index_html)?;

    // Generate per-namespace reference pages (100% from spec)
    let ontology = Ontology::full();
    for module in &ontology.namespaces {
        let html = generate_namespace_page(module, &site_nav_html, &docs_nav_html, base_path);
        let path = out_dir
            .join("namespaces")
            .join(format!("{}.html", module.namespace.prefix));
        writer::write_html(&path, &html)?;
    }

    // Generate concept pages from content/
    generate_content_pages(
        &content_dir.join("concepts"),
        &out_dir.join("concepts"),
        &index,
        &site_nav_html,
        &docs_nav_html,
        base_path,
    )?;

    // Generate concepts index page
    let concepts_index = render_docs_page(
        "Concepts",
        r#"<h1>Concepts</h1>
<p>Explanatory documentation for the key ideas in the UOR Foundation ontology.</p>
<ul>
<li><a href="ring.html">Ring — The algebraic substrate Z/(2^n)Z</a></li>
<li><a href="content-addressing.html">Content Addressing — Braille-encoded content identifiers</a></li>
<li><a href="critical-identity.html">Critical Identity — The fundamental equation neg(bnot(x)) = succ(x)</a></li>
<li><a href="partition.html">Partition — Decomposition of address spaces</a></li>
<li><a href="resolution.html">Resolution — The PRISM resolution pipeline</a></li>
<li><a href="type-system.html">Type System — Typed expressions and abstraction layers</a></li>
<li><a href="state-model.html">State Model — Execution contexts and binding frames</a></li>
</ul>"#,
        &site_nav_html,
        &docs_nav_html,
        &format!(
            r#"<a href="{base_path}/">Home</a> › <a href="{base_path}/docs/index.html">Docs</a> › Concepts"#
        ),
        base_path,
    );
    writer::write_html(
        &out_dir.join("concepts").join("index.html"),
        &concepts_index,
    )?;

    // Generate guide pages from content/
    generate_content_pages(
        &content_dir.join("guides"),
        &out_dir.join("guides"),
        &index,
        &site_nav_html,
        &docs_nav_html,
        base_path,
    )?;

    // Generate guides index page
    let guides_index = render_docs_page(
        "Guides",
        r#"<h1>Guides</h1>
<p>How-to guides for working with and implementing the UOR Framework.</p>
<ul>
<li><a href="implementing-prism.html">Implementing PRISM — Build a UOR-compliant resolver</a></li>
<li><a href="conformance.html">Conformance — Validate your implementation</a></li>
<li><a href="contributing.html">Contributing — How to contribute to UOR</a></li>
</ul>"#,
        &site_nav_html,
        &docs_nav_html,
        &format!(
            r#"<a href="{base_path}/">Home</a> › <a href="{base_path}/docs/index.html">Docs</a> › Guides"#
        ),
        base_path,
    );
    writer::write_html(&out_dir.join("guides").join("index.html"), &guides_index)?;

    // Generate overview and architecture pages
    generate_single_page(
        &content_dir.join("overview.md"),
        &out_dir.join("overview.html"),
        "Overview",
        &index,
        &site_nav_html,
        &docs_nav_html,
        base_path,
    )?;
    generate_single_page(
        &content_dir.join("architecture.md"),
        &out_dir.join("architecture.html"),
        "Architecture",
        &index,
        &site_nav_html,
        &docs_nav_html,
        base_path,
    )?;

    // Generate machine-generated README.md
    let readme_content = generate_readme(ontology);
    writer::write_text(readme_path, &readme_content)?;

    Ok(())
}

/// Generates the compact top-bar site navigation (4 items matching the main website).
fn build_site_nav_html(base_path: &str) -> String {
    format!(
        r#"<ul>
<li><a href="{base_path}/">Home</a></li>
<li><a href="{base_path}/namespaces/">Namespaces</a></li>
<li><a href="{base_path}/docs/index.html">Documentation</a></li>
<li><a href="{base_path}/search.html">Search</a></li>
</ul>"#
    )
}

/// Generates the docs-specific sidebar navigation tree.
fn build_docs_sidebar_html(base_path: &str) -> String {
    format!(
        r#"<ul>
<li><a href="{base_path}/docs/index.html">Documentation</a></li>
<li><a href="{base_path}/docs/overview.html">Overview</a></li>
<li><a href="{base_path}/docs/architecture.html">Architecture</a></li>
<li class="nav-group"><span>Namespaces</span>
<ul>
<li><a href="{base_path}/docs/namespaces/u.html">u</a></li>
<li><a href="{base_path}/docs/namespaces/schema.html">schema</a></li>
<li><a href="{base_path}/docs/namespaces/op.html">op</a></li>
<li><a href="{base_path}/docs/namespaces/query.html">query</a></li>
<li><a href="{base_path}/docs/namespaces/resolver.html">resolver</a></li>
<li><a href="{base_path}/docs/namespaces/type.html">type</a></li>
<li><a href="{base_path}/docs/namespaces/partition.html">partition</a></li>
<li><a href="{base_path}/docs/namespaces/observable.html">observable</a></li>
<li><a href="{base_path}/docs/namespaces/proof.html">proof</a></li>
<li><a href="{base_path}/docs/namespaces/derivation.html">derivation</a></li>
<li><a href="{base_path}/docs/namespaces/trace.html">trace</a></li>
<li><a href="{base_path}/docs/namespaces/cert.html">cert</a></li>
<li><a href="{base_path}/docs/namespaces/morphism.html">morphism</a></li>
<li><a href="{base_path}/docs/namespaces/state.html">state</a></li>
</ul>
</li>
<li class="nav-group"><span>Concepts</span>
<ul>
<li><a href="{base_path}/docs/concepts/ring.html">Ring</a></li>
<li><a href="{base_path}/docs/concepts/content-addressing.html">Content Addressing</a></li>
<li><a href="{base_path}/docs/concepts/critical-identity.html">Critical Identity</a></li>
<li><a href="{base_path}/docs/concepts/partition.html">Partition</a></li>
<li><a href="{base_path}/docs/concepts/resolution.html">Resolution</a></li>
<li><a href="{base_path}/docs/concepts/type-system.html">Type System</a></li>
<li><a href="{base_path}/docs/concepts/state-model.html">State Model</a></li>
</ul>
</li>
<li class="nav-group"><span>Guides</span>
<ul>
<li><a href="{base_path}/docs/guides/implementing-prism.html">Implementing PRISM</a></li>
<li><a href="{base_path}/docs/guides/conformance.html">Conformance</a></li>
<li><a href="{base_path}/docs/guides/contributing.html">Contributing</a></li>
</ul>
</li>
<li><a href="{base_path}/search.html">Search</a></li>
</ul>"#
    )
}

/// Generates the ontology inventory index page.
fn generate_index_page(
    index: &OntologyIndex,
    site_nav_html: &str,
    docs_nav_html: &str,
    base_path: &str,
) -> String {
    let mut rows = String::new();
    for module in &index.modules {
        let ns = &module.namespace;
        rows.push_str(&format!(
            "<tr><td><a href=\"namespaces/{prefix}.html\">{prefix}</a></td><td>{label}</td><td>{classes}</td><td>{props}</td><td>{inds}</td><td>{space}</td></tr>\n",
            prefix = escape_html(ns.prefix),
            label = escape_html(ns.label),
            classes = module.classes.len(),
            props = module.properties.len(),
            inds = module.individuals.len(),
            space = format!("{:?}", ns.space).to_lowercase(),
        ));
    }

    let content = format!(
        r#"<h1>UOR Foundation Ontology</h1>
<p>Version 1.0.0 — 14 namespaces, 82 classes, 120 properties, 14 named individuals.</p>
<table>
<thead>
<tr><th>Prefix</th><th>Label</th><th>Classes</th><th>Properties</th><th>Individuals</th><th>Space</th></tr>
</thead>
<tbody>
{rows}
</tbody>
</table>"#
    );

    render_docs_page(
        "Documentation Index",
        &content,
        site_nav_html,
        docs_nav_html,
        &format!(r#"<a href="{base_path}/">Home</a> › Documentation"#),
        base_path,
    )
}

/// Generates a namespace reference page from the spec (100% auto-generated).
fn generate_namespace_page(
    module: &NamespaceModule,
    site_nav_html: &str,
    docs_nav_html: &str,
    base_path: &str,
) -> String {
    let ns = &module.namespace;
    let mut content = format!(
        r#"<h1>{label}</h1>
<dl>
<dt>IRI</dt><dd><code>{iri}</code></dd>
<dt>Prefix</dt><dd><code>{prefix}:</code></dd>
<dt>Space</dt><dd>{space}</dd>
<dt>Comment</dt><dd>{comment}</dd>
</dl>
"#,
        label = escape_html(ns.label),
        iri = escape_html(ns.iri),
        prefix = escape_html(ns.prefix),
        space = format!("{:?}", ns.space).to_lowercase(),
        comment = escape_html(ns.comment),
    );

    // Imports
    if !ns.imports.is_empty() {
        content.push_str("<h2>Imports</h2><ul>\n");
        for imp in ns.imports {
            content.push_str(&format!("<li><code>{}</code></li>\n", escape_html(imp)));
        }
        content.push_str("</ul>\n");
    }

    // Classes
    if !module.classes.is_empty() {
        content.push_str("<h2>Classes</h2>\n<table>\n<thead><tr><th>Name</th><th>IRI</th><th>Subclass Of</th><th>Disjoint With</th><th>Comment</th></tr></thead>\n<tbody>\n");
        for class in &module.classes {
            content.push_str(&format!(
                "<tr id=\"{id_fragment}\"><td>{label}</td><td><code>{id}</code></td><td>{parents}</td><td>{disjoint}</td><td>{comment}</td></tr>\n",
                id_fragment = escape_html(&linker::fragment_from_iri(class.id)),
                label = escape_html(class.label),
                id = escape_html(class.id),
                parents = class.subclass_of.iter().map(|p| format!("<code>{}</code>", escape_html(p))).collect::<Vec<_>>().join(", "),
                disjoint = class.disjoint_with.iter().map(|d| format!("<code>{}</code>", escape_html(d))).collect::<Vec<_>>().join(", "),
                comment = escape_html(class.comment),
            ));
        }
        content.push_str("</tbody>\n</table>\n");
    }

    // Properties
    if !module.properties.is_empty() {
        content.push_str("<h2>Properties</h2>\n<table>\n<thead><tr><th>Name</th><th>Kind</th><th>Functional</th><th>Domain</th><th>Range</th><th>Comment</th></tr></thead>\n<tbody>\n");
        for prop in &module.properties {
            let kind = match prop.kind {
                PropertyKind::Datatype => "Datatype",
                PropertyKind::Object => "Object",
                PropertyKind::Annotation => "Annotation",
            };
            content.push_str(&format!(
                "<tr id=\"{id_fragment}\"><td>{label}</td><td>{kind}</td><td>{functional}</td><td><code>{domain}</code></td><td><code>{range}</code></td><td>{comment}</td></tr>\n",
                id_fragment = escape_html(&linker::fragment_from_iri(prop.id)),
                label = escape_html(prop.label),
                kind = kind,
                functional = prop.functional,
                domain = escape_html(prop.domain.unwrap_or("—")),
                range = escape_html(prop.range),
                comment = escape_html(prop.comment),
            ));
        }
        content.push_str("</tbody>\n</table>\n");
    }

    // Named individuals
    if !module.individuals.is_empty() {
        content.push_str("<h2>Named Individuals</h2>\n<table>\n<thead><tr><th>Name</th><th>Type</th><th>Properties</th><th>Comment</th></tr></thead>\n<tbody>\n");
        for ind in &module.individuals {
            let props_html = format_individual_properties(ind);
            content.push_str(&format!(
                "<tr id=\"{id_fragment}\"><td>{label}</td><td><code>{type_}</code></td><td>{props}</td><td>{comment}</td></tr>\n",
                id_fragment = escape_html(&linker::fragment_from_iri(ind.id)),
                label = escape_html(ind.label),
                type_ = escape_html(ind.type_),
                props = props_html,
                comment = escape_html(ind.comment),
            ));
        }
        content.push_str("</tbody>\n</table>\n");
    }

    render_docs_page(
        ns.label,
        &content,
        site_nav_html,
        docs_nav_html,
        &format!(
            r#"<a href="{base_path}/">Home</a> › <a href="{base_path}/docs/index.html">Docs</a> › {}"#,
            escape_html(ns.label)
        ),
        base_path,
    )
}

/// Formats an individual's property values as an HTML list.
fn format_individual_properties(ind: &Individual) -> String {
    if ind.properties.is_empty() {
        return String::from("—");
    }
    let mut items = String::from("<ul>");
    for (prop_iri, value) in ind.properties {
        let value_str = match value {
            IndividualValue::Str(s) => escape_html(s),
            IndividualValue::Int(i) => i.to_string(),
            IndividualValue::Bool(b) => b.to_string(),
            IndividualValue::IriRef(iri) => format!("<code>{}</code>", escape_html(iri)),
            IndividualValue::List(items) => {
                let joined = items
                    .iter()
                    .map(|i| format!("<code>{}</code>", escape_html(i)))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", joined)
            }
        };
        let prop_local = linker::fragment_from_iri(prop_iri);
        items.push_str(&format!(
            "<li><code>{}</code>: {}</li>",
            escape_html(&prop_local),
            value_str
        ));
    }
    items.push_str("</ul>");
    items
}

/// Generates HTML pages from Markdown files in `src_dir`, writing to `out_dir`.
///
/// # Errors
///
/// Returns an error if files cannot be read or written.
fn generate_content_pages(
    src_dir: &Path,
    out_dir: &Path,
    index: &OntologyIndex,
    site_nav_html: &str,
    docs_nav_html: &str,
    base_path: &str,
) -> Result<()> {
    if !src_dir.exists() {
        return Ok(());
    }

    for entry in std::fs::read_dir(src_dir)
        .map_err(|e| anyhow::anyhow!("Cannot read {}: {}", src_dir.display(), e))?
    {
        let entry = entry.map_err(|e| anyhow::anyhow!("Directory entry error: {}", e))?;
        let path = entry.path();
        if path.extension().map(|x| x == "md").unwrap_or(false) {
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("page");
            let out_path = out_dir.join(format!("{}.html", stem));
            generate_single_page(
                &path,
                &out_path,
                stem,
                index,
                site_nav_html,
                docs_nav_html,
                base_path,
            )?;
        }
    }

    Ok(())
}

/// Generates a single HTML page from a Markdown file.
///
/// # Errors
///
/// Returns an error if the source file cannot be read or the output cannot be written.
fn generate_single_page(
    src: &Path,
    out: &Path,
    title: &str,
    index: &OntologyIndex,
    site_nav_html: &str,
    docs_nav_html: &str,
    base_path: &str,
) -> Result<()> {
    let markdown = if src.exists() {
        std::fs::read_to_string(src)
            .map_err(|e| anyhow::anyhow!("Cannot read {}: {}", src.display(), e))?
    } else {
        // Generate a placeholder page if the source doesn't exist
        format!("# {}\n\nThis page is coming soon.\n", title)
    };

    let content_html = renderer::render_markdown(&markdown, index);
    let page = render_docs_page(
        title,
        &content_html,
        site_nav_html,
        docs_nav_html,
        &format!(
            r#"<a href="{base_path}/">Home</a> › <a href="{base_path}/docs/index.html">Docs</a> › {}"#,
            escape_html(title)
        ),
        base_path,
    );

    writer::write_html(out, &page)
}

/// Generates the machine-generated README.md content.
fn generate_readme(ontology: &Ontology) -> String {
    format!(
        r#"# UOR Framework

The Universal Object Reference (UOR) Framework is a Rust workspace implementing
the [UOR Foundation](https://uor.foundation/) ontology — a mathematical framework
for content-addressed, symmetric, multi-metric object spaces with algebraic
structure based on Z/(2^n)Z.

## Ontology

Version {version}: {ns} namespaces · {classes} classes · {props} properties · {inds} named individuals

All terms are encoded as typed Rust data in `spec/` and serialized to:
- `public/uor.foundation.json` — JSON-LD 1.1
- `public/uor.foundation.ttl` — Turtle 1.1
- `public/uor.foundation.nt` — N-Triples

## Repository Structure

| Directory | Role |
|-----------|------|
| `spec/` | Rust library: UOR ontology as typed static data + serializers |
| `conformance/` | Rust library: workspace-wide conformance validators |
| `docs/` | Rust library: documentation generator |
| `website/` | Rust library: static site generator |
| `clients/` | Rust binaries: build, conformance, docs, website |

## Building

```sh
# Build ontology artifacts
cargo run --bin uor-build

# Generate documentation
cargo run --bin uor-docs

# Generate website
cargo run --bin uor-website

# Run full conformance suite
cargo run --bin uor-conformance
```

## CI

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## License

Apache-2.0 — see [LICENSE](LICENSE).

---

*This README is machine-generated by `uor-docs`. Do not edit by hand.*
"#,
        version = ontology.version,
        ns = ontology.namespaces.len(),
        classes = ontology.class_count(),
        props = ontology.property_count(),
        inds = ontology.individual_count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_has_all_terms() {
        let index = OntologyIndex::from_spec();
        assert_eq!(index.modules.len(), 14);
        assert_eq!(index.classes.len(), 82);
        assert_eq!(index.properties.len(), 119);
        assert_eq!(index.individuals.len(), 14);
    }

    #[test]
    fn class_ref_resolves() {
        let index = OntologyIndex::from_spec();
        assert!(index.is_class("https://uor.foundation/schema/Ring"));
        assert!(index.is_class("https://uor.foundation/op/Identity"));
        assert!(!index.is_class("https://uor.foundation/nonexistent/Foo"));
    }

    #[test]
    fn directive_expansion_known_class() {
        let index = OntologyIndex::from_spec();
        let src = "See {@class https://uor.foundation/schema/Ring} for details.";
        let expanded = renderer::expand_directives(src, &index);
        assert!(
            expanded.contains("[Ring]"),
            "Expected [Ring] in: {}",
            expanded
        );
    }

    #[test]
    fn directive_expansion_unknown_ref() {
        let index = OntologyIndex::from_spec();
        let src = "{@class https://example.com/Unknown}";
        let expanded = renderer::expand_directives(src, &index);
        // Unknown references render as code spans
        assert!(expanded.contains('`'));
    }
}
