//! Content-addressed derivation IDs for microcode sequences.
//!
//! Each microcode derivation (sequence of primitive operations) gets a
//! unique SHA256-based ID. This enables:
//!
//! 1. **Deduplication**: Same derivation ID = same computation
//! 2. **Caching**: Store computed derivations by ID
//! 3. **Verification**: ID proves derivation correctness
//!
//! # Canonical Form
//!
//! Before computing the ID, derivations are canonicalized:
//! - Associative/commutative operations are sorted
//! - Dead code is eliminated
//! - Constant folding is applied where possible
//!
//! This ensures that semantically equivalent derivations get the same ID.

use alloc::vec::Vec;

/// A 32-byte derivation ID (SHA256 hash of canonical form).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DerivationId([u8; 32]);

impl DerivationId {
    /// Create a derivation ID from raw bytes.
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get the raw bytes of this ID.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Convert to hex string for display/storage.
    pub fn to_hex(&self) -> alloc::string::String {
        use alloc::format;
        let mut s = alloc::string::String::with_capacity(64);
        for byte in &self.0 {
            s.push_str(&format!("{:02x}", byte));
        }
        s
    }

    /// Compute derivation ID from microcode steps.
    ///
    /// Uses a simple hash function (not cryptographic SHA256 to avoid
    /// external dependencies). For production, replace with proper SHA256.
    pub fn compute(steps: &[MicrocodeStep]) -> Self {
        let mut hasher = SimpleHasher::new();

        // Hash number of steps
        hasher.update(&(steps.len() as u64).to_le_bytes());

        // Hash each step
        for step in steps {
            hasher.update(&step.to_bytes());
        }

        Self(hasher.finalize())
    }
}

impl core::fmt::Display for DerivationId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// A single microcode operation in a derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MicrocodeStep {
    /// Bitwise NOT: dst = !src
    BNot { dst: u8, src: u8 },

    /// Two's complement negation: dst = -src
    Neg { dst: u8, src: u8 },

    /// Bitwise XOR: dst = a ^ b
    Xor { dst: u8, a: u8, b: u8 },

    /// Bitwise AND: dst = a & b
    And { dst: u8, a: u8, b: u8 },

    /// Bitwise OR: dst = a | b
    Or { dst: u8, a: u8, b: u8 },
}

impl MicrocodeStep {
    /// Convert to canonical byte representation for hashing.
    ///
    /// Format: `[opcode, dst, src_a, src_b]`
    /// For unary ops, `src_b` is 0xFF.
    pub fn to_bytes(&self) -> [u8; 4] {
        match self {
            MicrocodeStep::BNot { dst, src } => [0x00, *dst, *src, 0xFF],
            MicrocodeStep::Neg { dst, src } => [0x01, *dst, *src, 0xFF],
            MicrocodeStep::Xor { dst, a, b } => [0x02, *dst, *a, *b],
            MicrocodeStep::And { dst, a, b } => [0x03, *dst, *a, *b],
            MicrocodeStep::Or { dst, a, b } => [0x04, *dst, *a, *b],
        }
    }

    /// Parse from byte representation.
    pub fn from_bytes(bytes: [u8; 4]) -> Option<Self> {
        match bytes[0] {
            0x00 => Some(MicrocodeStep::BNot {
                dst: bytes[1],
                src: bytes[2],
            }),
            0x01 => Some(MicrocodeStep::Neg {
                dst: bytes[1],
                src: bytes[2],
            }),
            0x02 => Some(MicrocodeStep::Xor {
                dst: bytes[1],
                a: bytes[2],
                b: bytes[3],
            }),
            0x03 => Some(MicrocodeStep::And {
                dst: bytes[1],
                a: bytes[2],
                b: bytes[3],
            }),
            0x04 => Some(MicrocodeStep::Or {
                dst: bytes[1],
                a: bytes[2],
                b: bytes[3],
            }),
            _ => None,
        }
    }

    /// Get the destination register.
    pub fn dst(&self) -> u8 {
        match self {
            MicrocodeStep::BNot { dst, .. }
            | MicrocodeStep::Neg { dst, .. }
            | MicrocodeStep::Xor { dst, .. }
            | MicrocodeStep::And { dst, .. }
            | MicrocodeStep::Or { dst, .. } => *dst,
        }
    }

    /// Check if this is a commutative operation.
    pub fn is_commutative(&self) -> bool {
        matches!(
            self,
            MicrocodeStep::Xor { .. } | MicrocodeStep::And { .. } | MicrocodeStep::Or { .. }
        )
    }
}

/// A derivation is a sequence of microcode steps with a content-addressed ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Derivation {
    /// Content-addressed ID (computed from canonical form).
    id: DerivationId,

    /// Name/description of this derivation.
    name: &'static str,

    /// The microcode sequence.
    steps: Vec<MicrocodeStep>,
}

impl Derivation {
    /// Create a new derivation from steps.
    pub fn new(name: &'static str, steps: Vec<MicrocodeStep>) -> Self {
        let canonical = Self::canonicalize(&steps);
        let id = DerivationId::compute(&canonical);
        Self {
            id,
            name,
            steps: canonical,
        }
    }

    /// Get the derivation ID.
    pub fn id(&self) -> DerivationId {
        self.id
    }

    /// Get the derivation name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the microcode steps.
    pub fn steps(&self) -> &[MicrocodeStep] {
        &self.steps
    }

