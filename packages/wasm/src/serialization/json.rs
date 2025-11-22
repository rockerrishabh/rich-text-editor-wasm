use crate::document::{Document, Range};
use crate::formatting::{BlockType, InlineFormat};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during JSON serialization/deserialization
#[derive(Debug, Error)]
pub enum JsonError {
    #[error("JSON parsing error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Unsupported schema version: {0}")]
    UnsupportedVersion(String),

    #[error("Invalid format data: {0}")]
    InvalidFormat(String),

    #[error("Invalid block data: {0}")]
    InvalidBlock(String),
}

/// Serializable representation of a format run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableFormatRun {
    pub start: usize,
    pub end: usize,
    pub formats: Vec<InlineFormat>,
}

/// Serializable representation of a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableBlock {
    pub start: usize,
    pub block_type: BlockType,
}

/// Serializable representation of document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
}

/// Serializable representation of a complete document
///
/// # JSON Format Structure
///
/// The document is serialized to JSON with the following structure:
///
/// ```json
/// {
///   "version": "1.0",
///   "text": "Document text content",
///   "content": "Document text content",
///   "formats": [
///     {
///       "start": 0,
///       "end": 5,
///       "formats": ["Bold", {"Link": {"url": "https://example.com"}}]
///     }
///   ],
///   "blocks": [
///     {
///       "start": 0,
///       "block_type": "Paragraph"
///     }
///   ],
///   "metadata": {
///     "created": "2024-01-01T00:00:00Z",
///     "modified": "2024-01-01T00:00:00Z"
///   }
/// }
/// ```
///
/// ## Version History
///
/// - **1.0**: Initial format with text/content, formats, blocks, and optional metadata
///
/// ## Format Types
///
/// Inline formats can be:
/// - Simple: `"Bold"`, `"Italic"`, `"Underline"`, `"Strikethrough"`, `"Code"`
/// - Complex: `{"Link": {"url": "..."}}`, `{"TextColor": {"color": "..."}}`, `{"BackgroundColor": {"color": "..."}}`
///
/// Block types can be:
/// - `"Paragraph"`
/// - `{"Heading": {"level": 1-6}}`
/// - `"BulletList"`
/// - `"NumberedList"`
/// - `"BlockQuote"`
/// - `"CodeBlock"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableDocument {
    pub version: String,
    #[serde(alias = "content")]
    pub text: String,
    pub formats: Vec<SerializableFormatRun>,
    pub blocks: Vec<SerializableBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DocumentMetadata>,
}

/// Migrates a document from an older version to the current version
///
/// This function handles backward compatibility by upgrading documents
/// serialized with older schema versions to the current format.
///
/// # Arguments
///
/// * `doc` - A SerializableDocument with any supported version
///
/// # Returns
///
/// A SerializableDocument upgraded to version 1.0
///
/// # Errors
///
/// Returns an error if the version is not supported or migration fails
fn migrate_document_version(doc: SerializableDocument) -> Result<SerializableDocument, JsonError> {
    match doc.version.as_str() {
        "1.0" => {
            // Already current version, no migration needed
            Ok(doc)
        }
        // Future versions would be handled here
        // "0.9" => migrate_from_0_9_to_1_0(doc),
        // "0.8" => migrate_from_0_8_to_1_0(doc),
        version => Err(JsonError::UnsupportedVersion(version.to_string())),
    }
}

// Example migration function for future use:
// fn migrate_from_0_9_to_1_0(mut doc: SerializableDocument) -> Result<SerializableDocument, JsonError> {
//     // Perform any necessary transformations
//     doc.version = "1.0".to_string();
//     Ok(doc)
// }

