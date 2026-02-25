# UOR Integration Guide

**How to use UOR in your domain**

---

## Overview

UOR is a standalone execution layer. Your domain imports UOR; UOR knows nothing about your domain. This dependency inversion ensures UOR remains universal.

For the conceptual foundation, see [FRAMEWORK.md](FRAMEWORK.md). For the formal execution model, see [CELLULAR_AUTOMATA.md](CELLULAR_AUTOMATA.md). For x86_64 implementation details, see [ARCHITECTURE.md](ARCHITECTURE.md).

---

## Quick Start

### Adding UOR to Your Project

```toml
# Cargo.toml
[dependencies]
uor = { path = "../uor" }  # or from crates.io when published
```

### Basic Usage - Taxon Identity

```rust
use uor::{Taxon, Domain};

// Create a taxon (a byte with identity)
let t = Taxon::new(42);

// Access identity properties
assert_eq!(t.value(), 42);
assert_eq!(t.codepoint(), 0x282A);
assert_eq!(t.braille(), '⠪');

// Domain classification (from T=3)
assert_eq!(t.domain(), Domain::Theta);  // 42 % 3 == 0
assert_eq!(t.rank(), 14);                // 42 / 3 == 14
```

### Basic Usage - Wavefront Execution

```rust
use uor::prelude::*;

// Create the executor (requires AVX2, SHA-NI, AES-NI)
let executor = Zen3Executor::new();

// Create zero-initialized state (624 taxons = 4992 bits)
let mut state = UorState::zero();

// Execute a single wavefront (all ports fire simultaneously)
unsafe { executor.step(&mut state, &Wavefront::all_xor()); }
```

---

## Implementing Addressable

The `Addressable` trait allows your types to integrate with UOR:

```rust
use uor::{Taxon, Addressable};

/// Your custom type
pub struct MyHash([u8; 32]);

impl Addressable for MyHash {
    const TAXON_COUNT: usize = 32;

    fn to_taxons(&self) -> impl Iterator<Item = Taxon> {
        self.0.iter().map(|&b| Taxon::new(b))
    }

    fn from_taxons(taxons: impl Iterator<Item = Taxon>) -> Option<Self> {
        let mut bytes = [0u8; 32];
        let mut count = 0;

        for (i, t) in taxons.enumerate() {
            if i >= 32 { return None; }
            bytes[i] = t.value();
            count += 1;
        }

        if count == 32 { Some(MyHash(bytes)) } else { None }
    }
}

// Now your type has UOR addressing
fn use_my_hash() {
    let hash = MyHash([0u8; 32]);

    // Every byte has a stable IRI
    for taxon in hash.to_taxons() {
        println!("{}", taxon.iri_string());
        // https://uor.foundation/u/U2800
    }
}
```

---

## Using Word Types

### Word Type Selection

| Type | Size | Use Case |
|------|------|----------|
| `Word2` | 2 bytes | u16 container |
| `Word4` | 4 bytes | u32 container, SHA-256 words |
| `Word8` | 8 bytes | u64 container |
| `Word32` | 32 bytes | SHA-256 hashes |
| `Word<N>` | N bytes | Custom sizes |

### Creating Words

```rust
use uor::{Word4, Word8, Taxon};

// From primitive types
let w1 = Word4::from(0x12345678u32);
let w2 = Word8::from(0x123456789ABCDEFu64);

// From byte arrays
let w3 = Word4::from_bytes([0x12, 0x34, 0x56, 0x78]);

// From taxon arrays
let w4 = Word4::new([
    Taxon::new(0x12),
    Taxon::new(0x34),
    Taxon::new(0x56),
    Taxon::new(0x78),
]);

// Converting back
let value: u32 = w1.into();
let bytes: [u8; 4] = w1.to_bytes();
```

**Note:** Word types are data containers. Operations occur at the wavefront level on UorState.

---

## Building Wavefront Programs

### Using ProgramBuilder

```rust
use uor::prelude::*;

// Build a program from wavefront patterns
let program = ProgramBuilder::new()
    .push(bitwise::xor())              // Single XOR wavefront
    .repeat(rotate::right(7), 3)       // 3 right rotations
    .extend(sha256::big_sigma0())      // SHA-256 Σ₀ pattern (3 wavefronts)
    .build();

assert_eq!(program.len(), 7);

// Execute with fused registers (~1 cycle/wavefront)
let executor = Zen3Executor::new();
let mut state = UorState::zero();
unsafe { executor.run_fused(&mut state, &program); }
```

