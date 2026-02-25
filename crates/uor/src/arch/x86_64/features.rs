//! CPU feature detection for UOR conformance.
//!
//! UOR requires specific CPU features to execute correctly:
//! - AVX2: Required for 256-bit SIMD operations
//! - SHA-NI: Required for SHA-256 acceleration
//! - AES-NI: Required for AES acceleration
//!
//! This module provides runtime detection and validation.
//!
//! # Conformance
//!
//! Missing features are a **conformance violation** and will panic.
//! There is no fallback - the UOR specification requires these features.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// Detected CPU features required for UOR execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuFeatures {
    /// AVX2 support (256-bit SIMD operations).
    pub avx2: bool,
    /// SHA-NI support (SHA-256 hardware acceleration).
    pub sha_ni: bool,
    /// AES-NI support (AES hardware acceleration).
    pub aes_ni: bool,
}

impl CpuFeatures {
    /// Detect CPU features at runtime using CPUID.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::arch::x86_64::CpuFeatures;
    ///
    /// let features = CpuFeatures::detect();
    /// println!("AVX2: {}", features.avx2);
    /// println!("SHA-NI: {}", features.sha_ni);
    /// println!("AES-NI: {}", features.aes_ni);
    /// ```
    #[must_use]
    #[cfg(feature = "std")]
    pub fn detect() -> Self {
        Self {
            avx2: std::is_x86_feature_detected!("avx2"),
            sha_ni: std::is_x86_feature_detected!("sha"),
            aes_ni: std::is_x86_feature_detected!("aes"),
        }
    }

    /// Detect CPU features at runtime (no_std fallback - assumes features present).
    ///
    /// # Safety
    ///
    /// In no_std environments, this assumes all features are present.
    /// Use only when you know the target CPU has the required features.
    #[must_use]
    #[cfg(not(feature = "std"))]
    pub fn detect() -> Self {
        Self {
            avx2: true,
            sha_ni: true,
            aes_ni: true,
        }
    }

    /// Check if all required features are present.
    #[must_use]
    pub const fn all_present(&self) -> bool {
        self.avx2 && self.sha_ni && self.aes_ni
    }

    /// Validate all required features are present.
    ///
    /// # Panics
    ///
    /// Panics with a conformance violation message if any required feature
    /// is missing. This is intentional - UOR has no fallback path.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::arch::x86_64::CpuFeatures;
    ///
    /// // Will panic on CPUs without required features
    /// CpuFeatures::detect().require_all();
    /// ```
    pub fn require_all(&self) {
        if !self.avx2 {
            panic!(
                "UOR CONFORMANCE VIOLATION: AVX2 not detected. \
                 UOR requires AVX2 for 256-bit SIMD operations."
            );
        }
        if !self.sha_ni {
            panic!(
                "UOR CONFORMANCE VIOLATION: SHA-NI not detected. \
                 UOR requires SHA-NI for SHA-256 hardware acceleration."
            );
        }
        if !self.aes_ni {
            panic!(
                "UOR CONFORMANCE VIOLATION: AES-NI not detected. \
                 UOR requires AES-NI for AES hardware acceleration."
            );
        }
    }

    /// Validate that only AVX2 is present (for non-crypto operations).
    ///
    /// Use this when executing wavefronts that don't use SHA-NI or AES-NI.
    ///
    /// # Panics
    ///
    /// Panics if AVX2 is not detected.
    pub fn require_avx2(&self) {
        if !self.avx2 {
            panic!(
                "UOR CONFORMANCE VIOLATION: AVX2 not detected. \
                 UOR requires AVX2 for 256-bit SIMD operations."
            );
        }
    }

    /// Returns a list of missing features as a human-readable string.
    #[must_use]
    pub fn missing_features(&self) -> Option<String> {
        let mut missing = Vec::new();
        if !self.avx2 {
            missing.push("AVX2");
        }
        if !self.sha_ni {
            missing.push("SHA-NI");
        }
        if !self.aes_ni {
            missing.push("AES-NI");
        }
        if missing.is_empty() {
            None
        } else {
            Some(missing.join(", "))
        }
    }
}

impl Default for CpuFeatures {
    fn default() -> Self {
        Self::detect()
    }
}

impl core::fmt::Display for CpuFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "CpuFeatures {{ avx2: {}, sha_ni: {}, aes_ni: {} }}",
            self.avx2, self.sha_ni, self.aes_ni
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn test_detect_features() {
        let features = CpuFeatures::detect();
        // Just verify detection runs without panic
        let _ = format!("Detected: {}", features);
    }

    #[test]
    fn test_all_present() {
        let all_true = CpuFeatures {
            avx2: true,
            sha_ni: true,
            aes_ni: true,
        };
        assert!(all_true.all_present());

        let missing_avx2 = CpuFeatures {
            avx2: false,
            sha_ni: true,
            aes_ni: true,
        };
        assert!(!missing_avx2.all_present());
    }

    #[test]
    fn test_missing_features() {
        let all_true = CpuFeatures {
            avx2: true,
            sha_ni: true,
            aes_ni: true,
        };
        assert!(all_true.missing_features().is_none());

        let missing_some = CpuFeatures {
            avx2: true,
            sha_ni: false,
            aes_ni: false,
        };
        let missing = missing_some.missing_features().unwrap();
        assert!(missing.contains("SHA-NI"));
        assert!(missing.contains("AES-NI"));
        assert!(!missing.contains("AVX2"));
    }

    #[test]
    fn test_display() {
        let features = CpuFeatures {
            avx2: true,
            sha_ni: true,
            aes_ni: false,
        };
        let s = format!("{}", features);
        assert!(s.contains("avx2: true"));
        assert!(s.contains("aes_ni: false"));
    }
}
