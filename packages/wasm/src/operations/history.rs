// Command history implementation

use super::{Command, CommandResult};
use crate::document::Document;

/// Manages undo/redo history for document commands.
///
/// # Performance Characteristics
///
/// ## Time Complexity
/// - **Push command**: O(1) amortized
///   - May be O(n) when max size is reached and oldest command is removed
/// - **Undo**: O(c) where c is the cost of undoing the specific command
///   - Typically O(n) where n is the size of text affected
/// - **Redo**: O(c) where c is the cost of redoing the specific command
/// - **Can undo/redo**: O(1) - just checks if stack is empty
/// - **Clear**: O(n) where n is the number of commands
///
/// ## Space Complexity
/// - Memory per command: ~64 bytes + size of affected text
/// - Default max size: 100 commands
/// - Total overhead: ~6.4KB + text data for 100 commands
/// - Each command stores the text it affected for undo
///
/// ## Performance Notes
/// - Commands are stored in a Vec for O(1) push/pop
/// - Redo stack is cleared when a new command is executed
/// - Max size limit prevents unbounded memory growth
/// - Large operations (e.g., paste 10KB text) store full text for undo
/// - Consider reducing max_size for memory-constrained environments
/// - Undo/redo of format operations is faster than text operations
///
/// # Example
/// ```
/// use rich_text_editor_wasm::operations::history::CommandHistory;
///
/// let mut history = CommandHistory::new();
/// assert_eq!(history.undo_count(), 0);
/// assert!(!history.can_undo());
/// ```
pub struct CommandHistory {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    max_size: usize,
}

impl CommandHistory {
    /// Default maximum size for the undo stack
    pub const DEFAULT_MAX_SIZE: usize = 100;

    /// Creates a new CommandHistory with the default maximum size
    pub fn new() -> Self {
        Self::with_max_size(Self::DEFAULT_MAX_SIZE)
    }

    /// Creates a new CommandHistory with a specified maximum size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    /// Executes a command and adds it to the undo stack
    pub fn execute(&mut self, mut cmd: Box<dyn Command>, doc: &mut Document) -> CommandResult<()> {
        // Execute the command
        cmd.execute(doc)?;

        // Clear the redo stack when a new command is executed
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push(cmd);

        // Enforce max size by removing oldest commands
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }

