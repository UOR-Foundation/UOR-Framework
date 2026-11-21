//! Phase 7d test: every Path-4 theory-deferred class has a Null stub in
//! the generated source carrying the exact `#[doc(hidden)]` +
//! THEORY-DEFERRED banner combination. Missing or drifted banner fails.

use std::fs;
use std::path::PathBuf;

use uor_codegen::classification::{classify_all, PathKind};
use uor_ontology::Ontology;

const BANNER_MARKER: &str =
    "THEORY-DEFERRED \\u{2014} not a valid implementation; see [docs/theory_deferred.md].";

fn find_workspace_root() -> PathBuf {
    let mut dir = std::env::current_dir().expect("cwd");
    loop {
        if dir.join("foundation/src/enforcement.rs").exists() {
            return dir;
        }
        dir = match dir.parent() {
            Some(p) => p.to_path_buf(),
            None => panic!("no workspace root"),
        };
    }
}

/// Load the generated source file for the class's ontology namespace.
///
/// Local names are not globally unique (e.g. stream:Epoch and
/// reduction:Epoch), so a repository-wide text search can inspect
/// an unrelated Null stub and report a false missing-banner failure.
fn load_namespace_source(namespace: &str) -> Option<String> {
    let root = find_workspace_root().join("foundation/src");
    for subdir in ["bridge", "kernel", "user"] {
        let candidate = root.join(subdir).join(format!("{namespace}.rs"));
        if candidate.is_file() {
            return fs::read_to_string(candidate).ok();
        }
    }
    None
}

#[test]
fn every_path4_class_has_null_stub_with_banner() {
    let ontology = Ontology::full();
    let entries = classify_all(ontology);
    let path4: Vec<(&str, &str)> = entries
        .iter()
        .filter(|e| matches!(e.path_kind, PathKind::Path4TheoryDeferred))
        .map(|e| (e.namespace, e.class_local))
        .collect();

    assert!(
        !path4.is_empty(),
        "Path-4 class list is empty — classifier regression"
    );

    let mut missing_stub: Vec<String> = Vec::new();
    let mut missing_banner: Vec<String> = Vec::new();
    for &(namespace, name) in &path4 {
        let qualified = format!("{namespace}:{name}");
        let Some(source) = load_namespace_source(namespace) else {
            missing_stub.push(qualified);
            continue;
        };
        let stub_decl = format!("pub struct Null{name}<H: HostTypes>");
        let stub_pos = match source.find(&stub_decl) {
            Some(p) => p,
            None => {
                missing_stub.push(qualified);
                continue;
            }
        };
        // The 400-character window preceding the declaration must contain
        // both `#[doc(hidden)]` and the THEORY-DEFERRED marker string.
        let lookback = stub_pos.saturating_sub(400);
        let window = &source[lookback..stub_pos];
        if !window.contains("#[doc(hidden)]") || !window.contains(BANNER_MARKER) {
            missing_banner.push(qualified);
        }
    }

    assert!(
        missing_stub.is_empty(),
        "Path-4 classes with no generated Null stub: {missing_stub:?}"
    );
    assert!(
        missing_banner.is_empty(),
        "Path-4 Null stubs missing the `#[doc(hidden)]` / THEORY-DEFERRED \
         banner: {missing_banner:?}"
    );
}
