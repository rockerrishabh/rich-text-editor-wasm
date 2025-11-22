//! Serialization module
//!
//! This module handles import and export of documents in multiple formats.
//! It provides serialization to and from JSON, Markdown, and HTML.
//!
//! # Responsibilities
//!
//! - Serialize documents to JSON with full fidelity
//! - Deserialize documents from JSON
//! - Export documents to Markdown format
//! - Import documents from Markdown
//! - Export documents to HTML with proper escaping
//! - Import documents from HTML with sanitization
//! - Prevent XSS vulnerabilities in HTML import
//!
//! # Key Types
//!
//! - JSON serialization/deserialization functions
//! - Markdown export/import functions
//! - HTML export/import functions with sanitization
//! - `HtmlSanitizer`: Prevents XSS attacks in HTML import
//! - `SerializationError`: Comprehensive error type for all serialization operations

pub mod errors;
pub mod html;
pub mod json;
pub mod markdown;

// Re-export error types
pub use errors::SerializationError;
