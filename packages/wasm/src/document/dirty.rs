use crate::document::{Position, Range};

/// Represents a region of the document that has been modified
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirtyRegion {
    pub range: Range,
}

impl DirtyRegion {
    /// Creates a new dirty region
    pub fn new(range: Range) -> Self {
        Self { range }
    }

    /// Returns true if this region overlaps with another region
    pub fn overlaps(&self, other: &DirtyRegion) -> bool {
        let self_range = self.range.normalize();
        let other_range = other.range.normalize();

        self_range.start_offset() < other_range.end_offset()
            && self_range.end_offset() > other_range.start_offset()
    }

    /// Returns true if this region is adjacent to another region
    pub fn is_adjacent(&self, other: &DirtyRegion) -> bool {
        let self_range = self.range.normalize();
        let other_range = other.range.normalize();

        self_range.end_offset() == other_range.start_offset()
            || other_range.end_offset() == self_range.start_offset()
    }

    /// Merges this region with another region
    pub fn merge(&self, other: &DirtyRegion) -> DirtyRegion {
        let self_range = self.range.normalize();
        let other_range = other.range.normalize();

        let start = Position::new(self_range.start_offset().min(other_range.start_offset()));
        let end = Position::new(self_range.end_offset().max(other_range.end_offset()));

        DirtyRegion::new(Range::new(start, end))
    }
}

/// Tracks dirty regions in the document for incremental updates
#[derive(Debug, Clone)]
pub struct DirtyTracker {
    regions: Vec<DirtyRegion>,
}

impl DirtyTracker {
    /// Creates a new empty dirty tracker
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    /// Marks a region as dirty
    pub fn mark_dirty(&mut self, range: Range) {
        let new_region = DirtyRegion::new(range);

        // Try to merge with existing regions
        let mut merged = false;
        let mut i = 0;

        while i < self.regions.len() {
            if self.regions[i].overlaps(&new_region) || self.regions[i].is_adjacent(&new_region) {
                // Merge the regions
                let merged_region = self.regions[i].merge(&new_region);
                self.regions.remove(i);

                // Recursively merge with other regions
                self.mark_dirty(merged_region.range);
                merged = true;
                break;
            }
            i += 1;
        }

        if !merged {
            self.regions.push(new_region);
        }
    }

    /// Returns all dirty regions
    pub fn get_dirty_regions(&self) -> Vec<Range> {
        self.regions.iter().map(|r| r.range).collect()
    }

    /// Clears all dirty flags
    pub fn clear_dirty_flags(&mut self) {
        self.regions.clear();
    }

    /// Returns true if there are any dirty regions
    pub fn has_dirty_regions(&self) -> bool {
        !self.regions.is_empty()
    }

    /// Adjusts dirty regions after text insertion
    pub fn adjust_for_insert(&mut self, pos: Position, length: usize) {
        let insert_offset = pos.offset();

        for region in &mut self.regions {
            let start_offset = region.range.start.offset();
            let end_offset = region.range.end.offset();

            if start_offset >= insert_offset {
                region.range.start = Position::new(start_offset + length);
            }
            if end_offset >= insert_offset {
                region.range.end = Position::new(end_offset + length);
            }
        }
    }

    /// Adjusts dirty regions after text deletion
    pub fn adjust_for_delete(&mut self, range: Range) {
        let normalized = range.normalize();
        let delete_start = normalized.start_offset();
        let delete_end = normalized.end_offset();
        let delete_length = delete_end - delete_start;

        let mut i = 0;
        while i < self.regions.len() {
            let region = &mut self.regions[i];
            let start_offset = region.range.start.offset();
            let end_offset = region.range.end.offset();

            // Region is completely within deleted range - remove it
            if start_offset >= delete_start && end_offset <= delete_end {
                self.regions.remove(i);
                continue;
            }

            // Region starts before and ends after deletion - adjust end
            if start_offset < delete_start && end_offset > delete_end {
                region.range.end = Position::new(end_offset - delete_length);
            }
            // Region starts before deletion and ends within it - truncate end
            else if start_offset < delete_start && end_offset > delete_start {
                region.range.end = Position::new(delete_start);
            }
            // Region starts within deletion and ends after it - adjust start
            else if start_offset < delete_end && end_offset > delete_end {
                region.range.start = Position::new(delete_start);
                region.range.end = Position::new(end_offset - delete_length);
            }
            // Region is completely after deletion - shift it
            else if start_offset >= delete_end {
                region.range.start = Position::new(start_offset - delete_length);
                region.range.end = Position::new(end_offset - delete_length);
            }

            i += 1;
        }
    }
}

