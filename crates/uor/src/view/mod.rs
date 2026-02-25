//! View system for function composition via lookup tables.
//!
//! An `ElementWiseView<256>` is a 256-entry byte-to-byte lookup table that maps
//! every possible input byte to an output byte. Since a byte has exactly 256 values,
//! the entire function is captured in 256 bytes.
//!
//! Views compose by chaining lookups. Given `view_a` and `view_b`, their composition
//! `view_a.then(view_b)` creates a new view where `composed[x] = view_b[view_a[x]]`
//! for all 256 values of x. Applying the composed view at runtime costs exactly one
//! array access -- regardless of how many views were composed.

mod simd;

pub mod provenance;

use core::fmt;

// Re-export provenance types
pub use provenance::{TrackedView, ViewId, ViewProvenance};

/// A 256-entry byte-to-byte lookup table for O(1) function application.
///
/// # Examples
///
/// ```
/// use uor::view::ElementWiseView;
///
/// // Identity view
/// let identity = ElementWiseView::identity();
/// assert_eq!(identity.apply(42), 42);
///
/// // Constant view (always returns 0)
/// let zero = ElementWiseView::constant(0);
/// assert_eq!(zero.apply(42), 0);
/// assert_eq!(zero.apply(255), 0);
///
/// // Composition
/// let increment = ElementWiseView::new(|x| x.wrapping_add(1));
/// let doubled = increment.then(&increment);
/// assert_eq!(doubled.apply(5), 7); // 5 + 1 + 1 = 7
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(align(64))] // Cache-line align for SIMD
pub struct ElementWiseView {
    /// The 256-entry lookup table (one byte per entry)
    table: [u8; 256],
}

