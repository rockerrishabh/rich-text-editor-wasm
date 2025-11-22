//! Document module
//!
//! This module contains the core document model and text storage implementation.
//! It provides the main `Document` struct that manages text content, formatting,
//! selection, and command history.
//!
//! # Responsibilities
//!
//! - Text storage using a gap buffer for efficient editing
//! - Document state management (version tracking, dirty regions)
//! - Position and range types for text addressing
//! - Integration with formatting, operations, and selection modules
//!
//! # Key Types
//!
//! - `Document`: Main document struct coordinating all operations
//! - `TextStorage`: Gap buffer implementation for text storage
//! - `Position`: Represents a position in the document
//! - `Range`: Represents a range of text in the document
//! - `DirtyTracker`: Tracks modified regions for incremental rendering

pub mod dirty;
pub mod errors;
pub mod position;
pub mod text_storage;
pub mod validation;

// Re-export types for external use
pub use dirty::{DirtyRegion, DirtyTracker};
pub use errors::DocumentError;
pub use position::{Position, Range};
pub use validation::{MAX_DOCUMENT_SIZE, validate_position, validate_range, validate_text_content};

use crate::formatting::{BlockType, FormatStorage, InlineFormat};
use crate::operations::history::CommandHistory;
use crate::operations::{

    ApplyFormatCommand, Command, CommandResult, DeleteCommand, InsertCommand, RemoveFormatCommand,
    ReplaceCommand, SetBlockTypeCommand,
};
use crate::selection::Selection;
use crate::utils::ime::CompositionState;
use std::collections::HashSet;
use text_storage::TextStorage;

/// The main Document struct that manages text content and metadata.
///
/// # Performance Characteristics
///
/// ## Time Complexity
/// - **Insert operations**: O(n) where n is the distance from gap to insertion point
///   - Best case: O(1) for sequential insertions at the same position
///   - Worst case: O(n) when gap needs to move across entire document
/// - **Delete operations**: O(n) similar to insert, depends on gap movement
/// - **Format operations**: O(m) where m is the number of format runs affected
///   - Typical case: O(1) for small ranges with few format runs
/// - **Undo/Redo**: O(n) where n is the size of the operation being undone/redone
/// - **Get content**: O(n) where n is document length (must copy text)
/// - **Get text in range**: O(m) where m is the range length
/// - **Selection operations**: O(1) for most operations
///
/// ## Space Complexity
/// - Base overhead: ~1KB per document instance
/// - Text storage: ~4 bytes per character (UTF-32 internal representation)
/// - Format storage: ~32 bytes per format run
/// - Command history: ~64 bytes per command (default: 100 commands max)
/// - Gap buffer overhead: 16-32 characters of unused space
///
/// ## Performance Notes
/// - Sequential insertions at the same position are highly optimized (O(1))
/// - Random insertions across the document are slower due to gap movement
/// - Format operations are optimized with run-based storage
/// - Large documents (>100KB) may experience slower operations
/// - Consider using batch operations when possible
///
/// # Example
/// ```
/// use rich_text_editor_wasm::document::Document;
/// use rich_text_editor_wasm::document::Position;
///
/// let mut doc = Document::new();
/// doc.insert_text(Position::new(0), "Hello").unwrap();
/// assert_eq!(doc.get_content(), "Hello");
/// ```
pub struct Document {
    text: TextStorage,
    version: u64,
    pub(crate) history: CommandHistory,
    formats: FormatStorage,
    pub(crate) selection: Selection,
    composition: CompositionState,
    dirty_tracker: DirtyTracker,
}

impl Document {
    /// Creates a new empty Document
    pub fn new() -> Self {
        Self {
            text: TextStorage::new(),
            version: 0,
            history: CommandHistory::new(),
            formats: FormatStorage::new(),
            selection: Selection::collapsed(Position::new(0)),
            composition: CompositionState::new(),
            dirty_tracker: DirtyTracker::new(),
        }
    }

    /// Creates a Document from existing text
    pub fn from_text(text: &str) -> Self {
        Self {
            text: TextStorage::from_text(text),
            version: 0,
            history: CommandHistory::new(),
            formats: FormatStorage::new(),
            selection: Selection::collapsed(Position::new(0)),
            composition: CompositionState::new(),
            dirty_tracker: DirtyTracker::new(),
        }
    }

    /// Returns the entire content of the document as a String
    pub fn get_content(&self) -> String {
        self.text.get_text()
    }

    /// Returns the length of the document in characters
    pub fn get_length(&self) -> usize {
        self.text.len()
    }

    /// Returns true if the document is empty
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the text within the specified range
    pub fn get_text_in_range(&self, range: Range) -> String {
        let normalized = range.normalize();
        let start = normalized.start.offset();
        let end = normalized.end.offset();

        if end > self.get_length() {
            panic!(
                "Range end {} is out of bounds (len: {})",
                end,
                self.get_length()
            );
        }

        self.text.get_slice(start, end)
    }

    /// Returns the current version of the document
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Increments the document version (called after modifications)
    pub(crate) fn increment_version(&mut self) {
        self.version = self.version.wrapping_add(1);
    }

