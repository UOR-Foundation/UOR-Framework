//! UOR Conformance Validation Utilities
//!
//! This module provides utilities to validate that UOR implementations
//! meet the performance and correctness requirements of the specification.
//!
//! # Conformance Targets
//!
//! | Criterion | Target | Description |
//! |-----------|--------|-------------|
//! | Single Wavefront | < 5 cycles | Individual operation latency |
//! | 64-Wavefront Sequence | < 200 cycles | Program throughput |
//! | Throughput | ≥ 512 bits/cycle | Sustained bandwidth |
//!
//! # Usage
//!
//! ```ignore
//! use uor::conformance;
//!
//! // Validate measured performance
//! let cycles = measure_wavefront_cycles();
//! conformance::validate_wavefront_latency(cycles).expect("Conformance check");
//! ```

extern crate alloc;

use crate::state::STATE_BITS;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

// =============================================================================
// CONFORMANCE TIER CONSTANTS
// =============================================================================

/// MINIMUM targets (hard requirements - implementation is non-conformant if violated)
pub const MIN_SINGLE_WAVEFRONT_CYCLES: u64 = 5;
pub const MIN_SEQUENCE_64_CYCLES: u64 = 200;
pub const MIN_BITS_PER_CYCLE: u64 = 512;

/// OPTIMAL targets (soft requirements - implementation is suboptimal if violated)
pub const OPT_SINGLE_WAVEFRONT_CYCLES: u64 = 3;
pub const OPT_SEQUENCE_64_CYCLES: u64 = 100;
pub const OPT_BITS_PER_CYCLE: u64 = 1600;

/// THEORETICAL targets (perfect implementation limits)
pub const THEORETICAL_SINGLE_WAVEFRONT_CYCLES: u64 = 1;
pub const THEORETICAL_SEQUENCE_64_CYCLES: u64 = 88; // 16 load + 64 compute + 8 store
pub const THEORETICAL_BITS_PER_CYCLE: u64 = 4992; // Full state in 1 cycle

// Legacy aliases for backwards compatibility
/// Target: Single wavefront must complete in < 5 cycles.
pub const TARGET_SINGLE_WAVEFRONT_CYCLES: u64 = MIN_SINGLE_WAVEFRONT_CYCLES;

/// Target: 64-wavefront sequence must complete in < 200 cycles.
pub const TARGET_SEQUENCE_64_CYCLES: u64 = MIN_SEQUENCE_64_CYCLES;

/// Target: Throughput must be ≥ 512 bits/cycle.
pub const TARGET_BITS_PER_CYCLE: u64 = MIN_BITS_PER_CYCLE;

/// UOR state size in bits (4992 = 624 taxons × 8 bits).
pub const UOR_STATE_BITS: u64 = STATE_BITS as u64;

// =============================================================================
// CONFORMANCE TIER ENUM
// =============================================================================

/// Conformance tier classification based on achieved performance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConformanceTier {
    /// Below MINIMUM targets - implementation is non-conformant.
    NonConformant = 0,
    /// Meets MINIMUM targets but below OPTIMAL.
    Minimum = 1,
    /// Meets OPTIMAL targets but below THEORETICAL.
    Optimal = 2,
    /// Meets THEORETICAL limits - perfect implementation.
    Theoretical = 3,
}

impl ConformanceTier {
    /// Compute the conformance tier from measured cycles.
    #[must_use]
    pub fn from_wavefront_cycles(cycles: u64) -> Self {
        if cycles > MIN_SINGLE_WAVEFRONT_CYCLES {
            Self::NonConformant
        } else if cycles > OPT_SINGLE_WAVEFRONT_CYCLES {
            Self::Minimum
        } else if cycles > THEORETICAL_SINGLE_WAVEFRONT_CYCLES {
            Self::Optimal
        } else {
            Self::Theoretical
        }
    }

    /// Compute the conformance tier from sequence cycles.
    #[must_use]
    pub fn from_sequence_cycles(cycles: u64) -> Self {
        if cycles > MIN_SEQUENCE_64_CYCLES {
            Self::NonConformant
        } else if cycles > OPT_SEQUENCE_64_CYCLES {
            Self::Minimum
        } else if cycles > THEORETICAL_SEQUENCE_64_CYCLES {
            Self::Optimal
        } else {
            Self::Theoretical
        }
    }

