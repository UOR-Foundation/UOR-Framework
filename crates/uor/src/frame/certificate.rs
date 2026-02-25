//! Transform certificates — content-addressed records of type resolutions.
//!
//! A `TransformCertificate` records the complete result of resolving a datum's
//! irreducibility under a given type declaration. It bundles:
//!
//! - The **type declaration** `T = (A, ⊗, ε)` that was used
//! - The computed **partition** (Irr/Red/Units/Ext classification)
//! - The **observable signature** (all seven families)
//! - A **content address** derived deterministically from the above
//!
//! # Content Addressing
//!
//! The certificate address is computed from the partition's irreducible set
//! and the binary operation, producing a unique Braille address. Two
//! certificates with the same address attest to the same type resolution.
//!
//! # Ontology
//!
//! Corresponds to `cert:TransformCertificate` (SIH §9) with properties:
//! `transformType`, `operation`, `verified`.
//!
//! # Examples
//!
//! ```
//! use uor::frame::{TypeDeclaration, TransformCertificate};
//!
//! let t2 = TypeDeclaration::integer_mul();
//! let cert = TransformCertificate::compute(&t2);
//! assert_eq!(cert.irr_count(), 54);
//! assert!(cert.verified());
//! ```

use super::{BinaryOp, DatumSet, ObservableSignature, Partition, TypeDeclaration};

/// Classification of a single datum under a type declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatumClass {
    /// Irreducible under the type's operation.
    Irreducible,
    /// Reducible (has a non-trivial factorization).
    Reducible,
    /// Unit (has a multiplicative inverse).
    Unit,
    /// External to the carrier set.
    External,
}

/// Content address of a transform certificate.
///
/// A compact, deterministic identifier derived from the certificate's content.
/// Two certificates with the same address attest to identical type resolutions.
///
/// The address encodes: `[op_tag, irr_count, irr_words[0..4] as bytes]`.
/// Total: 34 bytes (1 op + 1 count + 32 irr bitset).
///
/// Size: 34 bytes on stack.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CertificateAddress {
    bytes: [u8; 34],
}

impl CertificateAddress {
    /// Compute the address from a binary operation and irreducible set.
    fn compute(op: BinaryOp, irr: &DatumSet) -> Self {
        let mut bytes = [0u8; 34];
        bytes[0] = op_tag(op);
        bytes[1] = irr.len() as u8;
        let words = irr.words();
        for (i, &w) in words.iter().enumerate() {
            let base = 2 + i * 8;
            bytes[base..base + 8].copy_from_slice(&w.to_le_bytes());
        }
        Self { bytes }
    }

    /// Raw bytes of the address.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 34] {
        &self.bytes
    }

    /// Convert to a Braille string via the Address infrastructure.
    pub fn to_braille(&self) -> alloc::string::String {
        use crate::address::Address;
        Address::from_bytes(&self.bytes).to_string()
    }
}

impl core::fmt::Debug for CertificateAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CertAddr(op={}, irr={})", self.bytes[0], self.bytes[1])
    }
}