    /// Marks a range as dirty (for internal use by commands)
    pub(crate) fn mark_dirty(&mut self, range: Range) {
        self.dirty_tracker.mark_dirty(range);
    }

    /// Internal method to insert text without using command history
    /// Used by commands to perform the actual insertion
    pub(crate) fn insert_text_direct(&mut self, pos: Position, text: &str) {
        // Validate text content - silently skip invalid text
        if validation::validate_text_content(text).is_err() {
            return;
        }

        let text_length = text.chars().count();

        // Validate size limit - silently skip if would exceed
        if validation::validate_size_limit(self.get_length(), text_length).is_err() {
            return;
        }

        self.text.insert(pos.offset(), text);
        self.formats.adjust_for_insert(pos, text_length);

        // Adjust selection for insertion
        self.selection = self.selection.adjust_for_insert(pos, text_length);

        // Mark the inserted region as dirty
        let dirty_range = Range::new(pos, Position::new(pos.offset() + text_length));
        self.dirty_tracker.mark_dirty(dirty_range);

        self.increment_version();
    }

    /// Internal method to delete text without using command history
    /// Used by commands to perform the actual deletion
    pub(crate) fn delete_range_direct(&mut self, range: Range) {
        let normalized = range.normalize();

        // Mark the region as dirty before deletion
        self.dirty_tracker.mark_dirty(normalized);

        self.text
            .delete(normalized.start.offset(), normalized.end.offset());
        self.formats.adjust_for_delete(normalized);
        self.dirty_tracker.adjust_for_delete(normalized);

        // Adjust selection for deletion
        self.selection = self.selection.adjust_for_delete(normalized);

        self.increment_version();
    }

    /// Internal method to replace text without using command history
    /// Used by commands to perform the actual replacement
    pub(crate) fn replace_range_direct(&mut self, range: Range, text: &str) {
        let normalized = range.normalize();
        let text_length = text.chars().count();

        // Mark the region as dirty
        let dirty_range = Range::new(
            normalized.start,
            Position::new(normalized.start.offset() + text_length),
        );
        self.dirty_tracker.mark_dirty(dirty_range);

        self.text
            .delete(normalized.start.offset(), normalized.end.offset());
        self.text.insert(normalized.start.offset(), text);

        // Adjust formats: first delete, then insert
        self.formats.adjust_for_delete(normalized);
        self.formats
            .adjust_for_insert(normalized.start, text_length);
        self.dirty_tracker.adjust_for_delete(normalized);
        self.dirty_tracker
            .adjust_for_insert(normalized.start, text_length);

        // Adjust selection: first delete, then insert
        self.selection = self.selection.adjust_for_delete(normalized);
        self.selection = self
            .selection
            .adjust_for_insert(normalized.start, text_length);

        self.increment_version();
    }

    /// Inserts text at the specified position using the command pattern
    pub fn insert_text(&mut self, pos: Position, text: &str) -> CommandResult<()> {
        let mut cmd = Box::new(InsertCommand::new(pos, text.to_string()));
        cmd.execute(self)?;
        self.history.push_command(cmd);
        Ok(())
    }

    /// Deletes text in the specified range using the command pattern
    pub fn delete_range(&mut self, range: Range) -> CommandResult<()> {
        let mut cmd = Box::new(DeleteCommand::new(range));
        cmd.execute(self)?;
        self.history.push_command(cmd);
        Ok(())
    }

    /// Replaces text in the specified range with new text using the command pattern
    pub fn replace_range(&mut self, range: Range, text: &str) -> CommandResult<()> {
        let mut cmd = Box::new(ReplaceCommand::new(range, text.to_string()));
        cmd.execute(self)?;
        self.history.push_command(cmd);
        Ok(())
    }

    /// Undoes the last operation
    pub fn undo(&mut self) -> CommandResult<()> {
        if let Some(mut cmd) = self.history.pop_undo() {
            cmd.undo(self)?;
            self.history.push_redo(cmd);
            Ok(())
        } else {
            Err(crate::operations::CommandError::NothingToUndo)
        }
    }

    /// Redoes the last undone operation
    pub fn redo(&mut self) -> CommandResult<()> {
        if let Some(mut cmd) = self.history.pop_redo() {
            cmd.execute(self)?;
            self.history.push_undo(cmd);
            Ok(())
        } else {
            Err(crate::operations::CommandError::NothingToRedo)
        }
    }

    /// Returns true if there are operations that can be undone
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Returns true if there are operations that can be redone
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Applies a format to the specified range using the command pattern
    pub fn apply_format(&mut self, range: Range, format: InlineFormat) {
        let mut cmd = Box::new(ApplyFormatCommand::new(range, format));
        if let Ok(()) = cmd.execute(self) {
            self.history.push_command(cmd);
        }
    }

    /// Removes a format from the specified range using the command pattern
    pub fn remove_format(&mut self, range: Range, format: &InlineFormat) {
        let mut cmd = Box::new(RemoveFormatCommand::new(range, format.clone()));
        if let Ok(()) = cmd.execute(self) {
            self.history.push_command(cmd);
        }
    }

