//! Navigation tree builder.

use uor_ontology::Ontology;

use crate::model::NavItem;

/// Builds the primary site navigation tree.
pub fn build_nav(base_path: &str) -> Vec<NavItem> {
    use crate::pipeline::PRISM_STAGES;
    let ontology = Ontology::full();

    // Pipeline nav children: Define/Resolve/Certify stage links
    // Embeds "define", "resolve", "certify" in every page's nav (satisfies nav/prism-structure)
    let pipeline_children: Vec<NavItem> = PRISM_STAGES
        .iter()
        .map(|(name, section_id, _, _)| NavItem {
            label: name.to_string(),
            url: format!("{base_path}/pipeline/#{section_id}"),
            children: Vec::new(),
        })
        .collect();

    // Namespace nav children: space-grouped with separator items (url = "")
    let namespace_children = build_namespace_nav(ontology, base_path);

    vec![
        NavItem {
            label: "Home".to_string(),
            url: format!("{base_path}/"),
            children: Vec::new(),
        },
        NavItem {
            label: "Explore".to_string(),
            url: format!("{base_path}/explore/"),
            children: Vec::new(),
        },
        NavItem {
            label: "Pipeline".to_string(),
            url: format!("{base_path}/pipeline/"),
            children: pipeline_children,
        },
        NavItem {
            label: "Namespaces".to_string(),
            url: format!("{base_path}/namespaces/"),
            children: namespace_children,
        },
        NavItem {
            label: "Identities".to_string(),
            url: format!("{base_path}/identities/"),
            children: Vec::new(),
        },
        NavItem {
            label: "Concepts".to_string(),
            url: format!("{base_path}/concepts/"),
            children: Vec::new(),
        },
        NavItem {
            label: "Download".to_string(),
            url: format!("{base_path}/download/"),
            children: Vec::new(),
        },
        NavItem {
            label: "Documentation".to_string(),
            url: format!("{base_path}/docs/"),
            children: vec![
                NavItem {
                    label: "Overview".to_string(),
                    url: format!("{base_path}/docs/overview.html"),
                    children: Vec::new(),
                },
                NavItem {
                    label: "Architecture".to_string(),
                    url: format!("{base_path}/docs/architecture.html"),
                    children: Vec::new(),
                },
            ],
        },
        NavItem {
            label: "Search".to_string(),
            url: format!("{base_path}/search.html"),
            children: Vec::new(),
        },
    ]
}

/// Builds space-grouped namespace nav children.
///
/// Each space group starts with a group-label item (url = ""), followed by namespace items
/// in assembly order. The CSS class `nav-group-label` is applied by `render_nav_item`.
fn build_namespace_nav(ontology: &Ontology, base_path: &str) -> Vec<NavItem> {
    use uor_ontology::model::Space;
    let mut children = Vec::new();

    for (group_label, space) in &[
        ("Kernel", Space::Kernel),
        ("Bridge", Space::Bridge),
        ("User", Space::User),
    ] {
        // Group separator (url = "" signals a label, not a link)
        children.push(NavItem {
            label: group_label.to_string(),
            url: String::new(),
            children: Vec::new(),
        });

        for m in ontology
            .namespaces
            .iter()
            .filter(|m| &m.namespace.space == space)
        {
            children.push(NavItem {
                label: m.namespace.label.to_string(),
                url: format!("{base_path}/namespaces/{}/", m.namespace.prefix),
                children: Vec::new(),
            });
        }
    }

    children
}

/// Renders the navigation tree as an HTML string.
pub fn render_nav(nav: &[NavItem], current_path: &str) -> String {
    // Home is always the first item; its URL is the deployment root (e.g. "/" or
    // "/UOR-Framework/"). Prefix-matching against the root would mark Home as current
    // on every page, so it is used only as an exact-match guard in render_nav_item.
    let root_url = nav.first().map(|i| i.url.as_str()).unwrap_or("/");
    let mut html = String::from("<ul>\n");
    for item in nav {
        render_nav_item(&mut html, item, current_path, root_url, 1);
    }
    html.push_str("</ul>\n");
    html
}

/// Recursively renders a navigation item.
fn render_nav_item(
    html: &mut String,
    item: &NavItem,
    current_path: &str,
    root_url: &str,
    depth: usize,
) {
    // Exact match always marks an item current.
    // Prefix match marks parent items current when on a child page (e.g. Namespaces
    // highlighted when on /namespaces/op/, Concepts when on /concepts/ring.html),
    // but is suppressed for the root URL to prevent Home from matching every page.
    let is_current = !item.url.is_empty()
        && (current_path == item.url
            || (item.url != root_url && current_path.starts_with(&item.url)));
    let class = if is_current { " class=\"current\"" } else { "" };
    let indent = "  ".repeat(depth);

    if item.url.is_empty() && item.children.is_empty() {
        // Group label — not a link, styled as a separator
        html.push_str(&format!(
            "{indent}<li><span class=\"nav-group-label\">{label}</span></li>\n",
            label = escape_html(&item.label)
        ));
    } else if item.children.is_empty() {
        html.push_str(&format!(
            "{indent}<li{class}><a href=\"{url}\">{label}</a></li>\n",
            url = escape_html(&item.url),
            label = escape_html(&item.label)
        ));
    } else {
        html.push_str(&format!(
            "{indent}<li{class}><a href=\"{url}\">{label}</a>\n\
             {indent}<ul>\n",
            url = escape_html(&item.url),
            label = escape_html(&item.label),
        ));
        for child in &item.children {
            render_nav_item(html, child, current_path, root_url, depth + 1);
        }
        html.push_str(&format!("{indent}</ul>\n{indent}</li>\n"));
    }
}

/// Escapes HTML special characters.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
