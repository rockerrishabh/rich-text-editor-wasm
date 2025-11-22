// Error types for WASM bindings

use crate::operations::CommandError;
use crate::serialization::{html::HtmlError, json::JsonError, markdown::MarkdownError};
use std::fmt;
use wasm_bindgen::JsValue;

/// Comprehensive error type for the Editor System
///
/// This enum encompasses all possible error conditions that can occur
/// during editor operations, providing detailed context for debugging
/// and user-facing error messages.
#[derive(Debug, Clone)]
pub enum EditorError {
    /// Invalid position in document
    ///
    /// Occurs when an operation attempts to access a position beyond
    /// the document's length.
    ///
    /// # Context
    /// - `position`: The invalid position that was requested
    /// - `length`: The current document length
    InvalidPosition { position: usize, length: usize },

    /// Invalid range in document
    ///
    /// Occurs when an operation specifies a range where start > end
    /// or end exceeds the document length.
    ///
    /// # Context
    /// - `start`: The start position of the range
    /// - `end`: The end position of the range
    /// - `length`: The current document length
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
    /// - `operation`: The name of the operation that failed
    /// - `reason`: Detailed explanation of why it failed
    CommandFailed { operation: String, reason: String },

    /// Undo operation failed
    ///
    /// Occurs when there are no operations to undo or the undo
    /// operation encounters an error.
    ///
    /// # Context
    /// - `reason`: Explanation of why undo failed
    UndoFailed { reason: String },

    /// Redo operation failed
    ///
    /// Occurs when there are no operations to redo or the redo
    /// operation encounters an error.
    ///
    /// # Context
    /// - `reason`: Explanation of why redo failed
    RedoFailed { reason: String },

    /// JSON serialization error
    ///
    /// Occurs when document cannot be serialized to JSON format.
    ///
    /// # Context
    /// - `message`: Error message from the JSON serializer
    JsonSerializationError { message: String },

    /// JSON deserialization error
    ///
    /// Occurs when JSON cannot be parsed or contains invalid data.
    ///
    /// # Context
    /// - `message`: Error message from the JSON deserializer
    JsonDeserializationError { message: String },

    /// Markdown parsing error
    ///
    /// Occurs when Markdown content cannot be parsed correctly.
    ///
    /// # Context
    /// - `message`: Error message from the Markdown parser
    MarkdownParseError { message: String },

    /// HTML parsing error
    ///
    /// Occurs when HTML content cannot be parsed correctly.
    ///
    /// # Context
    /// - `message`: Error message from the HTML parser
    HtmlParseError { message: String },

    /// HTML sanitization error
    ///
    /// Occurs when HTML sanitization encounters dangerous content
    /// that cannot be safely processed.
    ///
    /// # Context
    /// - `message`: Description of the sanitization issue
    HtmlSanitizationError { message: String },

    /// Search operation failed
    ///
    /// Occurs when a search query is invalid (e.g., invalid regex pattern).
    ///
    /// # Context
    /// - `pattern`: The search pattern that caused the error
    /// - `reason`: Explanation of why the search failed
    SearchFailed { pattern: String, reason: String },

    /// Format operation failed
    ///
    /// Occurs when a formatting operation cannot be applied.
    ///
    /// # Context
    /// - `format_type`: The type of format that failed
    /// - `reason`: Explanation of why the format failed
    FormatFailed { format_type: String, reason: String },

    /// Memory allocation failed
    ///
    /// Occurs when the system runs out of memory during an operation.
    /// This is a critical error that typically requires user intervention.
    OutOfMemory,

    /// Generic operation error
    ///
    /// Catch-all for errors that don't fit other categories.
    ///
    /// # Context
    /// - `operation`: The operation that failed
    /// - `message`: Detailed error message
    OperationError { operation: String, message: String },
}

impl fmt::Display for EditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorError::InvalidPosition { position, length } => {
                write!(
                    f,
                    "Invalid position: {} (document length: {})",
                    position, length
                )
            }
            EditorError::InvalidRange { start, end, length } => {
                write!(
                    f,
                    "Invalid range: {}..{} (document length: {})",
                    start, end, length
                )
            }
            EditorError::CommandFailed { operation, reason } => {
                write!(f, "Command '{}' failed: {}", operation, reason)
            }
            EditorError::UndoFailed { reason } => {
                write!(f, "Undo failed: {}", reason)
            }
            EditorError::RedoFailed { reason } => {
                write!(f, "Redo failed: {}", reason)
            }
            EditorError::JsonSerializationError { message } => {
                write!(f, "JSON serialization error: {}", message)
            }
            EditorError::JsonDeserializationError { message } => {
                write!(f, "JSON deserialization error: {}", message)
            }
            EditorError::MarkdownParseError { message } => {
                write!(f, "Markdown parsing error: {}", message)
            }
            EditorError::HtmlParseError { message } => {
                write!(f, "HTML parsing error: {}", message)
            }
            EditorError::HtmlSanitizationError { message } => {
                write!(f, "HTML sanitization error: {}", message)
            }
            EditorError::SearchFailed { pattern, reason } => {
                write!(f, "Search failed for pattern '{}': {}", pattern, reason)
            }
            EditorError::FormatFailed {
                format_type,
                reason,
            } => {
                write!(f, "Format '{}' failed: {}", format_type, reason)
            }
            EditorError::OutOfMemory => {
                write!(f, "Memory allocation failed: out of memory")
            }
            EditorError::OperationError { operation, message } => {
                write!(f, "Operation '{}' failed: {}", operation, message)
            }
        }
    }
}

