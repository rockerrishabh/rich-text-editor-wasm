//! Serialization error types
//!
//! This module defines error types for all serialization operations.

use thiserror::Error;

/// Comprehensive error type for all serialization operations
///
/// This enum encompasses errors from JSON, HTML, and Markdown
/// serialization/deserialization, providing detailed context for debugging.
#[derive(Debug, Error)]
pub enum SerializationError {
    // JSON errors
    /// JSON parsing error
    ///
    /// Occurs when JSON cannot be parsed due to syntax errors.
    ///
    /// # Context
    /// - `message`: Error message from the JSON parser
    /// - `line`: Optional line number where the error occurred
    /// - `column`: Optional column number where the error occurred
    #[error("JSON parsing error{}: {message}", format_position(.line, .column))]
    JsonParseError {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },

    /// Unsupported JSON schema version
    ///
    /// Occurs when the JSON document uses a schema version that
    /// is not supported by this implementation.
    ///
    /// # Context
    /// - `version`: The unsupported version string
    /// - `supported_versions`: List of supported versions
    #[error("Unsupported schema version '{version}' (supported: {supported_versions})")]
    UnsupportedVersion {
        version: String,
        supported_versions: String,
    },

    /// Invalid format data in JSON
    ///
    /// Occurs when format data in JSON is malformed or invalid.
    ///
    /// # Context
    /// - `field`: The field that contains invalid data
    /// - `reason`: Explanation of why it's invalid
    #[error("Invalid format data in field '{field}': {reason}")]
    InvalidFormatData { field: String, reason: String },

    /// Invalid block data in JSON
    ///
    /// Occurs when block data in JSON is malformed or invalid.
    ///
    /// # Context
    /// - `field`: The field that contains invalid data
    /// - `reason`: Explanation of why it's invalid
    #[error("Invalid block data in field '{field}': {reason}")]
    InvalidBlockData { field: String, reason: String },

    // HTML errors
    /// HTML parsing error
    ///
    /// Occurs when HTML cannot be parsed due to malformed markup.
    ///
    /// # Context
    /// - `message`: Error message describing the parsing issue
    /// - `position`: Optional character position where the error occurred
    #[error("HTML parsing error{}: {message}", format_html_position(.position))]
    HtmlParseError {
        message: String,
        position: Option<usize>,
    },

    /// HTML sanitization error
    ///
    /// Occurs when HTML contains dangerous content that cannot be
    /// safely processed even after sanitization attempts.
    ///
    /// # Context
    /// - `element`: The HTML element or attribute that caused the issue
    /// - `reason`: Explanation of the security concern
    #[error("HTML sanitization error in '{element}': {reason}")]
    HtmlSanitizationError { element: String, reason: String },

    /// Invalid HTML structure
    ///
    /// Occurs when HTML has structural issues like mismatched tags.
    ///
    /// # Context
    /// - `message`: Description of the structural issue
    #[error("Invalid HTML structure: {message}")]
    InvalidHtmlStructure { message: String },

    // Markdown errors
    /// Markdown parsing error
    ///
    /// Occurs when Markdown cannot be parsed correctly.
    ///
    /// # Context
    /// - `message`: Error message describing the parsing issue
    /// - `line`: Optional line number where the error occurred
    #[error("Markdown parsing error{}: {message}", format_markdown_line(.line))]
    MarkdownParseError {
        message: String,
        line: Option<usize>,
    },

    /// Invalid Markdown syntax
    ///
    /// Occurs when Markdown contains invalid or unsupported syntax.
    ///
    /// # Context
    /// - `syntax`: The invalid syntax that was encountered
    /// - `reason`: Explanation of why it's invalid
    #[error("Invalid Markdown syntax '{syntax}': {reason}")]
    InvalidMarkdownSyntax { syntax: String, reason: String },

    // General errors
    /// Invalid content encoding
    ///
    /// Occurs when content has invalid character encoding.
    ///
    /// # Context
    /// - `encoding`: The encoding that was expected or detected
    /// - `reason`: Explanation of the encoding issue
    #[error("Invalid content encoding '{encoding}': {reason}")]
    InvalidEncoding { encoding: String, reason: String },

    /// Content too large
    ///
    /// Occurs when content exceeds maximum allowed size for serialization.
    ///
    /// # Context
    /// - `size`: The size of the content
    /// - `max_size`: The maximum allowed size
    #[error("Content too large: {size} bytes (max: {max_size} bytes)")]
    ContentTooLarge { size: usize, max_size: usize },

    /// Serialization failed
    ///
    /// Generic serialization error for cases not covered by specific errors.
    ///
    /// # Context
    /// - `format`: The format being serialized to (JSON, HTML, Markdown)
    /// - `reason`: Explanation of why serialization failed
    #[error("Serialization to {format} failed: {reason}")]
    SerializationFailed { format: String, reason: String },

    /// Deserialization failed
    ///
    /// Generic deserialization error for cases not covered by specific errors.
    ///
    /// # Context
    /// - `format`: The format being deserialized from (JSON, HTML, Markdown)
    /// - `reason`: Explanation of why deserialization failed
    #[error("Deserialization from {format} failed: {reason}")]
    DeserializationFailed { format: String, reason: String },
}

impl SerializationError {
    /// Creates a JSON parse error
    pub fn json_parse_error(message: impl Into<String>) -> Self {
        Self::JsonParseError {
            message: message.into(),
            line: None,
            column: None,
        }
    }

