//! Selection module
//!
//! This module manages text selection and cursor positioning.
//! It provides types and operations for working with text selections
//! and cursor movement.
//!
//! # Responsibilities
//!
//! - Define the `Selection` type with anchor and focus positions
//! - Provide cursor movement operations (word, line, document boundaries)
//! - Support both forward and backward selections
//! - Handle collapsed selections (cursor positions)
//!
//! # Key Types
//!
//! - `Selection`: Represents a text selection with anchor and focus
//! - `Cursor`: Provides cursor movement operations

pub mod cursor;

use crate::document::{Position, Range};

/// Represents a text selection with anchor and focus positions
/// The anchor is where the selection started, and the focus is where it ends
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Position,
    pub focus: Position,
}

impl Selection {
    /// Creates a new Selection with the specified anchor and focus positions
    pub fn new(anchor: Position, focus: Position) -> Self {
        Self { anchor, focus }
    }

    /// Creates a collapsed selection (cursor) at the specified position
    pub fn collapsed(pos: Position) -> Self {
        Self {
            anchor: pos,
            focus: pos,
        }
    }

    /// Returns true if the selection is collapsed (anchor == focus)
    pub fn is_collapsed(&self) -> bool {
        self.anchor == self.focus
    }

    /// Returns the selection as a Range
    /// The range is normalized so that start <= end
    pub fn range(&self) -> Range {
        Range::new(self.anchor, self.focus)
    }

    /// Returns true if the selection is forward (anchor <= focus)
    pub fn is_forward(&self) -> bool {
        self.anchor.offset() <= self.focus.offset()
    }

    /// Validates the selection against document length
    /// Returns true if both anchor and focus are within bounds
    pub fn is_valid(&self, doc_length: usize) -> bool {
        self.anchor.offset() <= doc_length && self.focus.offset() <= doc_length
    }

    /// Normalizes the selection to ensure it's within document bounds
    /// Clamps both anchor and focus to [0, doc_length]
    pub fn normalize(&self, doc_length: usize) -> Self {
        Self {
            anchor: Position::new(self.anchor.offset().min(doc_length)),
            focus: Position::new(self.focus.offset().min(doc_length)),
        }
    }

    /// Adjusts the selection after text insertion
    /// If the insertion is before or within the selection, shifts positions accordingly
    pub fn adjust_for_insert(&self, insert_pos: Position, insert_length: usize) -> Self {
        let insert_offset = insert_pos.offset();

        let new_anchor = if self.anchor.offset() >= insert_offset {
            Position::new(self.anchor.offset() + insert_length)
        } else {
            self.anchor
        };

        let new_focus = if self.focus.offset() >= insert_offset {
            Position::new(self.focus.offset() + insert_length)
        } else {
            self.focus
        };

        Self {
            anchor: new_anchor,
            focus: new_focus,
        }
    }

    /// Adjusts the selection after text deletion
    /// If the deletion affects the selection, adjusts positions accordingly
    pub fn adjust_for_delete(&self, delete_range: Range) -> Self {
        let normalized = delete_range.normalize();
        let delete_start = normalized.start.offset();
        let delete_end = normalized.end.offset();
        let delete_length = delete_end - delete_start;

        let new_anchor = if self.anchor.offset() <= delete_start {
            // Anchor is before deletion, no change
            self.anchor
        } else if self.anchor.offset() >= delete_end {
            // Anchor is after deletion, shift back
            Position::new(self.anchor.offset() - delete_length)
        } else {
            // Anchor is within deletion, move to start
            Position::new(delete_start)
        };

        let new_focus = if self.focus.offset() <= delete_start {
            // Focus is before deletion, no change
            self.focus
        } else if self.focus.offset() >= delete_end {
            // Focus is after deletion, shift back
            Position::new(self.focus.offset() - delete_length)
        } else {
            // Focus is within deletion, move to start
            Position::new(delete_start)
        };

        Self {
            anchor: new_anchor,
            focus: new_focus,
        }
    }

