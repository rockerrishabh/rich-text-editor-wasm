//! Operations module
//!
//! This module implements the command pattern for undoable operations
//! and provides clipboard and search functionality.
//!
//! # Responsibilities
//!
//! - Define the `Command` trait for undoable operations
//! - Implement concrete commands (insert, delete, replace, format)
//! - Manage command history for undo/redo functionality
//! - Provide clipboard operations with format preservation
//! - Implement search and replace functionality
//!
//! # Key Types
//!
//! - `Command`: Trait for operations that can be executed and undone
//! - `CommandHistory`: Manages undo/redo stacks
//! - `InsertCommand`, `DeleteCommand`, `ReplaceCommand`: Text operations
//! - `ApplyFormatCommand`, `RemoveFormatCommand`: Format operations
//! - `ClipboardContent`: Represents clipboard data with formatting

pub mod clipboard;
pub mod history;
pub mod search;

use crate::document::{Document, Position, Range};

// Re-export commonly used types
pub use clipboard::{ClipboardContent, SerializableFormatRun};
pub use history::CommandHistory;

/// Result type for command operations
pub type CommandResult<T> = Result<T, CommandError>;

/// Errors that can occur during command execution
///
/// This enum provides detailed context for all command-related errors,
/// including position information, operation details, and helpful error messages.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CommandError {
    /// Invalid position in document
    ///
    /// Occurs when a command attempts to access a position beyond
    /// the document's length.
    ///
    /// # Context
    /// - `position`: The invalid position that was requested
    /// - `length`: The current document length
    #[error("Invalid position {position} (document length: {length})")]
    InvalidPosition { position: usize, length: usize },

    /// Invalid range in document
    ///
    /// Occurs when a command specifies a range where start > end
    /// or end exceeds the document length.
    ///
    /// # Context
    /// - `start`: The start position of the range
    /// - `end`: The end position of the range
    /// - `length`: The current document length
    #[error("Invalid range: {start}..{end} (document length: {length})")]
    InvalidRange {
        start: usize,
        end: usize,
        length: usize,
    },

    /// Command execution failed
    ///
    /// Occurs when a command cannot be executed due to invalid state
    /// or preconditions not being met.
    ///
    /// # Context
    /// - `command`: The name of the command that failed
    /// - `reason`: Detailed explanation of why it failed
    #[error("Command '{command}' execution failed: {reason}")]
    ExecutionFailed { command: String, reason: String },

    /// Nothing to undo
    ///
    /// Occurs when undo is requested but the undo stack is empty.
    #[error("Nothing to undo")]
    NothingToUndo,

    /// Nothing to redo
    ///
    /// Occurs when redo is requested but the redo stack is empty.
    #[error("Nothing to redo")]
    NothingToRedo,

    /// Command not executed
    ///
    /// Occurs when attempting to undo a command that hasn't been executed.
    ///
    /// # Context
    /// - `command`: The name of the command
    #[error("Cannot undo command '{command}' that hasn't been executed")]
    CommandNotExecuted { command: String },

    /// History limit reached
    ///
    /// Occurs when the command history has reached its maximum size.
    ///
    /// # Context
    /// - `limit`: The maximum number of commands allowed in history
    #[error("Command history limit reached: {limit} commands")]
    HistoryLimitReached { limit: usize },
}

impl CommandError {
    /// Creates an invalid position error
    pub fn invalid_position(position: usize, length: usize) -> Self {
        Self::InvalidPosition { position, length }
    }

    /// Creates an invalid range error
    pub fn invalid_range(start: usize, end: usize, length: usize) -> Self {
        Self::InvalidRange { start, end, length }
    }

    /// Creates an execution failed error
    pub fn execution_failed(command: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ExecutionFailed {
            command: command.into(),
            reason: reason.into(),
        }
    }

    /// Creates a command not executed error
    pub fn command_not_executed(command: impl Into<String>) -> Self {
        Self::CommandNotExecuted {
            command: command.into(),
        }
    }

    /// Creates a history limit reached error
    pub fn history_limit_reached(limit: usize) -> Self {
        Self::HistoryLimitReached { limit }
    }
}

/// Trait for commands that can be executed and undone
pub trait Command: Send + Sync {
    /// Execute the command on the document
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()>;

    /// Undo the command on the document
    fn undo(&mut self, doc: &mut Document) -> CommandResult<()>;

    /// Get a description of the command
    fn description(&self) -> String;
}

/// Command that inserts text at a position
#[derive(Debug, Clone)]
pub struct InsertCommand {
    position: Position,
    text: String,
    /// Stores the text that was at the position (for undo)
    undo_state: Option<()>,
}

