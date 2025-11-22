// Cursor movement functionality

use crate::document::{Document, Position};
use crate::selection::Selection;

/// Helper functions for cursor movement
impl Document {
    /// Moves the cursor left by one character
    pub fn move_cursor_left(&mut self) {
        let current_pos = self.selection.focus.offset();
        if current_pos > 0 {
            let new_pos = Position::new(current_pos - 1);
            self.selection = Selection::collapsed(new_pos);
        }
    }

    /// Moves the cursor right by one character
    pub fn move_cursor_right(&mut self) {
        let current_pos = self.selection.focus.offset();
        let max_pos = self.get_length();
        if current_pos < max_pos {
            let new_pos = Position::new(current_pos + 1);
            self.selection = Selection::collapsed(new_pos);
        }
    }

    /// Moves the cursor up by one line
    pub fn move_cursor_up(&mut self) {
        let current_pos = self.selection.focus.offset();
        let content = self.get_content();

        // Find the start of the current line
        let line_start = self.find_line_start(current_pos, &content);

        if line_start == 0 {
            // Already at the first line, move to document start
            self.selection = Selection::collapsed(Position::new(0));
            return;
        }

        // Find the start of the previous line
        let prev_line_start = self.find_line_start(line_start - 1, &content);

        // Calculate column offset in current line
        let column = current_pos - line_start;

        // Find the end of the previous line
        let prev_line_end = line_start - 1;
        let prev_line_length = prev_line_end - prev_line_start;

        // Move to the same column in the previous line, or to the end if the line is shorter
        let new_pos = prev_line_start + column.min(prev_line_length);
        self.selection = Selection::collapsed(Position::new(new_pos));
    }

    /// Moves the cursor down by one line
    pub fn move_cursor_down(&mut self) {
        let current_pos = self.selection.focus.offset();
        let content = self.get_content();
        let max_pos = self.get_length();

        // Find the start of the current line
        let line_start = self.find_line_start(current_pos, &content);

        // Find the end of the current line
        let line_end = self.find_line_end(current_pos, &content);

        if line_end >= max_pos {
            // Already at the last line, move to document end
            self.selection = Selection::collapsed(Position::new(max_pos));
            return;
        }

        // Calculate column offset in current line
        let column = current_pos - line_start;

        // Find the start of the next line (skip the newline character)
        let next_line_start = line_end + 1;

        // Find the end of the next line
        let next_line_end = self.find_line_end(next_line_start, &content);
        let next_line_length = next_line_end - next_line_start;

        // Move to the same column in the next line, or to the end if the line is shorter
        let new_pos = next_line_start + column.min(next_line_length);
        self.selection = Selection::collapsed(Position::new(new_pos));
    }

    /// Moves the cursor to the start of the current line
    pub fn move_to_line_start(&mut self) {
        let current_pos = self.selection.focus.offset();
        let content = self.get_content();
        let line_start = self.find_line_start(current_pos, &content);
        self.selection = Selection::collapsed(Position::new(line_start));
    }

    /// Moves the cursor to the end of the current line
    pub fn move_to_line_end(&mut self) {
        let current_pos = self.selection.focus.offset();
        let content = self.get_content();
        let line_end = self.find_line_end(current_pos, &content);
        self.selection = Selection::collapsed(Position::new(line_end));
    }

    /// Moves the cursor to the start of the document
    pub fn move_to_document_start(&mut self) {
        self.selection = Selection::collapsed(Position::new(0));
    }

    /// Moves the cursor to the end of the document
    pub fn move_to_document_end(&mut self) {
        let end = Position::new(self.get_length());
        self.selection = Selection::collapsed(end);
    }

    /// Moves the cursor by word boundaries
    /// If forward is true, moves to the next word; otherwise, moves to the previous word
    pub fn move_by_word(&mut self, forward: bool) {
        let current_pos = self.selection.focus.offset();
        let content = self.get_content();

        let new_pos = if forward {
            self.find_next_word_boundary(current_pos, &content)
        } else {
            self.find_previous_word_boundary(current_pos, &content)
        };

        self.selection = Selection::collapsed(Position::new(new_pos));
    }

    // Helper methods for line and word boundary detection

    /// Finds the start of the line containing the given position
    fn find_line_start(&self, pos: usize, content: &str) -> usize {
        let chars: Vec<char> = content.chars().collect();
        let mut line_start = pos;

        while line_start > 0 && chars[line_start - 1] != '\n' {
            line_start -= 1;
        }

        line_start
    }