        Ok(())
    }

    /// Undoes the most recent command
    pub fn undo(&mut self, doc: &mut Document) -> CommandResult<()> {
        if let Some(mut cmd) = self.undo_stack.pop() {
            cmd.undo(doc)?;
            self.redo_stack.push(cmd);
            Ok(())
        } else {
            Err(super::CommandError::NothingToUndo)
        }
    }

    /// Redoes the most recently undone command
    pub fn redo(&mut self, doc: &mut Document) -> CommandResult<()> {
        if let Some(mut cmd) = self.redo_stack.pop() {
            cmd.execute(doc)?;
            self.undo_stack.push(cmd);
            Ok(())
        } else {
            Err(super::CommandError::NothingToRedo)
        }
    }

    /// Returns true if there are commands that can be undone
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns true if there are commands that can be redone
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clears all undo and redo history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Returns the number of commands in the undo stack
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Returns the number of commands in the redo stack
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Pushes a command onto the undo stack (used by Document)
    pub(crate) fn push_command(&mut self, cmd: Box<dyn Command>) {
        // Clear the redo stack when a new command is executed
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push(cmd);

        // Enforce max size by removing oldest commands
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Pops a command from the undo stack (used by Document)
    pub(crate) fn pop_undo(&mut self) -> Option<Box<dyn Command>> {
        self.undo_stack.pop()
    }

    /// Pushes a command onto the redo stack (used by Document)
    pub(crate) fn push_redo(&mut self, cmd: Box<dyn Command>) {
        self.redo_stack.push(cmd);
    }

    /// Pops a command from the redo stack (used by Document)
    pub(crate) fn pop_redo(&mut self) -> Option<Box<dyn Command>> {
        self.redo_stack.pop()
    }

    /// Pushes a command onto the undo stack (used by Document)
    pub(crate) fn push_undo(&mut self, cmd: Box<dyn Command>) {
        self.undo_stack.push(cmd);

        // Enforce max size by removing oldest commands
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Gets the current maximum history size
    ///
    /// # Returns
    /// The maximum number of commands that can be stored in the undo stack
    pub fn get_max_size(&self) -> usize {
        self.max_size
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
    /// - Reducing the limit immediately frees memory from removed commands
    /// - Each command stores ~64 bytes + the size of affected text
    /// - Example: Reducing from 100 to 50 commands saves ~3.2KB + text data
    ///
    /// # Example
    /// ```
    /// use rich_text_editor_wasm::operations::history::CommandHistory;
    ///
    /// let mut history = CommandHistory::new();
    /// assert_eq!(history.get_max_size(), 100);
    ///
    /// // Reduce history limit for memory-constrained environments
    /// history.set_max_size(50);
    /// assert_eq!(history.get_max_size(), 50);
    /// ```
    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;

        // Remove oldest commands if we exceed the new limit
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Position;
    use crate::operations::InsertCommand;

    #[test]
    fn test_new_history() {
        let history = CommandHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_execute_command() {
        let mut history = CommandHistory::new();
        let mut doc = Document::new();

        let cmd = Box::new(InsertCommand::new(Position::new(0), "Hello".to_string()));
        history.execute(cmd, &mut doc).unwrap();

        assert_eq!(doc.get_content(), "Hello");
        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 1);
    }

    #[test]
    fn test_undo() {
        let mut history = CommandHistory::new();
        let mut doc = Document::new();

        let cmd = Box::new(InsertCommand::new(Position::new(0), "Hello".to_string()));
        history.execute(cmd, &mut doc).unwrap();

        history.undo(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "");
        assert!(!history.can_undo());
        assert!(history.can_redo());
        assert_eq!(history.redo_count(), 1);
    }

    #[test]
    fn test_redo() {
        let mut history = CommandHistory::new();
        let mut doc = Document::new();

        let cmd = Box::new(InsertCommand::new(Position::new(0), "Hello".to_string()));
        history.execute(cmd, &mut doc).unwrap();
        history.undo(&mut doc).unwrap();

        history.redo(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello");
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_multiple_undo_redo() {
        let mut history = CommandHistory::new();
        let mut doc = Document::new();

        history
            .execute(
                Box::new(InsertCommand::new(Position::new(0), "Hello".to_string())),
                &mut doc,
            )
            .unwrap();
        history
            .execute(
                Box::new(InsertCommand::new(Position::new(5), " World".to_string())),
                &mut doc,
            )
            .unwrap();

        assert_eq!(doc.get_content(), "Hello World");
        assert_eq!(history.undo_count(), 2);

        history.undo(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello");

        history.undo(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "");

        history.redo(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello");

        history.redo(&mut doc).unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_redo_cleared_on_new_command() {
        let mut history = CommandHistory::new();
        let mut doc = Document::new();

        history
            .execute(
                Box::new(InsertCommand::new(Position::new(0), "Hello".to_string())),
                &mut doc,
            )
            .unwrap();
        history.undo(&mut doc).unwrap();

        assert!(history.can_redo());

        // Execute a new command - should clear redo stack
        history
            .execute(
                Box::new(InsertCommand::new(Position::new(0), "World".to_string())),
                &mut doc,
            )
            .unwrap();

        assert!(!history.can_redo());
        assert_eq!(doc.get_content(), "World");
    }

    #[test]
    fn test_max_size_limit() {
        let mut history = CommandHistory::with_max_size(3);
        let mut doc = Document::new();

        // Add 5 commands
        for i in 0..5 {
            history
                .execute(
                    Box::new(InsertCommand::new(Position::new(i), "a".to_string())),
                    &mut doc,
                )
                .unwrap();
        }

        // Should only keep the last 3
        assert_eq!(history.undo_count(), 3);

        // Undo all 3
        history.undo(&mut doc).unwrap();
        history.undo(&mut doc).unwrap();
        history.undo(&mut doc).unwrap();

        // Should not be able to undo more
        assert!(!history.can_undo());
    }

    #[test]
    fn test_clear() {
        let mut history = CommandHistory::new();
        let mut doc = Document::new();

        history
            .execute(
                Box::new(InsertCommand::new(Position::new(0), "Hello".to_string())),
                &mut doc,
            )
            .unwrap();
        history.undo(&mut doc).unwrap();

        assert!(history.can_redo());

        history.clear();

        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_undo_empty_stack() {
        let mut history = CommandHistory::new();
        let mut doc = Document::new();

        let result = history.undo(&mut doc);
        assert!(result.is_err());
    }

    #[test]
    fn test_redo_empty_stack() {
        let mut history = CommandHistory::new();
        let mut doc = Document::new();

        let result = history.redo(&mut doc);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_max_size() {
        let history = CommandHistory::new();
        assert_eq!(history.get_max_size(), CommandHistory::DEFAULT_MAX_SIZE);

        let history_custom = CommandHistory::with_max_size(50);
        assert_eq!(history_custom.get_max_size(), 50);
    }

    #[test]
    fn test_set_max_size() {
        let mut history = CommandHistory::new();
        assert_eq!(history.get_max_size(), 100);

        history.set_max_size(50);
        assert_eq!(history.get_max_size(), 50);

        history.set_max_size(20);
        assert_eq!(history.get_max_size(), 20);
    }

    #[test]
    fn test_set_max_size_removes_old_commands() {
        let mut history = CommandHistory::new();
        let mut doc = Document::new();

        // Add 10 commands
        for i in 0..10 {
            history
                .execute(
                    Box::new(InsertCommand::new(Position::new(i), "a".to_string())),
                    &mut doc,
                )
                .unwrap();
        }

        assert_eq!(history.undo_count(), 10);

        // Reduce limit to 5
        history.set_max_size(5);
        assert_eq!(history.undo_count(), 5);
        assert_eq!(history.get_max_size(), 5);

        // The 5 most recent commands should remain
        // Undo 5 times should work
        for _ in 0..5 {
            assert!(history.undo(&mut doc).is_ok());
        }

        // 6th undo should fail
        assert!(history.undo(&mut doc).is_err());
    }

    #[test]
    fn test_set_max_size_with_empty_history() {
        let mut history = CommandHistory::new();
        assert_eq!(history.undo_count(), 0);

        history.set_max_size(50);
        assert_eq!(history.get_max_size(), 50);
        assert_eq!(history.undo_count(), 0);
    }
}
