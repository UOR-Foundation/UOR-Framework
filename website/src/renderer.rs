//! HTML rendering for the website (no external template engine required).
//!
//! All HTML is generated directly in Rust for determinism and zero dependencies.

use uor_spec::{IndividualValue, NamespaceModule, PropertyKind};

use crate::model::{BreadcrumbItem, NamespaceSummary};

/// Renders a complete HTML page using the site layout.
pub fn render_page(
    title: &str,
    body: &str,
    nav_html: &str,
    breadcrumbs: &[BreadcrumbItem],
) -> String {
    let crumb_html = render_breadcrumbs(breadcrumbs);
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title} — UOR Foundation</title>
<link rel="stylesheet" href="/css/style.css">
</head>
<body>
<a href="#main-content" class="skip-link">Skip to main content</a>
<header class="site-header">
<a href="/" class="site-logo">UOR Foundation</a>
<nav aria-label="Site navigation" class="site-nav">
{nav_html}
</nav>
</header>
<main id="main-content">
<nav aria-label="Breadcrumb" class="breadcrumb">
{crumb_html}
</nav>
<article class="page-content">
{body}
</article>
</main>
<footer class="site-footer">
<p>UOR Foundation — <a href="https://uor.foundation/">uor.foundation</a> — Apache-2.0</p>
</footer>
<script src="/js/search.js" defer></script>
</body>
</html>"##,
        title = escape_html(title),
        nav_html = nav_html,
        crumb_html = crumb_html,
        body = body,
    )
}

/// Renders breadcrumb navigation as an ordered list.
pub fn render_breadcrumbs(crumbs: &[BreadcrumbItem]) -> String {
    let mut html = String::from("<ol>\n");
    for (i, crumb) in crumbs.iter().enumerate() {
        if i + 1 == crumbs.len() {
            html.push_str(&format!(
                "<li aria-current=\"page\">{}</li>\n",
                escape_html(&crumb.label)
            ));
        } else {
            html.push_str(&format!(
                "<li><a href=\"{}\">{}</a></li>\n",
                escape_html(&crumb.url),
                escape_html(&crumb.label)
            ));
        }
    }
    html.push_str("</ol>");
    html
}

/// Renders the homepage body with namespace grid.
pub fn render_homepage(summaries: &[NamespaceSummary]) -> String {
    let mut grid = String::new();
    for ns in summaries {
        grid.push_str(&format!(
            r#"<article class="ns-card ns-space-{space}">
<h3><a href="{url}">{label}</a></h3>
<p class="ns-iri"><code>{iri}</code></p>
<p>{comment}</p>
<dl class="ns-counts">
<dt>Classes</dt><dd>{classes}</dd>
<dt>Properties</dt><dd>{props}</dd>
<dt>Individuals</dt><dd>{inds}</dd>
</dl>
</article>
"#,
            space = escape_html(&ns.space),
            url = escape_html(&ns.url),
            label = escape_html(&ns.label),
            iri = escape_html(&ns.iri),
            comment = escape_html(&ns.comment),
            classes = ns.class_count,
            props = ns.property_count,
            inds = ns.individual_count,
        ));
    }

    format!(
        r#"<section class="hero">
<h1>UOR Foundation</h1>
<p class="tagline">Universal Object Reference — a formal ontology for content-addressed, algebraically-structured object spaces.</p>
<p>
<a href="/docs/overview.html" class="btn-primary">Get Started</a>
<a href="/namespaces/" class="btn-secondary">Browse Namespaces</a>
<a href="/search.html" class="btn-secondary">Search</a>
</p>
</section>

<section class="inventory">
<h2>Ontology Inventory</h2>
<p>14 namespaces · 82 classes · 119 properties · 14 named individuals</p>
</section>

<section class="namespace-grid">
<h2>Namespaces</h2>
<div class="grid">
{grid}
</div>
</section>"#
    )
}

/// Renders the namespaces index page listing all 14 namespaces.
pub fn render_namespaces_index(summaries: &[NamespaceSummary]) -> String {
    let mut rows = String::new();
    for ns in summaries {
        rows.push_str(&format!(
            r#"<tr class="ns-space-{space}">
<td><a href="{url}"><code>{prefix}</code></a></td>
<td>{label}</td>
<td>{classes}</td>
<td>{props}</td>
<td>{inds}</td>
<td>{space}</td>
</tr>
"#,
            space = escape_html(&ns.space),
            url = escape_html(&ns.url),
            prefix = escape_html(&ns.prefix),
            label = escape_html(&ns.label),
            classes = ns.class_count,
            props = ns.property_count,
            inds = ns.individual_count,
        ));
    }

    format!(
        r#"<h1>Namespaces</h1>
<p>The UOR Foundation ontology is organized into 14 namespaces spanning three space classifications: kernel, bridge, and user.</p>
<table>
<thead>
<tr><th>Prefix</th><th>Label</th><th>Classes</th><th>Properties</th><th>Individuals</th><th>Space</th></tr>
</thead>
<tbody>
{rows}
</tbody>
</table>"#
    )
}