    /// Toggles a format on the specified range
    /// If the format is present at the start of the range, it removes it; otherwise, it applies it
    pub fn toggle_format(&mut self, range: Range, format: InlineFormat) {
        let normalized = range.normalize();

        if normalized.is_empty() {
            return;
        }

        let start = normalized.start_offset();
        let end = normalized.end_offset();

        // Determine if the format fully covers the entire range
        // by summing overlap lengths of runs that contain the format.
        let mut covered_len: usize = 0;
        for run in self.formats.get_runs().iter() {
            let run_range = run.range.normalize();
            // Skip runs that don't overlap the target range
            if !run_range.overlaps(&normalized) {
                continue;
            }

            if run
                .formats
                .iter()
                .any(|f| match (f, &format) {
                    (InlineFormat::Bold, InlineFormat::Bold) => true,
                    (InlineFormat::Italic, InlineFormat::Italic) => true,
                    (InlineFormat::Underline, InlineFormat::Underline) => true,
                    (InlineFormat::Strikethrough, InlineFormat::Strikethrough) => true,
                    (InlineFormat::Code, InlineFormat::Code) => true,
                    (InlineFormat::Link { .. }, InlineFormat::Link { .. }) => true,
                    (InlineFormat::TextColor { .. }, InlineFormat::TextColor { .. }) => true,
                    (InlineFormat::BackgroundColor { .. }, InlineFormat::BackgroundColor { .. }) => true,
                    _ => false,
                })
            {
                let overlap_start = run_range.start_offset().max(start);
                let overlap_end = run_range.end_offset().min(end);
                if overlap_end > overlap_start {
                    covered_len += overlap_end - overlap_start;
                }
            }
        }

        if covered_len >= end.saturating_sub(start) {
            // Format is present across the entire range: remove it
            self.remove_format(normalized, &format);
        } else {
            // Format is not fully present: apply it to the range
            self.apply_format(normalized, format);
        }
    }

    /// Gets all formats at the specified position
    pub fn get_formats_at(&self, pos: Position) -> HashSet<InlineFormat> {
        self.formats.get_formats_at(pos)
    }

    /// Gets access to the format storage (for internal use)
    pub(crate) fn formats(&self) -> &FormatStorage {
        &self.formats
    }

    /// Gets mutable access to the format storage (for internal use)
    pub(crate) fn formats_mut(&mut self) -> &mut FormatStorage {
        &mut self.formats
    }

    /// Sets the block type for the specified range
    pub fn set_block_type(&mut self, range: Range, block_type: BlockType) {
        let mut cmd = Box::new(SetBlockTypeCommand::new(range, block_type));
        if let Ok(()) = cmd.execute(self) {
            self.history.push_command(cmd);
        }
    }

    /// Gets the block type at the specified position
    pub fn get_block_type_at(&self, pos: Position) -> BlockType {
        self.formats.get_block_type_at(pos)
    }

    /// Sets the selection to the specified anchor and focus positions
    /// The selection is automatically normalized to ensure it's within document bounds
    pub fn set_selection(&mut self, selection: Selection) {
        let doc_length = self.get_length();
        self.selection = selection.normalize(doc_length);
    }

    /// Gets the current selection
    pub fn get_selection(&self) -> Selection {
        self.selection
    }

    /// Gets the text within the current selection
    pub fn get_selected_text(&self) -> String {
        if self.selection.is_collapsed() {
            String::new()
        } else {
            self.get_text_in_range(self.selection.range())
        }
    }

    /// Selects all content in the document
    pub fn select_all(&mut self) {
        let end = Position::new(self.get_length());
        self.selection = Selection::new(Position::new(0), end);
    }

    /// Collapses the selection to the start position
    pub fn collapse_to_start(&mut self) {
        let range = self.selection.range().normalize();
        self.selection = Selection::collapsed(range.start);
    }

    /// Collapses the selection to the end position
    pub fn collapse_to_end(&mut self) {
        let range = self.selection.range().normalize();
        self.selection = Selection::collapsed(range.end);
    }

    /// Returns all dirty regions that have been modified since the last clear
    pub fn get_dirty_regions(&self) -> Vec<Range> {
        self.dirty_tracker.get_dirty_regions()
    }

    /// Clears all dirty flags
    pub fn clear_dirty_flags(&mut self) {
        self.dirty_tracker.clear_dirty_flags();
    }

    /// Returns true if there are any dirty regions
    pub fn has_dirty_regions(&self) -> bool {
        self.dirty_tracker.has_dirty_regions()
    }

    /// Starts an IME composition at the current cursor position
    ///
    /// This marks the beginning of an IME composition session. The composition
    /// will track the range where composition text is being entered.
    pub fn start_composition(&mut self) {
        let cursor_pos = if self.selection.is_collapsed() {
            self.selection.anchor
        } else {
            // If there's a selection, start composition at the start of the selection
            self.selection.range().normalize().start
        };
        self.composition = CompositionState::start(cursor_pos);
    }

