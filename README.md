![UOR Framework provisional banner: ontology, generation, and conformance. Placeholder not approved.](assets/readme-banner-PLACEHOLDER.png)

*Illustrative placeholder artwork, pending UOR Foundation brand approval.*

# UOR Framework

**A shared way to identify digital objects, explain their relationships, and check results.**

[![CI](https://github.com/UOR-Foundation/uor-framework/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/UOR-Foundation/uor-framework/actions/workflows/ci.yml)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.19068826.svg)](https://doi.org/10.5281/zenodo.19068826)
[![License: MIT](https://img.shields.io/badge/license-MIT-718096)](LICENSE)

## Why UOR exists

The same digital information can have different names, locations, and descriptions across software systems. That makes it harder to recognize when two things are the same, understand how they are related, or check how a result was produced.

**UOR is an effort to give digital objects a more consistent mathematical foundation.** It defines shared rules for identifying objects by their content, describing relationships, and recording how information is processed.

For example, two systems might store the same content under different file names. A content-based identifier focuses on the content rather than the file name. UOR builds on that idea with a structured vocabulary for describing objects and their relationships.

## What this repository provides

This repository implements the [UOR Foundation](https://uor.foundation/) framework in Rust. It includes a shared vocabulary (called an **ontology**), tools that generate software interfaces and machine-readable descriptions, and checks that help keep those outputs consistent.

| What you need | Where to start |
| --- | --- |
| Understand the ideas without digging into source code | [Framework overview](docs/content/overview.md) |
| Explore the mathematical definitions | [Developer reference](docs/DEVELOPER_REFERENCE.md) |
| Use the Rust interfaces | [Foundation crate](foundation/) and [consumer example](examples/consumer-smoke/) |
| Build a compatible resolver | [Implementation guide](docs/content/guides/implementing-prism.md) |
| Check an implementation against documented requirements | [Conformance guide](docs/content/guides/conformance.md) |

UOR defines a foundation and provides tooling for it. **It does not mean every other system already supports UOR, or that every use of the framework has been formally verified.** Verification claims are limited to the checks and evidence actually produced.

## Get started

To work from the source, install Git and the Rust toolchain described in [`rust-toolchain.toml`](rust-toolchain.toml). Then run:

```sh
git clone https://github.com/UOR-Foundation/uor-framework.git
cd uor-framework
cargo run --bin uor-build
cargo run --bin uor-conformance
```

The first command generates the framework's machine-readable artifacts. The second runs its conformance checks. For more build options, generated formats, and the exact technical scope, see the [developer reference](docs/DEVELOPER_REFERENCE.md).

## Contents

- [Why UOR exists](#why-uor-exists)
- [What this repository provides](#what-this-repository-provides)
- [Get started](#get-started)
- [How the pieces fit together](#how-the-pieces-fit-together)
- [Documentation and evidence](#documentation-and-evidence)
- [Citation and license](#citation-and-license)

## How the pieces fit together

1. **Define:** The [ontology source](spec/) describes the framework's concepts and rules.
2. **Generate:** The [code generator](codegen/) turns those definitions into Rust interfaces in [`foundation/`](foundation/) and other formats.
3. **Explain:** The [documentation generator](docs/) produces technical references and guides.
4. **Check:** The [conformance suite](conformance/) examines the generated outputs and supported requirements.

Advanced readers can explore the algebraic structure, including modular arithmetic, in the [technical overview](docs/content/overview.md). Those details matter to the implementation, but you do not need them to understand the project's purpose.

## Documentation and evidence

The [framework wiki](https://github.com/UOR-Foundation/UOR-Framework/wiki) records normative architecture and design decisions. The [developer reference](docs/DEVELOPER_REFERENCE.md) explains the repository's generated interfaces, formats, and development commands. The [conformance guide](docs/content/guides/conformance.md) describes what the validation tools check.

A passing test or build means the checks that ran passed. It is not a blanket guarantee about every possible application built with UOR.

## Citation and license

For academic use, see [`CITATION.cff`](CITATION.cff). This repository uses the [MIT license](LICENSE).

*This public-facing README is intentionally maintained by people. The technical reference is generated separately, so documentation builds do not overwrite this introduction.*