/// Renders a namespace detail page from the spec module.
pub fn render_namespace_page(module: &NamespaceModule) -> String {
    let ns = &module.namespace;
    let mut body = format!(
        r#"<h1>{label}</h1>
<dl class="ns-meta">
<dt>IRI</dt><dd><code>{iri}</code></dd>
<dt>Prefix</dt><dd><code>{prefix}:</code></dd>
<dt>Space</dt><dd class="space-{space}">{space}</dd>
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
        body.push_str("<h2>Imports</h2><ul>\n");
        for imp in ns.imports {
            body.push_str(&format!("<li><code>{}</code></li>\n", escape_html(imp)));
        }
        body.push_str("</ul>\n");
    }

    // Classes
    if !module.classes.is_empty() {
        body.push_str("<h2>Classes</h2>\n<table>\n");
        body.push_str("<thead><tr><th>Name</th><th>Subclass Of</th><th>Disjoint With</th><th>Comment</th></tr></thead>\n<tbody>\n");
        for class in &module.classes {
            let local = local_name(class.id);
            body.push_str(&format!(
                "<tr id=\"class-{local}\"><td><code>{label}</code></td><td>{parents}</td><td>{disjoints}</td><td>{comment}</td></tr>\n",
                local = escape_html(local),
                label = escape_html(class.label),
                parents = class.subclass_of.iter().map(|p| format!("<code>{}</code>", escape_html(local_name(p)))).collect::<Vec<_>>().join(", "),
                disjoints = class.disjoint_with.iter().map(|d| format!("<code>{}</code>", escape_html(local_name(d)))).collect::<Vec<_>>().join(", "),
                comment = escape_html(class.comment),
            ));
        }
        body.push_str("</tbody>\n</table>\n");
    }

    // Properties
    if !module.properties.is_empty() {
        body.push_str("<h2>Properties</h2>\n<table>\n");
        body.push_str("<thead><tr><th>Name</th><th>Kind</th><th>Functional</th><th>Domain</th><th>Range</th><th>Comment</th></tr></thead>\n<tbody>\n");
        for prop in &module.properties {
            let local = local_name(prop.id);
            let kind = match prop.kind {
                PropertyKind::Datatype => "Datatype",
                PropertyKind::Object => "Object",
                PropertyKind::Annotation => "Annotation",
            };
            body.push_str(&format!(
                "<tr id=\"prop-{local}\"><td><code>{label}</code></td><td>{kind}</td><td>{functional}</td><td><code>{domain}</code></td><td><code>{range}</code></td><td>{comment}</td></tr>\n",
                local = escape_html(local),
                label = escape_html(prop.label),
                kind = kind,
                functional = prop.functional,
                domain = escape_html(prop.domain.map(local_name).unwrap_or("—")),
                range = escape_html(local_name(prop.range)),
                comment = escape_html(prop.comment),
            ));
        }
        body.push_str("</tbody>\n</table>\n");
    }

    // Individuals
    if !module.individuals.is_empty() {
        body.push_str("<h2>Named Individuals</h2>\n<table>\n");
        body.push_str("<thead><tr><th>Name</th><th>Type</th><th>Comment</th></tr></thead>\n<tbody>\n");
        for ind in &module.individuals {
            let local = local_name(ind.id);
            body.push_str(&format!(
                "<tr id=\"ind-{local}\"><td><code>{label}</code></td><td><code>{type_}</code></td><td>{comment}</td></tr>\n",
                local = escape_html(local),
                label = escape_html(ind.label),
                type_ = escape_html(local_name(ind.type_)),
                comment = escape_html(ind.comment),
            ));
            // Individual properties as a nested list
            if !ind.properties.is_empty() {
                body.push_str("<tr><td colspan=\"3\"><ul>");
                for (prop_iri, value) in ind.properties {
                    let value_html = render_individual_value(value);
                    body.push_str(&format!(
                        "<li><code>{}</code>: {}</li>",
                        escape_html(local_name(prop_iri)),
                        value_html
                    ));
                }
                body.push_str("</ul></td></tr>\n");
            }
        }
        body.push_str("</tbody>\n</table>\n");
    }

    body
}

/// Renders an individual property value as HTML.
fn render_individual_value(value: &IndividualValue) -> String {
    match value {
        IndividualValue::Str(s) => escape_html(s),
        IndividualValue::Int(i) => i.to_string(),
        IndividualValue::Bool(b) => b.to_string(),
        IndividualValue::IriRef(iri) => {
            format!("<code>{}</code>", escape_html(local_name(iri)))
        }
        IndividualValue::List(items) => {
            let parts: Vec<String> = items
                .iter()
                .map(|i| format!("<code>{}</code>", escape_html(local_name(i))))
                .collect();
            format!("[{}]", parts.join(", "))
        }
    }
}

/// Renders the search page body.
pub fn render_search_page() -> String {
    r#"<h1>Search</h1>
<form role="search" action="/search.html" method="get">
<label for="search-input">Search ontology terms</label>
<input type="search" id="search-input" name="q" placeholder="e.g. Ring, criticalIdentity, partition…" autocomplete="off">
</form>
<ul id="search-results" aria-live="polite"></ul>"#.to_string()
}

/// Renders the sitemap.xml content.
pub fn render_sitemap(base_url: &str, paths: &[String]) -> String {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for path in paths {
        xml.push_str(&format!(
            "  <url><loc>{}{}</loc></url>\n",
            base_url, path
        ));
    }
    xml.push_str("</urlset>\n");
    xml
}

/// Escapes HTML special characters.
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Extracts the local name from an IRI.
fn local_name(iri: &str) -> &str {
    iri.rsplit('/').next().unwrap_or(iri)
}