impl Document {
    /// Serializes the document to JSON format
    ///
    /// # Returns
    ///
    /// A JSON string representation of the document including version, content,
    /// formats, blocks, and metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization fails.
    pub fn to_json(&self) -> Result<String, JsonError> {
        // Convert format runs to serializable format
        let formats: Vec<SerializableFormatRun> = self
            .formats()
            .get_runs()
            .iter()
            .map(|run| {
                let normalized = run.range.normalize();
                SerializableFormatRun {
                    start: normalized.start_offset(),
                    end: normalized.end_offset(),
                    formats: run.formats.iter().cloned().collect(),
                }
            })
            .collect();

        // Convert blocks to serializable format
        let blocks: Vec<SerializableBlock> = self
            .formats()
            .get_blocks()
            .iter()
            .map(|block| SerializableBlock {
                start: block.start_offset,
                block_type: block.block_type.clone(),
            })
            .collect();

        // Create serializable document
        let serializable = SerializableDocument {
            version: "1.0".to_string(),
            text: self.get_content(),
            formats,
            blocks,
            metadata: None, // Can be extended in the future
        };

        Ok(serde_json::to_string(&serializable)?)
    }

    /// Serializes the document to pretty-printed JSON format
    ///
    /// # Returns
    ///
    /// A formatted JSON string representation of the document.
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization fails.
    pub fn to_json_pretty(&self) -> Result<String, JsonError> {
        // Convert format runs to serializable format
        let formats: Vec<SerializableFormatRun> = self
            .formats()
            .get_runs()
            .iter()
            .map(|run| {
                let normalized = run.range.normalize();
                SerializableFormatRun {
                    start: normalized.start_offset(),
                    end: normalized.end_offset(),
                    formats: run.formats.iter().cloned().collect(),
                }
            })
            .collect();

        // Convert blocks to serializable format
        let blocks: Vec<SerializableBlock> = self
            .formats()
            .get_blocks()
            .iter()
            .map(|block| SerializableBlock {
                start: block.start_offset,
                block_type: block.block_type.clone(),
            })
            .collect();

        // Create serializable document
        let serializable = SerializableDocument {
            version: "1.0".to_string(),
            text: self.get_content(),
            formats,
            blocks,
            metadata: None,
        };

        Ok(serde_json::to_string_pretty(&serializable)?)
    }

    /// Deserializes a document from JSON format with version migration support
    ///
    /// This method automatically handles version migration, allowing documents
    /// serialized with older versions to be loaded and upgraded to the current format.
    ///
    /// # Arguments
    ///
    /// * `json` - A JSON string representation of a document
    ///
    /// # Returns
    ///
    /// A reconstructed Document with all content and formatting.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON parsing fails
    /// - Schema version is unsupported
    /// - Format or block data is invalid
    /// - Migration fails
    ///
    /// # Supported Versions
    ///
    /// - **1.0**: Current version with full feature support
    ///
    /// # Example
    ///
    /// ```
    /// use rich_text_editor_wasm::document::Document;
    ///
    /// let json = r#"{"version":"1.0","content":"Hello","formats":[],"blocks":[]}"#;
    /// let doc = Document::from_json(json).unwrap();
    /// assert_eq!(doc.get_content(), "Hello");
    /// ```
    pub fn from_json(json: &str) -> Result<Self, JsonError> {
        // Parse JSON into serializable structure
        let mut serializable: SerializableDocument = serde_json::from_str(json)?;

        // Perform version migration if needed
        serializable = migrate_document_version(serializable)?;

        // Validate schema version after migration
        if serializable.version != "1.0" {
            return Err(JsonError::UnsupportedVersion(serializable.version));
        }

        // Create document with content (support both text and content fields)
        let text_content = &serializable.text;
        let mut doc = Document::from_text(text_content);

        // Restore format runs
        for format_run in serializable.formats {
            // Validate range
            if format_run.start > format_run.end {
                return Err(JsonError::InvalidFormat(format!(
                    "Invalid range: start {} > end {}",
                    format_run.start, format_run.end
                )));
            }

            if format_run.end > doc.get_length() {
                return Err(JsonError::InvalidFormat(format!(
                    "Range end {} exceeds document length {}",
                    format_run.end,
                    doc.get_length()
                )));
            }

            // Apply each format in the run
            let range = Range::from_offsets(format_run.start, format_run.end);
            for format in format_run.formats {
                doc.apply_format(range, format);
            }
        }

        // Restore block types
        for block in serializable.blocks {
            // Validate block position
            if block.start > doc.get_length() {
                return Err(JsonError::InvalidBlock(format!(
                    "Block start {} exceeds document length {}",
                    block.start,
                    doc.get_length()
                )));
            }

            // Find the end of this block (start of next block or document end)
            let block_end = doc.get_length();
            let range = Range::from_offsets(block.start, block_end);
            doc.set_block_type(range, block.block_type);
        }

        // Clear undo/redo history since this is a freshly loaded document
        doc.history.clear();

        Ok(doc)
    }
}