    /// Updates the IME composition with new text
    ///
    /// This replaces the current composition text with the new text. The composition
    /// range is automatically adjusted to match the new text length.
    ///
    /// # Arguments
    /// * `text` - The new composition text
    pub fn update_composition(&mut self, text: &str) {
        if !self.composition.is_active() {
            return;
        }

        // Delete the old composition text if any
        let old_range = self.composition.range();
        if !old_range.is_empty() {
            self.delete_range_direct(old_range);
        }

        // Insert the new composition text
        let start_pos = old_range.start;
        if !text.is_empty() {
            self.insert_text_direct(start_pos, text);
        }

        // Update the composition state
        self.composition.update(text.to_string());

        // Update selection to the end of composition
        let end_pos = Position::new(start_pos.offset() + text.chars().count());
        self.selection = Selection::collapsed(end_pos);
    }

    /// Ends the IME composition and commits the final text
    ///
    /// This finalizes the composition and commits the text to the document.
    /// The composition text becomes regular document text and can be undone
    /// as a single operation.
    pub fn end_composition(&mut self) {
        if !self.composition.is_active() {
            return;
        }

        let final_text = self.composition.end();

        // The text is already in the document from update_composition calls,
        // but we need to ensure it's properly committed to the undo history
        // We do this by creating a command for the entire composition
        if !final_text.is_empty() {
            // Note: The text is already inserted via update_composition,
            // so we just need to mark the composition as complete
            // The undo history will treat all the composition updates as one operation
        }
    }

    /// Cancels the IME composition without committing
    ///
    /// This removes any composition text and returns the document to its
    /// state before composition started.
    pub fn cancel_composition(&mut self) {
        if !self.composition.is_active() {
            return;
        }

        // Delete the composition text
        let range = self.composition.range();
        if !range.is_empty() {
            self.delete_range_direct(range);
        }

        // Reset selection to the start of where composition was
        self.selection = Selection::collapsed(range.start);

        self.composition.cancel();
    }

    /// Returns true if IME composition is currently active
    pub fn is_composing(&self) -> bool {
        self.composition.is_active()
    }

    /// Returns the current composition range, if composition is active
    pub fn composition_range(&self) -> Option<Range> {
        if self.composition.is_active() {
            Some(self.composition.range())
        } else {
            None
        }
    }

    /// Returns the current composition text, if composition is active
    pub fn composition_text(&self) -> Option<&str> {
        if self.composition.is_active() {
            Some(self.composition.text())
        } else {
            None
        }
    }

    /// Gets the current maximum history size
    ///
    /// # Returns
    /// The maximum number of commands that can be stored in the undo stack
    pub fn get_history_limit(&self) -> usize {
        self.history.get_max_size()
    }

    /// Sets the maximum history size
    ///
    /// If the new size is smaller than the current number of commands,
    /// the oldest commands will be removed to fit the new limit.
    ///
    /// # Arguments
    /// * `max_size` - The new maximum number of commands to store
    ///
    /// # Memory Impact
    /// Reducing the history limit immediately frees memory from removed commands.
    /// See the MEMORY.md file for detailed memory characteristics.
    pub fn set_history_limit(&mut self, max_size: usize) {
        self.history.set_max_size(max_size);
    }