impl std::error::Error for EditorError {}

// Conversions from internal error types to EditorError

impl From<CommandError> for EditorError {
    fn from(err: CommandError) -> Self {
        match err {
            CommandError::InvalidPosition { position, length } => {
                EditorError::InvalidPosition { position, length }
            }
            CommandError::InvalidRange { start, end, length } => {
                EditorError::InvalidRange { start, end, length }
            }
            CommandError::ExecutionFailed { command, reason } => EditorError::CommandFailed {
                operation: command,
                reason,
            },
            CommandError::NothingToUndo => EditorError::UndoFailed {
                reason: "No operations to undo".to_string(),
            },
            CommandError::NothingToRedo => EditorError::RedoFailed {
                reason: "No operations to redo".to_string(),
            },
            CommandError::CommandNotExecuted { command } => EditorError::CommandFailed {
                operation: command,
                reason: "Command has not been executed".to_string(),
            },
            CommandError::HistoryLimitReached { limit } => EditorError::CommandFailed {
                operation: "history".to_string(),
                reason: format!("History limit of {} commands reached", limit),
            },
        }
    }
}

impl From<JsonError> for EditorError {
    fn from(err: JsonError) -> Self {
        match err {
            JsonError::ParseError(e) => EditorError::JsonDeserializationError {
                message: e.to_string(),
            },
            JsonError::UnsupportedVersion(v) => EditorError::JsonDeserializationError {
                message: format!("Unsupported schema version: {}", v),
            },
            JsonError::InvalidFormat(msg) => EditorError::JsonDeserializationError {
                message: format!("Invalid format data: {}", msg),
            },
            JsonError::InvalidBlock(msg) => EditorError::JsonDeserializationError {
                message: format!("Invalid block data: {}", msg),
            },
        }
    }
}

impl From<MarkdownError> for EditorError {
    fn from(err: MarkdownError) -> Self {
        match err {
            MarkdownError::ParseError(msg) => EditorError::MarkdownParseError { message: msg },
            MarkdownError::InvalidFormat(msg) => EditorError::MarkdownParseError {
                message: format!("Invalid format: {}", msg),
            },
        }
    }
}

impl From<HtmlError> for EditorError {
    fn from(err: HtmlError) -> Self {
        match err {
            HtmlError::ParseError(msg) => EditorError::HtmlParseError { message: msg },
            HtmlError::InvalidFormat(msg) => EditorError::HtmlParseError {
                message: format!("Invalid format: {}", msg),
            },
            HtmlError::SanitizationError(msg) => {
                EditorError::HtmlSanitizationError { message: msg }
            }
        }
    }
}

// Conversion to JsValue for WASM boundary

impl From<EditorError> for JsValue {
    fn from(err: EditorError) -> Self {
        // Create a JavaScript Error object with the error message
        let error_msg = err.to_string();
        JsValue::from_str(&error_msg)
    }
}

// Helper function to create EditorError from string message
impl EditorError {
    /// Creates a generic operation error
    pub fn operation_error(operation: impl Into<String>, message: impl Into<String>) -> Self {
        EditorError::OperationError {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Creates an invalid position error with context
    pub fn invalid_position(position: usize, length: usize) -> Self {
        EditorError::InvalidPosition { position, length }
    }

    /// Creates an invalid range error with context
    pub fn invalid_range(start: usize, end: usize, length: usize) -> Self {
        EditorError::InvalidRange { start, end, length }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;



    #[wasm_bindgen_test]
    fn test_invalid_position_error() {
        let err = EditorError::invalid_position(100, 50);
        let msg = err.to_string();
        assert!(msg.contains("Invalid position"));
        assert!(msg.contains("100"));
        assert!(msg.contains("50"));
    }

    #[wasm_bindgen_test]
    fn test_invalid_range_error() {
        let err = EditorError::invalid_range(10, 5, 20);
        let msg = err.to_string();
        assert!(msg.contains("Invalid range"));
        assert!(msg.contains("10"));
        assert!(msg.contains("5"));
    }

    #[wasm_bindgen_test]
    fn test_command_failed_error() {
        let err = EditorError::CommandFailed {
            operation: "insert".to_string(),
            reason: "invalid state".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("insert"));
        assert!(msg.contains("invalid state"));
    }

    #[wasm_bindgen_test]
    fn test_json_serialization_error() {
        let err = EditorError::JsonSerializationError {
            message: "invalid data".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("JSON serialization"));
        assert!(msg.contains("invalid data"));
    }

    #[wasm_bindgen_test]
    fn test_error_to_jsvalue() {
        let err = EditorError::OutOfMemory;
        let js_val: JsValue = err.into();
        assert!(js_val.is_string());
    }

    #[wasm_bindgen_test]
    fn test_operation_error_helper() {
        let err = EditorError::operation_error("test_op", "test message");
        let msg = err.to_string();
        assert!(msg.contains("test_op"));
        assert!(msg.contains("test message"));
    }

    #[wasm_bindgen_test]
    fn test_command_error_conversion() {
        let cmd_err = CommandError::invalid_position(42, 10);
        let editor_err: EditorError = cmd_err.into();
        let msg = editor_err.to_string();
        assert!(msg.contains("Invalid position"));
    }
}
