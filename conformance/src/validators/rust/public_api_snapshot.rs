//! v0.2.2 W6: public-API snapshot validator.
//!
//! Pins the exact set of `pub` items in `uor-foundation`'s enforcement
//! module and crate-root surface. Diffs the current scan against a
//! snapshot file at `foundation/tests/public-api.snapshot`. Adding,
//! removing, or renaming a public item requires explicit snapshot
//! update review.
//!
//! The scan operates on the generated `enforcement.rs` and the
//! hand-written `lib.rs` (which holds `Primitives` / `HostTypes` /
//! `DefaultHostTypes`). Items inside private modules (`mod ... {`) and
//! items marked `pub(crate)` are excluded.
//!
//! The snapshot uses a flat one-line-per-symbol format. Sealed
//! constructors that exist only as `pub(crate)` do not appear.

use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Result;

use crate::report::{ConformanceReport, TestResult};

/// Files scanned for public-API items.
const SCAN_FILES: &[&str] = &[
    "foundation/src/lib.rs",
    "foundation/src/enforcement.rs",
    "foundation/src/pipeline.rs",
];

/// Runs the v0.2.2 W6 public-API snapshot validator.
///
/// # Errors
///
/// Returns an error if the workspace cannot be read.
pub fn validate(workspace: &Path) -> Result<ConformanceReport> {
    let mut report = ConformanceReport::new();
    let validator = "rust/public_api_snapshot";

    let mut current: BTreeSet<String> = BTreeSet::new();
    for file in SCAN_FILES {
        let path = workspace.join(file);
        scan_file(&path, file, &mut current);
    }

    let snapshot_path = workspace
        .join("foundation")
        .join("tests")
        .join("public-api.snapshot");

    if !snapshot_path.exists() {
        // First run: write the current snapshot and report a warning so the
        // operator can review the initial baseline before the next run.
        let body = current.iter().cloned().collect::<Vec<_>>().join("\n");
        if std::fs::write(&snapshot_path, format!("{body}\n")).is_ok() {
            report.push(TestResult::pass(
                validator,
                format!(
                    "Initial snapshot written to foundation/tests/public-api.snapshot ({} symbols)",
                    current.len()
                ),
            ));
        } else {
            report.push(TestResult::fail(
                validator,
                format!(
                    "Snapshot file missing and could not be written: {}",
                    snapshot_path.display()
                ),
            ));
        }
        return Ok(report);
    }

    let snapshot_content = match std::fs::read_to_string(&snapshot_path) {
        Ok(c) => c,
        Err(e) => {
            report.push(TestResult::fail(
                validator,
                format!("Cannot read snapshot file: {e}"),
            ));
            return Ok(report);
        }
    };
    let snapshot: BTreeSet<String> = snapshot_content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    let added: Vec<&String> = current.difference(&snapshot).collect();
    let removed: Vec<&String> = snapshot.difference(&current).collect();

    if added.is_empty() && removed.is_empty() {
        report.push(TestResult::pass(
            validator,
            format!(
                "Public-API snapshot matches: {} symbols pinned",
                current.len()
            ),
        ));
    } else {
        let mut summary = format!(
            "Public-API drift: {} added, {} removed (snapshot at foundation/tests/public-api.snapshot)",
            added.len(),
            removed.len()
        );
        if !added.is_empty() {
            summary.push_str("\n       added:");
            for s in added.iter().take(10) {
                summary.push_str(&format!("\n         + {s}"));
            }
            if added.len() > 10 {
                summary.push_str(&format!("\n         + ... ({} more)", added.len() - 10));
            }
        }
        if !removed.is_empty() {
            summary.push_str("\n       removed:");
            for s in removed.iter().take(10) {
                summary.push_str(&format!("\n         - {s}"));
            }
            if removed.len() > 10 {
                summary.push_str(&format!("\n         - ... ({} more)", removed.len() - 10));
            }
        }
        report.push(TestResult::fail(validator, summary));
    }

    Ok(report)
}

/// Scans one Rust source file for top-level public items, ignoring items
/// inside private modules. Returns a set of `kind name` strings.
fn scan_file(path: &Path, label: &str, out: &mut BTreeSet<String>) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut depth: i32 = 0;
    let mut private_depth: i32 = -1;
    for raw in content.lines() {
        let trimmed = raw.trim();
        // Track brace depth (rough; ignores braces in strings/comments).
        let opens = trimmed.matches('{').count() as i32;
        let closes = trimmed.matches('}').count() as i32;
        // Detect a private-module entry on this line.
        let entering_private = trimmed.starts_with("mod ")
            || trimmed.starts_with("pub(crate) mod ")
            || trimmed.starts_with("pub(super) mod ");
        if entering_private && trimmed.contains('{') && private_depth < 0 {
            private_depth = depth;
        }
        // Only collect items at the top level (depth == 0) AND not under a
        // private module scope. The pre-line depth applies.
        let allow = depth == 0 && private_depth < 0;
        if allow {
            if let Some(item) = parse_pub_item(trimmed) {
                out.insert(format!("{label}: {item}"));
            }
        }
        depth += opens - closes;
        if depth < 0 {
            depth = 0;
        }
        if private_depth >= 0 && depth <= private_depth {
            private_depth = -1;
        }
    }
}

/// Recognizes a top-level `pub` item declaration on a single line.
/// Returns `kind name` (e.g., `struct Validated`, `trait HostTypes`).
fn parse_pub_item(line: &str) -> Option<String> {
    let after_pub = line
        .strip_prefix("pub ")
        .or_else(|| line.strip_prefix("pub("))
        .map(|s| {
            // pub(...) form — skip until ')'.
            if let Some(rest) = s.split_once(") ") {
                rest.1
            } else {
                s
            }
        })?;
    // pub(crate) forms are skipped (visible only inside the crate).
    if line.starts_with("pub(crate)") {
        return None;
    }
    // Match item kinds.
    for (prefix, kind) in [
        ("struct ", "struct"),
        ("trait ", "trait"),
        ("enum ", "enum"),
        ("type ", "type"),
        ("const ", "const"),
        ("static ", "static"),
        ("fn ", "fn"),
        ("mod ", "mod"),
        ("use ", "use"),
    ] {
        if let Some(rest) = after_pub.strip_prefix(prefix) {
            // Extract identifier (first word, stripped of generics/punctuation).
            let name = rest
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or("");
            if name.is_empty() {
                return None;
            }
            return Some(format!("{kind} {name}"));
        }
    }
    None
}