    /// Creates a JSON parse error with position information
    pub fn json_parse_error_at(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::JsonParseError {
            message: message.into(),
            line: Some(line),
            column: Some(column),
        }
    }

    /// Creates an unsupported version error
    pub fn unsupported_version(version: impl Into<String>) -> Self {
        Self::UnsupportedVersion {
            version: version.into(),
            supported_versions: "1.0".to_string(),
        }
    }

    /// Creates an invalid format data error
    pub fn invalid_format_data(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidFormatData {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Creates an invalid block data error
    pub fn invalid_block_data(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidBlockData {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Creates an HTML parse error
    pub fn html_parse_error(message: impl Into<String>) -> Self {
        Self::HtmlParseError {
            message: message.into(),
            position: None,
        }
    }

    /// Creates an HTML parse error with position
    pub fn html_parse_error_at(message: impl Into<String>, position: usize) -> Self {
        Self::HtmlParseError {
            message: message.into(),
            position: Some(position),
        }
    }

    /// Creates an HTML sanitization error
    pub fn html_sanitization_error(element: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::HtmlSanitizationError {
            element: element.into(),
            reason: reason.into(),
        }
    }

    /// Creates an invalid HTML structure error
    pub fn invalid_html_structure(message: impl Into<String>) -> Self {
        Self::InvalidHtmlStructure {
            message: message.into(),
        }
    }

    /// Creates a Markdown parse error
    pub fn markdown_parse_error(message: impl Into<String>) -> Self {
        Self::MarkdownParseError {
            message: message.into(),
            line: None,
        }
    }

    /// Creates a Markdown parse error with line number
    pub fn markdown_parse_error_at(message: impl Into<String>, line: usize) -> Self {
        Self::MarkdownParseError {
            message: message.into(),
            line: Some(line),
        }
    }

    /// Creates an invalid Markdown syntax error
    pub fn invalid_markdown_syntax(syntax: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidMarkdownSyntax {
            syntax: syntax.into(),
            reason: reason.into(),
        }
    }

    /// Creates an invalid encoding error
    pub fn invalid_encoding(encoding: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidEncoding {
            encoding: encoding.into(),
            reason: reason.into(),
        }
    }

    /// Creates a content too large error
    pub fn content_too_large(size: usize, max_size: usize) -> Self {
        Self::ContentTooLarge { size, max_size }
    }

    /// Creates a serialization failed error
    pub fn serialization_failed(format: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::SerializationFailed {
            format: format.into(),
            reason: reason.into(),
        }
    }

    /// Creates a deserialization failed error
    pub fn deserialization_failed(format: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::DeserializationFailed {
            format: format.into(),
            reason: reason.into(),
        }
    }
}

// Helper functions for formatting optional position information
fn format_position(line: &Option<usize>, column: &Option<usize>) -> String {
    match (line, column) {
        (Some(l), Some(c)) => format!(" at line {}, column {}", l, c),
        (Some(l), None) => format!(" at line {}", l),
        (None, Some(c)) => format!(" at column {}", c),
        (None, None) => String::new(),
    }
}

fn format_html_position(position: &Option<usize>) -> String {
    match position {
        Some(pos) => format!(" at position {}", pos),
        None => String::new(),
    }
}

fn format_markdown_line(line: &Option<usize>) -> String {
    match line {
        Some(l) => format!(" at line {}", l),
        None => String::new(),
    }
}

// Conversions from serde_json::Error
impl From<serde_json::Error> for SerializationError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonParseError {
            message: err.to_string(),
            line: Some(err.line()),
            column: Some(err.column()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parse_error() {
        let err = SerializationError::json_parse_error("unexpected token");
        let msg = err.to_string();
        assert!(msg.contains("JSON parsing error"));
        assert!(msg.contains("unexpected token"));
    }

    #[test]
    fn test_json_parse_error_with_position() {
        let err = SerializationError::json_parse_error_at("unexpected token", 5, 10);
        let msg = err.to_string();
        assert!(msg.contains("line 5"));
        assert!(msg.contains("column 10"));
    }

    #[test]
    fn test_unsupported_version() {
        let err = SerializationError::unsupported_version("2.0");
        let msg = err.to_string();
        assert!(msg.contains("2.0"));
        assert!(msg.contains("Unsupported"));
    }

    #[test]
    fn test_html_sanitization_error() {
        let err = SerializationError::html_sanitization_error("script", "dangerous content");
        let msg = err.to_string();
        assert!(msg.contains("script"));
        assert!(msg.contains("dangerous content"));
    }

    #[test]
    fn test_markdown_parse_error() {
        let err = SerializationError::markdown_parse_error_at("invalid syntax", 42);
        let msg = err.to_string();
        assert!(msg.contains("line 42"));
        assert!(msg.contains("invalid syntax"));
    }

    #[test]
    fn test_content_too_large() {
        let err = SerializationError::content_too_large(2000, 1000);
        let msg = err.to_string();
        assert!(msg.contains("2000"));
        assert!(msg.contains("1000"));
        assert!(msg.contains("too large"));
    }

    #[test]
    fn test_from_serde_json_error() {
        let json = r#"{"invalid": json}"#;
        let result: Result<serde_json::Value, _> = serde_json::from_str(json);

        if let Err(serde_err) = result {
            let err: SerializationError = serde_err.into();
            let msg = err.to_string();
            assert!(msg.contains("JSON parsing error"));
        }
    }
}