impl InsertCommand {
    /// Creates a new InsertCommand
    pub fn new(position: Position, text: String) -> Self {
        Self {
            position,
            text,
            undo_state: None,
        }
    }
}

impl Command for InsertCommand {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        let pos = self.position.offset();
        let length = doc.get_length();

        if pos > length {
            return Err(CommandError::invalid_position(pos, length));
        }

        doc.insert_text_direct(self.position, &self.text);
        self.undo_state = Some(());
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        if self.undo_state.is_none() {
            return Err(CommandError::command_not_executed("InsertCommand"));
        }

        let end_pos = Position::new(self.position.offset() + self.text.chars().count());
        let range = Range::new(self.position, end_pos);
        doc.delete_range_direct(range);
        self.undo_state = None;
        Ok(())
    }

    fn description(&self) -> String {
        format!(
            "Insert '{}' at position {}",
            self.text,
            self.position.offset()
        )
    }
}

/// Command that deletes text in a range
#[derive(Debug, Clone)]
pub struct DeleteCommand {
    range: Range,
    /// Stores the deleted text for undo
    deleted_text: Option<String>,
    /// Stores the deleted format runs for undo
    deleted_formats: Option<Vec<crate::formatting::FormatRun>>,
}

impl DeleteCommand {
    /// Creates a new DeleteCommand
    pub fn new(range: Range) -> Self {
        Self {
            range,
            deleted_text: None,
            deleted_formats: None,
        }
    }
}

impl Command for DeleteCommand {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        let normalized = self.range.normalize();
        let start = normalized.start.offset();
        let end = normalized.end.offset();
        let length = doc.get_length();

        if end > length {
            return Err(CommandError::invalid_range(start, end, length));
        }

        // Store the text being deleted for undo
        self.deleted_text = Some(doc.get_text_in_range(normalized));

        // Store the format runs that overlap with the deleted range
        let overlapping_runs: Vec<_> = doc
            .formats()
            .get_runs()
            .iter()
            .filter(|run| run.range.overlaps(&normalized))
            .cloned()
            .collect();
        self.deleted_formats = Some(overlapping_runs);

        doc.delete_range_direct(self.range);
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        if let Some(ref text) = self.deleted_text {
            let normalized = self.range.normalize();
            doc.insert_text_direct(normalized.start, text);

            // Restore the deleted format runs
            if let Some(ref format_runs) = self.deleted_formats {
                for run in format_runs {
                    for format in &run.formats {
                        doc.formats_mut().apply_format(run.range, format.clone());
                    }
                }
            }

            Ok(())
        } else {
            Err(CommandError::command_not_executed("DeleteCommand"))
        }
    }

    fn description(&self) -> String {
        format!(
            "Delete range {}..{}",
            self.range.start.offset(),
            self.range.end.offset()
        )
    }
}

/// Command that replaces text in a range with new text
#[derive(Debug, Clone)]
pub struct ReplaceCommand {
    range: Range,
    new_text: String,
    /// Stores the old text for undo
    old_text: Option<String>,
    /// Stores the old format runs for undo
    old_formats: Option<Vec<crate::formatting::FormatRun>>,
}

impl ReplaceCommand {
    /// Creates a new ReplaceCommand
    pub fn new(range: Range, new_text: String) -> Self {
        Self {
            range,
            new_text,
            old_text: None,
            old_formats: None,
        }
    }
}

impl Command for ReplaceCommand {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        let normalized = self.range.normalize();
        let start = normalized.start.offset();
        let end = normalized.end.offset();
        let length = doc.get_length();

        if end > length {
            return Err(CommandError::invalid_range(start, end, length));
        }

        // Store the old text for undo
        self.old_text = Some(doc.get_text_in_range(normalized));

        // Store the format runs that overlap with the replaced range
        let overlapping_runs: Vec<_> = doc
            .formats()
            .get_runs()
            .iter()
            .filter(|run| run.range.overlaps(&normalized))
            .cloned()
            .collect();
        self.old_formats = Some(overlapping_runs);

        doc.replace_range_direct(self.range, &self.new_text);
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        if let Some(ref old_text) = self.old_text {
            let normalized = self.range.normalize();
            let new_end = Position::new(normalized.start.offset() + self.new_text.chars().count());
            let current_range = Range::new(normalized.start, new_end);
            doc.replace_range_direct(current_range, old_text);

            // Restore the old format runs
            if let Some(ref format_runs) = self.old_formats {
                for run in format_runs {
                    for format in &run.formats {
                        doc.formats_mut().apply_format(run.range, format.clone());
                    }
                }
            }

            Ok(())
        } else {
            Err(CommandError::command_not_executed("ReplaceCommand"))
        }
    }

    fn description(&self) -> String {
        format!(
            "Replace range {}..{} with '{}'",
            self.range.start.offset(),
            self.range.end.offset(),
            self.new_text
        )
    }
}