    /// Finds the end of the line containing the given position
    fn find_line_end(&self, pos: usize, content: &str) -> usize {
        let chars: Vec<char> = content.chars().collect();
        let mut line_end = pos;

        while line_end < chars.len() && chars[line_end] != '\n' {
            line_end += 1;
        }

        line_end
    }

    /// Finds the next word boundary after the given position
    fn find_next_word_boundary(&self, pos: usize, content: &str) -> usize {
        let chars: Vec<char> = content.chars().collect();
        let len = chars.len();

        if pos >= len {
            return len;
        }

        let mut i = pos;

        // Skip current word characters
        while i < len && Self::is_word_char(chars[i]) {
            i += 1;
        }

        // Skip non-word characters (whitespace and punctuation)
        while i < len && !Self::is_word_char(chars[i]) {
            i += 1;
        }

        i
    }

    /// Finds the previous word boundary before the given position
    fn find_previous_word_boundary(&self, pos: usize, content: &str) -> usize {
        let chars: Vec<char> = content.chars().collect();

        if pos == 0 {
            return 0;
        }

        let mut i = pos;

        // Move back one position
        i -= 1;

        // Skip whitespace
        while i > 0 && Self::is_whitespace(chars[i]) {
            i -= 1;
        }

        // Skip word characters to find the start of the word
        while i > 0 && Self::is_word_char(chars[i - 1]) {
            i -= 1;
        }

        i
    }

    /// Returns true if the character is a word character (alphanumeric or underscore)
    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    /// Returns true if the character is whitespace
    fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_cursor_left() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(Selection::collapsed(Position::new(3)));

        doc.move_cursor_left();
        assert_eq!(doc.get_selection().focus.offset(), 2);