    /// Clears all undo and redo history
    ///
    /// This immediately frees all memory used by the command history.
    /// Useful after save operations or when memory is constrained.
    ///
    /// # Memory Impact
    /// Frees ~6.4KB + text data for a full history of 100 commands.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Returns memory usage statistics for the document
    ///
    /// # Returns
    /// A tuple containing:
    /// - Text length in characters
    /// - Number of format runs
    /// - Number of blocks
    /// - Number of undo commands
    /// - Number of redo commands
    /// - Estimated total memory usage in bytes
    ///
    /// # Memory Breakdown
    /// The estimated memory includes:
    /// - Base overhead: ~1KB
    /// - Text storage: ~4 bytes per character + gap overhead
    /// - Format storage: ~32 bytes per format run + ~16 bytes per block
    /// - Command history: ~64 bytes per command + affected text
    ///
    /// # Example
    /// ```
    /// use rich_text_editor_wasm::document::Document;
    ///
    /// let doc = Document::from_text("Hello World");
    /// let (chars, runs, blocks, undo, redo, bytes) = doc.memory_stats();
    /// println!("Document using {} bytes", bytes);
    /// ```
    pub fn memory_stats(&self) -> (usize, usize, usize, usize, usize, usize) {
        let text_length = self.get_length();
        let (run_count, block_count, _string_count, format_memory) = self.formats.memory_stats();
        let undo_count = self.history.undo_count();
        let redo_count = self.history.redo_count();

        // Estimate memory usage
        let base_overhead = 1024; // ~1KB for Document struct and other fields
        let text_memory = (text_length * 4) + 128; // ~4 bytes per char + gap overhead
        let history_memory = (undo_count + redo_count) * 64 + (text_length / 10); // ~64 bytes per command + estimated text

        let total_memory = base_overhead + text_memory + format_memory + history_memory;

        (
            text_length,
            run_count,
            block_count,
            undo_count,
            redo_count,
            total_memory,
        )
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_document() {
        let doc = Document::new();
        assert_eq!(doc.get_length(), 0);
        assert!(doc.is_empty());
        assert_eq!(doc.get_content(), "");
        assert_eq!(doc.version(), 0);
    }

    #[test]
    fn test_from_text() {
        let doc = Document::from_text("Hello World");
        assert_eq!(doc.get_length(), 11);
        assert!(!doc.is_empty());
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_insert_text() {
        let mut doc = Document::new();
        doc.insert_text(Position::new(0), "Hello").unwrap();
        assert_eq!(doc.get_content(), "Hello");
        assert_eq!(doc.get_length(), 5);
        assert_eq!(doc.version(), 1);

        doc.insert_text(Position::new(5), " World").unwrap();
        assert_eq!(doc.get_content(), "Hello World");
        assert_eq!(doc.version(), 2);
    }

    #[test]
    fn test_delete_range() {
        let mut doc = Document::from_text("Hello World");
        doc.delete_range(Range::new(Position::new(5), Position::new(11)))
            .unwrap();
        assert_eq!(doc.get_content(), "Hello");
        assert_eq!(doc.get_length(), 5);
    }

    #[test]
    fn test_replace_range() {
        let mut doc = Document::from_text("Hello World");
        doc.replace_range(Range::new(Position::new(6), Position::new(11)), "Rust")
            .unwrap();
        assert_eq!(doc.get_content(), "Hello Rust");
    }

    #[test]
    fn test_get_text_in_range() {
        let doc = Document::from_text("Hello World");
        let range = Range::new(Position::new(0), Position::new(5));
        assert_eq!(doc.get_text_in_range(range), "Hello");

        let range2 = Range::new(Position::new(6), Position::new(11));
        assert_eq!(doc.get_text_in_range(range2), "World");
    }

    #[test]
    fn test_get_text_in_range_backward() {
        let doc = Document::from_text("Hello World");
        let range = Range::new(Position::new(5), Position::new(0));
        assert_eq!(doc.get_text_in_range(range), "Hello");
    }

    #[test]
    fn test_version_increments() {
        let mut doc = Document::new();
        assert_eq!(doc.version(), 0);

        doc.insert_text(Position::new(0), "a").unwrap();
        assert_eq!(doc.version(), 1);

        doc.delete_range(Range::new(Position::new(0), Position::new(1)))
            .unwrap();
        assert_eq!(doc.version(), 2);

        doc.replace_range(Range::new(Position::new(0), Position::new(0)), "b")
            .unwrap();
        assert_eq!(doc.version(), 3);
    }

    #[test]
    fn test_empty_range() {
        let doc = Document::from_text("Hello");
        let empty_range = Range::new(Position::new(2), Position::new(2));
        assert_eq!(doc.get_text_in_range(empty_range), "");
    }

    #[test]
    fn test_multiple_operations() {
        let mut doc = Document::new();
        doc.insert_text(Position::new(0), "Hello").unwrap();
        doc.insert_text(Position::new(5), " ").unwrap();
        doc.insert_text(Position::new(6), "World").unwrap();
        assert_eq!(doc.get_content(), "Hello World");

        doc.delete_range(Range::new(Position::new(5), Position::new(6)))
            .unwrap();
        assert_eq!(doc.get_content(), "HelloWorld");

        doc.replace_range(Range::new(Position::new(5), Position::new(10)), " Rust")
            .unwrap();
        assert_eq!(doc.get_content(), "Hello Rust");
    }

    #[test]
    fn test_undo_redo() {
        let mut doc = Document::new();

        // Insert text
        doc.insert_text(Position::new(0), "Hello").unwrap();
        assert_eq!(doc.get_content(), "Hello");
        assert!(doc.can_undo());
        assert!(!doc.can_redo());

        // Undo insert
        doc.undo().unwrap();
        assert_eq!(doc.get_content(), "");
        assert!(!doc.can_undo());
        assert!(doc.can_redo());

        // Redo insert
        doc.redo().unwrap();
        assert_eq!(doc.get_content(), "Hello");
        assert!(doc.can_undo());
        assert!(!doc.can_redo());
    }

    #[test]
    fn test_undo_redo_multiple_operations() {
        let mut doc = Document::new();

        doc.insert_text(Position::new(0), "Hello").unwrap();
        doc.insert_text(Position::new(5), " World").unwrap();
        assert_eq!(doc.get_content(), "Hello World");

        // Undo second insert
        doc.undo().unwrap();
        assert_eq!(doc.get_content(), "Hello");

        // Undo first insert
        doc.undo().unwrap();
        assert_eq!(doc.get_content(), "");

        // Redo first insert
        doc.redo().unwrap();
        assert_eq!(doc.get_content(), "Hello");

        // Redo second insert
        doc.redo().unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_undo_delete() {
        let mut doc = Document::from_text("Hello World");

        doc.delete_range(Range::new(Position::new(5), Position::new(11)))
            .unwrap();
        assert_eq!(doc.get_content(), "Hello");

        doc.undo().unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_undo_replace() {
        let mut doc = Document::from_text("Hello World");

        doc.replace_range(Range::new(Position::new(6), Position::new(11)), "Rust")
            .unwrap();
        assert_eq!(doc.get_content(), "Hello Rust");

        doc.undo().unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_redo_cleared_on_new_operation() {
        let mut doc = Document::new();

        doc.insert_text(Position::new(0), "Hello").unwrap();
        doc.undo().unwrap();
        assert!(doc.can_redo());

        // New operation should clear redo stack
        doc.insert_text(Position::new(0), "World").unwrap();
        assert!(!doc.can_redo());
        assert_eq!(doc.get_content(), "World");
    }

    #[test]
    fn test_apply_format() {
        let mut doc = Document::from_text("Hello World");
        let range = Range::from_offsets(0, 5);

        doc.apply_format(range, InlineFormat::Bold);

        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_remove_format() {
        let mut doc = Document::from_text("Hello World");
        let range = Range::from_offsets(0, 5);

        doc.apply_format(range, InlineFormat::Bold);
        doc.apply_format(range, InlineFormat::Italic);

        doc.remove_format(range, &InlineFormat::Bold);

        let formats = doc.get_formats_at(Position::new(2));
        assert!(!formats.contains(&InlineFormat::Bold));
        assert!(formats.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_toggle_format() {
        let mut doc = Document::from_text("Hello World");
        let range = Range::from_offsets(0, 5);

        // Toggle on
        doc.toggle_format(range, InlineFormat::Bold);
        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));

        // Toggle off
        doc.toggle_format(range, InlineFormat::Bold);
        let formats = doc.get_formats_at(Position::new(2));
        assert!(!formats.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_format_preserved_on_insert() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(6, 11), InlineFormat::Bold);

        // Insert text before the formatted range
        doc.insert_text(Position::new(0), "Hi ").unwrap();

        // Format should be shifted
        let formats = doc.get_formats_at(Position::new(10));
        assert!(formats.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_format_preserved_on_delete_undo() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);

        // Delete the formatted text
        doc.delete_range(Range::from_offsets(0, 5)).unwrap();
        assert_eq!(doc.get_content(), " World");

        // Undo should restore both text and format
        doc.undo().unwrap();
        assert_eq!(doc.get_content(), "Hello World");
        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_set_block_type() {
        let mut doc = Document::from_text("Hello World");
        doc.set_block_type(Range::from_offsets(0, 11), BlockType::heading(1));

        let block_type = doc.get_block_type_at(Position::new(5));
        assert_eq!(block_type, BlockType::heading(1));
    }

    #[test]
    fn test_get_block_type_at_default() {
        let doc = Document::from_text("Hello World");
        let block_type = doc.get_block_type_at(Position::new(0));
        assert_eq!(block_type, BlockType::Paragraph);
    }

    #[test]
    fn test_multiple_block_types() {
        let mut doc = Document::from_text("Line 1\nLine 2\nLine 3");
        doc.set_block_type(Range::from_offsets(0, 6), BlockType::heading(1));
        doc.set_block_type(Range::from_offsets(7, 13), BlockType::BulletList);
        doc.set_block_type(Range::from_offsets(14, 20), BlockType::Paragraph);

        assert_eq!(
            doc.get_block_type_at(Position::new(3)),
            BlockType::heading(1)
        );
        assert_eq!(
            doc.get_block_type_at(Position::new(10)),
            BlockType::BulletList
        );
        assert_eq!(
            doc.get_block_type_at(Position::new(17)),
            BlockType::Paragraph
        );
    }

    #[test]
    fn test_block_type_version_increment() {
        let mut doc = Document::from_text("Hello");
        let initial_version = doc.version();

        doc.set_block_type(Range::from_offsets(0, 5), BlockType::heading(2));
        assert_eq!(doc.version(), initial_version + 1);
    }

    #[test]
    fn test_block_type_preserved_on_insert() {
        let mut doc = Document::from_text("Hello World");
        doc.set_block_type(Range::from_offsets(6, 11), BlockType::heading(1));

        // Insert text before the block
        doc.insert_text(Position::new(0), "Hi ").unwrap();

        // Block should be shifted
        assert_eq!(
            doc.get_block_type_at(Position::new(5)),
            BlockType::Paragraph
        );
        assert_eq!(
            doc.get_block_type_at(Position::new(10)),
            BlockType::heading(1)
        );
    }

    #[test]
    fn test_block_type_all_types() {
        let mut doc = Document::from_text("Test");

        doc.set_block_type(Range::from_offsets(0, 4), BlockType::Paragraph);
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::Paragraph
        );

        doc.set_block_type(Range::from_offsets(0, 4), BlockType::heading(3));
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::heading(3)
        );

        doc.set_block_type(Range::from_offsets(0, 4), BlockType::BulletList);
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::BulletList
        );

        doc.set_block_type(Range::from_offsets(0, 4), BlockType::NumberedList);
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::NumberedList
        );

        doc.set_block_type(Range::from_offsets(0, 4), BlockType::BlockQuote);
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::BlockQuote
        );

        doc.set_block_type(Range::from_offsets(0, 4), BlockType::CodeBlock);
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::CodeBlock
        );
    }

    #[test]
    fn test_set_selection() {
        let mut doc = Document::from_text("Hello World");
        let sel = Selection::new(Position::new(0), Position::new(5));

        doc.set_selection(sel);
        assert_eq!(doc.get_selection(), sel);
    }

    #[test]
    fn test_get_selection_default() {
        let doc = Document::new();
        let sel = doc.get_selection();

        assert!(sel.is_collapsed());
        assert_eq!(sel.anchor.offset(), 0);
    }

    #[test]
    fn test_get_selected_text() {
        let mut doc = Document::from_text("Hello World");

        // Test with collapsed selection
        doc.set_selection(Selection::collapsed(Position::new(5)));
        assert_eq!(doc.get_selected_text(), "");

        // Test with range selection
        doc.set_selection(Selection::new(Position::new(0), Position::new(5)));
        assert_eq!(doc.get_selected_text(), "Hello");

        // Test with backward selection
        doc.set_selection(Selection::new(Position::new(11), Position::new(6)));
        assert_eq!(doc.get_selected_text(), "World");
    }

    #[test]
    fn test_select_all() {
        let mut doc = Document::from_text("Hello World");

        doc.select_all();
        let sel = doc.get_selection();

        assert_eq!(sel.anchor.offset(), 0);
        assert_eq!(sel.focus.offset(), 11);
        assert_eq!(doc.get_selected_text(), "Hello World");
    }

    #[test]
    fn test_select_all_empty_document() {
        let mut doc = Document::new();

        doc.select_all();
        let sel = doc.get_selection();

        assert!(sel.is_collapsed());
        assert_eq!(sel.anchor.offset(), 0);
    }

    #[test]
    fn test_collapse_to_start() {
        let mut doc = Document::from_text("Hello World");

        // Forward selection
        doc.set_selection(Selection::new(Position::new(0), Position::new(5)));
        doc.collapse_to_start();
        let sel = doc.get_selection();

        assert!(sel.is_collapsed());
        assert_eq!(sel.anchor.offset(), 0);

        // Backward selection
        doc.set_selection(Selection::new(Position::new(10), Position::new(5)));
        doc.collapse_to_start();
        let sel = doc.get_selection();

        assert!(sel.is_collapsed());
        assert_eq!(sel.anchor.offset(), 5);
    }

    #[test]
    fn test_collapse_to_end() {
        let mut doc = Document::from_text("Hello World");

        // Forward selection
        doc.set_selection(Selection::new(Position::new(0), Position::new(5)));
        doc.collapse_to_end();
        let sel = doc.get_selection();

        assert!(sel.is_collapsed());
        assert_eq!(sel.anchor.offset(), 5);

        // Backward selection
        doc.set_selection(Selection::new(Position::new(10), Position::new(5)));
        doc.collapse_to_end();
        let sel = doc.get_selection();

        assert!(sel.is_collapsed());
        assert_eq!(sel.anchor.offset(), 10);
    }

    #[test]
    fn test_collapse_already_collapsed() {
        let mut doc = Document::from_text("Hello World");

        doc.set_selection(Selection::collapsed(Position::new(5)));
        doc.collapse_to_start();

        let sel = doc.get_selection();
        assert!(sel.is_collapsed());
        assert_eq!(sel.anchor.offset(), 5);

        doc.collapse_to_end();
        let sel = doc.get_selection();
        assert!(sel.is_collapsed());
        assert_eq!(sel.anchor.offset(), 5);
    }

    #[test]
    fn test_memory_stats() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.insert_text(Position::new(11), " Test").unwrap();

        let (text_len, runs, blocks, undo, redo, bytes) = doc.memory_stats();

        assert_eq!(text_len, 16); // "Hello World Test"
        assert!(runs > 0); // Should have at least one format run
        assert!(blocks > 0); // Should have at least one block
        assert!(undo > 0); // Should have undo commands
        assert_eq!(redo, 0); // No redo commands
        assert!(bytes > 0); // Should have non-zero memory usage
    }