### Pre-built Patterns

```rust
use uor::wavefront::{sha256, aes, bitwise, arith, rotate};

// SHA-256 patterns
let sigma0 = sha256::big_sigma0();     // Σ₀(a) = ROTR²⊕ROTR¹³⊕ROTR²²
let sigma1 = sha256::big_sigma1();     // Σ₁(e) = ROTR⁶⊕ROTR¹¹⊕ROTR²⁵
let ch = sha256::ch();                 // Ch(e,f,g) = (e∧f)⊕(¬e∧g)
let maj = sha256::maj();               // Maj(a,b,c) = (a∧b)⊕(a∧c)⊕(b∧c)
let sha_ni = sha256::round_sha_ni();   // Hardware round (2 rounds per wavefront)

// AES patterns
let aes_enc = aes::enc_round();        // Single AES encryption round
let aes_dec = aes::dec_round();        // Single AES decryption round
let aes128 = aes::aes128_encrypt();    // Full AES-128 (10 rounds)
let aes256 = aes::aes256_encrypt();    // Full AES-256 (14 rounds)

// Bitwise patterns
let xor = bitwise::xor();              // XOR all registers
let and = bitwise::and();              // AND all registers
let or = bitwise::or();                // OR all registers
let not = bitwise::not();              // NOT (complement)

// Arithmetic patterns
let add = arith::add();                // Wrapping addition
let sub = arith::sub();                // Wrapping subtraction

// Rotation patterns
let rotr = rotate::right(7);           // Rotate right by 7 bits
let rotl = rotate::left(13);           // Rotate left by 13 bits
```

### Complete Program Example

```rust
use uor::prelude::*;

// SHA-256 compression: 64 rounds via SHA-NI (2 rounds per wavefront)
let sha256_compress = sha256_compress_program();
assert_eq!(sha256_compress.len(), 32);

// AES-128 encryption: 10 rounds
let aes128 = aes128_encrypt_program();
assert_eq!(aes128.len(), 10);

// Execute
let executor = Zen3Executor::new();
let mut state = UorState::zero();
unsafe { executor.run_fused(&mut state, &sha256_compress); }
```

---

## Ring Operations

Taxons support ring arithmetic (mod 256):

```rust
use uor::{Taxon, ring};

let a = Taxon::new(3);
let b = Taxon::new(5);

// Multiplication (mod 256)
assert_eq!(ring::mul(a, b), Taxon::new(15));

// Power (mod 256)
assert_eq!(ring::pow(a, 4), Taxon::new(81));  // 3^4 = 81

// Multiplicative inverse (only exists for odd numbers)
assert_eq!(ring::mul_inverse(a), Some(Taxon::new(171)));  // 3 × 171 ≡ 1 (mod 256)

// Division via inverse
assert_eq!(ring::div(Taxon::new(15), a), Some(b));  // 15 / 3 = 5

// Rotation and shift
assert_eq!(ring::rotate_left(Taxon::new(1), 4), Taxon::new(16));
assert_eq!(ring::shr(Taxon::new(16), 4), Taxon::new(1));
```

---

## Domain Classification

Use triadic domains for semantic organization:

```rust
use uor::{Taxon, Domain};

fn classify_byte(value: u8) -> &'static str {
    let taxon = Taxon::new(value);
    match taxon.domain() {
        Domain::Theta => "Structure (mod 3 = 0)",
        Domain::Psi => "Unity (mod 3 = 1)",
        Domain::Delta => "Duality (mod 3 = 2)",
    }
}

fn group_by_domain(data: &[u8]) -> [Vec<Taxon>; 3] {
    let mut groups = [Vec::new(), Vec::new(), Vec::new()];

    for &byte in data {
        let taxon = Taxon::new(byte);
        groups[taxon.domain().residue() as usize].push(taxon);
    }

    groups
}
```

---

## IRI Integration

Use stable IRIs for semantic web applications:

