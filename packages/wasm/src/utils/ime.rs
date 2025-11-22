// IME composition handling

use crate::document::{Position, Range};

/// Represents the state of an IME composition session
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositionState {
    /// The range where composition is occurring
    pub range: Range,
    /// The current composition text
    pub text: String,
    /// Whether composition is currently active
    pub active: bool,
}

impl CompositionState {
    /// Creates a new inactive composition state
    pub fn new() -> Self {
        Self {
            range: Range::from_offsets(0, 0),
            text: String::new(),
            active: false,
        }
    }

    /// Creates a new active composition state at the given position
    pub fn start(position: Position) -> Self {
        Self {
            range: Range::from_offsets(position.offset(), position.offset()),
            text: String::new(),
            active: true,
        }
    }

    /// Returns true if composition is currently active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Returns the composition range
    pub fn range(&self) -> Range {
        self.range
    }

    /// Returns the current composition text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Updates the composition text and range
    pub fn update(&mut self, text: String) {
        let start = self.range.start;
        let text_length = text.chars().count();
        self.text = text;
        self.range = Range::from_offsets(start.offset(), start.offset() + text_length);
    }

    /// Ends the composition and returns the final text
    pub fn end(&mut self) -> String {
        self.active = false;
        let final_text = self.text.clone();
        self.text.clear();
        self.range = Range::from_offsets(0, 0);
        final_text
    }

    /// Cancels the composition without committing
    pub fn cancel(&mut self) {
        self.active = false;
        self.text.clear();
        self.range = Range::from_offsets(0, 0);
    }
}

impl Default for CompositionState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_composition_state() {
        let state = CompositionState::new();
        assert!(!state.is_active());
        assert_eq!(state.text(), "");
        assert_eq!(state.range(), Range::from_offsets(0, 0));
    }

    #[test]
    fn test_start_composition() {
        let state = CompositionState::start(Position::new(5));
        assert!(state.is_active());
        assert_eq!(state.text(), "");
        assert_eq!(state.range(), Range::from_offsets(5, 5));
    }

    #[test]
    fn test_update_composition() {
        let mut state = CompositionState::start(Position::new(5));
        state.update("こ".to_string());

        assert!(state.is_active());
        assert_eq!(state.text(), "こ");
        assert_eq!(state.range(), Range::from_offsets(5, 6));

        state.update("こん".to_string());
        assert_eq!(state.text(), "こん");
        assert_eq!(state.range(), Range::from_offsets(5, 7));
    }

    #[test]
    fn test_end_composition() {
        let mut state = CompositionState::start(Position::new(5));
        state.update("こんにちは".to_string());

        let final_text = state.end();

        assert_eq!(final_text, "こんにちは");
        assert!(!state.is_active());
        assert_eq!(state.text(), "");
        assert_eq!(state.range(), Range::from_offsets(0, 0));
    }

    #[test]
    fn test_cancel_composition() {
        let mut state = CompositionState::start(Position::new(5));
        state.update("こんにちは".to_string());

        state.cancel();

        assert!(!state.is_active());
        assert_eq!(state.text(), "");
        assert_eq!(state.range(), Range::from_offsets(0, 0));
    }

    #[test]
    fn test_composition_with_multibyte_chars() {
        let mut state = CompositionState::start(Position::new(0));

        // Test with various CJK characters
        state.update("你".to_string());
        assert_eq!(state.range(), Range::from_offsets(0, 1));

        state.update("你好".to_string());
        assert_eq!(state.range(), Range::from_offsets(0, 2));

        state.update("你好世界".to_string());
        assert_eq!(state.range(), Range::from_offsets(0, 4));
    }

    #[test]
    fn test_composition_state_default() {
        let state = CompositionState::default();
        assert!(!state.is_active());
        assert_eq!(state.text(), "");
    }
}
