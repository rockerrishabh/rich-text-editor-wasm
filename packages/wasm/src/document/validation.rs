//! Document validation utilities
//!
//! This module provides validation functions for document operations.

use super::Range;
use super::errors::DocumentError;

/// Maximum document size in characters (10 million characters = ~40MB)
pub const MAX_DOCUMENT_SIZE: usize = 10_000_000;

/// Validates a position against document length
///
/// # Arguments
/// * `position` - The position to validate
/// * `length` - The current document length
///
/// # Returns
/// Ok(()) if valid, Err(DocumentError) if invalid
pub fn validate_position(position: usize, length: usize) -> Result<(), DocumentError> {
    if position > length {
        return Err(DocumentError::out_of_bounds(position, length));
    }
    Ok(())
}

/// Validates a range against document length
///
/// # Arguments
/// * `start` - The start position of the range
/// * `end` - The end position of the range
/// * `length` - The current document length
///
/// # Returns
/// Ok(()) if valid, Err(DocumentError) if invalid
pub fn validate_range(start: usize, end: usize, length: usize) -> Result<(), DocumentError> {
    // Check if start is valid
    if start > length {
        return Err(DocumentError::out_of_bounds(start, length));
    }

    // Check if end is valid
    if end > length {
        return Err(DocumentError::invalid_range(start, end, length));
    }

    // Note: We allow start > end as ranges can be normalized
    Ok(())
}

/// Validates that a range is not empty (collapsed)
///
/// # Arguments
/// * `range` - The range to validate
///
/// # Returns
/// Ok(()) if non-empty, Err(DocumentError) if empty
pub fn validate_non_empty_range(range: &Range) -> Result<(), DocumentError> {
    let normalized = range.normalize();
    if normalized.start == normalized.end {
        return Err(DocumentError::zero_length_range(normalized.start.offset()));
    }
    Ok(())
}

/// Validates that a document is not empty
///
/// # Arguments
/// * `length` - The current document length
///
/// # Returns
/// Ok(()) if non-empty, Err(DocumentError) if empty
pub fn validate_non_empty_document(length: usize) -> Result<(), DocumentError> {
    if length == 0 {
        return Err(DocumentError::EmptyDocument);
    }
    Ok(())
}

/// Validates that an operation won't exceed maximum document size
///
/// # Arguments
/// * `current_size` - Current document size in characters
/// * `additional_size` - Size to be added
///
/// # Returns
/// Ok(()) if within limits, Err(DocumentError) if would exceed
pub fn validate_size_limit(
    current_size: usize,
    additional_size: usize,
) -> Result<(), DocumentError> {
    let new_size = current_size.saturating_add(additional_size);
    if new_size > MAX_DOCUMENT_SIZE {
        return Err(DocumentError::size_limit_exceeded(
            current_size,
            new_size,
            MAX_DOCUMENT_SIZE,
        ));
    }
    Ok(())
}

/// Validates text content for invalid characters
///
/// # Arguments
/// * `text` - The text to validate
///
/// # Returns
/// Ok(()) if valid, Err(DocumentError) if invalid
pub fn validate_text_content(text: &str) -> Result<(), DocumentError> {
    // Check for null bytes
    if text.contains('\0') {
        return Err(DocumentError::invalid_text("contains null bytes"));
    }

    // Check for other control characters that might cause issues
    // Allow common whitespace: space, tab, newline, carriage return
    for ch in text.chars() {
        if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
            return Err(DocumentError::invalid_text(format!(
                "contains invalid control character: U+{:04X}",
                ch as u32
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_position_valid() {
        assert!(validate_position(0, 10).is_ok());
        assert!(validate_position(5, 10).is_ok());
        assert!(validate_position(10, 10).is_ok());
    }

    #[test]
    fn test_validate_position_invalid() {
        let result = validate_position(11, 10);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DocumentError::OutOfBounds { .. }
        ));
    }

    #[test]
    fn test_validate_range_valid() {
        assert!(validate_range(0, 5, 10).is_ok());
        assert!(validate_range(5, 10, 10).is_ok());
        assert!(validate_range(0, 10, 10).is_ok());
        // Backward ranges are allowed (will be normalized)
        assert!(validate_range(5, 0, 10).is_ok());
    }

    #[test]
    fn test_validate_range_invalid_start() {
        let result = validate_range(11, 15, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_range_invalid_end() {
        let result = validate_range(0, 15, 10);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DocumentError::InvalidRange { .. }
        ));
    }

    #[test]
    fn test_validate_non_empty_range() {
        let range = Range::from_offsets(0, 5);
        assert!(validate_non_empty_range(&range).is_ok());

        let empty_range = Range::from_offsets(5, 5);
        let result = validate_non_empty_range(&empty_range);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DocumentError::ZeroLengthRange { .. }
        ));
    }

    #[test]
    fn test_validate_non_empty_document() {
        assert!(validate_non_empty_document(10).is_ok());

        let result = validate_non_empty_document(0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DocumentError::EmptyDocument));
    }

    #[test]
    fn test_validate_size_limit() {
        assert!(validate_size_limit(1000, 1000).is_ok());
        assert!(validate_size_limit(MAX_DOCUMENT_SIZE - 100, 50).is_ok());

        let result = validate_size_limit(MAX_DOCUMENT_SIZE - 100, 200);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DocumentError::SizeLimitExceeded { .. }
        ));
    }

    #[test]
    fn test_validate_text_content_valid() {
        assert!(validate_text_content("Hello World").is_ok());
        assert!(validate_text_content("Line 1\nLine 2").is_ok());
        assert!(validate_text_content("Tab\there").is_ok());
        assert!(validate_text_content("Unicode: 你好 🎉").is_ok());
    }

    #[test]
    fn test_validate_text_content_null_byte() {
        let result = validate_text_content("Hello\0World");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, DocumentError::InvalidText { .. }));
        assert!(err.to_string().contains("null bytes"));
    }

    #[test]
    fn test_validate_text_content_control_chars() {
        // Bell character (U+0007)
        let result = validate_text_content("Hello\x07World");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, DocumentError::InvalidText { .. }));
        assert!(err.to_string().contains("control character"));
    }

    #[test]
    fn test_validate_text_content_allowed_whitespace() {
        // These should all be allowed
        assert!(validate_text_content(" ").is_ok()); // space
        assert!(validate_text_content("\t").is_ok()); // tab
        assert!(validate_text_content("\n").is_ok()); // newline
        assert!(validate_text_content("\r").is_ok()); // carriage return
        assert!(validate_text_content("\r\n").is_ok()); // CRLF
    }
}