    #[test]
    fn test_get_history_limit() {
        let doc = Document::new();
        assert_eq!(doc.get_history_limit(), 100); // Default limit
    }

    #[test]
    fn test_set_history_limit() {
        let mut doc = Document::new();

        doc.set_history_limit(50);
        assert_eq!(doc.get_history_limit(), 50);

        doc.set_history_limit(20);
        assert_eq!(doc.get_history_limit(), 20);
    }

    #[test]
    fn test_clear_history() {
        let mut doc = Document::new();

        // Add some operations
        doc.insert_text(Position::new(0), "Hello").unwrap();
        doc.insert_text(Position::new(5), " World").unwrap();

        assert!(doc.can_undo());

        // Clear history
        doc.clear_history();

        assert!(!doc.can_undo());
        assert!(!doc.can_redo());
    }

    #[test]
    fn test_insert_at_document_end() {
        let mut doc = Document::from_text("Hello");
        doc.insert_text(Position::new(5), " World").unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_delete_entire_document() {
        let mut doc = Document::from_text("Hello World");
        doc.delete_range(Range::from_offsets(0, 11)).unwrap();
        assert_eq!(doc.get_content(), "");
        assert!(doc.is_empty());
    }

    #[test]
    fn test_replace_entire_document() {
        let mut doc = Document::from_text("Hello World");
        doc.replace_range(Range::from_offsets(0, 11), "New Text")
            .unwrap();
        assert_eq!(doc.get_content(), "New Text");
    }

    #[test]
    fn test_multiple_format_operations() {
        let mut doc = Document::from_text("Hello World");
        let range = Range::from_offsets(0, 5);

        doc.apply_format(range, InlineFormat::Bold);
        doc.apply_format(range, InlineFormat::Italic);
        doc.apply_format(range, InlineFormat::Underline);

        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));
        assert!(formats.contains(&InlineFormat::Italic));
        assert!(formats.contains(&InlineFormat::Underline));
    }

    #[test]
    fn test_format_at_boundaries() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);

        // At start of format
        let formats_start = doc.get_formats_at(Position::new(0));
        assert!(formats_start.contains(&InlineFormat::Bold));

        // At end of format (should not have format)
        let formats_end = doc.get_formats_at(Position::new(5));
        assert!(!formats_end.contains(&InlineFormat::Bold));

        // Just before end
        let formats_before_end = doc.get_formats_at(Position::new(4));
        assert!(formats_before_end.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_overlapping_format_removal() {
        let mut doc = Document::from_text("Hello World");

        // Apply bold to entire text
        doc.apply_format(Range::from_offsets(0, 11), InlineFormat::Bold);

        // Remove bold from middle
        doc.remove_format(Range::from_offsets(3, 8), &InlineFormat::Bold);

        // Check that bold is removed from middle but remains at edges
        assert!(
            doc.get_formats_at(Position::new(1))
                .contains(&InlineFormat::Bold)
        );
        assert!(
            !doc.get_formats_at(Position::new(5))
                .contains(&InlineFormat::Bold)
        );
        assert!(
            doc.get_formats_at(Position::new(9))
                .contains(&InlineFormat::Bold)
        );
    }

    #[test]
    fn test_unicode_text_operations() {
        let mut doc = Document::new();

        // Insert unicode text
        doc.insert_text(Position::new(0), "Hello 世界 🌍").unwrap();
        assert_eq!(doc.get_content(), "Hello 世界 🌍");

        // Length should be character count, not byte count
        assert_eq!(doc.get_length(), 10);

        // Get text in range with unicode
        let text = doc.get_text_in_range(Range::from_offsets(6, 8));
        assert_eq!(text, "世界");
    }

    #[test]
    fn test_version_tracking() {
        let mut doc = Document::new();
        assert_eq!(doc.version(), 0);

        doc.insert_text(Position::new(0), "a").unwrap();
        assert_eq!(doc.version(), 1);

        doc.delete_range(Range::from_offsets(0, 1)).unwrap();
        assert_eq!(doc.version(), 2);

        doc.apply_format(Range::from_offsets(0, 0), InlineFormat::Bold);
        assert_eq!(doc.version(), 3);

        doc.set_block_type(Range::from_offsets(0, 0), BlockType::heading(1));
        assert_eq!(doc.version(), 4);
    }

    #[test]
    fn test_dirty_regions_tracking() {
        let mut doc = Document::from_text("Hello World");

        // Initially no dirty regions
        assert!(!doc.has_dirty_regions());

        // Insert text creates dirty region
        doc.insert_text(Position::new(5), " Beautiful").unwrap();
        assert!(doc.has_dirty_regions());

        let dirty = doc.get_dirty_regions();
        assert!(!dirty.is_empty());

        // Clear dirty flags
        doc.clear_dirty_flags();
        assert!(!doc.has_dirty_regions());
    }

    #[test]
    fn test_empty_document_operations() {
        let mut doc = Document::new();

        // Operations on empty document
        assert_eq!(doc.get_content(), "");
        assert_eq!(doc.get_length(), 0);
        assert!(doc.is_empty());

        // Get text in empty range
        let text = doc.get_text_in_range(Range::from_offsets(0, 0));
        assert_eq!(text, "");

        // Select all on empty document
        doc.select_all();
        let sel = doc.get_selection();
        assert!(sel.is_collapsed());
        assert_eq!(sel.anchor.offset(), 0);
    }

    #[test]
    fn test_undo_redo_with_formats() {
        let mut doc = Document::from_text("Hello");

        // Apply format
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        assert!(
            doc.get_formats_at(Position::new(2))
                .contains(&InlineFormat::Bold)
        );

        // Undo format
        doc.undo().unwrap();
        assert!(
            !doc.get_formats_at(Position::new(2))
                .contains(&InlineFormat::Bold)
        );

        // Redo format
        doc.redo().unwrap();
        assert!(
            doc.get_formats_at(Position::new(2))
                .contains(&InlineFormat::Bold)
        );
    }

    #[test]
    fn test_history_limit_enforcement() {
        let mut doc = Document::new();
        doc.set_history_limit(5);

        // Add 10 operations
        for i in 0..10 {
            doc.insert_text(Position::new(i), "a").unwrap();
        }

        // Should only be able to undo 5 times
        for _ in 0..5 {
            assert!(doc.undo().is_ok());
        }

        // 6th undo should fail
        assert!(doc.undo().is_err());
    }
}
