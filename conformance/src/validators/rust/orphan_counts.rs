//! Phase 7e conformance check (minimum-viable): count how many generated
//! traits have zero `impl` sites in the workspace.
//!
//! Algorithm (per §Phase 7e of docs/orphan-closure/completion-plan.md):
//!
//! 1. Parse every `pub trait {Name}<H: HostTypes>` declaration across
//!    `foundation/src/**/*.rs`.
//! 2. For each trait, search the workspace for the impl regex
//!    `^\s*impl(<[^>]*>)?\s+(crate::)?([\w_]+::)*{Name}(<[^>]*>)?\s+for\s+`
//!    excluding lines inside `#[cfg(test)]` blocks.
//! 3. Zero impl matches ⇒ orphan.
//! 4. Pass ↔ orphan count ≤ `MAX_PERMITTED_ORPHANS` (the Path-4
//!    theory-deferred count, each of which now has a Phase-7d
//!    `#[doc(hidden)]` stub).
//!
//! Phase 13a will replace this with the advanced classifier-integrated
//! version that also cross-checks impl targets against Phase-0
//! classifications.

use std::path::{Path, PathBuf};

use anyhow::Result;
use regex::Regex;

use crate::report::{ConformanceReport, TestResult};

const VALIDATOR: &str = "rust/orphan_counts";

/// Permitted orphan count: matches the Path-4 theory-deferred class
/// count (see `spec/src/counts.rs::CLASSIFICATION_PATH4`). Each Path-4
/// class has a Phase-7d `#[doc(hidden)]` Null stub, but the stub IS an
/// impl — so Path-4 traits are NOT orphans. The ratchet is therefore
/// the full expected-closed count — Phase 7e asserts **zero** orphan
/// traits after the cascade unblockers.
///
/// Left as `usize` rather than `0` for defensive headroom: if a new
/// Path-4 class lands without its stub, the count goes up by one and
/// the test fails with a clear diff (rather than a hard zero-bound
/// off-by-one).
const MAX_PERMITTED_ORPHANS: usize = 0;

/// Runs the Phase 7e minimum-viable orphan-count validation.
///
/// # Errors
///
/// Returns an error if a workspace file cannot be read or the impl regex
/// fails to compile.
pub fn validate(workspace: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();

    // 1. Trait enumeration.
    let foundation_src = workspace.join("foundation/src");
    let mut trait_names: Vec<String> = Vec::new();
    collect_traits(&foundation_src, &mut trait_names)?;
    trait_names.sort();
    trait_names.dedup();

    if trait_names.is_empty() {
        report.push(TestResult::fail(
            VALIDATOR,
            "No `pub trait ... <H: HostTypes>` declarations found — \
             foundation regeneration regressed"
                .to_string(),
        ));
        return Ok(report);
    }

    // 2. Collect every candidate `.rs` file under the workspace.
    let search_roots = [
        workspace.join("foundation/src"),
        workspace.join("uor-foundation-sdk/src"),
        workspace.join("conformance/src"),
        workspace.join("uor-foundation-test-helpers/src"),
        workspace.join("uor-foundation-verify/src"),
        workspace.join("clients/src"),
        workspace.join("cargo-uor/src"),
    ];
    let mut sources: Vec<String> = Vec::new();
    for root in &search_roots {
        collect_source_text(root, &mut sources)?;
    }
    let cleaned: Vec<String> = sources.iter().map(|s| strip_cfg_test_blocks(s)).collect();

    // 3. For each trait, scan for impl sites.
    let mut orphans: Vec<String> = Vec::new();
    for name in &trait_names {
        let pattern = format!(
            r"(?m)^\s*impl(<[^>]*>)?\s+(crate::)?([\w_]+(::[\w_]+)*::)?{name}(<[^>]*>)?\s+for\s+",
        );
        let re = match Regex::new(&pattern) {
            Ok(r) => r,
            Err(e) => {
                report.push(TestResult::fail(
                    VALIDATOR,
                    format!("regex compile failed for {name}: {e}"),
                ));
                return Ok(report);
            }
        };
        let mut matched = false;
        for src in &cleaned {
            if re.is_match(src) {
                matched = true;
                break;
            }
        }
        if !matched {
            orphans.push(name.clone());
        }
    }

    #[allow(clippy::absurd_extreme_comparisons)]
    let within_budget = orphans.len() <= MAX_PERMITTED_ORPHANS;
    if within_budget {
        report.push(TestResult::pass(
            VALIDATOR,
            format!(
                "Orphan count: {} / {} traits closed (≤ {MAX_PERMITTED_ORPHANS} permitted)",
                trait_names.len() - orphans.len(),
                trait_names.len(),
            ),
        ));
    } else {
        let preview: Vec<&str> = orphans.iter().take(20).map(String::as_str).collect();
        report.push(TestResult::fail(
            VALIDATOR,
            format!(
                "Orphan count drift: {} orphan trait(s) (max permitted {MAX_PERMITTED_ORPHANS}). \
                 First {}: {:?}",
                orphans.len(),
                preview.len(),
                preview,
            ),
        ));
    }

    Ok(report)
}

fn collect_traits(root: &Path, out: &mut Vec<String>) -> Result<()> {
    if !root.exists() {
        return Ok(());
    }
    let re = Regex::new(r"(?m)^pub trait (\w+)<H: HostTypes>")?;
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|x| x == "rs") {
                let src = std::fs::read_to_string(&path)?;
                for cap in re.captures_iter(&src) {
                    if let Some(m) = cap.get(1) {
                        out.push(m.as_str().to_string());
                    }
                }
            }
        }
    }
    Ok(())
}

fn collect_source_text(root: &Path, out: &mut Vec<String>) -> Result<()> {
    if !root.exists() {
        return Ok(());
    }
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|x| x == "rs") {
                out.push(std::fs::read_to_string(&path)?);
            }
        }
    }
    Ok(())
}

/// Removes `#[cfg(test)] mod { ... }` blocks (brace-counted) from a
/// source string so impl sites inside test modules don't count as
/// closures of the main trait.
///
/// Works on byte slices but advances in full UTF-8 char boundaries so
/// multi-byte characters in doc comments don't panic.
fn strip_cfg_test_blocks(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut rest = src;
    loop {
        match rest.find("#[cfg(test)]") {
            None => {
                out.push_str(rest);
                break;
            }
            Some(pos) => {
                out.push_str(&rest[..pos]);
                let tail = &rest[pos + "#[cfg(test)]".len()..];
                // Find the opening `{` after the attribute.
                match tail.find('{') {
                    None => {
                        // Malformed input — bail, preserve tail verbatim.
                        out.push_str(&rest[pos..]);
                        break;
                    }
                    Some(brace_off) => {
                        // Walk chars from `brace_off` counting braces.
                        let mut depth: i32 = 0;
                        let mut closed_at: Option<usize> = None;
                        for (byte_idx, ch) in tail[brace_off..].char_indices() {
                            match ch {
                                '{' => depth += 1,
                                '}' => {
                                    depth -= 1;
                                    if depth == 0 {
                                        closed_at = Some(brace_off + byte_idx + ch.len_utf8());
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                        match closed_at {
                            Some(end) => {
                                rest = &tail[end..];
                            }
                            None => {
                                // Unmatched brace — bail.
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    out
}