/// Command that applies formatting to a range
#[derive(Debug, Clone)]
pub struct ApplyFormatCommand {
    range: Range,
    format: crate::formatting::InlineFormat,
    /// Stores the previous format runs for undo
    previous_state: Option<Vec<crate::formatting::FormatRun>>,
}

impl ApplyFormatCommand {
    /// Creates a new ApplyFormatCommand
    pub fn new(range: Range, format: crate::formatting::InlineFormat) -> Self {
        Self {
            range,
            format,
            previous_state: None,
        }
    }
}

impl Command for ApplyFormatCommand {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        let normalized = self.range.normalize();
        let start = normalized.start.offset();
        let end = normalized.end.offset();
        let length = doc.get_length();

        if end > length {
            return Err(CommandError::invalid_range(start, end, length));
        }

        // Store the current format runs that overlap with the range
        let overlapping_runs: Vec<_> = doc
            .formats()
            .get_runs()
            .iter()
            .filter(|run| run.range.overlaps(&normalized))
            .cloned()
            .collect();
        self.previous_state = Some(overlapping_runs);

        // Apply the format
        doc.formats_mut()
            .apply_format(self.range, self.format.clone());
        doc.mark_dirty(self.range);
        doc.increment_version();
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        if let Some(ref previous_runs) = self.previous_state {
            let normalized = self.range.normalize();

            // Clear formats in the range
            doc.formats_mut().remove_format(normalized, &self.format);

            // Restore the previous format state
            for run in previous_runs {
                for format in &run.formats {
                    doc.formats_mut().apply_format(run.range, format.clone());
                }
            }

            doc.mark_dirty(normalized);
            doc.increment_version();
            Ok(())
        } else {
            Err(CommandError::command_not_executed("ApplyFormatCommand"))
        }
    }

    fn description(&self) -> String {
        format!(
            "Apply format {:?} to range {}..{}",
            self.format,
            self.range.start.offset(),
            self.range.end.offset()
        )
    }
}

/// Command that removes formatting from a range
#[derive(Debug, Clone)]
pub struct RemoveFormatCommand {
    range: Range,
    format: crate::formatting::InlineFormat,
    /// Stores the previous format runs for undo
    previous_state: Option<Vec<crate::formatting::FormatRun>>,
}

impl RemoveFormatCommand {
    /// Creates a new RemoveFormatCommand
    pub fn new(range: Range, format: crate::formatting::InlineFormat) -> Self {
        Self {
            range,
            format,
            previous_state: None,
        }
    }
}

impl Command for RemoveFormatCommand {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        let normalized = self.range.normalize();
        let start = normalized.start.offset();
        let end = normalized.end.offset();
        let length = doc.get_length();

        if end > length {
            return Err(CommandError::invalid_range(start, end, length));
        }

        // Store the current format runs that overlap with the range
        let overlapping_runs: Vec<_> = doc
            .formats()
            .get_runs()
            .iter()
            .filter(|run| run.range.overlaps(&normalized))
            .cloned()
            .collect();
        self.previous_state = Some(overlapping_runs);

        // Remove the format
        doc.formats_mut().remove_format(self.range, &self.format);
        doc.mark_dirty(self.range);
        doc.increment_version();
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        if let Some(ref previous_runs) = self.previous_state {
            let normalized = self.range.normalize();

            // Restore the previous format state
            for run in previous_runs {
                for format in &run.formats {
                    doc.formats_mut().apply_format(run.range, format.clone());
                }
            }

            doc.mark_dirty(normalized);
            doc.increment_version();
            Ok(())
        } else {
            Err(CommandError::command_not_executed("RemoveFormatCommand"))
        }
    }

    fn description(&self) -> String {
        format!(
            "Remove format {:?} from range {}..{}",
            self.format,
            self.range.start.offset(),
            self.range.end.offset()
        )
    }
}

/// Command that sets block type for a range
#[derive(Debug, Clone)]
pub struct SetBlockTypeCommand {
    range: Range,
    block_type: crate::formatting::BlockType,
    /// Snapshot of previous blocks for undo
    previous_blocks: Option<Vec<crate::formatting::storage::BlockInfo>>, 
}

impl SetBlockTypeCommand {
    /// Creates a new SetBlockTypeCommand
    pub fn new(range: Range, block_type: crate::formatting::BlockType) -> Self {
        Self {
            range,
            block_type,
            previous_blocks: None,
        }
    }
}