    /// Compute the conformance tier from bits per cycle.
    #[must_use]
    pub fn from_bits_per_cycle(bpc: u64) -> Self {
        if bpc < MIN_BITS_PER_CYCLE {
            Self::NonConformant
        } else if bpc < OPT_BITS_PER_CYCLE {
            Self::Minimum
        } else if bpc < THEORETICAL_BITS_PER_CYCLE {
            Self::Optimal
        } else {
            Self::Theoretical
        }
    }

    /// Get the minimum (worst) tier from multiple measurements.
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        if (self as u8) < (other as u8) {
            self
        } else {
            other
        }
    }

    /// Check if this tier meets at least the minimum conformance level.
    #[must_use]
    pub fn is_conformant(self) -> bool {
        self >= Self::Minimum
    }

    /// Check if this tier meets the optimal level.
    #[must_use]
    pub fn is_optimal(self) -> bool {
        self >= Self::Optimal
    }
}

impl core::fmt::Display for ConformanceTier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonConformant => write!(f, "NON-CONFORMANT"),
            Self::Minimum => write!(f, "MINIMUM"),
            Self::Optimal => write!(f, "OPTIMAL"),
            Self::Theoretical => write!(f, "THEORETICAL"),
        }
    }
}

/// Conformance validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceViolation {
    /// Description of the violation.
    pub message: String,
    /// Measured value that violated conformance.
    pub measured: u64,
    /// Target value that should have been met.
    pub target: u64,
}

impl core::fmt::Display for ConformanceViolation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "UOR CONFORMANCE VIOLATION: {} (measured: {}, target: {})",
            self.message, self.measured, self.target
        )
    }
}

// Note: Error trait implementation is available when std is enabled
// impl std::error::Error for ConformanceViolation {}

/// Validate that throughput meets the conformance target.
///
/// # Arguments
///
/// * `cycles_per_wavefront` - Measured cycles for a single wavefront
///
/// # Returns
///
/// * `Ok(bits_per_cycle)` - The achieved bits/cycle if conformant
/// * `Err(ConformanceViolation)` - Details if target not met
///
/// # Example
///
/// ```
/// use uor::conformance::validate_throughput;
///
/// // 4992 bits / 5 cycles = 998 bits/cycle (passes)
/// assert!(validate_throughput(5).is_ok());
///
/// // 4992 bits / 20 cycles = 249 bits/cycle (fails)
/// assert!(validate_throughput(20).is_err());
/// ```
pub fn validate_throughput(cycles_per_wavefront: u64) -> Result<u64, ConformanceViolation> {
    if cycles_per_wavefront == 0 {
        return Err(ConformanceViolation {
            message: "cycles_per_wavefront cannot be zero".to_string(),
            measured: 0,
            target: TARGET_BITS_PER_CYCLE,
        });
    }

    let bits_per_cycle = UOR_STATE_BITS / cycles_per_wavefront;

    if bits_per_cycle >= TARGET_BITS_PER_CYCLE {
        Ok(bits_per_cycle)
    } else {
        Err(ConformanceViolation {
            message: format!(
                "{} bits/cycle < {} target ({} cycles/wavefront)",
                bits_per_cycle, TARGET_BITS_PER_CYCLE, cycles_per_wavefront
            ),
            measured: bits_per_cycle,
            target: TARGET_BITS_PER_CYCLE,
        })
    }
}

/// Validate that single wavefront latency meets the target.
///
/// # Arguments
///
/// * `cycles` - Measured cycles for a single wavefront
///
/// # Returns
///
/// * `Ok(())` - If latency is within target
/// * `Err(ConformanceViolation)` - If latency exceeds target
///
/// # Example
///
/// ```
/// use uor::conformance::validate_wavefront_latency;
///
/// assert!(validate_wavefront_latency(3).is_ok());
/// assert!(validate_wavefront_latency(10).is_err());
/// ```
pub fn validate_wavefront_latency(cycles: u64) -> Result<(), ConformanceViolation> {
    if cycles <= TARGET_SINGLE_WAVEFRONT_CYCLES {
        Ok(())
    } else {
        Err(ConformanceViolation {
            message: format!(
                "{} cycles > {} target for single wavefront",
                cycles, TARGET_SINGLE_WAVEFRONT_CYCLES
            ),
            measured: cycles,
            target: TARGET_SINGLE_WAVEFRONT_CYCLES,
        })
    }
}

