//! Document error types
//!
//! This module defines error types specific to document operations.

use thiserror::Error;

/// Errors that can occur during document operations
///
/// This enum provides detailed context for all document-related errors,
/// including position information, operation details, and helpful error messages.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum DocumentError {
    /// Position is out of bounds
    ///
    /// Occurs when an operation attempts to access a position beyond
    /// the document's length.
    ///
    /// # Context
    /// - `position`: The invalid position that was requested
    /// - `length`: The current document length
    #[error("Position {position} is out of bounds (document length: {length})")]
    OutOfBounds { position: usize, length: usize },

    /// Range is invalid
    ///
    /// Occurs when an operation specifies a range where start > end
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

    /// Empty document operation
    ///
    /// Occurs when an operation requires content but the document is empty.
    #[error("Cannot perform operation on empty document")]
    EmptyDocument,

    /// Zero-length range
    ///
    /// Occurs when an operation requires a non-empty range but receives
    /// a collapsed range (start == end).
    ///
    /// # Context
    /// - `position`: The position where the range is collapsed
    #[error("Cannot perform operation on zero-length range at position {position}")]
    ZeroLengthRange { position: usize },

    /// Document size limit exceeded
    ///
    /// Occurs when an operation would cause the document to exceed
    /// the maximum allowed size.
    ///
    /// # Context
    /// - `current_size`: Current document size in characters
    /// - `attempted_size`: Size that was attempted
    /// - `max_size`: Maximum allowed size
    #[error(
        "Document size limit exceeded: attempted {attempted_size}, max {max_size} (current: {current_size})"
    )]
    SizeLimitExceeded {
        current_size: usize,
        attempted_size: usize,
        max_size: usize,
    },

    /// Invalid text content
    ///
    /// Occurs when text content contains invalid characters or sequences.
    ///
    /// # Context
    /// - `reason`: Explanation of why the text is invalid
    #[error("Invalid text content: {reason}")]
    InvalidText { reason: String },

    /// Operation not allowed in current state
    ///
    /// Occurs when an operation cannot be performed due to document state.
    ///
    /// # Context
    /// - `operation`: The operation that was attempted
    /// - `reason`: Explanation of why it's not allowed
    #[error("Operation '{operation}' not allowed: {reason}")]
    OperationNotAllowed { operation: String, reason: String },
}

impl DocumentError {
    /// Creates an out of bounds error
    pub fn out_of_bounds(position: usize, length: usize) -> Self {
        Self::OutOfBounds { position, length }
    }

    /// Creates an invalid range error
    pub fn invalid_range(start: usize, end: usize, length: usize) -> Self {
        Self::InvalidRange { start, end, length }
    }

    /// Creates a zero-length range error
    pub fn zero_length_range(position: usize) -> Self {
        Self::ZeroLengthRange { position }
    }

    /// Creates a size limit exceeded error
    pub fn size_limit_exceeded(
        current_size: usize,
        attempted_size: usize,
        max_size: usize,
    ) -> Self {
        Self::SizeLimitExceeded {
            current_size,
            attempted_size,
            max_size,
        }
    }

    /// Creates an invalid text error
    pub fn invalid_text(reason: impl Into<String>) -> Self {
        Self::InvalidText {
            reason: reason.into(),
        }
    }

    /// Creates an operation not allowed error
    pub fn operation_not_allowed(operation: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::OperationNotAllowed {
            operation: operation.into(),
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_bounds_error() {
        let err = DocumentError::out_of_bounds(100, 50);
        let msg = err.to_string();
        assert!(msg.contains("100"));
        assert!(msg.contains("50"));
        assert!(msg.contains("out of bounds"));
    }

    #[test]
    fn test_invalid_range_error() {
        let err = DocumentError::invalid_range(10, 5, 20);
        let msg = err.to_string();
        assert!(msg.contains("10"));
        assert!(msg.contains("5"));
        assert!(msg.contains("20"));
        assert!(msg.contains("Invalid range"));
    }

    #[test]
    fn test_empty_document_error() {
        let err = DocumentError::EmptyDocument;
        let msg = err.to_string();
        assert!(msg.contains("empty document"));
    }

    #[test]
    fn test_zero_length_range_error() {
        let err = DocumentError::zero_length_range(42);
        let msg = err.to_string();
        assert!(msg.contains("42"));
        assert!(msg.contains("zero-length"));
    }

    #[test]
    fn test_size_limit_exceeded_error() {
        let err = DocumentError::size_limit_exceeded(1000, 2000, 1500);
        let msg = err.to_string();
        assert!(msg.contains("1000"));
        assert!(msg.contains("2000"));
        assert!(msg.contains("1500"));
        assert!(msg.contains("exceeded"));
    }

    #[test]
    fn test_invalid_text_error() {
        let err = DocumentError::invalid_text("contains null bytes");
        let msg = err.to_string();
        assert!(msg.contains("null bytes"));
    }

    #[test]
    fn test_operation_not_allowed_error() {
        let err = DocumentError::operation_not_allowed("delete", "document is read-only");
        let msg = err.to_string();
        assert!(msg.contains("delete"));
        assert!(msg.contains("read-only"));
    }

    #[test]
    fn test_error_equality() {
        let err1 = DocumentError::out_of_bounds(10, 5);
        let err2 = DocumentError::out_of_bounds(10, 5);
        let err3 = DocumentError::out_of_bounds(11, 5);

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}