```rust
use uor::Taxon;

fn generate_linked_data(data: &[u8]) -> String {
    let mut json = String::from(r#"{
  "@context": "https://uor.foundation/schema/uor.jsonld",
  "taxons": [
"#);

    for (i, &byte) in data.iter().enumerate() {
        let taxon = Taxon::new(byte);
        if i > 0 { json.push_str(",\n"); }
        json.push_str(&format!(
            r#"    {{"@id": "{}", "braille": "{}"}}"#,
            taxon.iri_string(),
            taxon.braille()
        ));
    }

    json.push_str("\n  ]\n}");
    json
}
```

---

## Testing Your Integration

### Roundtrip Tests

```rust
use uor::{Taxon, Addressable};

#[test]
fn test_addressable_roundtrip() {
    let original: [u8; 32] = [0x42; 32];
    let taxons: Vec<_> = original.to_taxons().collect();
    let restored = <[u8; 32]>::from_taxons(taxons.into_iter()).unwrap();
    assert_eq!(original, restored);
}
```

### Ring Operation Tests

```rust
use uor::{Taxon, ring};

#[test]
fn test_ring_inverse_roundtrip() {
    // All odd numbers should have inverses
    for i in (1..=255u8).step_by(2) {
        let t = Taxon::new(i);
        let inv = ring::mul_inverse(t).unwrap();
        assert_eq!(ring::mul(t, inv), Taxon::ONE);
    }
}
```

### Conformance Tests

```rust
use uor::conformance::{
    validate_wavefront_latency,
    validate_sequence_latency,
    validate_throughput,
};

#[test]
fn test_performance_conformance() {
    // Single wavefront must be < 5 cycles
    assert!(validate_wavefront_latency(3).is_ok());

    // 64 wavefronts must be < 200 cycles
    assert!(validate_sequence_latency(180).is_ok());

    // Throughput must be >= 512 bits/cycle
    // 4992 bits / 5 cycles = 998 bits/cycle
    assert!(validate_throughput(5).is_ok());
}
```

---

## Best Practices

### Do

- Use `Addressable` trait for semantic web integration
- Use `ProgramBuilder` to compose wavefront sequences
- Use pre-built patterns for SHA-256 and AES
- Use fused execution (`run_fused`) for best performance
- Test against conformance targets

### Don't

- Don't expect operations on individual Taxon or Word types (they're just data)
- Don't use UOR on platforms without AVX2/SHA-NI/AES-NI (no fallback)
- Don't assume memory operations during wavefront execution (there are none)
- Don't modify state during wavefront execution from other threads

---

## Example: SHA-256 with Hardware Acceleration

```rust
use uor::prelude::*;

/// Execute SHA-256 compression using hardware SHA-NI.
///
/// State should be pre-loaded with:
/// - ymm0-1: current hash state (A-H)
/// - ymm2-5: message schedule (W[0..15])
pub unsafe fn sha256_compress_hw(
    executor: &Zen3Executor,
    state: &mut UorState,
) {
    // 32 wavefronts = 64 SHA-256 rounds (2 rounds per sha256rnds2)
    let program = sha256_compress_program();
    executor.run_fused(state, &program);
}

/// Full SHA-256 computation.
pub fn sha256_via_uor(message: &[u8]) -> [u8; 32] {
    let executor = Zen3Executor::new();
    let mut state = UorState::zero();

    // Initialize state with H[0..7] constants
    // (loading omitted for brevity)

    // Process each 64-byte block
    for block in message.chunks(64) {
        // Load message schedule into state
        // (loading omitted for brevity)

        // Compress
        unsafe { sha256_compress_hw(&executor, &mut state); }
    }

    // Extract final hash from state
    let mut result = [0u8; 32];
    // (extraction omitted for brevity)
    result
}
```

---

## Summary

1. **Import UOR** - add dependency to your project
2. **Use Taxon identity** - braille, domain, rank, IRI
3. **Implement Addressable** - enable semantic web integration
4. **Build wavefront programs** - compose pre-built patterns
5. **Execute with fusion** - use `run_fused` for ~1 cycle/wavefront
6. **Test conformance** - verify performance targets are met

---

## Related Documentation

| Document | Scope |
|----------|-------|
| [FRAMEWORK.md](FRAMEWORK.md) | UOR Framework conceptual foundation |
| [CELLULAR_AUTOMATA.md](CELLULAR_AUTOMATA.md) | Formal execution model and guarantees |
| [ARCHITECTURE.md](ARCHITECTURE.md) | x86_64 backend specifics |
| [PERFORMANCE.md](PERFORMANCE.md) | Benchmarks and conformance targets |