impl ElementWiseView {
    /// Create a new view from a lookup table.
    ///
    /// # Examples
    ///
    /// ```
    /// use uor::view::ElementWiseView;
    ///
    /// let mut table = [0u8; 256];
    /// for i in 0..=255u8 {
    ///     table[i as usize] = i.wrapping_add(1);
    /// }
    /// let view = ElementWiseView::from_table(table);
    /// assert_eq!(view.apply(0), 1);
    /// assert_eq!(view.apply(255), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_table(table: [u8; 256]) -> Self {
        Self { table }
    }

    /// Create a new view by applying a function to all 256 possible byte values.
    ///
    /// # Examples
    ///
    /// ```
    /// use uor::view::ElementWiseView;
    ///
    /// let increment = ElementWiseView::new(|x| x.wrapping_add(1));
    /// assert_eq!(increment.apply(0), 1);
    /// assert_eq!(increment.apply(255), 0);
    /// ```
    #[must_use]
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(u8) -> u8,
    {
        let mut table = [0u8; 256];
        for i in 0..=255u8 {
            table[i as usize] = f(i);
        }
        Self { table }
    }

    /// Create the identity view (maps every byte to itself).
    ///
    /// # Examples
    ///
    /// ```
    /// use uor::view::ElementWiseView;
    ///
    /// let identity = ElementWiseView::identity();
    /// for i in 0..=255u8 {
    ///     assert_eq!(identity.apply(i), i);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn identity() -> Self {
        let mut table = [0u8; 256];
        let mut i = 0u8;
        loop {
            table[i as usize] = i;
            if i == 255 {
                break;
            }
            i += 1;
        }
        Self { table }
    }

    /// Create a constant view (maps every byte to the same value).
    ///
    /// # Examples
    ///
    /// ```
    /// use uor::view::ElementWiseView;
    ///
    /// let zero = ElementWiseView::constant(0);
    /// for i in 0..=255u8 {
    ///     assert_eq!(zero.apply(i), 0);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn constant(value: u8) -> Self {
        Self {
            table: [value; 256],
        }
    }

    /// Apply the view to a single byte.
    ///
    /// # Examples
    ///
    /// ```
    /// use uor::view::ElementWiseView;
    ///
    /// let view = ElementWiseView::new(|x| x ^ 0xFF);
    /// assert_eq!(view.apply(0x00), 0xFF);
    /// assert_eq!(view.apply(0xFF), 0x00);
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn apply(&self, byte: u8) -> u8 {
        self.table[byte as usize]
    }

    /// Compose two views: `self.then(other)` returns a view where
    /// `composed(x) = other(self(x))` for all x.
    ///
    /// # Examples
    ///
    /// ```
    /// use uor::view::ElementWiseView;
    ///
    /// let inc = ElementWiseView::new(|x| x.wrapping_add(1));
    /// let double = inc.then(&inc); // Apply inc twice
    /// assert_eq!(double.apply(5), 7);
    /// ```
    #[must_use]
    pub fn then(&self, other: &Self) -> Self {
        let mut table = [0u8; 256];
        for i in 0..=255u8 {
            table[i as usize] = other.apply(self.apply(i));
        }
        Self { table }
    }

    /// Check if this view is bijective (a permutation).
    ///
    /// A view is bijective if every output value appears exactly once.
    ///
    /// # Examples
    ///
    /// ```
    /// use uor::view::ElementWiseView;
    ///
    /// let identity = ElementWiseView::identity();
    /// assert!(identity.is_bijective());
    ///
    /// let constant = ElementWiseView::constant(0);
    /// assert!(!constant.is_bijective());
    /// ```
    #[must_use]
    pub fn is_bijective(&self) -> bool {
        let mut seen = [false; 256];
        for &output in &self.table {
            if seen[output as usize] {
                return false; // Duplicate output
            }
            seen[output as usize] = true;
        }
        true
    }

    /// Compute the inverse of this view if it is bijective.
    ///
    /// Returns `None` if the view is not bijective.
    ///
    /// # Examples
    ///
    /// ```
    /// use uor::view::ElementWiseView;
    ///
    /// let inc = ElementWiseView::new(|x| x.wrapping_add(1));
    /// let dec = inc.inverse().unwrap();
    /// assert_eq!(dec.apply(inc.apply(42)), 42);
    ///
    /// let constant = ElementWiseView::constant(0);
    /// assert!(constant.inverse().is_none());
    /// ```
    #[must_use]
    pub fn inverse(&self) -> Option<Self> {
        if !self.is_bijective() {
            return None;
        }

        let mut inverse_table = [0u8; 256];
        for input in 0..=255u8 {
            let output = self.table[input as usize];
            inverse_table[output as usize] = input;
        }

        Some(Self {
            table: inverse_table,
        })
    }

    /// Apply this view to a slice of bytes in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use uor::view::ElementWiseView;
    ///
    /// let inc = ElementWiseView::new(|x| x.wrapping_add(1));
    /// let mut data = [0, 1, 2, 3, 4];
    /// inc.apply_slice(&mut data);
    /// assert_eq!(data, [1, 2, 3, 4, 5]);
    /// ```
    pub fn apply_slice(&self, data: &mut [u8]) {
        // Use SIMD path if available and data is large enough
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        if data.len() >= 32 {
            return simd::apply_avx2(self, data);
        }

        #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
        if data.len() >= 16 {
            return simd::apply_sse42(self, data);
        }

        // Scalar fallback
        for byte in data {
            *byte = self.apply(*byte);
        }
    }

    /// Apply this view to an input slice, writing results to an output slice.
    ///
    /// # Panics
    ///
    /// Panics if `input.len() != output.len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use uor::view::ElementWiseView;
    ///
    /// let inc = ElementWiseView::new(|x| x.wrapping_add(1));
    /// let input = [0, 1, 2, 3, 4];
    /// let mut output = [0u8; 5];
    /// inc.apply_to(&input, &mut output);
    /// assert_eq!(output, [1, 2, 3, 4, 5]);
    /// ```
    pub fn apply_to(&self, input: &[u8], output: &mut [u8]) {
        assert_eq!(
            input.len(),
            output.len(),
            "Input and output slices must have the same length"
        );

        // Use SIMD path if available and data is large enough
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        if input.len() >= 32 {
            return simd::apply_to_avx2(self, input, output);
        }

        #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
        if input.len() >= 16 {
            return simd::apply_to_sse42(self, input, output);
        }

        // Scalar fallback
        for (i, &byte) in input.iter().enumerate() {
            output[i] = self.apply(byte);
        }
    }

    /// Get a reference to the underlying lookup table.
    #[inline]
    #[must_use]
    pub const fn table(&self) -> &[u8; 256] {
        &self.table
    }

    /// Consume the view and return the underlying lookup table.
    #[inline]
    #[must_use]
    pub const fn into_table(self) -> [u8; 256] {
        self.table
    }
}