impl Command for SetBlockTypeCommand {
    fn execute(&mut self, doc: &mut Document) -> CommandResult<()> {
        let normalized = self.range.normalize();
        let start = normalized.start.offset();
        let end = normalized.end.offset();
        let length = doc.get_length();

        if end > length {
            return Err(CommandError::invalid_range(start, end, length));
        }

        // Snapshot current blocks
        self.previous_blocks = Some(doc.formats().get_blocks().to_vec());

        // Apply new block type
        doc.formats_mut().set_block_type(self.range, self.block_type.clone());
        doc.mark_dirty(self.range);
        doc.increment_version();
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        if let Some(ref snapshot) = self.previous_blocks {
            // Restore previous blocks
            doc.formats_mut().set_blocks(snapshot.clone());
            let normalized = self.range.normalize();
            doc.mark_dirty(normalized);
            doc.increment_version();
            Ok(())
        } else {
            Err(CommandError::command_not_executed("SetBlockTypeCommand"))
        }
    }

    fn description(&self) -> String {
        format!(
            "Set block type {:?} for range {}..{}",
            self.block_type,
            self.range.start.offset(),
            self.range.end.offset()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_command() {
        let mut doc = Document::new();
        let mut cmd = InsertCommand::new(Position::new(0), "Hello".to_string());

        cmd.execute(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello");

        cmd.undo(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "");
    }

    #[test]
    fn test_insert_command_in_middle() {
        let mut doc = Document::from_text("Hello World");
        let mut cmd = InsertCommand::new(Position::new(5), " Beautiful".to_string());

        cmd.execute(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello Beautiful World");

        cmd.undo(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_delete_command() {
        let mut doc = Document::from_text("Hello World");
        let mut cmd = DeleteCommand::new(Range::new(Position::new(5), Position::new(11)));

        cmd.execute(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello");

        cmd.undo(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_replace_command() {
        let mut doc = Document::from_text("Hello World");
        let mut cmd = ReplaceCommand::new(
            Range::new(Position::new(6), Position::new(11)),
            "Rust".to_string(),
        );

        cmd.execute(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello Rust");

        cmd.undo(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_command_description() {
        let insert_cmd = InsertCommand::new(Position::new(0), "test".to_string());
        assert!(insert_cmd.description().contains("Insert"));

        let delete_cmd = DeleteCommand::new(Range::new(Position::new(0), Position::new(5)));
        assert!(delete_cmd.description().contains("Delete"));

        let replace_cmd = ReplaceCommand::new(
            Range::new(Position::new(0), Position::new(5)),
            "new".to_string(),
        );
        assert!(replace_cmd.description().contains("Replace"));
    }

    #[test]
    fn test_invalid_position() {
        let mut doc = Document::from_text("Hello");
        let mut cmd = InsertCommand::new(Position::new(100), "test".to_string());

        let result = cmd.execute(&mut doc);
        assert!(result.is_err());
    }

    #[test]
    fn test_undo_without_execute() {
        let mut doc = Document::new();
        let mut cmd = InsertCommand::new(Position::new(0), "test".to_string());

        let result = cmd.undo(&mut doc);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_format_command() {
        use crate::formatting::InlineFormat;

        let mut doc = Document::from_text("Hello World");
        let range = Range::from_offsets(0, 5);
        let mut cmd = ApplyFormatCommand::new(range, InlineFormat::Bold);

        cmd.execute(&mut doc).unwrap();
        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));

        cmd.undo(&mut doc).unwrap();
        let formats = doc.get_formats_at(Position::new(2));
        assert!(!formats.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_remove_format_command() {
        use crate::formatting::InlineFormat;

        let mut doc = Document::from_text("Hello World");
        let range = Range::from_offsets(0, 5);

        // Apply format first
        doc.apply_format(range, InlineFormat::Bold);
        doc.apply_format(range, InlineFormat::Italic);

        // Remove one format
        let mut cmd = RemoveFormatCommand::new(range, InlineFormat::Bold);
        cmd.execute(&mut doc).unwrap();

        let formats = doc.get_formats_at(Position::new(2));
        assert!(!formats.contains(&InlineFormat::Bold));
        assert!(formats.contains(&InlineFormat::Italic));

        // Undo should restore the format
        cmd.undo(&mut doc).unwrap();
        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));
        assert!(formats.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_format_command_with_history() {
        use crate::formatting::InlineFormat;

        let mut doc = Document::from_text("Hello World");
        let range = Range::from_offsets(0, 5);

        // Apply format using command history
        let mut cmd = Box::new(ApplyFormatCommand::new(range, InlineFormat::Bold));
        cmd.execute(&mut doc).unwrap();
        doc.history.push_command(cmd);

        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));

        // Undo through document
        doc.undo().unwrap();
        let formats = doc.get_formats_at(Position::new(2));
        assert!(!formats.contains(&InlineFormat::Bold));

        // Redo through document
        doc.redo().unwrap();
        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));
    }
}
