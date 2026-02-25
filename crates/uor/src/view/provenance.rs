//! Provenance tracking for view compositions.
//!
//! This module provides data lineage tracking for ElementWiseViews, allowing
//! you to trace which transformations were applied to produce a final view.
//! This is crucial for debugging, auditing, and understanding compilation
//! pipeline decisions.

use super::ElementWiseView;
use core::fmt;

/// Unique identifier for a view, computed from its content.
///
/// This is a content-addressed ID - identical views will have identical IDs,
/// enabling deduplication and caching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewId(pub [u8; 32]);

impl ViewId {
    /// Compute a view ID from a 256-byte lookup table.
    ///
    /// Uses a simple hash for now - in production this would use SHA-256
    /// or BLAKE3 for content addressing.
    #[must_use]
    pub fn from_table(table: &[u8; 256]) -> Self {
        // Simple content hash (FNV-1a variant)
        let mut hash = [0u8; 32];
        let mut state: u64 = 0xcbf2_9ce4_8422_2325; // FNV offset basis

        for &byte in table {
            state ^= u64::from(byte);
            state = state.wrapping_mul(0x100_0000_01b3); // FNV prime
        }

        // Spread the 64-bit hash across 32 bytes
        for (i, chunk) in hash.chunks_mut(8).enumerate() {
            let val = state.wrapping_add(i as u64);
            chunk.copy_from_slice(&val.to_le_bytes());
        }

        Self(hash)
    }

    /// Get the hash bytes.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for ViewId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display as hex (first 16 bytes for brevity)
        for byte in &self.0[..16] {
            write!(f, "{byte:02x}")?;
        }
        write!(f, "...")
    }
}

/// Provenance information for a view.
///
/// Tracks the composition history and metadata for debugging and auditing.
#[derive(Debug, Clone)]
pub struct ViewProvenance {
    /// Unique identifier for this view (content hash)
    pub id: ViewId,

    /// Human-readable operation name
    pub operation: &'static str,

    /// Source views that were composed to create this view
    pub sources: Vec<ViewId>,

    /// Timestamp when this view was created (monotonic)
    #[cfg(feature = "std")]
    pub timestamp: std::time::Instant,
}

impl ViewProvenance {
    /// Create provenance for a new view.
    #[must_use]
    pub fn new(id: ViewId, operation: &'static str) -> Self {
        Self {
            id,
            operation,
            sources: Vec::new(),
            #[cfg(feature = "std")]
            timestamp: std::time::Instant::now(),
        }
    }

    /// Create provenance for a composed view.
    #[must_use]
    pub fn composed(id: ViewId, operation: &'static str, sources: Vec<ViewId>) -> Self {
        Self {
            id,
            operation,
            sources,
            #[cfg(feature = "std")]
            timestamp: std::time::Instant::now(),
        }
    }

    /// Check if this view is a composition of multiple views.
    #[inline]
    #[must_use]
    pub fn is_composed(&self) -> bool {
        !self.sources.is_empty()
    }

    /// Get the number of source views.
    #[inline]
    #[must_use]
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }
}

/// A view with provenance tracking.
///
/// This wraps an `ElementWiseView` with metadata about its origin and
/// composition history, enabling data lineage tracing through the
/// compilation pipeline.
///
/// # Examples
///
/// ```
/// use uor::view::{ElementWiseView, TrackedView};
///
/// // Create a tracked view
/// let inc = TrackedView::new(
///     ElementWiseView::new(|x| x.wrapping_add(1)),
///     "increment"
/// );
///
/// let double_inc = inc.then_tracked(
///     ElementWiseView::new(|x| x.wrapping_add(1)),
///     "increment"
/// );
///
/// // Check provenance
/// assert!(double_inc.provenance().is_composed());
/// assert_eq!(double_inc.provenance().source_count(), 1);
/// ```
pub struct TrackedView {
    /// The underlying view
    view: ElementWiseView,

    /// Provenance metadata
    provenance: ViewProvenance,
}

impl TrackedView {
    /// Create a new tracked view.
    #[must_use]
    pub fn new(view: ElementWiseView, operation: &'static str) -> Self {
        let id = ViewId::from_table(view.table());
        Self {
            view,
            provenance: ViewProvenance::new(id, operation),
        }
    }

    /// Get the underlying view.
    #[inline]
    #[must_use]
    pub const fn view(&self) -> &ElementWiseView {
        &self.view
    }

    /// Get the provenance information.
    #[inline]
    #[must_use]
    pub const fn provenance(&self) -> &ViewProvenance {
        &self.provenance
    }

    /// Get the view ID.
    #[inline]
    #[must_use]
    pub const fn id(&self) -> ViewId {
        self.provenance.id
    }