/// Validate that 64-wavefront sequence latency meets the target.
///
/// # Arguments
///
/// * `cycles` - Measured cycles for 64 consecutive wavefronts
///
/// # Returns
///
/// * `Ok(cycles_per_wavefront)` - Average cycles per wavefront if conformant
/// * `Err(ConformanceViolation)` - If total latency exceeds target
///
/// # Example
///
/// ```
/// use uor::conformance::validate_sequence_latency;
///
/// assert!(validate_sequence_latency(180).is_ok());
/// assert!(validate_sequence_latency(250).is_err());
/// ```
pub fn validate_sequence_latency(cycles: u64) -> Result<u64, ConformanceViolation> {
    if cycles <= TARGET_SEQUENCE_64_CYCLES {
        Ok(cycles / 64) // Average cycles per wavefront
    } else {
        Err(ConformanceViolation {
            message: format!(
                "{} cycles > {} target for 64-wavefront sequence",
                cycles, TARGET_SEQUENCE_64_CYCLES
            ),
            measured: cycles,
            target: TARGET_SEQUENCE_64_CYCLES,
        })
    }
}

/// Calculate bits per cycle from nanoseconds and CPU frequency.
///
/// # Arguments
///
/// * `nanoseconds` - Measured time in nanoseconds
/// * `cpu_ghz` - CPU frequency in GHz (e.g., 3.5 for 3.5 GHz)
///
/// # Returns
///
/// The calculated bits per cycle.
///
/// # Example
///
/// ```
/// use uor::conformance::bits_per_cycle_from_ns;
///
/// // 1.5 ns at 3.5 GHz = 5.25 cycles
/// // 4992 bits / 5.25 cycles = 951 bits/cycle
/// let bpc = bits_per_cycle_from_ns(1.5, 3.5);
/// assert!(bpc > 900);
/// ```
pub fn bits_per_cycle_from_ns(nanoseconds: f64, cpu_ghz: f64) -> u64 {
    if nanoseconds <= 0.0 || cpu_ghz <= 0.0 {
        return 0;
    }

    let cycles = nanoseconds * cpu_ghz;
    if cycles < 1.0 {
        return UOR_STATE_BITS; // Sub-cycle = max throughput
    }

    (UOR_STATE_BITS as f64 / cycles) as u64
}

/// Convert nanoseconds to estimated CPU cycles.
///
/// # Arguments
///
/// * `nanoseconds` - Measured time in nanoseconds
/// * `cpu_ghz` - CPU frequency in GHz (e.g., 3.5 for 3.5 GHz)
///
/// # Returns
///
/// The estimated number of CPU cycles (rounded up).
///
/// # Example
///
/// ```
/// use uor::conformance::ns_to_cycles;
///
/// // 1.5 ns at 3.5 GHz = 5.25 cycles → 6 (ceil)
/// let cycles = ns_to_cycles(1.5, 3.5);
/// assert_eq!(cycles, 6);
///
/// // 1.0 ns at 4.0 GHz = 4.0 cycles → 4
/// let cycles = ns_to_cycles(1.0, 4.0);
/// assert_eq!(cycles, 4);
/// ```
pub fn ns_to_cycles(nanoseconds: f64, cpu_ghz: f64) -> u64 {
    if nanoseconds <= 0.0 || cpu_ghz <= 0.0 {
        return 0;
    }
    (nanoseconds * cpu_ghz).ceil() as u64
}

/// Conformance report for a UOR implementation.
#[derive(Debug, Clone)]
pub struct ConformanceReport {
    /// Single wavefront latency in cycles.
    pub single_wavefront_cycles: Option<u64>,
    /// 64-wavefront sequence latency in cycles.
    pub sequence_64_cycles: Option<u64>,
    /// Achieved bits per cycle.
    pub bits_per_cycle: Option<u64>,
    /// List of conformance violations (if any).
    pub violations: Vec<ConformanceViolation>,
    /// Overall conformance tier.
    pub tier: ConformanceTier,
}

impl ConformanceReport {
    /// Create a new empty report.
    #[must_use]
    pub fn new() -> Self {
        Self {
            single_wavefront_cycles: None,
            sequence_64_cycles: None,
            bits_per_cycle: None,
            violations: Vec::new(),
            tier: ConformanceTier::NonConformant,
        }
    }

    /// Record a single wavefront measurement.
    pub fn record_single_wavefront(&mut self, cycles: u64) {
        self.single_wavefront_cycles = Some(cycles);
        if let Err(v) = validate_wavefront_latency(cycles) {
            self.violations.push(v);
        }
        self.update_tier();
    }