impl fmt::Debug for ElementWiseView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ElementWiseView {{ ")?;
        write!(f, "bijective: {}, ", self.is_bijective())?;
        write!(f, "table: [")?;
        for (i, &byte) in self.table.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            if i >= 8 {
                write!(f, "... ({} more)", 256 - 8)?;
                break;
            }
            write!(f, "{byte}")?;
        }
        write!(f, "] }}")
    }
}

impl Default for ElementWiseView {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let identity = ElementWiseView::identity();
        for i in 0..=255u8 {
            assert_eq!(identity.apply(i), i);
        }
    }

    #[test]
    fn test_constant() {
        let zero = ElementWiseView::constant(0);
        for i in 0..=255u8 {
            assert_eq!(zero.apply(i), 0);
        }

        let ff = ElementWiseView::constant(0xFF);
        for i in 0..=255u8 {
            assert_eq!(ff.apply(i), 0xFF);
        }
    }

    #[test]
    fn test_new() {
        let inc = ElementWiseView::new(|x| x.wrapping_add(1));
        assert_eq!(inc.apply(0), 1);
        assert_eq!(inc.apply(255), 0);

        let xor_ff = ElementWiseView::new(|x| x ^ 0xFF);
        assert_eq!(xor_ff.apply(0x00), 0xFF);
        assert_eq!(xor_ff.apply(0xFF), 0x00);
    }

    #[test]
    fn test_composition() {
        let inc = ElementWiseView::new(|x| x.wrapping_add(1));
        let double = inc.then(&inc);

        assert_eq!(double.apply(0), 2);
        assert_eq!(double.apply(5), 7);
        assert_eq!(double.apply(255), 1);
    }

    #[test]
    fn test_composition_identity() {
        let inc = ElementWiseView::new(|x| x.wrapping_add(1));
        let identity = ElementWiseView::identity();

        let view1 = inc.then(&identity);
        let view2 = identity.then(&inc);

        for i in 0..=255u8 {
            assert_eq!(view1.apply(i), inc.apply(i));
            assert_eq!(view2.apply(i), inc.apply(i));
        }
    }

    #[test]
    fn test_is_bijective() {
        let identity = ElementWiseView::identity();
        assert!(identity.is_bijective());

        let inc = ElementWiseView::new(|x| x.wrapping_add(1));
        assert!(inc.is_bijective());

        let constant = ElementWiseView::constant(0);
        assert!(!constant.is_bijective());

        let xor_ff = ElementWiseView::new(|x| x ^ 0xFF);
        assert!(xor_ff.is_bijective());
    }

    #[test]
    fn test_inverse() {
        let inc = ElementWiseView::new(|x| x.wrapping_add(1));
        let dec = inc.inverse().unwrap();

        for i in 0..=255u8 {
            assert_eq!(dec.apply(inc.apply(i)), i);
            assert_eq!(inc.apply(dec.apply(i)), i);
        }

        let constant = ElementWiseView::constant(0);
        assert!(constant.inverse().is_none());
    }

    #[test]
    fn test_apply_slice() {
        let inc = ElementWiseView::new(|x| x.wrapping_add(1));
        let mut data = [0, 1, 2, 3, 4, 255];
        inc.apply_slice(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5, 0]);
    }

    #[test]
    fn test_apply_to() {
        let inc = ElementWiseView::new(|x| x.wrapping_add(1));
        let input = [0, 1, 2, 3, 4, 255];
        let mut output = [0u8; 6];
        inc.apply_to(&input, &mut output);
        assert_eq!(output, [1, 2, 3, 4, 5, 0]);
    }

    #[test]
    #[should_panic(expected = "Input and output slices must have the same length")]
    fn test_apply_to_length_mismatch() {
        let inc = ElementWiseView::new(|x| x.wrapping_add(1));
        let input = [0, 1, 2];
        let mut output = [0u8; 5];
        inc.apply_to(&input, &mut output);
    }

    #[test]
    fn test_default_is_identity() {
        let view = ElementWiseView::default();
        for i in 0..=255u8 {
            assert_eq!(view.apply(i), i);
        }
    }
}