        doc.move_cursor_left();
        assert_eq!(doc.get_selection().focus.offset(), 1);
    }

    #[test]
    fn test_move_cursor_left_at_start() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(Selection::collapsed(Position::new(0)));

        doc.move_cursor_left();
        assert_eq!(doc.get_selection().focus.offset(), 0);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(Selection::collapsed(Position::new(2)));

        doc.move_cursor_right();
        assert_eq!(doc.get_selection().focus.offset(), 3);

        doc.move_cursor_right();
        assert_eq!(doc.get_selection().focus.offset(), 4);
    }

    #[test]
    fn test_move_cursor_right_at_end() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(Selection::collapsed(Position::new(5)));

        doc.move_cursor_right();
        assert_eq!(doc.get_selection().focus.offset(), 5);
    }

    #[test]
    fn test_move_to_line_start() {
        let mut doc = Document::from_text("Hello World");
        doc.set_selection(Selection::collapsed(Position::new(7)));

        doc.move_to_line_start();
        assert_eq!(doc.get_selection().focus.offset(), 0);
    }

    #[test]
    fn test_move_to_line_start_multiline() {
        let mut doc = Document::from_text("Line 1\nLine 2\nLine 3");
        doc.set_selection(Selection::collapsed(Position::new(10)));

        doc.move_to_line_start();
        assert_eq!(doc.get_selection().focus.offset(), 7);
    }

    #[test]
    fn test_move_to_line_end() {
        let mut doc = Document::from_text("Hello World");
        doc.set_selection(Selection::collapsed(Position::new(3)));

        doc.move_to_line_end();
        assert_eq!(doc.get_selection().focus.offset(), 11);
    }

    #[test]
    fn test_move_to_line_end_multiline() {
        let mut doc = Document::from_text("Line 1\nLine 2\nLine 3");
        doc.set_selection(Selection::collapsed(Position::new(2)));

        doc.move_to_line_end();
        assert_eq!(doc.get_selection().focus.offset(), 6);
    }

    #[test]
    fn test_move_to_document_start() {
        let mut doc = Document::from_text("Hello World");
        doc.set_selection(Selection::collapsed(Position::new(7)));

        doc.move_to_document_start();
        assert_eq!(doc.get_selection().focus.offset(), 0);
    }

    #[test]
    fn test_move_to_document_end() {
        let mut doc = Document::from_text("Hello World");
        doc.set_selection(Selection::collapsed(Position::new(3)));

        doc.move_to_document_end();
        assert_eq!(doc.get_selection().focus.offset(), 11);
    }

    #[test]
    fn test_move_cursor_up() {
        let mut doc = Document::from_text("Line 1\nLine 2\nLine 3");
        doc.set_selection(Selection::collapsed(Position::new(10))); // "L" in "Line 2"

        doc.move_cursor_up();
        assert_eq!(doc.get_selection().focus.offset(), 3); // "e" in "Line 1"
    }

    #[test]
    fn test_move_cursor_up_at_first_line() {
        let mut doc = Document::from_text("Line 1\nLine 2");
        doc.set_selection(Selection::collapsed(Position::new(3)));

        doc.move_cursor_up();
        assert_eq!(doc.get_selection().focus.offset(), 0);
    }

    #[test]
    fn test_move_cursor_down() {
        let mut doc = Document::from_text("Line 1\nLine 2\nLine 3");
        doc.set_selection(Selection::collapsed(Position::new(3))); // "e" in "Line 1"

        doc.move_cursor_down();
        assert_eq!(doc.get_selection().focus.offset(), 10); // "e" in "Line 2"
    }

    #[test]
    fn test_move_cursor_down_at_last_line() {
        let mut doc = Document::from_text("Line 1\nLine 2");
        doc.set_selection(Selection::collapsed(Position::new(10)));

        doc.move_cursor_down();
        assert_eq!(doc.get_selection().focus.offset(), 13);
    }

    #[test]
    fn test_move_cursor_up_down_shorter_line() {
        let mut doc = Document::from_text("Long line here\nShort\nAnother long line");
        doc.set_selection(Selection::collapsed(Position::new(10))); // Middle of first line

        doc.move_cursor_down();
        // Should move to end of shorter line
        assert_eq!(doc.get_selection().focus.offset(), 20); // End of "Short"
    }

    #[test]
    fn test_move_by_word_forward() {
        let mut doc = Document::from_text("Hello World Test");
        doc.set_selection(Selection::collapsed(Position::new(0)));

        doc.move_by_word(true);
        assert_eq!(doc.get_selection().focus.offset(), 6); // After "Hello "

        doc.move_by_word(true);
        assert_eq!(doc.get_selection().focus.offset(), 12); // After "World "
    }

    #[test]
    fn test_move_by_word_backward() {
        let mut doc = Document::from_text("Hello World Test");
        doc.set_selection(Selection::collapsed(Position::new(16)));

        doc.move_by_word(false);
        assert_eq!(doc.get_selection().focus.offset(), 12); // Start of "Test"

        doc.move_by_word(false);
        assert_eq!(doc.get_selection().focus.offset(), 6); // Start of "World"
    }

    #[test]
    fn test_move_by_word_forward_at_end() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(Selection::collapsed(Position::new(5)));

        doc.move_by_word(true);
        assert_eq!(doc.get_selection().focus.offset(), 5);
    }

    #[test]
    fn test_move_by_word_backward_at_start() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(Selection::collapsed(Position::new(0)));

        doc.move_by_word(false);
        assert_eq!(doc.get_selection().focus.offset(), 0);
    }

    #[test]
    fn test_move_by_word_with_punctuation() {
        let mut doc = Document::from_text("Hello, World!");
        doc.set_selection(Selection::collapsed(Position::new(0)));

        doc.move_by_word(true);
        // Moves to end of "Hello" (position 5), then skips ", " to reach "World"
        assert_eq!(doc.get_selection().focus.offset(), 7); // Start of "World"

        // Move back
        doc.move_by_word(false);
        assert_eq!(doc.get_selection().focus.offset(), 0); // Start of "Hello"
    }

    #[test]
    fn test_move_by_word_multiple_spaces() {
        let mut doc = Document::from_text("Hello   World");
        doc.set_selection(Selection::collapsed(Position::new(0)));

        doc.move_by_word(true);
        assert_eq!(doc.get_selection().focus.offset(), 8); // After "Hello   "
    }

    #[test]
    fn test_cursor_movement_preserves_collapsed_selection() {
        let mut doc = Document::from_text("Hello World");
        doc.set_selection(Selection::collapsed(Position::new(5)));

        doc.move_cursor_right();
        assert!(doc.get_selection().is_collapsed());

        doc.move_cursor_left();
        assert!(doc.get_selection().is_collapsed());

        doc.move_to_line_start();
        assert!(doc.get_selection().is_collapsed());

        doc.move_to_line_end();
        assert!(doc.get_selection().is_collapsed());
    }
}