    /// Record a 64-wavefront sequence measurement.
    pub fn record_sequence(&mut self, cycles: u64) {
        self.sequence_64_cycles = Some(cycles);
        if let Err(v) = validate_sequence_latency(cycles) {
            self.violations.push(v);
        }
        self.update_tier();
    }

    /// Record throughput measurement.
    pub fn record_throughput(&mut self, cycles_per_wavefront: u64) {
        match validate_throughput(cycles_per_wavefront) {
            Ok(bpc) => self.bits_per_cycle = Some(bpc),
            Err(v) => {
                self.bits_per_cycle = Some(UOR_STATE_BITS / cycles_per_wavefront.max(1));
                self.violations.push(v);
            }
        }
        self.update_tier();
    }

    /// Update the overall tier based on all measurements.
    fn update_tier(&mut self) {
        let mut tier = ConformanceTier::Theoretical;

        if let Some(cycles) = self.single_wavefront_cycles {
            tier = tier.min(ConformanceTier::from_wavefront_cycles(cycles));
        }

        if let Some(cycles) = self.sequence_64_cycles {
            tier = tier.min(ConformanceTier::from_sequence_cycles(cycles));
        }

        if let Some(bpc) = self.bits_per_cycle {
            tier = tier.min(ConformanceTier::from_bits_per_cycle(bpc));
        }

        // If no measurements, tier is NonConformant
        if self.single_wavefront_cycles.is_none()
            && self.sequence_64_cycles.is_none()
            && self.bits_per_cycle.is_none()
        {
            tier = ConformanceTier::NonConformant;
        }

        self.tier = tier;
    }

    /// Check if the implementation is conformant (no violations).
    #[must_use]
    pub fn is_conformant(&self) -> bool {
        self.violations.is_empty() && self.tier.is_conformant()
    }

    /// Check if the implementation achieves optimal performance.
    #[must_use]
    pub fn is_optimal(&self) -> bool {
        self.tier.is_optimal()
    }

    /// Get the number of violations.
    #[must_use]
    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }

    /// Get the overall conformance tier.
    #[must_use]
    pub fn get_tier(&self) -> ConformanceTier {
        self.tier
    }
}

impl Default for ConformanceReport {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for ConformanceReport {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "UOR Conformance Report")?;
        writeln!(f, "=====================")?;
        writeln!(f, "Overall Tier: {}", self.tier)?;
        writeln!(f)?;

        if let Some(cycles) = self.single_wavefront_cycles {
            let tier = ConformanceTier::from_wavefront_cycles(cycles);
            writeln!(f, "Single Wavefront: {} cycles [{}]", cycles, tier)?;
            writeln!(
                f,
                "  MIN: <{}  OPT: <{}  THEORETICAL: {}",
                MIN_SINGLE_WAVEFRONT_CYCLES,
                OPT_SINGLE_WAVEFRONT_CYCLES,
                THEORETICAL_SINGLE_WAVEFRONT_CYCLES
            )?;
        }

        if let Some(cycles) = self.sequence_64_cycles {
            let tier = ConformanceTier::from_sequence_cycles(cycles);
            writeln!(f, "64-Wavefront Seq:  {} cycles [{}]", cycles, tier)?;
            writeln!(
                f,
                "  MIN: <{}  OPT: <{}  THEORETICAL: {}",
                MIN_SEQUENCE_64_CYCLES, OPT_SEQUENCE_64_CYCLES, THEORETICAL_SEQUENCE_64_CYCLES
            )?;
        }

        if let Some(bpc) = self.bits_per_cycle {
            let tier = ConformanceTier::from_bits_per_cycle(bpc);
            writeln!(f, "Throughput:        {} bits/cycle [{}]", bpc, tier)?;
            writeln!(
                f,
                "  MIN: >={}  OPT: >={}  THEORETICAL: {}",
                MIN_BITS_PER_CYCLE, OPT_BITS_PER_CYCLE, THEORETICAL_BITS_PER_CYCLE
            )?;
        }

