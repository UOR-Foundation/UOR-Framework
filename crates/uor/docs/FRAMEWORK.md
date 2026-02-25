# UOR Framework

**Universal Object Reference - A Framework for Clock-Coherent Type Execution**

---

## What is UOR?

UOR (Universal Object Reference) is a framework for mapping arbitrary types to address-spaces where operations execute in clock-coherent wavefronts. UOR provides:

1. **Type Maps**: Every type T has a map `T → UOR address-space`
2. **Operation Maps**: Every operation on T has a map `Op(T) → wavefront sequence`
3. **Arbitrary Scaling**: Address-spaces scale with type cardinality
4. **Clock Coherence**: All operations decompose to the processor's clock norm

UOR is not a specific implementation—it is an abstract model that can be realized across different modalities and hardware backends.

---

## The Universal Property

UOR's representation space scales arbitrarily to accommodate any type.

### Address-Space Scaling

| Type | Cardinality | Address-Space Size |
|------|-------------|-------------------|
| u8 | 2^8 | 256 elements |
| u16 | 2^16 | 65,536 elements |
| u32 | 2^32 | 4,294,967,296 elements |
| u64 | 2^64 | 18,446,744,073,709,551,616 elements |
| Arbitrary | 2^n | 2^n elements |

The address-space is not a runtime lookup table. UOR computes addresses at **compile time**, emitting wavefront sequences that execute at runtime with no address resolution overhead.

### The Scaling Guarantee

For any type T with cardinality |T|:
- UOR can represent all |T| values
- Each value has a unique identity in the address-space
- Operations on T decompose to a fixed number of wavefronts
- The wavefront count is determined at compile time

This is the universal property: UOR scales to any type while preserving clock-coherent execution.

---

## Fundamental Type Maps

UOR provides mappings from types to address-spaces. Each map establishes:
- **Identity**: Every value has a unique representation
- **Addressing**: Values can be referenced via IRIs
- **Domain Classification**: Values partition into triadic domains

### The Braille Bijection (Prototype Map)

The Braille bijection maps u8 to Unicode Braille characters:

```
u8 value (0-255) ↔ Unicode Braille (U+2800-U+28FF)
```

| Value | Codepoint | Glyph | IRI |
|-------|-----------|-------|-----|
| 0 | U+2800 | ⠀ | uor:U2800 |
| 1 | U+2801 | ⠁ | uor:U2801 |
| 42 | U+282A | ⠪ | uor:U282A |
| 255 | U+28FF | ⣿ | uor:U28FF |

Why Braille?
- **Bijective**: Exactly 256 codepoints for 256 byte values
- **Stable**: Unicode standard, semantically permanent
- **Visual**: Each value has a distinct glyph
- **Demonstrative**: Proves the pattern that extends to all types

The Braille map is the **prototype**—the first and clearest demonstration of UOR's mapping approach. All other type maps follow the same pattern.

### Extended Type Maps

UOR publishes fundamental maps for primitive types:

| Type | Map | Status |
|------|-----|--------|
| u8 | Braille bijection | Implemented |
| ASCII | 7-bit subset of Braille | Planned |
| Unicode | Full scalar value space | Planned |
| u16, u32, u64 | Extended address-spaces | Planned |
| i8, i16, i32, i64 | Signed variants | Planned |
| f32, f64 | IEEE 754 representations | Planned |
| Algebraic types | Composite address-spaces | Planned |

Each map enables any value of that type to:
1. Have a stable identity in UOR address-space
2. Be referenced via a unique IRI
3. Participate in clock-coherent operations

---

## Backend Abstraction

UOR is architecture-independent. The framework hooks into the parallelizable ports of any device through a **backend** abstraction.

### What is a Backend?

A backend is a driver that implements the UOR contract for a specific architecture. It declares:
- Available execution ports
- Port capabilities (which operations each port supports)
- Register/state width
- Native instruction mappings

### Supported and Planned Backends

| Backend | Ports | Status |
|---------|-------|--------|
| x86_64 | AVX2/SHA-NI/AES-NI (3 ports) | Implemented |
| aarch64 | NEON/SVE | Planned |
| RISC-V | Vector extension | Planned |
| GPU | Shader cores | Planned |
| TPU | Matrix units | Planned |

### The Backend Contract

A conforming backend must:

1. **Declare ports**: Count, width, and capabilities
2. **Map operations**: Translate UOR operations to native instructions
3. **Map state**: Translate UorState to backend-native registers
4. **Guarantee timing**: Each wavefront executes in exactly one clock cycle

The framework compiles type operations into backend-specific wavefront sequences. At runtime, execution is pure port operations with no UOR overhead.

---

## Multi-Modal Representations

UOR is not limited to computational execution. The framework supports multiple modalities:

### Computational Mode
- Wavefront execution on hardware ports
- Register-resident state
- Clock-coherent timing
- This implementation demonstrates computational mode

### Visual Mode
- Braille glyphs render values visually
- Domain colors (Theta/Psi/Delta) provide semantic grouping
- Transformations can be visualized as glyph animations

### Semantic Mode
- IRIs provide web-addressable identities
- JSON-LD graphs represent type relationships
- Linked data integration via standard protocols

### Future Modalities
- Audio: Frequency/harmonic mappings
- Spatial: 3D address-space projections
- Tactile: Physical Braille rendering

The UOR Framework defines the abstract model; modalities are instantiations of that model in different representational spaces.

---

## Axiom Derivation

All UOR constants derive from two foundational axioms:

| Axiom | Value | Meaning |
|-------|-------|---------|
| **T** | 3 | Triality - prime order of symmetry |
| **O** | 8 | Octonion dimension |

### Derived Constants

| Constant | Formula | Value | Purpose |
|----------|---------|-------|---------|
| BYTE_CARDINALITY | 2^O | 256 | Taxon count (u8 address-space) |
| DOMAIN_COUNT | T | 3 | Number of triadic domains |
| BASIS_ELEMENTS | O | 8 | Binary decomposition basis |

### Triadic Domains

From T=3, every value partitions into one of three domains:

| Domain | Symbol | Residue | Meaning |
|--------|--------|---------|---------|
| Theta | θ | n mod 3 = 0 | Structure |
| Psi | ψ | n mod 3 = 1 | Unity |
| Delta | δ | n mod 3 = 2 | Duality |

Domain classification provides semantic structure independent of the specific type map.

---

## Relationship to Implementation

This document describes the **UOR Framework**—the abstract model.

The accompanying documents describe **implementations**:

| Document | Scope |
|----------|-------|
| [CELLULAR_AUTOMATA.md](CELLULAR_AUTOMATA.md) | Formal execution model |
| [ARCHITECTURE.md](ARCHITECTURE.md) | x86_64 backend specifics |
| [INTEGRATION.md](INTEGRATION.md) | Usage patterns |
| [PERFORMANCE.md](PERFORMANCE.md) | Benchmarks and conformance |

The framework is permanent; implementations evolve as new backends and modalities are added.

---

## Summary

UOR is a framework for mapping types to clock-coherent execution:

1. **Universal**: Any type maps to UOR address-space
2. **Scalable**: Address-spaces grow with type cardinality
3. **Coherent**: Operations decompose to clock cycles
4. **Abstract**: Independent of specific hardware or modality
5. **Extensible**: New backends and modalities can be added

The Braille bijection demonstrates UOR for u8. The same pattern extends to all types, all backends, and all modalities.