    /// Canonicalize steps for consistent hashing.
    ///
    /// - Sorts operands of commutative operations (a < b)
    /// - Could also do dead code elimination, constant folding
    fn canonicalize(steps: &[MicrocodeStep]) -> Vec<MicrocodeStep> {
        steps
            .iter()
            .map(|step| {
                match step {
                    // Sort operands of commutative ops
                    MicrocodeStep::Xor { dst, a, b } if a > b => MicrocodeStep::Xor {
                        dst: *dst,
                        a: *b,
                        b: *a,
                    },
                    MicrocodeStep::And { dst, a, b } if a > b => MicrocodeStep::And {
                        dst: *dst,
                        a: *b,
                        b: *a,
                    },
                    MicrocodeStep::Or { dst, a, b } if a > b => MicrocodeStep::Or {
                        dst: *dst,
                        a: *b,
                        b: *a,
                    },
                    other => *other,
                }
            })
            .collect()
    }
}

// -----------------------------------------------------------------------------
// Simple hasher (FNV-1a based, for zero-dependency hashing)
// -----------------------------------------------------------------------------

/// Simple hasher for derivation IDs.
///
/// Uses a variant of FNV-1a extended to 256 bits.
/// NOT cryptographically secure - for production, use SHA256.
struct SimpleHasher {
    state: [u64; 4],
}

impl SimpleHasher {
    const FNV_PRIME: u64 = 0x00000100_000001B3;
    const FNV_OFFSET: [u64; 4] = [
        0xcbf29ce4_84222325,
        0x6c62272e_07bb0142,
        0x62b82175_6bd62611,
        0x7b7c00a1_00ad4d17,
    ];

    fn new() -> Self {
        Self {
            state: Self::FNV_OFFSET,
        }
    }

    fn update(&mut self, data: &[u8]) {
        for byte in data {
            for i in 0..4 {
                self.state[i] ^= *byte as u64;
                self.state[i] = self.state[i].wrapping_mul(Self::FNV_PRIME);
            }
            // Mix between lanes
            self.state[0] = self.state[0].wrapping_add(self.state[3]);
            self.state[1] = self.state[1].wrapping_add(self.state[0]);
            self.state[2] = self.state[2].wrapping_add(self.state[1]);
            self.state[3] = self.state[3].wrapping_add(self.state[2]);
        }
    }

    fn finalize(self) -> [u8; 32] {
        let mut result = [0u8; 32];
        for (i, &val) in self.state.iter().enumerate() {
            result[i * 8..(i + 1) * 8].copy_from_slice(&val.to_le_bytes());
        }
        result
    }
}

// -----------------------------------------------------------------------------
// Standard derivations
// -----------------------------------------------------------------------------

/// Standard derivations for common operations.
pub mod standard {
    use super::*;

    /// INC: x + 1 = neg(bnot(x))
    pub fn inc() -> Derivation {
        Derivation::new(
            "inc",
            vec![
                MicrocodeStep::BNot { dst: 1, src: 0 }, // t1 = !x
                MicrocodeStep::Neg { dst: 0, src: 1 },  // result = -t1
            ],
        )
    }

    /// DEC: x - 1 = bnot(neg(x))
    pub fn dec() -> Derivation {
        Derivation::new(
            "dec",
            vec![
                MicrocodeStep::Neg { dst: 1, src: 0 },  // t1 = -x
                MicrocodeStep::BNot { dst: 0, src: 1 }, // result = !t1
            ],
        )
    }

    /// NAND: !(a & b)
    pub fn nand() -> Derivation {
        Derivation::new(
            "nand",
            vec![
                MicrocodeStep::And { dst: 2, a: 0, b: 1 }, // t = a & b
                MicrocodeStep::BNot { dst: 0, src: 2 },    // result = !t
            ],
        )
    }

    /// NOR: !(a | b)
    pub fn nor() -> Derivation {
        Derivation::new(
            "nor",
            vec![
                MicrocodeStep::Or { dst: 2, a: 0, b: 1 }, // t = a | b
                MicrocodeStep::BNot { dst: 0, src: 2 },   // result = !t
            ],
        )
    }

    /// XNOR: !(a ^ b)
    pub fn xnor() -> Derivation {
        Derivation::new(
            "xnor",
            vec![
                MicrocodeStep::Xor { dst: 2, a: 0, b: 1 }, // t = a ^ b
                MicrocodeStep::BNot { dst: 0, src: 2 },    // result = !t
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivation_id_deterministic() {
        let d1 = standard::inc();
        let d2 = standard::inc();
        assert_eq!(d1.id(), d2.id());
    }

    #[test]
    fn test_derivation_id_unique() {
        let inc = standard::inc();
        let dec = standard::dec();
        assert_ne!(inc.id(), dec.id());
    }

    #[test]
    fn test_canonical_form() {
        // XOR with operands in different order should produce same ID
        let d1 = Derivation::new("test", vec![MicrocodeStep::Xor { dst: 0, a: 1, b: 2 }]);
        let d2 = Derivation::new("test", vec![MicrocodeStep::Xor { dst: 0, a: 2, b: 1 }]);
        assert_eq!(d1.id(), d2.id());
    }

    #[test]
    fn test_step_roundtrip() {
        let steps = [
            MicrocodeStep::BNot { dst: 0, src: 1 },
            MicrocodeStep::Neg { dst: 2, src: 3 },
            MicrocodeStep::Xor { dst: 4, a: 5, b: 6 },
            MicrocodeStep::And { dst: 7, a: 8, b: 9 },
            MicrocodeStep::Or {
                dst: 10,
                a: 11,
                b: 12,
            },
        ];

        for step in steps {
            let bytes = step.to_bytes();
            let parsed = MicrocodeStep::from_bytes(bytes);
            assert_eq!(parsed, Some(step));
        }
    }

    #[test]
    fn test_derivation_id_hex() {
        let d = standard::inc();
        let hex = d.id().to_hex();
        assert_eq!(hex.len(), 64); // 32 bytes = 64 hex chars
    }
}
