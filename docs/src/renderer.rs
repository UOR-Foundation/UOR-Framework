//! Renders Markdown content with `{@class}`, `{@prop}`, `{@ind}` DSL expansion.

use pulldown_cmark::{html, Options, Parser};

use crate::extractor::OntologyIndex;
use crate::linker::resolve_ref;

/// Renders a Markdown file to HTML, expanding `{@class}`, `{@prop}`, `{@ind}` directives.
///
/// # Errors
///
/// This function is infallible (returns `String`).
pub fn render_markdown(source: &str, index: &OntologyIndex) -> String {
    let expanded = expand_directives(source, index);
    markdown_to_html(&expanded)
}

/// Expands `{@class iri}`, `{@prop iri}`, `{@ind iri}` directives into Markdown links.
pub fn expand_directives(source: &str, index: &OntologyIndex) -> String {
    let mut result = String::with_capacity(source.len());
    let mut remaining = source;

    while let Some(start) = remaining.find("{@") {
        result.push_str(&remaining[..start]);
        remaining = &remaining[start..];

        let end = match remaining.find('}') {
            Some(e) => e,
            None => {
                result.push_str(remaining);
                return result;
            }
        };

        let directive = &remaining[2..end];
        remaining = &remaining[end + 1..];

        let parts: Vec<&str> = directive.splitn(2, ' ').collect();
        if parts.len() != 2 {
            result.push_str(&format!("{{@{}}}", directive));
            continue;
        }

        let kind = parts[0].trim();
        let iri = parts[1].trim();

        let link = match kind {
            "class" => resolve_class_ref(iri, index),
            "prop" => resolve_prop_ref(iri, index),
            "ind" => resolve_ind_ref(iri, index),
            _ => format!("{{@{} {}}}", kind, iri),
        };

        result.push_str(&link);
    }

    result.push_str(remaining);
    result
}

/// Resolves a `{@class iri}` to a Markdown link.
fn resolve_class_ref(iri: &str, index: &OntologyIndex) -> String {
    if let Some(class) = index.classes.iter().find(|c| c.id == iri) {
        let href = resolve_ref("class", iri, index);
        format!("[{}]({})", class.label, href)
    } else {
        format!("`{}`", iri)
    }
}

/// Resolves a `{@prop iri}` to a Markdown link.
fn resolve_prop_ref(iri: &str, index: &OntologyIndex) -> String {
    if let Some(prop) = index.properties.iter().find(|p| p.id == iri) {
        let href = resolve_ref("prop", iri, index);
        format!("[{}]({})", prop.label, href)
    } else {
        format!("`{}`", iri)
    }
}

/// Resolves a `{@ind iri}` to a Markdown link.
fn resolve_ind_ref(iri: &str, index: &OntologyIndex) -> String {
    if let Some(ind) = index.individuals.iter().find(|i| i.id == iri) {
        let href = resolve_ref("ind", iri, index);
        format!("[{}]({})", ind.label, href)
    } else {
        format!("`{}`", iri)
    }
}

/// Converts Markdown to HTML using pulldown-cmark.
pub fn markdown_to_html(markdown: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(markdown, opts);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Renders a page inside the standard HTML shell with nav, main, and footer.
pub fn render_page(
    title: &str,
    content_html: &str,
    nav_html: &str,
    breadcrumb: &str,
) -> String {
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
<nav aria-label="Site navigation">
{nav_html}
</nav>
<main id="main-content">
<nav aria-label="Breadcrumb" class="breadcrumb">{breadcrumb}</nav>
<article>
{content_html}
</article>
</main>
<footer>
<p>UOR Foundation — <a href="https://uor.foundation/">uor.foundation</a></p>
</footer>
<script src="/js/search.js" defer></script>
</body>
</html>"##,
        title = escape_html(title),
        nav_html = nav_html,
        breadcrumb = breadcrumb,
        content_html = content_html,
    )
}

/// Escapes HTML special characters in a string.
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