    /// Extends the selection to include the specified position
    /// Keeps the anchor fixed and moves the focus
    pub fn extend_to(&self, pos: Position) -> Self {
        Self {
            anchor: self.anchor,
            focus: pos,
        }
    }

    /// Returns the start position of the selection (minimum of anchor and focus)
    pub fn start(&self) -> Position {
        if self.anchor.offset() <= self.focus.offset() {
            self.anchor
        } else {
            self.focus
        }
    }

    /// Returns the end position of the selection (maximum of anchor and focus)
    pub fn end(&self) -> Position {
        if self.anchor.offset() >= self.focus.offset() {
            self.anchor
        } else {
            self.focus
        }
    }

    /// Returns the length of the selection in characters
    pub fn length(&self) -> usize {
        let start = self.start().offset();
        let end = self.end().offset();
        end - start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_new() {
        let sel = Selection::new(Position::new(5), Position::new(10));
        assert_eq!(sel.anchor.offset(), 5);
        assert_eq!(sel.focus.offset(), 10);
    }

    #[test]
    fn test_selection_collapsed() {
        let sel = Selection::collapsed(Position::new(5));
        assert_eq!(sel.anchor.offset(), 5);
        assert_eq!(sel.focus.offset(), 5);
        assert!(sel.is_collapsed());
    }

    #[test]
    fn test_selection_is_collapsed() {
        let collapsed = Selection::collapsed(Position::new(5));
        let not_collapsed = Selection::new(Position::new(5), Position::new(10));

        assert!(collapsed.is_collapsed());
        assert!(!not_collapsed.is_collapsed());
    }

    #[test]
    fn test_selection_range() {
        let sel = Selection::new(Position::new(5), Position::new(10));
        let range = sel.range();

        assert_eq!(range.start.offset(), 5);
        assert_eq!(range.end.offset(), 10);
    }

    #[test]
    fn test_selection_range_backward() {
        let sel = Selection::new(Position::new(10), Position::new(5));
        let range = sel.range();

        // Range should still have the original positions
        assert_eq!(range.start.offset(), 10);
        assert_eq!(range.end.offset(), 5);

        // But when normalized, it should be correct
        let normalized = range.normalize();
        assert_eq!(normalized.start.offset(), 5);
        assert_eq!(normalized.end.offset(), 10);
    }

    #[test]
    fn test_selection_is_forward() {
        let forward = Selection::new(Position::new(5), Position::new(10));
        let backward = Selection::new(Position::new(10), Position::new(5));
        let collapsed = Selection::collapsed(Position::new(5));

        assert!(forward.is_forward());
        assert!(!backward.is_forward());
        assert!(collapsed.is_forward()); // Collapsed is considered forward
    }

    #[test]
    fn test_selection_equality() {
        let sel1 = Selection::new(Position::new(5), Position::new(10));
        let sel2 = Selection::new(Position::new(5), Position::new(10));
        let sel3 = Selection::new(Position::new(10), Position::new(5));

        assert_eq!(sel1, sel2);
        assert_ne!(sel1, sel3);
    }

    #[test]
    fn test_selection_is_valid() {
        let sel = Selection::new(Position::new(5), Position::new(10));

        assert!(sel.is_valid(20)); // Both within bounds
        assert!(sel.is_valid(10)); // Focus at boundary
        assert!(!sel.is_valid(9)); // Focus out of bounds
        assert!(!sel.is_valid(4)); // Anchor out of bounds
    }

    #[test]
    fn test_selection_normalize() {
        let sel = Selection::new(Position::new(5), Position::new(15));

        // Within bounds - no change
        let normalized = sel.normalize(20);
        assert_eq!(normalized.anchor.offset(), 5);
        assert_eq!(normalized.focus.offset(), 15);

        // Focus out of bounds - clamped
        let normalized = sel.normalize(10);
        assert_eq!(normalized.anchor.offset(), 5);
        assert_eq!(normalized.focus.offset(), 10);

        // Both out of bounds - both clamped
        let normalized = sel.normalize(3);
        assert_eq!(normalized.anchor.offset(), 3);
        assert_eq!(normalized.focus.offset(), 3);
    }

    #[test]
    fn test_selection_adjust_for_insert() {
        let sel = Selection::new(Position::new(5), Position::new(10));

        // Insert before selection - both shift
        let adjusted = sel.adjust_for_insert(Position::new(2), 3);
        assert_eq!(adjusted.anchor.offset(), 8);
        assert_eq!(adjusted.focus.offset(), 13);

        // Insert at anchor - both shift
        let adjusted = sel.adjust_for_insert(Position::new(5), 3);
        assert_eq!(adjusted.anchor.offset(), 8);
        assert_eq!(adjusted.focus.offset(), 13);

        // Insert within selection - only focus shifts
        let adjusted = sel.adjust_for_insert(Position::new(7), 3);
        assert_eq!(adjusted.anchor.offset(), 5);
        assert_eq!(adjusted.focus.offset(), 13);

        // Insert after selection - no change
        let adjusted = sel.adjust_for_insert(Position::new(15), 3);
        assert_eq!(adjusted.anchor.offset(), 5);
        assert_eq!(adjusted.focus.offset(), 10);
    }

    #[test]
    fn test_selection_adjust_for_delete() {
        let sel = Selection::new(Position::new(10), Position::new(20));

        // Delete before selection - both shift back
        let adjusted = sel.adjust_for_delete(Range::from_offsets(2, 5));
        assert_eq!(adjusted.anchor.offset(), 7);
        assert_eq!(adjusted.focus.offset(), 17);

        // Delete after selection - no change
        let adjusted = sel.adjust_for_delete(Range::from_offsets(25, 30));
        assert_eq!(adjusted.anchor.offset(), 10);
        assert_eq!(adjusted.focus.offset(), 20);

        // Delete overlapping start - anchor moves to delete start
        let adjusted = sel.adjust_for_delete(Range::from_offsets(8, 12));
        assert_eq!(adjusted.anchor.offset(), 8);
        assert_eq!(adjusted.focus.offset(), 16);

        // Delete overlapping end - focus moves to delete start
        let adjusted = sel.adjust_for_delete(Range::from_offsets(18, 25));
        assert_eq!(adjusted.anchor.offset(), 10);
        assert_eq!(adjusted.focus.offset(), 18);

        // Delete entire selection - both collapse to delete start
        let adjusted = sel.adjust_for_delete(Range::from_offsets(5, 25));
        assert_eq!(adjusted.anchor.offset(), 5);
        assert_eq!(adjusted.focus.offset(), 5);
    }

    #[test]
    fn test_selection_extend_to() {
        let sel = Selection::new(Position::new(5), Position::new(10));

        // Extend forward
        let extended = sel.extend_to(Position::new(15));
        assert_eq!(extended.anchor.offset(), 5);
        assert_eq!(extended.focus.offset(), 15);

        // Extend backward
        let extended = sel.extend_to(Position::new(3));
        assert_eq!(extended.anchor.offset(), 5);
        assert_eq!(extended.focus.offset(), 3);
    }

    #[test]
    fn test_selection_start_end() {
        // Forward selection
        let sel = Selection::new(Position::new(5), Position::new(10));
        assert_eq!(sel.start().offset(), 5);
        assert_eq!(sel.end().offset(), 10);

        // Backward selection
        let sel = Selection::new(Position::new(10), Position::new(5));
        assert_eq!(sel.start().offset(), 5);
        assert_eq!(sel.end().offset(), 10);

        // Collapsed selection
        let sel = Selection::collapsed(Position::new(7));
        assert_eq!(sel.start().offset(), 7);
        assert_eq!(sel.end().offset(), 7);
    }

    #[test]
    fn test_selection_length() {
        let sel = Selection::new(Position::new(5), Position::new(10));
        assert_eq!(sel.length(), 5);

        let sel = Selection::new(Position::new(10), Position::new(5));
        assert_eq!(sel.length(), 5);

        let sel = Selection::collapsed(Position::new(5));
        assert_eq!(sel.length(), 0);
    }
}