#[cfg(test)]
mod tests {
    use crate::document::Position;

    use super::*;

    #[test]
    fn test_to_json_empty_document() {
        let doc = Document::new();
        let json = doc.to_json().unwrap();

        // Parse to verify it's valid JSON
        let parsed: SerializableDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.version, "1.0");
        assert_eq!(parsed.text, "");
        assert_eq!(parsed.formats.len(), 0);
    }

    #[test]
    fn test_to_json_with_text() {
        let doc = Document::from_text("Hello World");
        let json = doc.to_json().unwrap();

        let parsed: SerializableDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.text, "Hello World");
    }

    #[test]
    fn test_to_json_with_formats() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.apply_format(Range::from_offsets(6, 11), InlineFormat::Italic);

        let json = doc.to_json().unwrap();
        let parsed: SerializableDocument = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.formats.len(), 2);

        // Find the bold format
        let bold_format = parsed
            .formats
            .iter()
            .find(|f| f.start == 0 && f.end == 5)
            .unwrap();
        assert!(bold_format.formats.contains(&InlineFormat::Bold));

        // Find the italic format
        let italic_format = parsed
            .formats
            .iter()
            .find(|f| f.start == 6 && f.end == 11)
            .unwrap();
        assert!(italic_format.formats.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_to_json_with_blocks() {
        let mut doc = Document::from_text("Heading\nParagraph");
        doc.set_block_type(Range::from_offsets(0, 7), BlockType::heading(1));

        let json = doc.to_json().unwrap();
        let parsed: SerializableDocument = serde_json::from_str(&json).unwrap();

        // Should have at least the heading block
        assert!(
            parsed
                .blocks
                .iter()
                .any(|b| b.block_type == BlockType::heading(1))
        );
    }

    #[test]
    fn test_to_json_pretty() {
        let doc = Document::from_text("Hello");
        let json = doc.to_json_pretty().unwrap();

        // Pretty JSON should contain newlines
        assert!(json.contains('\n'));

        // Should still be valid JSON
        let parsed: SerializableDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.text, "Hello");
    }

    #[test]
    fn test_to_json_with_multiple_formats_on_same_range() {
        let mut doc = Document::from_text("Hello");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Italic);

        let json = doc.to_json().unwrap();
        let parsed: SerializableDocument = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.formats.len(), 1);
        assert_eq!(parsed.formats[0].formats.len(), 2);
        assert!(parsed.formats[0].formats.contains(&InlineFormat::Bold));
        assert!(parsed.formats[0].formats.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_to_json_with_link_format() {
        let mut doc = Document::from_text("Click here");
        doc.apply_format(
            Range::from_offsets(0, 10),
            InlineFormat::Link {
                url: "https://example.com".to_string(),
            },
        );

        let json = doc.to_json().unwrap();
        let parsed: SerializableDocument = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.formats.len(), 1);
        assert!(parsed.formats[0].formats.contains(&InlineFormat::Link {
            url: "https://example.com".to_string()
        }));
    }

    #[test]
    fn test_to_json_with_colors() {
        let mut doc = Document::from_text("Colored text");
        doc.apply_format(
            Range::from_offsets(0, 12),
            InlineFormat::TextColor {
                color: String::from("#FF0000"),
            },
        );
        doc.apply_format(
            Range::from_offsets(0, 12),
            InlineFormat::BackgroundColor {
                color: String::from("#FFFF00"),
            },
        );

        let json = doc.to_json().unwrap();
        let parsed: SerializableDocument = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.formats.len(), 1);
        assert_eq!(parsed.formats[0].formats.len(), 2);
    }

    #[test]
    fn test_from_json_empty_document() {
        let json = r#"{"version":"1.0","text":"","formats":[],"blocks":[{"start":0,"block_type":"Paragraph"}]}"#;
        let doc = Document::from_json(json).unwrap();

        assert_eq!(doc.get_content(), "");
        assert_eq!(doc.get_length(), 0);
    }

    #[test]
    fn test_from_json_with_text() {
        let json = r#"{"version":"1.0","text":"Hello World","formats":[],"blocks":[{"start":0,"block_type":"Paragraph"}]}"#;
        let doc = Document::from_json(json).unwrap();

        assert_eq!(doc.get_content(), "Hello World");
        assert_eq!(doc.get_length(), 11);
    }

    #[test]
    fn test_from_json_with_formats() {
        let json = r#"{
            "version":"1.0",
            "text":"Hello World",
            "formats":[
                {"start":0,"end":5,"formats":["Bold"]},
                {"start":6,"end":11,"formats":["Italic"]}
            ],
            "blocks":[{"start":0,"block_type":"Paragraph"}]
        }"#;
        let doc = Document::from_json(json).unwrap();

        assert_eq!(doc.get_content(), "Hello World");

        // Check bold format
        let formats_at_2 = doc.get_formats_at(Position::new(2));
        assert!(formats_at_2.contains(&InlineFormat::Bold));

        // Check italic format
        let formats_at_8 = doc.get_formats_at(Position::new(8));
        assert!(formats_at_8.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_from_json_with_blocks() {
        let json = r#"{
            "version":"1.0",
            "text":"Heading",
            "formats":[],
            "blocks":[{"start":0,"block_type":{"Heading":{"level":1}}}]
        }"#;
        let doc = Document::from_json(json).unwrap();

        assert_eq!(doc.get_content(), "Heading");
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::heading(1)
        );
    }

    #[test]
    fn test_from_json_unsupported_version() {
        let json = r#"{"version":"2.0","text":"Hello","formats":[],"blocks":[]}"#;
        let result = Document::from_json(json);

        assert!(result.is_err());
        match result {
            Err(JsonError::UnsupportedVersion(v)) => assert_eq!(v, "2.0"),
            _ => panic!("Expected UnsupportedVersion error"),
        }
    }

    #[test]
    fn test_from_json_invalid_range() {
        let json = r#"{
            "version":"1.0",
            "text":"Hello",
            "formats":[{"start":10,"end":5,"formats":["Bold"]}],
            "blocks":[]
        }"#;
        let result = Document::from_json(json);

        assert!(result.is_err());
        match result {
            Err(JsonError::InvalidFormat(_)) => {}
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_from_json_range_exceeds_length() {
        let json = r#"{
            "version":"1.0",
            "text":"Hello",
            "formats":[{"start":0,"end":100,"formats":["Bold"]}],
            "blocks":[]
        }"#;
        let result = Document::from_json(json);

        assert!(result.is_err());
        match result {
            Err(JsonError::InvalidFormat(_)) => {}
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_from_json_invalid_block_position() {
        let json = r#"{
            "version":"1.0",
            "text":"Hello",
            "formats":[],
            "blocks":[{"start":100,"block_type":"Paragraph"}]
        }"#;
        let result = Document::from_json(json);

        assert!(result.is_err());
        match result {
            Err(JsonError::InvalidBlock(_)) => {}
            _ => panic!("Expected InvalidBlock error"),
        }
    }

    #[test]
    fn test_roundtrip_serialization() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.apply_format(Range::from_offsets(6, 11), InlineFormat::Italic);
        doc.set_block_type(Range::from_offsets(0, 11), BlockType::heading(2));

        // Serialize
        let json = doc.to_json().unwrap();

        // Deserialize
        let restored = Document::from_json(&json).unwrap();

        // Verify content
        assert_eq!(restored.get_content(), "Hello World");

        // Verify formats
        let formats_at_2 = restored.get_formats_at(Position::new(2));
        assert!(formats_at_2.contains(&InlineFormat::Bold));

        let formats_at_8 = restored.get_formats_at(Position::new(8));
        assert!(formats_at_8.contains(&InlineFormat::Italic));

        // Verify block type
        assert_eq!(
            restored.get_block_type_at(Position::new(0)),
            BlockType::heading(2)
        );
    }

    #[test]
    fn test_from_json_with_link() {
        let json = r#"{
            "version":"1.0",
            "text":"Click here",
            "formats":[{"start":0,"end":10,"formats":[{"Link":{"url":"https://example.com"}}]}],
            "blocks":[{"start":0,"block_type":"Paragraph"}]
        }"#;
        let doc = Document::from_json(json).unwrap();

        let formats = doc.get_formats_at(Position::new(5));
        assert!(formats.contains(&InlineFormat::Link {
            url: "https://example.com".to_string()
        }));
    }

    #[test]
    fn test_from_json_clears_history() {
        let json = r#"{"version":"1.0","text":"Hello","formats":[],"blocks":[{"start":0,"block_type":"Paragraph"}]}"#;
        let doc = Document::from_json(json).unwrap();

        // History should be empty for a freshly loaded document
        assert!(!doc.can_undo());
        assert!(!doc.can_redo());
    }

    #[test]
    fn test_from_json_invalid_json() {
        let json = r#"{"invalid json"#;
        let result = Document::from_json(json);

        assert!(result.is_err());
        match result {
            Err(JsonError::ParseError(_)) => {}
            _ => panic!("Expected ParseError"),
        }
    }

    // Versioned serialization tests

    #[test]
    fn test_json_version_field_present() {
        let doc = Document::from_text("Hello");
        let json = doc.to_json().unwrap();

        let parsed: SerializableDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.version, "1.0");
    }

    #[test]
    fn test_json_version_in_output() {
        let doc = Document::from_text("Test");
        let json = doc.to_json().unwrap();

        // Verify version field is in the JSON string
        assert!(json.contains("\"version\":\"1.0\""));
    }

    #[test]
    fn test_from_json_current_version() {
        let json = r#"{
            "version":"1.0",
            "text":"Hello World",
            "formats":[],
            "blocks":[{"start":0,"block_type":"Paragraph"}]
        }"#;

        let doc = Document::from_json(json).unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_from_json_unsupported_future_version() {
        let json = r#"{
            "version":"2.0",
            "text":"Hello",
            "formats":[],
            "blocks":[]
        }"#;

        let result = Document::from_json(json);
        assert!(result.is_err());
        match result {
            Err(JsonError::UnsupportedVersion(v)) => assert_eq!(v, "2.0"),
            _ => panic!("Expected UnsupportedVersion error"),
        }
    }

    #[test]
    fn test_from_json_unsupported_old_version() {
        let json = r#"{
            "version":"0.9",
            "text":"Hello",
            "formats":[],
            "blocks":[]
        }"#;

        let result = Document::from_json(json);
        assert!(result.is_err());
        match result {
            Err(JsonError::UnsupportedVersion(v)) => assert_eq!(v, "0.9"),
            _ => panic!("Expected UnsupportedVersion error"),
        }
    }

    #[test]
    fn test_json_format_structure_documentation() {
        // This test verifies the documented JSON structure is correct
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.apply_format(
            Range::from_offsets(6, 11),
            InlineFormat::Link {
                url: "https://example.com".to_string(),
            },
        );
        doc.set_block_type(Range::from_offsets(0, 11), BlockType::heading(1));

        let json = doc.to_json_pretty().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify structure matches documentation
        assert!(parsed.get("version").is_some());
        assert!(parsed.get("text").is_some());
        assert!(parsed.get("formats").is_some());
        assert!(parsed.get("blocks").is_some());

        // Verify version is string
        assert!(parsed["version"].is_string());
        assert_eq!(parsed["version"], "1.0");

        // Verify text is string
        assert!(parsed["text"].is_string());

        // Verify formats is array
        assert!(parsed["formats"].is_array());

        // Verify blocks is array
        assert!(parsed["blocks"].is_array());
    }

    #[test]
    fn test_json_format_run_structure() {
        let mut doc = Document::from_text("Hello");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);

        let json = doc.to_json().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        let formats = parsed["formats"].as_array().unwrap();
        assert_eq!(formats.len(), 1);

        let format_run = &formats[0];
        assert!(format_run.get("start").is_some());
        assert!(format_run.get("end").is_some());
        assert!(format_run.get("formats").is_some());

        assert_eq!(format_run["start"], 0);
        assert_eq!(format_run["end"], 5);
        assert!(format_run["formats"].is_array());
    }

    #[test]
    fn test_json_block_structure() {
        let mut doc = Document::from_text("Heading");
        doc.set_block_type(Range::from_offsets(0, 7), BlockType::heading(2));

        let json = doc.to_json().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        let blocks = parsed["blocks"].as_array().unwrap();
        assert!(!blocks.is_empty());

        let block = &blocks[0];
        assert!(block.get("start").is_some());
        assert!(block.get("block_type").is_some());
    }

    #[test]
    fn test_json_complex_format_structure() {
        let mut doc = Document::from_text("Link");
        doc.apply_format(
            Range::from_offsets(0, 4),
            InlineFormat::Link {
                url: "https://example.com".to_string(),
            },
        );

        let json = doc.to_json().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        let formats = parsed["formats"].as_array().unwrap();
        let format_run = &formats[0];
        let format_list = format_run["formats"].as_array().unwrap();

        // Link format should be an object with url field
        assert!(format_list[0].is_object());
        assert!(format_list[0].get("Link").is_some());
        assert!(format_list[0]["Link"].get("url").is_some());
    }

    #[test]
    fn test_json_metadata_optional() {
        let doc = Document::from_text("Test");
        let json = doc.to_json().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Metadata should be optional (not present by default)
        // If present, it should be null or an object
        if let Some(metadata) = parsed.get("metadata") {
            assert!(metadata.is_null() || metadata.is_object());
        }
    }

    #[test]
    fn test_roundtrip_preserves_version() {
        let mut doc = Document::from_text("Hello");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);

        let json = doc.to_json().unwrap();
        let restored = Document::from_json(&json).unwrap();

        // Verify content is preserved
        assert_eq!(restored.get_content(), "Hello");

        // Verify formatting is preserved
        let formats = restored.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));

        // Verify we can serialize again with same version
        let json2 = restored.to_json().unwrap();
        let parsed: SerializableDocument = serde_json::from_str(&json2).unwrap();
        assert_eq!(parsed.version, "1.0");
    }

    #[test]
    fn test_migration_function_current_version() {
        let doc = SerializableDocument {
            version: "1.0".to_string(),
            text: "Test".to_string(),
            formats: vec![],
            blocks: vec![],
            metadata: None,
        };

        let migrated = migrate_document_version(doc).unwrap();
        assert_eq!(migrated.version, "1.0");
        assert_eq!(migrated.text, "Test");
    }

    #[test]
    fn test_migration_function_unsupported_version() {
        let doc = SerializableDocument {
            version: "0.5".to_string(),
            text: "Test".to_string(),
            formats: vec![],
            blocks: vec![],
            metadata: None,
        };

        let result = migrate_document_version(doc);
        assert!(result.is_err());
        match result {
            Err(JsonError::UnsupportedVersion(v)) => assert_eq!(v, "0.5"),
            _ => panic!("Expected UnsupportedVersion error"),
        }
    }
}