        if !self.violations.is_empty() {
            writeln!(f)?;
            writeln!(f, "Violations ({}):", self.violations.len())?;
            for v in &self.violations {
                writeln!(f, "  - {}", v)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn test_validate_throughput_pass() {
        // 4992 bits / 5 cycles = 998 bits/cycle (> 512 target)
        let result = validate_throughput(5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 998);
    }

    #[test]
    fn test_validate_throughput_fail() {
        // 4992 bits / 20 cycles = 249 bits/cycle (< 512 target)
        let result = validate_throughput(20);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.measured, 249);
        assert_eq!(err.target, 512);
    }

    #[test]
    fn test_validate_wavefront_latency_pass() {
        assert!(validate_wavefront_latency(1).is_ok());
        assert!(validate_wavefront_latency(5).is_ok());
    }

    #[test]
    fn test_validate_wavefront_latency_fail() {
        assert!(validate_wavefront_latency(6).is_err());
        assert!(validate_wavefront_latency(100).is_err());
    }

    #[test]
    fn test_validate_sequence_latency_pass() {
        let result = validate_sequence_latency(180);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2); // 180 / 64 = 2 avg
    }

    #[test]
    fn test_validate_sequence_latency_fail() {
        assert!(validate_sequence_latency(201).is_err());
    }

    #[test]
    fn test_bits_per_cycle_from_ns() {
        // 1 ns at 4 GHz = 4 cycles
        // 4992 bits / 4 cycles = 1248 bits/cycle
        let bpc = bits_per_cycle_from_ns(1.0, 4.0);
        assert_eq!(bpc, 1248);

        // 10 ns at 3.5 GHz = 35 cycles
        // 4992 bits / 35 cycles = 142 bits/cycle
        let bpc = bits_per_cycle_from_ns(10.0, 3.5);
        assert_eq!(bpc, 142);
    }

    #[test]
    fn test_ns_to_cycles() {
        // 1.5 ns at 3.5 GHz = 5.25 cycles → 6 (ceil)
        assert_eq!(ns_to_cycles(1.5, 3.5), 6);

        // 1.0 ns at 4.0 GHz = 4.0 cycles → 4
        assert_eq!(ns_to_cycles(1.0, 4.0), 4);

        // Edge cases
        assert_eq!(ns_to_cycles(0.0, 3.5), 0);
        assert_eq!(ns_to_cycles(1.0, 0.0), 0);
        assert_eq!(ns_to_cycles(-1.0, 3.5), 0);

        // Sub-cycle should round up to 1
        assert_eq!(ns_to_cycles(0.1, 3.5), 1);
    }

    #[test]
    fn test_conformance_report() {
        let mut report = ConformanceReport::new();

        report.record_single_wavefront(3);
        assert!(report.is_conformant());
        assert!(report.tier >= ConformanceTier::Minimum);

        report.record_sequence(250);
        assert!(!report.is_conformant());
        assert_eq!(report.violation_count(), 1);
        assert_eq!(report.tier, ConformanceTier::NonConformant);

        report.record_throughput(5);
        assert_eq!(report.violation_count(), 1); // Only sequence failed
    }

    #[test]
    fn test_conformance_report_display() {
        let mut report = ConformanceReport::new();
        report.record_single_wavefront(3);
        report.record_throughput(5);

        let output = format!("{}", report);
        assert!(output.contains("Overall Tier"));
        assert!(output.contains("Single Wavefront"));
    }

    // =============================================================================
    // TIER TESTS
    // =============================================================================

    #[test]
    fn test_tier_from_wavefront_cycles() {
        // NonConformant: > 5 cycles
        assert_eq!(
            ConformanceTier::from_wavefront_cycles(6),
            ConformanceTier::NonConformant
        );
        assert_eq!(
            ConformanceTier::from_wavefront_cycles(100),
            ConformanceTier::NonConformant
        );

        // Minimum: <= 5, > 3 cycles
        assert_eq!(
            ConformanceTier::from_wavefront_cycles(5),
            ConformanceTier::Minimum
        );
        assert_eq!(
            ConformanceTier::from_wavefront_cycles(4),
            ConformanceTier::Minimum
        );

        // Optimal: <= 3, > 1 cycles
        assert_eq!(
            ConformanceTier::from_wavefront_cycles(3),
            ConformanceTier::Optimal
        );
        assert_eq!(
            ConformanceTier::from_wavefront_cycles(2),
            ConformanceTier::Optimal
        );

        // Theoretical: <= 1 cycle
        assert_eq!(
            ConformanceTier::from_wavefront_cycles(1),
            ConformanceTier::Theoretical
        );
    }