    /// Compose with another view, tracking provenance.
    #[must_use]
    pub fn then_tracked(self, other: ElementWiseView, operation: &'static str) -> Self {
        let composed_view = self.view.then(&other);
        let new_id = ViewId::from_table(composed_view.table());

        Self {
            view: composed_view,
            provenance: ViewProvenance::composed(new_id, operation, vec![self.provenance.id]),
        }
    }

    /// Compose two tracked views, merging provenance.
    #[must_use]
    pub fn compose(self, other: Self, operation: &'static str) -> Self {
        let composed_view = self.view.then(other.view());
        let new_id = ViewId::from_table(composed_view.table());

        Self {
            view: composed_view,
            provenance: ViewProvenance::composed(
                new_id,
                operation,
                vec![self.provenance.id, other.provenance.id],
            ),
        }
    }

    /// Apply the view to a single byte.
    #[inline]
    #[must_use]
    pub fn apply(&self, byte: u8) -> u8 {
        self.view.apply(byte)
    }

    /// Apply the view to a slice in place.
    pub fn apply_slice(&self, data: &mut [u8]) {
        self.view.apply_slice(data);
    }

    /// Apply the view to an input slice, writing to output.
    pub fn apply_to(&self, input: &[u8], output: &mut [u8]) {
        self.view.apply_to(input, output);
    }

    /// Consume the tracked view and return the underlying view.
    #[inline]
    #[must_use]
    pub fn into_view(self) -> ElementWiseView {
        self.view
    }
}

impl fmt::Debug for TrackedView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrackedView")
            .field("id", &self.provenance.id)
            .field("operation", &self.provenance.operation)
            .field("sources", &self.provenance.sources.len())
            .field("view", &"<256-byte table>")
            .finish()
    }
}

impl Clone for TrackedView {
    fn clone(&self) -> Self {
        Self {
            view: self.view,
            provenance: self.provenance.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_id_from_table() {
        let table1 = [0u8; 256];
        let table2 = [0u8; 256];
        let mut table3 = [0u8; 256];
        table3[0] = 1;

        let id1 = ViewId::from_table(&table1);
        let id2 = ViewId::from_table(&table2);
        let id3 = ViewId::from_table(&table3);

        // Same tables produce same IDs
        assert_eq!(id1, id2);

        // Different tables produce different IDs
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_view_id_display() {
        let table = [0u8; 256];
        let id = ViewId::from_table(&table);
        let display = format!("{id}");

        // Should be 32 hex chars + "..."
        assert_eq!(display.len(), 35);
        assert!(display.ends_with("..."));
    }

    #[test]
    fn test_tracked_view_creation() {
        let view = ElementWiseView::identity();
        let tracked = TrackedView::new(view, "identity");

        assert_eq!(tracked.provenance().operation, "identity");
        assert!(!tracked.provenance().is_composed());
        assert_eq!(tracked.provenance().source_count(), 0);
    }

    #[test]
    fn test_tracked_view_composition() {
        let inc = TrackedView::new(ElementWiseView::new(|x| x.wrapping_add(1)), "increment");

        let double_inc = inc.then_tracked(
            ElementWiseView::new(|x| x.wrapping_add(1)),
            "double_increment",
        );

        assert_eq!(double_inc.provenance().operation, "double_increment");
        assert!(double_inc.provenance().is_composed());
        assert_eq!(double_inc.provenance().source_count(), 1);

        // Verify it actually works
        assert_eq!(double_inc.apply(5), 7);
    }

    #[test]
    fn test_tracked_view_compose_two() {
        let view1 = TrackedView::new(ElementWiseView::new(|x| x.wrapping_add(1)), "increment");

        let view2 = TrackedView::new(ElementWiseView::new(|x| x ^ 0xFF), "invert");

        let composed = view1.compose(view2, "increment_then_invert");

        assert_eq!(composed.provenance().operation, "increment_then_invert");
        assert!(composed.provenance().is_composed());
        assert_eq!(composed.provenance().source_count(), 2);

        // Verify: increment then invert
        assert_eq!(composed.apply(0), 0xFE); // (0 + 1) ^ 0xFF = 0xFE
    }

    #[test]
    fn test_tracked_view_apply_operations() {
        let tracked = TrackedView::new(ElementWiseView::new(|x| x.wrapping_add(10)), "add_10");

        // Single byte
        assert_eq!(tracked.apply(5), 15);

        // Slice in place
        let mut data = [0, 1, 2, 3, 4];
        tracked.apply_slice(&mut data);
        assert_eq!(data, [10, 11, 12, 13, 14]);

        // Slice with output
        let input = [0, 1, 2];
        let mut output = [0u8; 3];
        tracked.apply_to(&input, &mut output);
        assert_eq!(output, [10, 11, 12]);
    }

    #[test]
    fn test_tracked_view_into_view() {
        let tracked = TrackedView::new(ElementWiseView::identity(), "identity");
        let view = tracked.into_view();

        assert_eq!(view.apply(42), 42);
    }
}
