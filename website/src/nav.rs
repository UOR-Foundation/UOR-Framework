//! Navigation tree builder.

use uor_spec::Ontology;

use crate::model::NavItem;

/// Builds the primary site navigation tree.
pub fn build_nav() -> Vec<NavItem> {
    let ontology = Ontology::full();

    let namespace_children: Vec<NavItem> = ontology
        .namespaces
        .iter()
        .map(|m| NavItem {
            label: m.namespace.label.to_string(),
            url: format!("/namespaces/{}/", m.namespace.prefix),
            children: Vec::new(),
        })
        .collect();

    vec![
        NavItem {
            label: "Home".to_string(),
            url: "/".to_string(),
            children: Vec::new(),
        },
        NavItem {
            label: "Namespaces".to_string(),
            url: "/namespaces/".to_string(),
            children: namespace_children,
        },
        NavItem {
            label: "Documentation".to_string(),
            url: "/docs/".to_string(),
            children: vec![
                NavItem {
                    label: "Overview".to_string(),
                    url: "/docs/overview.html".to_string(),
                    children: Vec::new(),
                },
                NavItem {
                    label: "Architecture".to_string(),
                    url: "/docs/architecture.html".to_string(),
                    children: Vec::new(),
                },
                NavItem {
                    label: "Concepts".to_string(),
                    url: "/docs/concepts/".to_string(),
                    children: Vec::new(),
                },
                NavItem {
                    label: "Guides".to_string(),
                    url: "/docs/guides/".to_string(),
                    children: Vec::new(),
                },
            ],
        },
        NavItem {
            label: "Search".to_string(),
            url: "/search.html".to_string(),
            children: Vec::new(),
        },
    ]
}

/// Renders the navigation tree as an HTML string.
pub fn render_nav(nav: &[NavItem], current_path: &str) -> String {
    let mut html = String::from("<ul>\n");
    for item in nav {
        render_nav_item(&mut html, item, current_path, 1);
    }
    html.push_str("</ul>\n");
    html
}

/// Recursively renders a navigation item.
fn render_nav_item(html: &mut String, item: &NavItem, current_path: &str, depth: usize) {
    let is_current = !item.url.is_empty() && current_path.starts_with(&item.url);
    let class = if is_current { " class=\"current\"" } else { "" };

    if item.url.is_empty() || item.children.is_empty() {
        if item.url.is_empty() {
            html.push_str(&format!(
                "{}<li{class}><span>{label}</span>",
                "  ".repeat(depth),
                class = class,
                label = escape_html(&item.label)
            ));
        } else {
            html.push_str(&format!(
                "{}<li{class}><a href=\"{url}\">{label}</a>",
                "  ".repeat(depth),
                class = class,
                url = escape_html(&item.url),
                label = escape_html(&item.label)
            ));
        }
    } else {
        html.push_str(&format!(
            "{}<li{class}><a href=\"{url}\">{label}</a>\n{}<ul>\n",
            "  ".repeat(depth),
            "  ".repeat(depth),
            class = class,
            url = escape_html(&item.url),
            label = escape_html(&item.label),
        ));
        for child in &item.children {
            render_nav_item(html, child, current_path, depth + 1);
        }
        html.push_str(&format!("{}</ul>\n", "  ".repeat(depth)));
    }

    html.push_str("</li>\n");
}

/// Escapes HTML special characters.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