impl core::fmt::Display for CertificateAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Display as hex for readability
        for b in &self.bytes {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

/// A content-addressed record of a type resolution.
///
/// Captures the complete result of classifying all 256 datum values under
/// a type declaration. The certificate is self-verifying: calling `verified()`
/// checks that the partition invariant holds and all four classes are disjoint
/// and cover the full Q0 space.
///
/// # Size
///
/// 266 bytes:
/// - TypeDeclaration: 40
/// - Partition: 128
/// - ObservableSignature: 56
/// - CertificateAddress: 34
/// - padding: 8
///
/// Fits comfortably in L1 cache alongside the LUT tables.
#[derive(Debug, Clone, Copy)]
pub struct TransformCertificate {
    decl: TypeDeclaration,
    partition: Partition,
    signature: ObservableSignature,
    address: CertificateAddress,
}

impl TransformCertificate {
    /// Compute a certificate for a type declaration.
    ///
    /// Performs the full partition computation and observable signature
    /// extraction, then derives the content address.
    pub fn compute(decl: &TypeDeclaration) -> Self {
        let partition = Partition::compute(decl);
        let signature = ObservableSignature::compute(partition.irr());
        let address = CertificateAddress::compute(decl.op(), partition.irr());
        Self {
            decl: *decl,
            partition,
            signature,
            address,
        }
    }

    /// The type declaration this certificate attests to.
    #[inline]
    pub const fn decl(&self) -> &TypeDeclaration {
        &self.decl
    }

    /// The computed partition.
    #[inline]
    pub const fn partition(&self) -> &Partition {
        &self.partition
    }

    /// The observable signature of the irreducible set.
    #[inline]
    pub const fn signature(&self) -> &ObservableSignature {
        &self.signature
    }

    /// The content address of this certificate.
    #[inline]
    pub const fn address(&self) -> &CertificateAddress {
        &self.address
    }

    /// Number of irreducible elements.
    #[inline]
    pub fn irr_count(&self) -> u32 {
        self.partition.irr().len()
    }

    /// Number of reducible elements.
    #[inline]
    pub fn red_count(&self) -> u32 {
        self.partition.red().len()
    }

    /// Classify a single datum under this certificate's type.
    pub fn classify(&self, datum: u8) -> DatumClass {
        if self.partition.irr().contains(datum) {
            DatumClass::Irreducible
        } else if self.partition.red().contains(datum) {
            DatumClass::Reducible
        } else if self.partition.units().contains(datum) {
            DatumClass::Unit
        } else {
            DatumClass::External
        }
    }

    /// Verify the certificate: partition invariant holds.
    #[inline]
    pub fn verified(&self) -> bool {
        self.partition.verify()
    }

    /// Check if two certificates attest to the same type resolution.
    ///
    /// Two certificates are equivalent iff their content addresses match.
    #[inline]
    pub fn same_resolution(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

/// Map a `BinaryOp` to a single-byte tag for address computation.
const fn op_tag(op: BinaryOp) -> u8 {
    match op {
        BinaryOp::IntegerMul => 0x01,
        BinaryOp::PolyGf2Mul => 0x02,
        BinaryOp::PolyGf3Mul => 0x03,
        BinaryOp::PolyGf5Mul => 0x05,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cert_t2() -> TransformCertificate {
        TransformCertificate::compute(&TypeDeclaration::integer_mul())
    }

    fn cert_poly() -> TransformCertificate {
        TransformCertificate::compute(&TypeDeclaration::poly_gf2())
    }

    // -- Construction --

    #[test]
    fn compute_t2_certificate() {
        let cert = cert_t2();
        assert_eq!(cert.irr_count(), 54);
        assert!(cert.verified());
    }

    #[test]
    fn compute_poly_certificate() {
        let cert = cert_poly();
        assert_eq!(cert.irr_count(), 41);
        assert!(cert.verified());
    }

    // -- Address uniqueness --

    #[test]
    fn t2_and_poly_have_different_addresses() {
        let c1 = cert_t2();
        let c2 = cert_poly();
        assert_ne!(c1.address(), c2.address());
    }

    #[test]
    fn same_type_same_address() {
        let c1 = cert_t2();
        let c2 = cert_t2();
        assert_eq!(c1.address(), c2.address());
    }

    #[test]
    fn same_resolution_check() {
        let c1 = cert_t2();
        let c2 = cert_t2();
        let c3 = cert_poly();
        assert!(c1.same_resolution(&c2));
        assert!(!c1.same_resolution(&c3));
    }

    // -- Datum classification --

    #[test]
    fn classify_prime_under_t2() {
        let cert = cert_t2();
        assert_eq!(cert.classify(2), DatumClass::Irreducible);
        assert_eq!(cert.classify(3), DatumClass::Irreducible);
        assert_eq!(cert.classify(251), DatumClass::Irreducible);
    }

    #[test]
    fn classify_composite_under_t2() {
        let cert = cert_t2();
        assert_eq!(cert.classify(4), DatumClass::Reducible);
        assert_eq!(cert.classify(6), DatumClass::Reducible);
        assert_eq!(cert.classify(100), DatumClass::Reducible);
    }

    #[test]
    fn classify_external_under_t2() {
        let cert = cert_t2();
        assert_eq!(cert.classify(0), DatumClass::External);
        assert_eq!(cert.classify(1), DatumClass::External);
    }

    #[test]
    fn datum_5_type_relative() {
        let c_t2 = cert_t2();
        let c_poly = cert_poly();
        // 5 is prime (irreducible under integer mul)
        assert_eq!(c_t2.classify(5), DatumClass::Irreducible);
        // 5 = x²+1 = (x+1)² is reducible over GF(2)
        assert_eq!(c_poly.classify(5), DatumClass::Reducible);
    }

    #[test]
    fn datum_25_type_relative() {
        let c_t2 = cert_t2();
        let c_poly = cert_poly();
        // 25 = 5*5 is reducible under integer mul
        assert_eq!(c_t2.classify(25), DatumClass::Reducible);
        // 25 = x⁴+x³+1 is irreducible over GF(2)
        assert_eq!(c_poly.classify(25), DatumClass::Irreducible);
    }

    // -- Observable signature --

    #[test]
    fn signature_populated() {
        let cert = cert_t2();
        let sig = cert.signature();
        assert!(sig.stratum > 0.0);
        assert!(sig.hamming_metric > 0.0);
        assert!(sig.cascade > 0.0);
    }

    #[test]
    fn signatures_differ_between_types() {
        let c1 = cert_t2();
        let c2 = cert_poly();
        let s1 = c1.signature().as_array();
        let s2 = c2.signature().as_array();
        for i in 0..6 {
            assert!(
                (s1[i] - s2[i]).abs() > 1e-6,
                "family {i} should differ: {} vs {}",
                s1[i],
                s2[i],
            );
        }
    }

    // -- Address format --

    #[test]
    fn address_bytes_length() {
        let cert = cert_t2();
        assert_eq!(cert.address().as_bytes().len(), 34);
    }

    #[test]
    fn address_encodes_op_tag() {
        let c1 = cert_t2();
        let c2 = cert_poly();
        assert_eq!(c1.address().as_bytes()[0], 0x01); // IntegerMul
        assert_eq!(c2.address().as_bytes()[0], 0x02); // PolyGf2Mul
    }

    #[test]
    fn address_encodes_irr_count() {
        let c1 = cert_t2();
        let c2 = cert_poly();
        assert_eq!(c1.address().as_bytes()[1], 54);
        assert_eq!(c2.address().as_bytes()[1], 41);
    }

    #[test]
    fn address_display_hex() {
        let cert = cert_t2();
        let hex = cert.address().to_string();
        assert_eq!(hex.len(), 68); // 34 bytes * 2 hex chars
    }

    #[test]
    fn address_to_braille() {
        let cert = cert_t2();
        let braille = cert.address().to_braille();
        // 34 bytes → 34 Braille characters
        assert_eq!(braille.chars().count(), 34);
        // All characters in Braille range U+2800-U+28FF
        for ch in braille.chars() {
            assert!(
                (0x2800..=0x28FF).contains(&(ch as u32)),
                "non-Braille character: U+{:04X}",
                ch as u32,
            );
        }
    }

    // -- Verified --

    #[test]
    fn certificates_are_verified() {
        assert!(cert_t2().verified());
        assert!(cert_poly().verified());
    }

    // -- Counts --

    #[test]
    fn irr_plus_red_covers_carrier() {
        let cert = cert_t2();
        // Carrier = {2..=255} = 254 elements, no units
        assert_eq!(cert.irr_count() + cert.red_count(), 254);
    }

    // -- Determinism --

    #[test]
    fn certificates_are_deterministic() {
        let c1 = cert_t2();
        let c2 = cert_t2();
        assert_eq!(c1.address(), c2.address());
        assert_eq!(c1.irr_count(), c2.irr_count());
        assert_eq!(c1.signature().as_array(), c2.signature().as_array());
    }
}
