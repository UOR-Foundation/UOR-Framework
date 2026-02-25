//! Triadic domains derived from T=3.
//!
//! Each byte value belongs to exactly one of three domains based on its
//! residue modulo 3. This triadic structure is fundamental to categorical
//! computation.

use super::constants::T;
use core::fmt;

/// One of the three triadic domains.
///
/// Derived from the axiom T=3, each byte belongs to exactly one domain
/// based on its residue mod 3. The domains represent fundamental aspects
/// of categorical structure.
///
/// # Domains
///
/// - **Theta (θ)**: Structure - values where n % 3 == 0
/// - **Psi (ψ)**: Unity - values where n % 3 == 1
/// - **Delta (δ)**: Duality - values where n % 3 == 2
///
/// # Example
///
/// ```
/// use uor::Domain;
///
/// let d = Domain::from_residue(17 % 3);
/// assert_eq!(d, Domain::Delta);
/// assert_eq!(d.symbol(), 'δ');
/// assert_eq!(d.name(), "Duality");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum Domain {
    /// Theta (θ) - Structure domain.
    /// Contains values where n % 3 == 0: 0, 3, 6, 9, ..., 255
    #[default]
    Theta = 0,

    /// Psi (ψ) - Unity domain.
    /// Contains values where n % 3 == 1: 1, 4, 7, 10, ..., 253
    Psi = 1,

    /// Delta (δ) - Duality domain.
    /// Contains values where n % 3 == 2: 2, 5, 8, 11, ..., 254
    Delta = 2,
}

impl Domain {
    /// All domains in canonical order.
    pub const ALL: [Self; T] = [Self::Theta, Self::Psi, Self::Delta];

    /// Creates a domain from its residue value (0, 1, or 2).
    ///
    /// Values >= 3 are reduced modulo 3.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Domain;
    ///
    /// assert_eq!(Domain::from_residue(0), Domain::Theta);
    /// assert_eq!(Domain::from_residue(1), Domain::Psi);
    /// assert_eq!(Domain::from_residue(2), Domain::Delta);
    /// assert_eq!(Domain::from_residue(5), Domain::Delta); // 5 % 3 == 2
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_residue(residue: u8) -> Self {
        match residue % 3 {
            0 => Self::Theta,
            1 => Self::Psi,
            _ => Self::Delta,
        }
    }

    /// Returns the residue value (0, 1, or 2).
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Domain;
    ///
    /// assert_eq!(Domain::Theta.residue(), 0);
    /// assert_eq!(Domain::Psi.residue(), 1);
    /// assert_eq!(Domain::Delta.residue(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub const fn residue(self) -> u8 {
        self as u8
    }

    /// Returns the Greek letter symbol for this domain.
    ///
    /// - Theta: θ (U+03B8)
    /// - Psi: ψ (U+03C8)
    /// - Delta: δ (U+03B4)
    #[inline]
    #[must_use]
    pub const fn symbol(self) -> char {
        match self {
            Self::Theta => 'θ',
            Self::Psi => 'ψ',
            Self::Delta => 'δ',
        }
    }

    /// Returns the semantic name of this domain.
    ///
    /// - Theta: "Structure"
    /// - Psi: "Unity"
    /// - Delta: "Duality"
    #[inline]
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Theta => "Structure",
            Self::Psi => "Unity",
            Self::Delta => "Duality",
        }
    }

    /// Returns the short name (slug) for this domain.
    ///
    /// - Theta: "theta"
    /// - Psi: "psi"
    /// - Delta: "delta"
    #[inline]
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Theta => "theta",
            Self::Psi => "psi",
            Self::Delta => "delta",
        }
    }

    /// Returns the number of taxons in this domain.
    ///
    /// - Theta: 86 taxons (0, 3, 6, ..., 255)
    /// - Psi: 85 taxons (1, 4, 7, ..., 253)
    /// - Delta: 85 taxons (2, 5, 8, ..., 254)
    #[inline]
    #[must_use]
    pub const fn cardinality(self) -> usize {
        super::constants::DOMAIN_CARDINALITIES[self as usize]
    }

    /// Returns the successor domain in the cyclic order (θ → ψ → δ → θ).
    #[inline]
    #[must_use]
    pub const fn succ(self) -> Self {
        Self::from_residue(self.residue() + 1)
    }

    /// Returns the predecessor domain in the cyclic order (θ → δ → ψ → θ).
    #[inline]
    #[must_use]
    pub const fn pred(self) -> Self {
        Self::from_residue(self.residue() + 2) // +2 ≡ -1 (mod 3)
    }
}

impl fmt::Debug for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name(), self.symbol())
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_domains() {
        assert_eq!(Domain::ALL.len(), T);
        assert_eq!(Domain::ALL[0], Domain::Theta);
        assert_eq!(Domain::ALL[1], Domain::Psi);
        assert_eq!(Domain::ALL[2], Domain::Delta);
    }

    #[test]
    fn test_from_residue() {
        for i in 0..=255u8 {
            let d = Domain::from_residue(i);
            assert_eq!(d.residue(), i % 3);
        }
    }

    #[test]
    fn test_symbols() {
        assert_eq!(Domain::Theta.symbol(), 'θ');
        assert_eq!(Domain::Psi.symbol(), 'ψ');
        assert_eq!(Domain::Delta.symbol(), 'δ');
    }

    #[test]
    fn test_names() {
        assert_eq!(Domain::Theta.name(), "Structure");
        assert_eq!(Domain::Psi.name(), "Unity");
        assert_eq!(Domain::Delta.name(), "Duality");
    }

    #[test]
    fn test_cardinality_sum() {
        let sum: usize = Domain::ALL.iter().map(|d| d.cardinality()).sum();
        assert_eq!(sum, 256);
    }

    #[test]
    fn test_succ_pred_cycle() {
        for d in Domain::ALL {
            assert_eq!(d.succ().pred(), d);
            assert_eq!(d.pred().succ(), d);
        }
    }

    #[test]
    fn test_succ_cycle() {
        assert_eq!(Domain::Theta.succ(), Domain::Psi);
        assert_eq!(Domain::Psi.succ(), Domain::Delta);
        assert_eq!(Domain::Delta.succ(), Domain::Theta);
    }
}