impl Default for DirtyTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirty_region_creation() {
        let range = Range::from_offsets(0, 10);
        let region = DirtyRegion::new(range);
        assert_eq!(region.range, range);
    }

    #[test]
    fn test_dirty_region_overlaps() {
        let region1 = DirtyRegion::new(Range::from_offsets(0, 10));
        let region2 = DirtyRegion::new(Range::from_offsets(5, 15));
        let region3 = DirtyRegion::new(Range::from_offsets(20, 30));

        assert!(region1.overlaps(&region2));
        assert!(region2.overlaps(&region1));
        assert!(!region1.overlaps(&region3));
    }

    #[test]
    fn test_dirty_region_adjacent() {
        let region1 = DirtyRegion::new(Range::from_offsets(0, 10));
        let region2 = DirtyRegion::new(Range::from_offsets(10, 20));
        let region3 = DirtyRegion::new(Range::from_offsets(25, 30));

        assert!(region1.is_adjacent(&region2));
        assert!(region2.is_adjacent(&region1));
        assert!(!region1.is_adjacent(&region3));
    }

    #[test]
    fn test_dirty_region_merge() {
        let region1 = DirtyRegion::new(Range::from_offsets(0, 10));
        let region2 = DirtyRegion::new(Range::from_offsets(5, 15));

        let merged = region1.merge(&region2);
        assert_eq!(merged.range.start_offset(), 0);
        assert_eq!(merged.range.end_offset(), 15);
    }

    #[test]
    fn test_dirty_tracker_mark_dirty() {
        let mut tracker = DirtyTracker::new();

        tracker.mark_dirty(Range::from_offsets(0, 10));
        assert_eq!(tracker.get_dirty_regions().len(), 1);
        assert!(tracker.has_dirty_regions());
    }

    #[test]
    fn test_dirty_tracker_merge_overlapping() {
        let mut tracker = DirtyTracker::new();

        tracker.mark_dirty(Range::from_offsets(0, 10));
        tracker.mark_dirty(Range::from_offsets(5, 15));

        // Should be merged into one region
        let regions = tracker.get_dirty_regions();
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].start_offset(), 0);
        assert_eq!(regions[0].end_offset(), 15);
    }

    #[test]
    fn test_dirty_tracker_merge_adjacent() {
        let mut tracker = DirtyTracker::new();

        tracker.mark_dirty(Range::from_offsets(0, 10));
        tracker.mark_dirty(Range::from_offsets(10, 20));

        // Should be merged into one region
        let regions = tracker.get_dirty_regions();
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].start_offset(), 0);
        assert_eq!(regions[0].end_offset(), 20);
    }

    #[test]
    fn test_dirty_tracker_separate_regions() {
        let mut tracker = DirtyTracker::new();

        tracker.mark_dirty(Range::from_offsets(0, 10));
        tracker.mark_dirty(Range::from_offsets(20, 30));

        // Should remain as two separate regions
        let regions = tracker.get_dirty_regions();
        assert_eq!(regions.len(), 2);
    }

    #[test]
    fn test_dirty_tracker_clear() {
        let mut tracker = DirtyTracker::new();

        tracker.mark_dirty(Range::from_offsets(0, 10));
        tracker.clear_dirty_flags();

        assert_eq!(tracker.get_dirty_regions().len(), 0);
        assert!(!tracker.has_dirty_regions());
    }

    #[test]
    fn test_dirty_tracker_adjust_for_insert() {
        let mut tracker = DirtyTracker::new();

        tracker.mark_dirty(Range::from_offsets(10, 20));
        tracker.adjust_for_insert(Position::new(5), 5);

        let regions = tracker.get_dirty_regions();
        assert_eq!(regions[0].start_offset(), 15);
        assert_eq!(regions[0].end_offset(), 25);
    }

    #[test]
    fn test_dirty_tracker_adjust_for_delete() {
        let mut tracker = DirtyTracker::new();

        tracker.mark_dirty(Range::from_offsets(20, 30));
        tracker.adjust_for_delete(Range::from_offsets(5, 15));

        let regions = tracker.get_dirty_regions();
        assert_eq!(regions[0].start_offset(), 10);
        assert_eq!(regions[0].end_offset(), 20);
    }

    #[test]
    fn test_dirty_tracker_delete_within_region() {
        let mut tracker = DirtyTracker::new();

        tracker.mark_dirty(Range::from_offsets(10, 30));
        tracker.adjust_for_delete(Range::from_offsets(15, 20));

        let regions = tracker.get_dirty_regions();
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].start_offset(), 10);
        assert_eq!(regions[0].end_offset(), 25);
    }
}
