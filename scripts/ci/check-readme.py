#!/usr/bin/env python3
"""Fast, dependency-free checks for the human-curated public README.

This does not replace editorial review, generator drift validation, or the
full conformance suite. It only catches objective presentation regressions.
"""
from pathlib import Path
import re
import sys
from urllib.parse import unquote

root = Path(__file__).resolve().parents[2]
readme = root / "README.md"
text = readme.read_text(encoding="utf-8")
failures = []

required = (
    "## Why UOR exists",
    "## What this repository provides",
    "## Get started",
    "## Contents",
)
for heading in required:
    if heading not in text:
        failures.append(f"missing reader-facing section: {heading}")
if text.count("# UOR Framework\n") != 1:
    failures.append("expected one repository title")
if "Z/(2^n)Z" in text.split("## Why UOR exists", 1)[0]:
    failures.append("unexplained algebra in the first impression")
if "docs/DEVELOPER_REFERENCE.md" not in text:
    failures.append("missing link to generated developer reference")
if not (root / "docs/DEVELOPER_REFERENCE.md").is_file():
    failures.append("generated developer reference is missing")

# GitHub-compatible heading slugs for the ordinary ASCII headings used here.
headings = set()
for match in re.finditer(r"^#{1,6}\s+(.+)$", text, re.M):
    heading = re.sub(r"[^\w -]", "", match.group(1).lower())
    headings.add(re.sub(r"\s+", "-", heading.strip()))

for match in re.finditer(r"!?(?:\[[^\]]*\])\(([^)]+)\)", text):
    raw = match.group(1).strip().split(" ", 1)[0]
    if raw.startswith(("https://", "http://", "mailto:")):
        continue
    local, _, anchor = raw.partition("#")
    if not local:
        if anchor and anchor not in headings:
            failures.append(f"unknown table-of-contents anchor: #{anchor}")
        continue
    target = (root / unquote(local)).resolve()
    if not target.is_relative_to(root):
        failures.append(f"link escapes the repository: {local}")
    elif not target.exists():
        failures.append(f"local link target is missing: {local}")

if failures:
    for failure in failures:
        print(f"README check: {failure}", file=sys.stderr)
    sys.exit(1)
print("README check passed: purpose, navigation, generated reference and local assets")
