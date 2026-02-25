# uor-foundation

The complete [UOR Foundation](https://uor.foundation/) ontology encoded as
typed Rust traits. Import and implement.

## Contents

- 14 namespaces
- 98 OWL classes (one trait each)
- 167 OWL properties (one method each)
- 18 named individuals (constants and enums)

## Quick start

```toml
[dependencies]
uor-foundation = "2.0.0"
```

```rust
use uor_foundation::Primitives;

struct MyImpl;
impl Primitives for MyImpl {
    type String = str;
    type Integer = i64;
    type NonNegativeInteger = u64;
    type PositiveInteger = u64;
    type Decimal = f64;
    type Boolean = bool;
}
```

Then implement any foundation trait with your chosen primitives:

```rust,ignore
use uor_foundation::bridge::partition::FiberBudget;

impl FiberBudget<MyImpl> for MyFiberBudget {
    // ...
}
```

## Module structure

| Module | Space | Description |
|--------|-------|-------------|
| `kernel::address` | Kernel | Content-addressed glyph space |
| `kernel::schema` | Kernel | Ring schema: Datum, Term, Ring |
| `kernel::op` | Kernel | Primitive operations and the dihedral group |
| `bridge::query` | Bridge | Query hierarchy |
| `bridge::resolver` | Bridge | Resolution state machine |
| `bridge::partition` | Bridge | Irreducibility partitions and fiber budgets |
| `bridge::observable` | Bridge | Observable measurements |
| `bridge::proof` | Bridge | Proof and witness data |
| `bridge::derivation` | Bridge | Derivation and rewrite steps |
| `bridge::trace` | Bridge | Computation traces |
| `bridge::cert` | Bridge | Certificate hierarchy |
| `user::type_` | User | Type definitions and constraints |
| `user::morphism` | User | Transforms and composition laws |
| `user::state` | User | Context, bindings, frames, transitions |

## Features

This crate is `#![no_std]` with zero mandatory dependencies.

## License

Apache-2.0 — see [LICENSE](https://github.com/UOR-Foundation/UOR-Framework/blob/main/LICENSE).