    #[test]
    fn test_tier_from_sequence_cycles() {
        // NonConformant: > 200 cycles
        assert_eq!(
            ConformanceTier::from_sequence_cycles(201),
            ConformanceTier::NonConformant
        );

        // Minimum: <= 200, > 100 cycles
        assert_eq!(
            ConformanceTier::from_sequence_cycles(200),
            ConformanceTier::Minimum
        );
        assert_eq!(
            ConformanceTier::from_sequence_cycles(150),
            ConformanceTier::Minimum
        );

        // Optimal: <= 100, > 88 cycles
        assert_eq!(
            ConformanceTier::from_sequence_cycles(100),
            ConformanceTier::Optimal
        );
        assert_eq!(
            ConformanceTier::from_sequence_cycles(90),
            ConformanceTier::Optimal
        );

        // Theoretical: <= 88 cycles
        assert_eq!(
            ConformanceTier::from_sequence_cycles(88),
            ConformanceTier::Theoretical
        );
        assert_eq!(
            ConformanceTier::from_sequence_cycles(80),
            ConformanceTier::Theoretical
        );
    }

    #[test]
    fn test_tier_from_bits_per_cycle() {
        // NonConformant: < 512 bpc
        assert_eq!(
            ConformanceTier::from_bits_per_cycle(500),
            ConformanceTier::NonConformant
        );

        // Minimum: >= 512, < 1600 bpc
        assert_eq!(
            ConformanceTier::from_bits_per_cycle(512),
            ConformanceTier::Minimum
        );
        assert_eq!(
            ConformanceTier::from_bits_per_cycle(1000),
            ConformanceTier::Minimum
        );

        // Optimal: >= 1600, < 4992 bpc
        assert_eq!(
            ConformanceTier::from_bits_per_cycle(1600),
            ConformanceTier::Optimal
        );
        assert_eq!(
            ConformanceTier::from_bits_per_cycle(2000),
            ConformanceTier::Optimal
        );

        // Theoretical: >= 4992 bpc
        assert_eq!(
            ConformanceTier::from_bits_per_cycle(4992),
            ConformanceTier::Theoretical
        );
        assert_eq!(
            ConformanceTier::from_bits_per_cycle(5000),
            ConformanceTier::Theoretical
        );
    }

    #[test]
    fn test_tier_ordering() {
        assert!(ConformanceTier::NonConformant < ConformanceTier::Minimum);
        assert!(ConformanceTier::Minimum < ConformanceTier::Optimal);
        assert!(ConformanceTier::Optimal < ConformanceTier::Theoretical);
    }

    #[test]
    fn test_tier_min() {
        let optimal = ConformanceTier::Optimal;
        let minimum = ConformanceTier::Minimum;
        let non_conformant = ConformanceTier::NonConformant;

        assert_eq!(optimal.min(minimum), minimum);
        assert_eq!(minimum.min(optimal), minimum);
        assert_eq!(optimal.min(non_conformant), non_conformant);
    }

    #[test]
    fn test_tier_is_conformant() {
        assert!(!ConformanceTier::NonConformant.is_conformant());
        assert!(ConformanceTier::Minimum.is_conformant());
        assert!(ConformanceTier::Optimal.is_conformant());
        assert!(ConformanceTier::Theoretical.is_conformant());
    }

    #[test]
    fn test_tier_is_optimal() {
        assert!(!ConformanceTier::NonConformant.is_optimal());
        assert!(!ConformanceTier::Minimum.is_optimal());
        assert!(ConformanceTier::Optimal.is_optimal());
        assert!(ConformanceTier::Theoretical.is_optimal());
    }

    #[test]
    fn test_report_tier_computation() {
        // All optimal measurements
        let mut report = ConformanceReport::new();
        report.record_single_wavefront(2); // Optimal
        report.record_sequence(90); // Optimal
        report.record_throughput(2); // 4992/2 = 2496 bpc = Optimal
        assert_eq!(report.tier, ConformanceTier::Optimal);

        // One minimum measurement drags down tier
        let mut report = ConformanceReport::new();
        report.record_single_wavefront(2); // Optimal
        report.record_sequence(150); // Minimum
        assert_eq!(report.tier, ConformanceTier::Minimum);

        // One non-conformant measurement fails everything
        let mut report = ConformanceReport::new();
        report.record_single_wavefront(2); // Optimal
        report.record_sequence(300); // NonConformant
        assert_eq!(report.tier, ConformanceTier::NonConformant);
    }
}
